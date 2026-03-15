local chapter_number = nil
local chapter_title = nil
local chapter_subtitle = nil

function Header(el)
  if el.level == 1 then
    chapter_number = pandoc.utils.stringify(el.content)
    return {}
  elseif el.level == 2 then
    chapter_title = pandoc.utils.stringify(el.content)
    return {}
  elseif el.level == 3 then
    chapter_subtitle = pandoc.utils.stringify(el.content)
    return {}
  end
end

function Para(el)
  if chapter_number ~= nil or chapter_title ~= nil or chapter_subtitle ~= nil then
    local lines = { "#chapitre-page(" }

    if chapter_number ~= nil then
      lines[#lines + 1] = string.format("  numero: [%s],", chapter_number)
    end
    if chapter_title ~= nil then
      lines[#lines + 1] = string.format("  titre: [%s],", chapter_title)
    end
    if chapter_subtitle ~= nil then
      lines[#lines + 1] = string.format("  sous-titre: [%s],", chapter_subtitle)
    end

    lines[#lines + 1] = ")"

    chapter_number = nil
    chapter_title = nil
    chapter_subtitle = nil

    return {
      pandoc.RawBlock("typst", table.concat(lines, "\n")),
      el,
    }
  end
end

return {
  {
    -- Header = Header,
    -- Para = Para,
  },
}
