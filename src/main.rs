use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOption};
use serde::{Deserialize, Serialize};
use simply_colored::*;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Glob(#[from] glob::PatternError),

    #[error(transparent)]
    Handlebars(#[from] handlebars::RenderError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    MissingCover(PathBuf),

    #[error("No CSS for PDF")]
    NoCssForPdf,

    #[error(transparent)]
    Pandoc(#[from] pandoc::PandocError),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

#[derive(Debug)]
enum MetadataType {
    Date,
    Epub,
    #[allow(dead_code)]
    Pdf,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PageType {
    Copyright,
    Cover,
    Dedication,
    Empty,
    Epigraph,
    Foreword,
    HalfTitle,
    Preface,
    Title,
}

#[derive(Debug)]
enum FilterType {
    SpecialPages,
}

#[derive(Clone, Debug, Deserialize)]
struct Metadata {
    filename: String,
    cover: String,
    title: String,
    author: String,
    series: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EditionType {
    Epub,
    Pdf,
}

#[derive(Clone, Debug, Deserialize)]
struct Edition {
    #[serde(rename = "type")]
    r#type: EditionType,
    isbn: String,
    date: String,
    copyright_year: String,
    legal_deposit: String,
    publisher: String,
    summary: String,
    preface: String,
    foreword: String,
    dedication: String,
    epigraph: String,
    pages: Vec<PageType>,
}

#[derive(Clone, Debug)]
struct PandocInputs {
    book_path: PathBuf,
    edition_path: PathBuf,
    cover_path: PathBuf,
    metadata: Metadata,
    edition: Edition,
}

#[derive(Clone, Debug, Serialize)]
struct DateMetadata {
    date: String,
}

#[derive(Clone, Debug, Serialize)]
struct EpubMetadata {
    title: Vec<EpubTitleMetadata>,
    creator: Vec<EpubCreatorMetadata>,
    language: String,
}

#[derive(Clone, Debug, Serialize)]
struct EpubTitleMetadata {
    #[serde(rename = "type")]
    r#type: String,
    text: String,
}

#[derive(Clone, Debug, Serialize)]
struct EpubCreatorMetadata {
    role: String,
    text: String,
}

#[derive(Clone, Debug, Serialize)]
struct PdfMetadata {
    geometry: String,
}

#[derive(Clone, Debug, Serialize)]
struct Replacements {
    title: String,
    author: String,
    series: String,
    copyright_year: String,
    legal_deposit: String,
    isbn_number: String,
    cover_path: PathBuf,
    foreword: String,
    dedication: String,
    publisher: String,
    epigraph: String,
    preface: String,
}

impl From<PandocInputs> for Replacements {
    fn from(inputs: PandocInputs) -> Self {
        Replacements {
            title: inputs.metadata.title,
            author: inputs.metadata.author,
            series: inputs.metadata.series,
            copyright_year: inputs.edition.copyright_year,
            legal_deposit: inputs.edition.legal_deposit,
            isbn_number: inputs.edition.isbn,
            cover_path: inputs.cover_path,
            foreword: inputs.edition.foreword,
            dedication: inputs.edition.dedication,
            publisher: inputs.edition.publisher,
            epigraph: inputs.edition.epigraph,
            preface: inputs.edition.preface,
        }
    }
}

fn main() -> Result<(), Error> {
    let books_metadata = glob::glob("./**/book.yml")?;

    for metadata_path in books_metadata.flatten() {
        if let Some(book_path) = metadata_path.parent() {
            process_book(book_path, metadata_path.as_path())?
        }
    }

    Ok(())
}

fn process_book(book_path: &Path, metadata_path: &Path) -> Result<(), Error> {
    println!("{BLACK}{BG_MAGENTA}{BOLD} {} {RESET}", book_path.display());

    // Read metadata of the book
    let metadata: Metadata = read_yaml_as(&metadata_path)?;

    println!("{BOLD}  Filename{RESET} {}", metadata.filename);
    println!("{BOLD}  Cover   {RESET} {}", metadata.cover);
    println!("{BOLD}  Title   {RESET} {}", metadata.title);
    println!("{BOLD}  Author  {RESET} {}", metadata.author);

    // Get path to the cover image
    let cover_path = book_path.join(&metadata.cover);

    if !fs::exists(&cover_path)? {
        return Err(Error::MissingCover(cover_path));
    }

    // Get editions manifests
    let editions = glob::glob(&format!("{}/**/edition.yml", book_path.display()))?;

    for edition_path in editions.flatten() {
        println!(
            "  {BG_BLUE}{BOLD} {} {RESET}",
            edition_path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );

        let edition: Edition = read_yaml_as(&edition_path)?;
        println!("{BOLD}    Type        {RESET} {:?}", edition.r#type);
        println!("{BOLD}    ISBN        {RESET} {}", edition.isbn);
        println!("{BOLD}    Summary     {RESET} {}", is_set(&edition.summary));
        println!("{BOLD}    Preface     {RESET} {}", is_set(&edition.preface));
        println!(
            "{BOLD}    Foreword    {RESET} {}",
            is_set(&edition.foreword)
        );
        println!(
            "{BOLD}    Dedication  {RESET} {}",
            is_set(&edition.dedication)
        );
        println!(
            "{BOLD}    Epigraph    {RESET} {}",
            is_set(&edition.epigraph)
        );

        match edition.r#type {
            EditionType::Epub => {
                make_epub(PandocInputs {
                    book_path: book_path.to_path_buf(),
                    edition_path: edition_path.parent().unwrap().to_path_buf(),
                    cover_path: cover_path.clone(),
                    metadata: metadata.clone(),
                    edition,
                })?;
            }

            EditionType::Pdf => {
                make_pdf(PandocInputs {
                    book_path: book_path.to_path_buf(),
                    edition_path: edition_path.parent().unwrap().to_path_buf(),
                    cover_path: cover_path.clone(),
                    metadata: metadata.clone(),
                    edition,
                })?;
            }
        }
    }

    Ok(())
}

fn make_epub(inputs: PandocInputs) -> Result<(), Error> {
    let PandocInputs {
        book_path,
        edition_path,
        metadata,
        ..
    } = inputs.clone();

    let mut pandoc_inputs = Vec::new();

    // Create output directory and output file
    let output_path = edition_path.join("output");

    if !fs::exists(&output_path)? {
        fs::create_dir(&output_path)?;
    }

    let epub_path = output_path.join(format!("{}.epub", metadata.filename));

    // Create CSS options
    let css_options = make_css_options(&inputs)?;

    // Create metadata files
    for r#type in [MetadataType::Epub, MetadataType::Date] {
        let _meta = make_metadata(r#type, &inputs, &output_path)?;
        // pandoc_inputs.push(meta);
    }

    // Create pages
    for page in &inputs.edition.pages {
        pandoc_inputs.push(make_page(page, &inputs, &output_path)?)
    }

    // Collect chapters
    let chapters_path = book_path.join("chapters");

    let mut chapters: Vec<_> = glob::glob(&format!("{}/**/*.md", chapters_path.display()))?
        .filter_map(|p| p.ok())
        .collect();

    pandoc_inputs.append(&mut chapters);

    // Create the EPUB file using Pandoc
    Pandoc::new()
        // .set_show_cmdline(true)
        .set_input(InputKind::Files(pandoc_inputs))
        .set_input_format(InputFormat::Markdown, vec![])
        .set_output(OutputKind::File(epub_path.clone()))
        .set_output_format(OutputFormat::Epub, vec![])
        // .add_option(PandocOption::EpubCoverImage(cover_path))
        .add_options(&css_options)
        .arg("split-level", "1")
        .clone()
        .execute()?;

    println!("{YELLOW}{BOLD}    {} {RESET}", epub_path.display());

    Ok(())
}

fn make_pdf(inputs: PandocInputs) -> Result<(), Error> {
    let PandocInputs {
        book_path,
        edition_path,
        metadata,
        ..
    } = inputs.clone();

    let mut pandoc_inputs = Vec::new();

    // Create output directory and output file
    let output_path = edition_path.join("output");

    if !fs::exists(&output_path)? {
        fs::create_dir(&output_path)?;
    }

    let pdf_path = output_path.join(format!("{}.pdf", metadata.filename));

    // Create filters
    let f_special_pages = make_filter(FilterType::SpecialPages, &inputs, &output_path)?;

    // Create metadata files
    for r#type in [MetadataType::Pdf, MetadataType::Date] {
        let meta = make_metadata(r#type, &inputs, &output_path)?;
        pandoc_inputs.push(meta);
    }

    // Create pages
    for page in &inputs.edition.pages {
        pandoc_inputs.push(make_page(page, &inputs, &output_path)?)
    }

    // Collect chapters
    let chapters_path = book_path.join("chapters");

    let mut chapters: Vec<_> = glob::glob(&format!("{}/**/*.md", chapters_path.display()))?
        .filter_map(|p| p.ok())
        .collect();

    pandoc_inputs.append(&mut chapters);

    // Create the PDF file using Pandoc
    Pandoc::new()
        // Input/output
        .set_input(InputKind::Files(pandoc_inputs))
        .set_input_format(InputFormat::Markdown, vec![])
        .set_output(OutputKind::File(pdf_path.clone()))
        .set_output_format(OutputFormat::Pdf, vec![])
        // Typst engine
        .add_option(PandocOption::PdfEngine(PathBuf::from("typst")))
        .add_option(PandocOption::Template(PathBuf::from(
            "templates/template.typ",
        )))
        .arg("variable", "mainfont=EB Garamond")
        // Pandoc filters
        .arg("lua-filter", &f_special_pages.display().to_string())
        // .arg("split-level", "1")
        .clone()
        .execute()?;

    println!("{YELLOW}{BOLD}    {} {RESET}", pdf_path.display());

    Ok(())
}

fn make_css_options(inputs: &PandocInputs) -> Result<Vec<PandocOption>, Error> {
    match inputs.edition.r#type {
        EditionType::Epub => {
            let blitz_content = include_str!("../css/epub/blitz.css");
            let content = include_str!("../css/epub/style.css");

            let mut file = std::fs::File::create("/tmp/koob_epub_style.css")?;
            file.write_all(blitz_content.as_bytes())?;
            file.write_all(content.as_bytes())?;

            let opts = vec![PandocOption::Css("/tmp/koob_epub_style.css".to_string())];
            Ok(opts)
        }

        EditionType::Pdf => Err(Error::NoCssForPdf),
    }
}

fn make_filter(
    r#type: FilterType,
    inputs: &PandocInputs,
    output_path: &Path,
) -> Result<PathBuf, Error> {
    let (filepath, content) = match r#type {
        FilterType::SpecialPages => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => "",
                EditionType::Pdf => include_str!("../filters/pdf/special_pages.lua"),
            };

            (output_path.join("special_pages.lua"), content)
        }
    };

    std::fs::write(&filepath, content)?;

    println!("{DIM_RED}    {} {RESET}", filepath.display());

    Ok(filepath)
}

fn make_metadata(
    r#type: MetadataType,
    inputs: &PandocInputs,
    output_path: &Path,
) -> Result<PathBuf, Error> {
    let (filepath, content) = match r#type {
        MetadataType::Date => {
            let data = DateMetadata {
                date: inputs.edition.date.clone(),
            };

            (output_path.join("date.yml"), serde_yaml::to_string(&data)?)
        }

        MetadataType::Epub => {
            let data = EpubMetadata {
                title: vec![EpubTitleMetadata {
                    r#type: "main".to_string(),
                    text: inputs.metadata.title.clone(),
                }],
                creator: vec![EpubCreatorMetadata {
                    role: "author".to_string(),
                    text: inputs.metadata.author.clone(),
                }],
                language: "fr-FR".to_string(),
            };

            (output_path.join("epub.yml"), serde_yaml::to_string(&data)?)
        }

        MetadataType::Pdf => {
            let data = PdfMetadata {
                geometry: "left=2.5cm,right=2.5cm,top=2cm,bottom=2cm".to_string(),
            };

            (output_path.join("pdf.yml"), serde_yaml::to_string(&data)?)
        }
    };

    let yaml = format!("---\n{content}...");

    std::fs::write(&filepath, yaml)?;

    println!("{DIM_GREEN}    {} {RESET}", filepath.display());

    Ok(filepath)
}

fn make_page(
    r#type: &PageType,
    inputs: &PandocInputs,
    output_path: &Path,
) -> Result<PathBuf, Error> {
    let (filename, content) = match r#type {
        PageType::Foreword => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/foreword.md"),
                EditionType::Pdf => include_str!("../pages/pdf/foreword.md"),
            };

            ("foreword.md", content)
        }

        PageType::Empty => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/empty.md"),
                EditionType::Pdf => include_str!("../pages/pdf/empty.md"),
            };

            ("empty.md", content)
        }

        PageType::Cover => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/cover.md"),
                EditionType::Pdf => include_str!("../pages/pdf/cover.md"),
            };

            ("cover.md", content)
        }

        PageType::Dedication => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/dedication.md"),
                EditionType::Pdf => include_str!("../pages/pdf/dedication.md"),
            };

            ("dedication.md", content)
        }

        PageType::Epigraph => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/epigraph.md"),
                EditionType::Pdf => include_str!("../pages/pdf/epigraph.md"),
            };

            ("epigraph.md", content)
        }

        PageType::HalfTitle => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/half_title.md"),
                EditionType::Pdf => include_str!("../pages/pdf/half_title.md"),
            };

            ("half_title.md", content)
        }

        PageType::Copyright => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/copyright.md"),
                EditionType::Pdf => include_str!("../pages/pdf/copyright.md"),
            };

            ("copyright.md", content)
        }

        PageType::Preface => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/preface.md"),
                EditionType::Pdf => include_str!("../pages/pdf/preface.md"),
            };

            ("preface.md", content)
        }

        PageType::Title => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/title.md"),
                EditionType::Pdf => include_str!("../pages/pdf/title.md"),
            };

            ("title.md", content)
        }
    };

    let filepath = output_path.join(filename);
    let content = apply_replacements(content, inputs, inputs.clone().into())?;

    std::fs::write(&filepath, content)?;

    println!("{DIM_BLUE}    {} {RESET}", filepath.display());

    Ok(filepath)
}

fn read_yaml_as<T, P: AsRef<Path>>(path: &P) -> Result<T, Error>
where
    T: for<'a> Deserialize<'a>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value: T = serde_yaml::from_reader(reader)?;

    Ok(value)
}

fn is_set(value: &str) -> &str {
    if value.is_empty() {
        ""
    } else {
        "✅"
    }
}

fn apply_replacements(
    content: &str,
    inputs: &PandocInputs,
    replacements: Replacements,
) -> Result<String, Error> {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    let mut content = handlebars.render_template(content, &replacements)?;

    if inputs.edition.r#type == EditionType::Pdf {
        content = content.replace("_", "\\_");
    }

    Ok(content)
}
