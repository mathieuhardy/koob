function Para(el)
  if #el.content == 1 and el.content[1].text == "===" then
    return pandoc.RawBlock("tex", "\\newpage{}")
  elseif el.content == "::: {.chapitre}" then
    return {
      pandoc.RawBlock("tex", "\\newpage{}"),
      el.content,
    }
  end
end

return {
  {
    Para = Para,
  },
}
