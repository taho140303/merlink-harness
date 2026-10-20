-- ============================================================
-- NARRATOR CONFIGS SEED
-- Spec: docs/schema/system.dbml (Table narrator_configs)
-- Reference: docs/schema/runtime/narrative.json
-- Enums:
--   GrammaticalPerson: FIRST_PERSON, SECOND_PERSON, THIRD_PERSON
--   PersonSubtype:
--     Ngôi 1: PROTAGONIST, WITNESS_PERIPHERAL, UNRELIABLE_FIRST
--     Ngôi 2: DIRECT_ADDRESS, SELF_REFLECTION, INTERACTIVE
--     Ngôi 3: OMNISCIENT, LIMITED, OBJECTIVE
--   PovScope: SINGLE_POV, MULTIPLE_POV, SHIFTING_OMNISCIENT
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO narrator_configs (
    id,
    code,
    name,
    person,
    person_subtype,
    pov_scope,
    is_reliable,
    breaks_fourth_wall
)
SELECT
    gen_random_uuid(),
    v.code,
    v.name,
    v.person::GrammaticalPerson,
    v.person_subtype::PersonSubtype,
    v.pov_scope::PovScope,
    v.is_reliable,
    v.breaks_fourth_wall
FROM (VALUES
    -- 1. Ngôi 1 - Nhân vật chính tự thuật (Tiêu chuẩn)
    (
        'first_person_protagonist',
        'Ngôi 1 - Nhân vật chính độc thoại nội tâm',
        'FIRST_PERSON',
        'PROTAGONIST',
        'SINGLE_POV',
        true,
        false
    ),
    -- 2. Ngôi 1 - Nhân chứng quan sát (Dạng Sherlock Holmes / Gatsby)
    (
        'first_person_witness_peripheral',
        'Ngôi 1 - Nhân chứng ngoại vi quan sát',
        'FIRST_PERSON',
        'WITNESS_PERIPHERAL',
        'SINGLE_POV',
        true,
        false
    ),
    -- 3. Ngôi 1 - Kẻ kể chuyện không đáng tin (Unreliable Narrator)
    (
        'first_person_unreliable',
        'Ngôi 1 - Kẻ dẫn chuyện không đáng tin cậy',
        'FIRST_PERSON',
        'UNRELIABLE_FIRST',
        'SINGLE_POV',
        false,
        false
    ),
    -- 4. Ngôi 1 - Phá vỡ bức tường thứ 4 (Deadpool / Metafiction)
    (
        'first_person_meta_fourth_wall',
        'Ngôi 1 - Tự thức phá vỡ bức tường thứ 4',
        'FIRST_PERSON',
        'PROTAGONIST',
        'SINGLE_POV',
        true,
        true
    ),
    -- 5. Ngôi 1 - Đa điểm nhìn hoán đổi (Multiple First Person POV)
    (
        'first_person_multi_pov',
        'Ngôi 1 - Đa điểm nhìn luân phiên',
        'FIRST_PERSON',
        'PROTAGONIST',
        'MULTIPLE_POV',
        true,
        false
    ),

    -- 6. Ngôi 2 - Hướng lời trực tiếp (Direct Address / Epistolary)
    (
        'second_person_direct_address',
        'Ngôi 2 - Hướng lời trực tiếp tới đối tượng',
        'SECOND_PERSON',
        'DIRECT_ADDRESS',
        'SINGLE_POV',
        true,
        false
    ),
    -- 7. Ngôi 2 - Tự vấn nội tâm (Self-reflection monologue)
    (
        'second_person_self_reflection',
        'Ngôi 2 - Độc thoại tự phân tách nội tâm',
        'SECOND_PERSON',
        'SELF_REFLECTION',
        'SINGLE_POV',
        true,
        false
    ),
    -- 8. Ngôi 2 - Tương tác nhập vai (Interactive Choose-your-own-adventure)
    (
        'second_person_interactive',
        'Ngôi 2 - Tương tác nhập vai mô phỏng',
        'SECOND_PERSON',
        'INTERACTIVE',
        'SINGLE_POV',
        true,
        false
    ),

    -- 9. Ngôi 3 - Hạn tri bám sát một tiêu điểm (Third Person Limited)
    (
        'third_person_limited',
        'Ngôi 3 - Hạn tri bám sát nhân vật tiêu điểm',
        'THIRD_PERSON',
        'LIMITED',
        'SINGLE_POV',
        true,
        false
    ),
    -- 10. Ngôi 3 - Hạn tri đa góc nhìn luân phiên (A Song of Ice and Fire style)
    (
        'third_person_limited_multi_pov',
        'Ngôi 3 - Hạn tri đa điểm nhìn chương hồi',
        'THIRD_PERSON',
        'LIMITED',
        'MULTIPLE_POV',
        true,
        false
    ),
    -- 11. Ngôi 3 - Toàn tri truyền thống (Third Person Omniscient)
    (
        'third_person_omniscient',
        'Ngôi 3 - Toàn tri thấu thị nội tâm',
        'THIRD_PERSON',
        'OMNISCIENT',
        'SINGLE_POV',
        true,
        false
    ),
    -- 12. Ngôi 3 - Toàn tri dịch chuyển linh hoạt (Cinematic Shifting Omniscient)
    (
        'third_person_shifting_omniscient',
        'Ngôi 3 - Toàn tri dịch chuyển góc nhìn linh hoạt',
        'THIRD_PERSON',
        'OMNISCIENT',
        'SHIFTING_OMNISCIENT',
        true,
        false
    ),
    -- 13. Ngôi 3 - Khách quan ống kính máy quay (Fly-on-the-wall / Hemingway)
    (
        'third_person_objective_camera',
        'Ngôi 3 - Ống kính khách quan hành vi',
        'THIRD_PERSON',
        'OBJECTIVE',
        'SINGLE_POV',
        true,
        false
    ),
    -- 14. Ngôi 3 - Toàn tri trào phúng can thiệp văn bản (Editorial / Satirical Omniscient)
    (
        'third_person_editorial_fourth_wall',
        'Ngôi 3 - Bình luận thế sự phá vỡ khoảng cách',
        'THIRD_PERSON',
        'OMNISCIENT',
        'SHIFTING_OMNISCIENT',
        true,
        true
    )
) AS v(code, name, person, person_subtype, pov_scope, is_reliable, breaks_fourth_wall)
ON CONFLICT (code) DO UPDATE SET
    name = EXCLUDED.name,
    person = EXCLUDED.person,
    person_subtype = EXCLUDED.person_subtype,
    pov_scope = EXCLUDED.pov_scope,
    is_reliable = EXCLUDED.is_reliable,
    breaks_fourth_wall = EXCLUDED.breaks_fourth_wall;

COMMIT;
