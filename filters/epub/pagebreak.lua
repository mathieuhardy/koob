function Para(el)
  if #el.content == 1 and el.content[1].text == "===" then
    local pagebreak = '<p style="page-break-after: always;"></p>'
    return pandoc.RawBlock("html", pagebreak)
  end
end

return {
  {
    Para = Para,
  },
}
