use anyhow::{anyhow, Result};
use pandoc::{InputFormat, InputKind, OutputFormat, OutputKind, Pandoc, PandocOption};
use serde::{Deserialize, Serialize};
use simply_colored::*;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

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
    Vide,
    Couverture,
    AvantPropos,
    Dedicace,
    Epigraphe,
    FauxTitre,
    Ours,
    Preface,
    Titre,
}

#[derive(Debug)]
enum FilterType {
    PageBreak,
    Title,
}

#[derive(Clone, Debug, Deserialize)]
struct Metadata {
    filename: String,
    cover: String,
    titre: String,
    auteur: String,
    collection: String,
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
    annee_copyright: String,
    date_depot_legal: String,
    editeur: String,
    resume: String,
    preface: String,
    avant_propos: String,
    dedicace: String,
    epigraphe: String,
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

fn main() -> Result<()> {
    let books_metadata = glob::glob("./**/metadata.yml")?;

    for metadata_path in books_metadata.flatten() {
        if let Some(book_path) = metadata_path.parent() {
            process_book(book_path, metadata_path.as_path())?
        }
    }

    Ok(())
}

fn process_book(book_path: &Path, metadata_path: &Path) -> Result<()> {
    println!("{BG_MAGENTA}{BOLD} {} {RESET}", book_path.display());

    // Read metadata of the book
    let metadata: Metadata = read_yaml_as(&metadata_path)?;

    println!("{BOLD}  Filename{RESET} {}", metadata.filename);
    println!("{BOLD}  Cover   {RESET} {}", metadata.cover);
    println!("{BOLD}  Titre   {RESET} {}", metadata.titre);
    println!("{BOLD}  Auteur  {RESET} {}", metadata.auteur);

    // Get path to the cover image
    let cover_path = book_path.join(&metadata.cover);

    if !fs::exists(&cover_path)? {
        return Err(anyhow!("Missing cover: {:?}", cover_path));
    }

    // Get editions manifests
    let manifests = glob::glob(&format!("{}/**/manifest.yml", book_path.display()))?;

    for manifest_path in manifests.flatten() {
        println!(
            "  {BG_BLUE}{BOLD} {} {RESET}",
            manifest_path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );

        let edition: Edition = read_yaml_as(&manifest_path)?;
        println!("{BOLD}    Type        {RESET} {:?}", edition.r#type);
        println!("{BOLD}    ISBN        {RESET} {}", edition.isbn);
        println!("{BOLD}    Resume      {RESET} {}", is_set(&edition.resume));
        println!("{BOLD}    Preface     {RESET} {}", is_set(&edition.preface));
        println!(
            "{BOLD}    Avant-propos{RESET} {}",
            is_set(&edition.avant_propos)
        );
        println!(
            "{BOLD}    Dedicace    {RESET} {}",
            is_set(&edition.dedicace)
        );
        println!(
            "{BOLD}    Epigraphe   {RESET} {}",
            is_set(&edition.epigraphe)
        );

        match edition.r#type {
            EditionType::Epub => {
                make_epub(PandocInputs {
                    book_path: book_path.to_path_buf(),
                    edition_path: manifest_path.parent().unwrap().to_path_buf(),
                    cover_path: cover_path.clone(),
                    metadata: metadata.clone(),
                    edition,
                })?;
            }

            EditionType::Pdf => {
                make_pdf(PandocInputs {
                    book_path: book_path.to_path_buf(),
                    edition_path: manifest_path.parent().unwrap().to_path_buf(),
                    cover_path: cover_path.clone(),
                    metadata: metadata.clone(),
                    edition,
                })?;
            }
        }
    }

    Ok(())
}

fn make_epub(inputs: PandocInputs) -> Result<()> {
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

    // Create filters
    let f_pagebreak = make_filter(FilterType::PageBreak, &inputs, &output_path)?;

    // Create metadata files
    for r#type in [MetadataType::Epub, MetadataType::Date] {
        let _meta = make_metadata(r#type, &inputs, &output_path)?;
        // pandoc_inputs.push(meta);
    }

    // Create pages
    let page_vide = make_page(PageType::Vide, &inputs, &output_path)?;
    let page_couverture = make_page(PageType::Couverture, &inputs, &output_path)?;
    let page_avant_propos = make_page(PageType::AvantPropos, &inputs, &output_path)?;
    let page_dedicace = make_page(PageType::Dedicace, &inputs, &output_path)?;
    let page_epigraphe = make_page(PageType::Epigraphe, &inputs, &output_path)?;
    let page_faux_titre = make_page(PageType::FauxTitre, &inputs, &output_path)?;
    let page_ours = make_page(PageType::Ours, &inputs, &output_path)?;
    let page_preface = make_page(PageType::Preface, &inputs, &output_path)?;
    let page_titre = make_page(PageType::Titre, &inputs, &output_path)?;

    for page in inputs.edition.pages {
        match page {
            PageType::Couverture => pandoc_inputs.push(page_couverture.clone()),
            PageType::Vide => pandoc_inputs.push(page_vide.clone()),
            PageType::AvantPropos => pandoc_inputs.push(page_avant_propos.clone()),
            PageType::Dedicace => pandoc_inputs.push(page_dedicace.clone()),
            PageType::Epigraphe => pandoc_inputs.push(page_epigraphe.clone()),
            PageType::FauxTitre => pandoc_inputs.push(page_faux_titre.clone()),
            PageType::Ours => pandoc_inputs.push(page_ours.clone()),
            PageType::Preface => pandoc_inputs.push(page_preface.clone()),
            PageType::Titre => pandoc_inputs.push(page_titre.clone()),
        }
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
        .arg("lua-filter", &f_pagebreak.display().to_string())
        .arg("split-level", "1")
        .clone()
        .execute()?;

    println!("{YELLOW}{BOLD}    {} {RESET}", epub_path.display());

    Ok(())
}

fn make_pdf(inputs: PandocInputs) -> Result<()> {
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

    // Create CSS options
    let css_options = make_css_options(&inputs)?;

    // Create filters
    let f_pagebreak = make_filter(FilterType::PageBreak, &inputs, &output_path)?;
    let f_title = make_filter(FilterType::Title, &inputs, &output_path)?;

    // Create metadata files
    for r#type in [MetadataType::Pdf, MetadataType::Date] {
        let meta = make_metadata(r#type, &inputs, &output_path)?;
        pandoc_inputs.push(meta);
    }

    // Create pages
    let page_vide = make_page(PageType::Vide, &inputs, &output_path)?;
    let page_couverture = make_page(PageType::Couverture, &inputs, &output_path)?;
    let page_avant_propos = make_page(PageType::AvantPropos, &inputs, &output_path)?;
    let page_dedicace = make_page(PageType::Dedicace, &inputs, &output_path)?;
    let page_epigraphe = make_page(PageType::Epigraphe, &inputs, &output_path)?;
    let page_faux_titre = make_page(PageType::FauxTitre, &inputs, &output_path)?;
    let page_ours = make_page(PageType::Ours, &inputs, &output_path)?;
    let page_preface = make_page(PageType::Preface, &inputs, &output_path)?;
    let page_titre = make_page(PageType::Titre, &inputs, &output_path)?;

    for page in inputs.edition.pages {
        match page {
            PageType::Couverture => pandoc_inputs.push(page_couverture.clone()),
            PageType::AvantPropos => pandoc_inputs.push(page_avant_propos.clone()),
            PageType::Vide => pandoc_inputs.push(page_vide.clone()),
            PageType::Dedicace => pandoc_inputs.push(page_dedicace.clone()),
            PageType::Epigraphe => pandoc_inputs.push(page_epigraphe.clone()),
            PageType::FauxTitre => pandoc_inputs.push(page_faux_titre.clone()),
            PageType::Ours => pandoc_inputs.push(page_ours.clone()),
            PageType::Preface => pandoc_inputs.push(page_preface.clone()),
            PageType::Titre => pandoc_inputs.push(page_titre.clone()),
        }
    }

    // Collect chapters
    let chapters_path = book_path.join("chapters");

    let mut chapters: Vec<_> = glob::glob(&format!("{}/**/*.md", chapters_path.display()))?
        .filter_map(|p| p.ok())
        .collect();

    pandoc_inputs.append(&mut chapters);

    // Create the PDF file using Pandoc
    Pandoc::new()
        // .set_show_cmdline(true)
        .set_input(InputKind::Files(pandoc_inputs))
        .set_input_format(InputFormat::Markdown, vec![])
        .set_output(OutputKind::File(pdf_path.clone()))
        .set_output_format(OutputFormat::Pdf, vec![])
        .add_options(&css_options)
        .arg("lua-filter", &f_pagebreak.display().to_string())
        .arg("lua-filter", &f_title.display().to_string())
        // .arg("split-level", "1")
        .clone()
        .execute()?;

    println!("{YELLOW}{BOLD}    {} {RESET}", pdf_path.display());

    Ok(())
}

fn make_css_options(inputs: &PandocInputs) -> Result<Vec<PandocOption>> {
    let opts = match inputs.edition.r#type {
        EditionType::Epub => {
            let content = include_str!("../css/epub/style.css");

            let mut file = std::fs::File::create("/tmp/koob_epub_style.css")?;
            file.write_all(content.as_bytes())?;

            vec![PandocOption::Css("/tmp/koob_epub_style.css".to_string())]
        }

        EditionType::Pdf => {
            let content = include_str!("../css/pdf/style.css");

            let mut file = std::fs::File::create("/tmp/koob_pdf_style.css")?;
            file.write_all(content.as_bytes())?;

            vec![PandocOption::Css("/tmp/koob_pdf_style.css".to_string())]
        }
    };

    Ok(opts)
}

fn make_filter(r#type: FilterType, inputs: &PandocInputs, output_path: &Path) -> Result<PathBuf> {
    let (filepath, content) = match r#type {
        FilterType::PageBreak => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../filters/epub/pagebreak.lua"),
                EditionType::Pdf => include_str!("../filters/pdf/pagebreak.lua"),
            };

            (output_path.join("pagebreak.lua"), content)
        }

        FilterType::Title => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => "",
                EditionType::Pdf => include_str!("../filters/pdf/title.lua"),
            };

            (output_path.join("title.lua"), content)
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
) -> Result<PathBuf> {
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
                    text: inputs.metadata.titre.clone(),
                }],
                creator: vec![EpubCreatorMetadata {
                    role: "author".to_string(),
                    text: inputs.metadata.auteur.clone(),
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

fn make_page(r#type: PageType, inputs: &PandocInputs, output_path: &Path) -> Result<PathBuf> {
    let (filename, content) = match r#type {
        PageType::AvantPropos => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/avant_propos.md"),
                EditionType::Pdf => include_str!("../pages/pdf/avant_propos.md"),
            };

            ("avant_propos.md", content)
        }

        PageType::Vide => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/vide.md"),
                EditionType::Pdf => include_str!("../pages/pdf/vide.md"),
            };

            ("vide.md", content)
        }

        PageType::Couverture => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/couverture.md"),
                EditionType::Pdf => include_str!("../pages/pdf/couverture.md"),
            };

            ("couverture.md", content)
        }

        PageType::Dedicace => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/dedicace.md"),
                EditionType::Pdf => include_str!("../pages/pdf/dedicace.md"),
            };

            ("dedicace.md", content)
        }

        PageType::Epigraphe => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/epigraphe.md"),
                EditionType::Pdf => include_str!("../pages/pdf/epigraphe.md"),
            };

            ("epigraphe.md", content)
        }

        PageType::FauxTitre => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/faux_titre.md"),
                EditionType::Pdf => include_str!("../pages/pdf/faux_titre.md"),
            };

            ("faux_titre.md", content)
        }

        PageType::Ours => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/ours.md"),
                EditionType::Pdf => include_str!("../pages/pdf/ours.md"),
            };

            ("ours.md", content)
        }

        PageType::Preface => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/preface.md"),
                EditionType::Pdf => include_str!("../pages/pdf/preface.md"),
            };

            ("preface.md", content)
        }

        PageType::Titre => {
            let content = match inputs.edition.r#type {
                EditionType::Epub => include_str!("../pages/epub/titre.md"),
                EditionType::Pdf => include_str!("../pages/pdf/titre.md"),
            };

            ("titre.md", content)
        }
    };

    let filepath = output_path.join(filename);
    let content = apply_replacements(content, &inputs);

    std::fs::write(&filepath, content)?;

    println!("{DIM_BLUE}    {} {RESET}", filepath.display());

    Ok(filepath)
}

fn read_yaml_as<T, P: AsRef<Path>>(path: &P) -> Result<T>
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

fn apply_replacements(content: &str, inputs: &PandocInputs) -> String {
    let mut content = content
        .to_string()
        .replace("ANNEE_COPYRIGHT", &inputs.edition.annee_copyright)
        .replace("AUTEUR", &inputs.metadata.auteur)
        .replace("AVANT_PROPOS", &inputs.edition.avant_propos)
        .replace("COLLECTION", &inputs.metadata.collection)
        .replace("COVER_PATH", inputs.cover_path.to_str().unwrap())
        .replace("DATE_DEPOT_LEGAL", &inputs.edition.date_depot_legal)
        .replace("DEDICACE", &inputs.edition.dedicace)
        .replace("EDITEUR", &inputs.edition.editeur)
        .replace("EPIGRAPHE", &inputs.edition.epigraphe)
        .replace("ISBN_NUMBER", &inputs.edition.isbn)
        .replace("PREFACE", &inputs.edition.preface)
        .replace("TITRE", &inputs.metadata.titre);

    if inputs.edition.r#type == EditionType::Pdf {
        content = content.replace("_", "\\_");
    }

    content
}
