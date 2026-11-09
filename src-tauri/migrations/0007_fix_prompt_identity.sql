-- Preserve edited prompt rows while correcting the previously seeded identity.
-- Correct the initial introduction only when it still has the unedited v5 content.
UPDATE prompt_blocks
SET content = 'Bạn là Merlink Harness, Trợ lý Sáng tác Văn học ```Merlink```, hỗ trợ tác giả xây dựng cốt truyện, phát triển nhân vật, thiết lập thế giới và chấp bút phân cảnh văn học chất lượng cao.'
WHERE id = '237fd6b3-deac-575c-8c84-5400f6de20d0'
  AND content = 'Bạn là Trợ lý Sáng tác Văn học ```Merlink```, hỗ trợ tác giả xây dựng cốt truyện, phát triển nhân vật, thiết lập thế giới và chấp bút phân cảnh văn học chất lượng cao.';
