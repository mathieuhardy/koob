local function extract_headers(el)
  local headers = {}

  for _, block in ipairs(el.content) do
    if block.t == "Header" then
      headers[block.level] = pandoc.utils.stringify(block)
    elseif block.t == "Div" then
      local child_headers = extract_headers(block)

      for level, value in pairs(child_headers) do
        headers[level] = value
      end
    end
  end
  return headers
end

function Div(el)
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Cover
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  if el.classes:includes("cover") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format(
        '#page-cover(title: [%s], author: [%s], series: [%s], cover-path: "%s")',
        h[1] or "",
        h[2] or "",
        h[3] or "",
        h[4] or ""
      )
    )

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Empty pages
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("empty") then
    return pandoc.RawBlock("typst", string.format("#page-empty()"))

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Half-title page
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("half-title") then
    local h = extract_headers(el)
    return pandoc.RawBlock("typst", string.format("#page-half-title(title: [%s])", h[1] or ""))

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Title page
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("title") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format("#page-title(title: [%s], author: [%s], series: [%s])", h[1] or "", h[2] or "", h[3] or "")
    )

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Copyright page
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("copyright") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format(
        "#page-copyright(author: [%s], year: [%s], publisher: [%s], isbn: [%s], legal_deposit: [%s])",
        h[1] or "",
        h[2] or "",
        h[3] or "",
        h[4] or "",
        h[5] or ""
      )
    )

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Dedication
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("dedication") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-dedication([\n%s\n])", content))

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Epigraphe
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("epigraph") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-epigraph([\n%s\n])", content))

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Preface
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("preface") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-preface([\n%s\n])", content))

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Avant-propos
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("foreword") then
    local content = pandoc.write(pandoc.Pandoc(el.content), "typst")
    return pandoc.RawBlock("typst", string.format("#page-foreword([\n%s\n])", content))

  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  -- Chapter
  -- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  elseif el.classes:includes("chapter") then
    local h = extract_headers(el)
    return pandoc.RawBlock(
      "typst",
      string.format(
        "#page-chapter(number: [%s], title: [%s], sub-title: [%s])",
        h[1] or "prout",
        h[2] or "",
        h[3] or ""
      )
    )
  end
end

return {
  {
    Div = Div,
  },
}
