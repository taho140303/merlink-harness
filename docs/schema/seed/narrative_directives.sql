-- ============================================================
-- NARRATIVE DIRECTIVES SEED
-- Spec: docs/schema/system.dbml (Table narrative_directives)
-- Enums:
--   DirectiveCategory: PROSE_PRINCIPLE, RESTRAINT, INFORMATION_CONTROL
-- Requirements: PostgreSQL 13+ (gen_random_uuid); idempotent by code.
-- ============================================================
BEGIN;

INSERT INTO narrative_directives (
    id,
    code,
    description,
    category,
    is_system
)
SELECT
    gen_random_uuid(),
    v.code,
    v.description,
    v.category::DirectiveCategory,
    true
FROM (VALUES
    -- ========================================================
    -- NHÓM 1: PROSE_PRINCIPLE (Nguyên tắc văn phong & mỹ cảm)
    -- ========================================================
    (
        'clarity_first',
        'Ưu tiên sự trong sáng và mạch lạc của câu từ; tránh cấu trúc tối nghĩa hay câu phức vô căn cứ làm đứt gãy luồng cảm nhận của độc giả.',
        'PROSE_PRINCIPLE'
    ),
    (
        'show_dont_tell',
        'Thể hiện cảm xúc, tính cách và mâu thuẫn qua hành động vi mô, biến đổi thể lý, nhịp thở và chi tiết giác quan thay vì chỉ dán nhãn trừu tượng.',
        'PROSE_PRINCIPLE'
    ),
    (
        'technique_rotation',
        'Luân phiên linh hoạt các kỹ thuật tự sự (tả cảnh, đối thoại sắc gọn, độc thoại nội tâm, tổng thuật nhịp nhanh) để tạo nhịp điệu sinh động.',
        'PROSE_PRINCIPLE'
    ),
    (
        'sensory_grounding',
        'Neo giữ bối cảnh bằng ít nhất 2 giác quan cụ thể (thị giác, xúc giác, âm thanh hoặc mùi hương) trong mỗi đoạn chuyển cảnh.',
        'PROSE_PRINCIPLE'
    ),
    (
        'rhythm_cadence_variation',
        'Biến thiên độ dài câu văn và nhịp điệu: câu ngắn dứt khoát tạo xung lực hành động hoặc căng thẳng, câu dài phức hợp tạo độ ngân vang chiêm nghiệm.',
        'PROSE_PRINCIPLE'
    ),
    (
        'active_verbs_precision',
        'Triệt để sử dụng động từ hành động chính xác, cụ thể; hạn chế thể bị động và các trợ động từ suy yếu xung lực câu.',
        'PROSE_PRINCIPLE'
    ),
    (
        'metaphor_organic_consistency',
        'Hình ảnh ẩn dụ và so sánh phải bám rễ tự nhiên vào thế giới quan, nghề nghiệp và tâm cảnh thực tế của nhân vật tiêu điểm.',
        'PROSE_PRINCIPLE'
    ),
    (
        'dialogue_subtext_layering',
        'Mỗi câu thoại phải chứa ẩn ý (subtext); nhân vật hiếm khi nói thẳng 100% động cơ thật, luôn có khoảng cách giữa lời nói và tâm ý.',
        'PROSE_PRINCIPLE'
    ),

    -- ========================================================
    -- NHÓM 2: RESTRAINT (Quy tắc tiết chế & kỷ luật tự sự)
    -- ========================================================
    (
        'avoid_melodrama',
        'Tiết chế bi lụy và cường điệu hóa; không dùng tính từ thái quá để cưỡng ép cảm xúc người đọc mà để sức nặng của tình thế tự bộc lộ.',
        'RESTRAINT'
    ),
    (
        'no_filter_words',
        'Loại bỏ các từ lọc nhận thức dư thừa (như "anh ấy cảm thấy", "cô nhận thấy", "hắn nghe thấy") để kéo độc giả trực tiếp vào trải nghiệm.',
        'RESTRAINT'
    ),
    (
        'anti_cliche_expression',
        'Nghiêm cấm các lối ví von sáo mòn, khuôn sáo truyền thống; tìm kiếm góc nhìn độc đáo, chân thực và có tính cá thể hóa cao.',
        'RESTRAINT'
    ),
    (
        'no_info_dumping',
        'Tuyệt đối không nhồi nhét bối cảnh, lịch sử thế giới (worldbuilding) thành khối văn thuyết minh thô thiển; chỉ hé lộ khi cốt truyện đòi hỏi.',
        'RESTRAINT'
    ),
    (
        'adverb_economy',
        'Hạn chế lạm dụng phó từ chỉ cách thức bổ nghĩa cho động từ thoại (ví dụ: "hắn nói một cách giận dữ"); hãy để từ ngữ đối thoại tự toát lên thái độ.',
        'RESTRAINT'
    ),
    (
        'emotional_understatement',
        'Áp dụng thủ pháp nén cảm xúc (kìm nén / understatement); nỗi đau tột cùng thường thể hiện qua sự im lặng hoặc hành động cơ học vụn vặt.',
        'RESTRAINT'
    ),
    (
        'avoid_unearned_epiphany',
        'Không để nhân vật đột ngột giác ngộ chân lý đạo đức một cách dễ dãi nếu chưa trải qua va đập, trả giá và khủng hoảng nội tâm tương xứng.',
        'RESTRAINT'
    ),

    -- ========================================================
    -- NHÓM 3: INFORMATION_CONTROL (Kiểm soát thông tin & điểm nhìn)
    -- ========================================================
    (
        'strict_pov_containment',
        'Tuyệt đối không xâm phạm điểm nhìn (head-hopping); người kể chỉ được biết những gì nhân vật tiêu điểm đang quan sát và suy nghĩ.',
        'INFORMATION_CONTROL'
    ),
    (
        'asymmetric_suspense',
        'Tạo chênh lệch thông tin có chủ đích: để độc giả biết trước hiểm nguy mà nhân vật chưa hay (suspense), hoặc ngược lại (mystery).',
        'INFORMATION_CONTROL'
    ),
    (
        'delayed_revelation',
        'Trì hoãn việc lý giải biến cố bí ẩn; đưa ra các manh mối rời rạc trước khi ráp nối thành bức tranh hoàn chỉnh ở thời điểm đắt giá.',
        'INFORMATION_CONTROL'
    ),
    (
        'unreliable_distortion',
        'Cho phép bộ lọc tâm lý của nhân vật (định kiến, say rượu, sang chấn, chối bỏ) làm méo mó việc tiếp nhận hiện thực có kiểm soát.',
        'INFORMATION_CONTROL'
    ),
    (
        'foreshadowing_planted_early',
        'Các bước ngoặt then chốt phải được gài cắm hạt giống chi tiết (Chekhov’s gun) từ sớm một cách kín đáo nhưng không được đánh động.',
        'INFORMATION_CONTROL'
    ),
    (
        'interiority_vs_exteriority_balance',
        'Kiểm soát tỉ lệ giữa dòng suy tưởng nội tâm và nhịp vận động ngoại cảnh; tránh để nhân vật chìm đắm độc thoại quá lâu làm đóng băng hiện trường.',
        'INFORMATION_CONTROL'
    )
) AS v(code, description, category)
ON CONFLICT (code) DO UPDATE SET
    description = EXCLUDED.description,
    category = EXCLUDED.category,
    is_system = true;

COMMIT;
