local function extract_headers(el)
  local headers = {}
  for _, block in ipairs(el.content) do
    if block.t == "Header" then
      headers[block.level] = pandoc.utils.stringify(block)
    elseif block.t == "Div" then
      -- Descend dans les divs enfants
      local child_headers = extract_headers(block)
      for level, value in pairs(child_headers) do
        headers[level] = value
      end
    end
  end
  return headers
end

function Para(el)
  if #el.content == 1 and el.content[1].text == ":::" then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-speciale([])"))
    -- elseif el.content == "::: {.chapitre}" then
    --   return {
    --     pandoc.RawBlock("typst", "#pagebreak()"),
    --     el.content,
    --   }
  end
end

function Div(el)
  if el.classes:includes("avant-propos") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-speciale([\n%s\n])", content))
  elseif el.classes:includes("couverture") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format(
        '#page-couverture(titre: [%s], auteur: [%s], collection: [%s], cover-path: "%s")',
        h[1] or "",
        h[2] or "",
        h[3] or "",
        h[4] or ""
      )
    )
  elseif el.classes:includes("dedicace") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-speciale([\n%s\n])", content))
  elseif el.classes:includes("epigraphe") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-speciale([\n%s\n])", content))
  elseif el.classes:includes("faux-titre") then
    local h = extract_headers(el)
    return pandoc.RawBlock("typst", string.format("#page-faux-titre(titre: [%s])", h[1] or ""))
  elseif el.classes:includes("ours") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format(
        "#page-ours(auteur: [%s], annee: [%s], editeur: [%s], isbn: [%s], depot: [%s])",
        h[1] or "",
        h[2] or "",
        h[3] or "",
        h[4] or "",
        h[5] or ""
      )
    )
  elseif el.classes:includes("preface") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-speciale([\n%s\n])", content))
  elseif el.classes:includes("titre") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format("#page-titre(titre: [%s], auteur: [%s], collection: [%s])", h[1] or "", h[2] or "", h[3] or "")
    )
  elseif el.classes:includes("chapitre") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format(
        "#chapitre-page(numero: [%s], titre: [%s], sous-titre: [%s])",
        h[1] or "prout",
        h[2] or "",
        h[3] or ""
      )
    )
  end
end

return {
  {
    Para = Para,
    Div = Div,
  },
}
