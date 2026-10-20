-- ============================================================
-- EVENT_TRANSITIONS SEED
-- Spec: docs/schema/system.dbml (Table event_transitions)
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by (from_event_block_id, to_event_block_id).
-- ============================================================
BEGIN;

INSERT INTO event_transitions (id, name, from_event_block_id, to_event_block_id, priority)
SELECT gen_random_uuid(), v.name, eb_from.id, eb_to.id, v.priority
FROM (VALUES
  ('Giao ước được ký kết', 'eb_faust_1', 'eb_faust_2', 1),
  ('Nhận ra cái giá phải trả', 'eb_faust_2', 'eb_faust_3', 1),
  ('Thời hạn cạn kiệt và bị thu nợ', 'eb_faust_3', 'eb_faust_4', 1),
  ('Thỏa hiệp bước đầu', 'eb_mcorrupt_1', 'eb_mcorrupt_2', 1),
  ('Lún sâu và trơ lì đạo đức', 'eb_mcorrupt_2', 'eb_mcorrupt_3', 1),
  ('Phán xét và trả giá cục diện', 'eb_mcorrupt_3', 'eb_mcorrupt_4', 1),
  ('Chấp nhận cùng hợp tác vì nguy cơ lớn hơn', 'eb_relall_1', 'eb_relall_2', 1),
  ('Nhận ra phẩm chất và đồng cam cộng khổ', 'eb_relall_2', 'eb_relall_3', 1),
  ('Phá vỡ giới nghiêm ở ngưỡng cửa hoàn tất', 'eb_prohib_1', 'eb_prohib_2', 1),
  ('Mất đi vĩnh viễn điều quý giá nhất', 'eb_prohib_2', 'eb_prohib_3', 1),
  ('Nhìn thấy hình bóng xa lạ của chính mình', 'eb_mirror_1', 'eb_mirror_2', 1),
  ('Chấp nhận sự thật nội tâm sau phản chiếu', 'eb_mirror_2', 'eb_mirror_3', 1),
  ('Tiếp đón kẻ lữ hành và thực thi nghĩa vụ', 'eb_xenia_1', 'eb_xenia_2', 1),
  ('Khắc ghi ân nghĩa trở thành minh ước', 'eb_xenia_2', 'eb_xenia_3', 1)
) AS v(name, from_eb_code, to_eb_code, priority)
JOIN event_blocks eb_from ON eb_from.code = v.from_eb_code
JOIN event_blocks eb_to ON eb_to.code = v.to_eb_code
ON CONFLICT (from_event_block_id, to_event_block_id) DO NOTHING;

COMMIT;
