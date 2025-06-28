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
    local latex = [[
\begin{titlepage}
  \begin{center}
    \vspace*{\fill}
    \begin{minipage}{\textwidth}
      \centering
]]

    if chapter_number ~= nil then
      latex = latex .. string.format([[ {\Large %s} \\[1cm] ]], chapter_number)
    end

    if chapter_title ~= nil then
      latex = latex .. string.format([[ {\Large %s} \\[1cm] ]], chapter_title)
    end

    if chapter_subtitle ~= nil then
      latex = latex .. string.format([[ {\normalsize %s} \\[1cm] ]], chapter_subtitle)
    end

    local latex = latex .. [[
    \end{minipage}
    \vspace*{\fill}
  \end{center}
\end{titlepage}
]]

    chapter_number = nil
    chapter_title = nil
    chapter_subtitle = nil

    return {
      pandoc.RawBlock("latex", latex),
      el,
    }
  end
end

return {
  {
    Header = Header,
    Para = Para,
  },
}
