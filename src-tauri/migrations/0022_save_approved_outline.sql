-- 0022: Durable handoff of an author-approved outline (`save_approved_outline`).
--
-- Chat may present a proposed outline as prose. When the author approves it, the model writes the
-- approved text into `BusinessFlowData.decision_log[]` through this tool. The workflow composes
-- its prompts from the draft snapshot (it has no chat history), so `decision_log` is the handoff
-- that lets Phase 1 honour the approved outline instead of regenerating one from scratch.

INSERT OR IGNORE INTO tools (id, code, display_name, description, input_schema) VALUES (
  '5f0c2f0a-3a57-4b2f-9f43-6a1e3c8d2b71',
  'save_approved_outline',
  'Lưu dàn ý đã được tác giả duyệt',
  'Ghi dàn ý hoặc quyết định đã được tác giả duyệt trong chat vào decision_log để các phase sau (workflow) đọc lại. Gọi khi tác giả xác nhận một dàn ý/quyết định; summary là toàn bộ nội dung đã duyệt, declared_sources.summary = user.',
  '{"type":"object","additionalProperties":false,"properties":{"summary":{"type":"string","minLength":1,"description":"Toàn bộ nội dung dàn ý/quyết định tác giả đã duyệt"},"declared_sources":{"type":"object","properties":{"summary":{"type":"string","enum":["user","ai_generated"]}},"required":["summary"],"additionalProperties":false}},"required":["summary","declared_sources"]}'
);

-- Every planning phase can persist an approval; the tool writes a planning decision, not chapter
-- content, so it is never granted to Writing Mode.
INSERT OR IGNORE INTO prompt_block_tools (id, prompt_block_id, tool_id)
SELECT
  lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-' ||
  lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(2))) || '-' || lower(hex(randomblob(6))),
  pb.id, t.id
FROM prompt_blocks pb
JOIN tools t ON t.code = 'save_approved_outline'
WHERE pb.prompt_code IN (
  'planning_phase_0', 'planning_phase_1', 'planning_phase_2', 'planning_phase_3',
  'planning_phase_4', 'planning_phase_5', 'planning_phase_6'
);

-- Phase 0 instruction: capture the approved outline instead of dropping it after the turn.
UPDATE prompt_blocks
SET content = content || char(10) || 'Khi tác giả duyệt một dàn ý, danh sách quyết định hoặc một thoả thuận cần giữ lại, gọi tool save_approved_outline với summary là toàn bộ nội dung đã duyệt và declared_sources.summary = user. Không chỉ trả lời bằng lời: phase sau (workflow) không đọc được lịch sử chat.'
WHERE prompt_code = 'planning_phase_0';

-- Phase 1 instruction: read the durable approval before writing arcs.
UPDATE prompt_blocks
SET content = content || char(10) || 'Trước khi viết Arc, đọc decision_log trong story context. Nếu có dàn ý đã được tác giả duyệt (save_approved_outline), arc/chương phải nhất quán với dàn ý đó thay vì tạo lại từ đầu.'
WHERE prompt_code = 'planning_phase_1';

PRAGMA user_version = 22;
