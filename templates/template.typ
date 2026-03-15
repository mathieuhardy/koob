
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Variables
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let font-name = sys.inputs.at("mainfont", default: "EB Garamond")

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Empty page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-empty() = {
  pagebreak()
  page(numbering: none)[]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Cover page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-cover(title: [], author: [], series: [], cover-path: "") = {
  page(
    numbering: none,
    margin: 2cm,
  )[
    #align(center)[
      #text(size: 12pt)[#series]
    ]
    #v(1fr)
    #align(center)[
      #text(size: 32pt, weight: "bold")[#title]
      #v(1cm)
      #if cover-path != "" {
        image(cover-path, width: 90%)
      }
    ]
    #v(1fr)
    #align(center)[
      #text(size: 22pt)[#author]
    ]
  ]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Half-title page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-half-title(title: []) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 28pt, weight: "bold")[#title]
    ]
  ]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Title page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-title(title: [], author: [], series: []) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 28pt, weight: "bold")[#title]
      #v(1cm)
      #text(size: 18pt)[#author]
      #v(1cm)
      #text(size: 18pt)[#series]
    ]
  ]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Copyright page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-copyright(author: [], year: [], publisher: [], isbn: [], legal_deposit: []) = {
  pagebreak()
  page(numbering: none)[
    #align(center + bottom)[
      #set text(size: 9pt)
      © #author, #year \
      Éditeur : #publisher \
      ISBN : #isbn\
      Dépôt légal : #legal_deposit
    ]
  ]
}


// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Dedication page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-dedication(content) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 14pt)[#content]
    ]
  ]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Epigraph page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-epigraph(content) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 14pt)[#content]
    ]
  ]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Preface page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-preface(content) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 14pt)[#content]
    ]
  ]
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Foreword page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#let page-foreword(content) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 14pt)[#content]
    ]
  ]
}


// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Chapter page
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// ── Fonction page de chapitre ────────────────────────────────────────────────
#let page-chapter(number: none, title: none, sub-title: none) = {
  pagebreak()
  page(
    margin: (left: 0pt, right: 0pt, top: 0pt, bottom: 0pt),
    numbering: none,
  )[
    #set align(center + horizon)
    #block(width: 100%, height: 100%, fill: rgb("#1a1a2e"))[
      #set text(fill: white)
      #align(center + horizon)[
        #if number != none {
          text(size: 72pt, weight: "bold")[#number]
          v(1cm)
        }
        #if title != none {
          text(size: 28pt)[#title]
          v(0.5cm)
        }
        #if sub-title != none {
          text(size: 16pt, fill: luma(180))[#sub-title]
        }
      ]
    ]
  ]
  pagebreak()
}


// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Document
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#set text(font: font-name, size: 11pt, lang: "fr")
#set page(paper: "a4", numbering: "1", margin: (x: 2.5cm, y: 3cm))
#set par(justify: true)
#set heading(numbering: none)

$body$
