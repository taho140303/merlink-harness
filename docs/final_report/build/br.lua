-- Chuyển <br> trong ô bảng thành ngắt dòng LaTeX
function RawInline(el)
  if el.format:match('html') and el.text:match('^<br%s*/?>$') then
    return pandoc.LineBreak()
  end
end
