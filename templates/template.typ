// ── Variables injectées par Pandoc ──────────────────────────────────────────
#let font-name = sys.inputs.at("mainfont", default: "New Computer Modern")

// ── Fonction page de chapitre ────────────────────────────────────────────────
#let chapitre-page(numero: none, titre: none, sous-titre: none) = {
  pagebreak()
  page(
    margin: (left: 0pt, right: 0pt, top: 0pt, bottom: 0pt),
    numbering: none,
  )[
    #set align(center + horizon)
    #block(width: 100%, height: 100%, fill: rgb("#1a1a2e"))[
      #set text(fill: white)
      #align(center + horizon)[
        #if numero != none {
          text(size: 72pt, weight: "bold")[#numero]
          v(1cm)
        }
        #if titre != none {
          text(size: 28pt)[#titre]
          v(0.5cm)
        }
        #if sous-titre != none {
          text(size: 16pt, fill: luma(180))[#sous-titre]
        }
      ]
    ]
  ]
  pagebreak()
}

// ── Fonction page spéciale (préface, dédicace, etc.) ─────────────────────────
#let page-speciale(contenu) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 14pt)[#contenu]
    ]
  ]
}

// ── Faux-titre ────────────────────────────────────────────────────────────────
#let page-faux-titre(titre: []) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 28pt, weight: "bold")[#titre]
    ]
  ]
}

// ── Titre ─────────────────────────────────────────────────────────────────────
#let page-titre(titre: [], auteur: [], collection: []) = {
  pagebreak()
  page(numbering: none)[
    #align(center + horizon)[
      #text(size: 28pt, weight: "bold")[#titre]
      #v(1cm)
      #text(size: 18pt)[#auteur]
      #v(1cm)
      #text(size: 18pt)[#collection]
    ]
  ]
}

// ── Ours ──────────────────────────────────────────────────────────────────────
#let page-ours(auteur: [], annee: [], editeur: [], isbn: [], depot: []) = {
  pagebreak()
  page(numbering: none)[
    #align(center + bottom)[
      #set text(size: 9pt)
      © #auteur, #annee \ 
      Éditeur : #editeur \
      ISBN : #isbn\
      Dépôt légal : #depot
    ]
  ]
}

// ── Couverture ────────────────────────────────────────────────────────────────
#let page-couverture(titre: [], auteur: [], collection: [], cover-path: "") = {
  page(
    numbering: none,
    margin: 2cm,
  )[
    #align(center)[
      #text(size: 12pt)[#collection]
    ]
    #v(1fr)
    #align(center)[
      #text(size: 32pt, weight: "bold")[#titre]
      #v(1cm)
      #if cover-path != "" {
        image(cover-path, width: 90%)
      }
    ]
    #v(1fr)
    #align(center)[
      #text(size: 22pt)[#auteur]
    ]
  ]
}

// ── Config document ───────────────────────────────────────────────────────────
#set text(font: font-name, size: 11pt, lang: "fr")
#set page(paper: "a4", numbering: "1", margin: (x: 2.5cm, y: 3cm))
#set par(justify: true)
#set heading(numbering: none)

$body$
