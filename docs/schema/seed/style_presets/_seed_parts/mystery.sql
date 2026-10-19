-- ==== mystery ====
-- preset: mystery.alibi_check | name: "Xác minh chứng ngoại phạm"
-- tone.pace.raw: "chậm rãi, máy móc"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["chắc hẳn","có lẽ"]
-- unmapped.rules.avoid_structure: ["câu cảm thán, câu hỏi tu từ"]
-- unmapped.rules.emotion_limit: "không phẫn nộ, không mỉa mai"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'mystery.alibi_check') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
        'mystery.alibi_check', 'mystery.alibi_check',
        'LOW', 'khách quan, căng thẳng thầm lặng', true,
        'HIGH', 'LOW', 'LOW', 'NONE', 'NONE',
        'LOW', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ pháp lý', 'con số, thời gian, địa điểm', 'từ so sánh, đối chiếu'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'tính nhất quán', 'khoảng trống thời gian', 'mâu thuẫn nhỏ'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_doi_chieu_thoi_gian', 'primary', 'intensity=medium; phrase=Phép liệt kê & đối chiếu thời gian'),
        ('phep_so_sanh_mau_thuan', 'secondary', 'intensity=medium; phrase=Phép so sánh mâu thuẫn'),
        ('diep_cau_truc_xac_minh', 'secondary', 'intensity=medium; phrase=Điệp cấu trúc xác minh'),
        ('cau_hoi_tu_tu', 'avoid', 'intensity=medium; phrase=Câu hỏi tu từ'),
        ('ngon_ngu_cam_xuc_manh', 'avoid', 'intensity=medium; phrase=Ngôn ngữ cảm xúc mạnh')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: mystery.clue_hunting | name: "Tìm kiếm manh mối"
-- tone.pace.raw: "chậm, tỉ mỉ"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["rõ ràng","hiển nhiên"]
-- unmapped.rules.avoid_structure: ["câu cảm thán ngắn"]
-- unmapped.rules.emotion_limit: "không bộc lộ cảm xúc mạnh"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'mystery.clue_hunting') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
        'mystery.clue_hunting', 'mystery.clue_hunting',
        'LOW', 'tò mò, căng thẳng âm ỉ', true,
        'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE',
        'LOW', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ miêu tả chi tiết', 'thuật ngữ pháp y / khoa học', 'động từ quan sát'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'dấu vết', 'bố cục không gian', 'chi tiết bất thường'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_chi_tiet_nhan_chi_tiet_bat_thuong', 'primary', 'intensity=medium; phrase=Phép liệt kê chi tiết & nhấn chi tiết bất thường'),
        ('diep_cau_truc_quan_sat', 'secondary', 'intensity=medium; phrase=Điệp cấu trúc quan sát'),
        ('phep_doi_dau_vet', 'secondary', 'intensity=medium; phrase=Phép đối dấu vết'),
        ('cau_cam_than_ngan', 'avoid', 'intensity=medium; phrase=Câu cảm thán ngắn'),
        ('phong_dai', 'avoid', 'intensity=medium; phrase=Phóng đại')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: mystery.deduction | name: "Suy luận logic"
-- tone.pace.raw: "chậm, trầm lắng"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["ngẫu nhiên","trùng hợp"]
-- unmapped.rules.avoid_structure: ["câu cụt, câu quá ngắn"]
-- unmapped.rules.emotion_limit: "hoàn toàn lý trí"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'mystery.deduction') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
        'mystery.deduction', 'mystery.deduction',
        'LOW', 'tập trung, lạnh lùng', true,
        'LOW', 'LOW', 'NONE', 'NONE', 'NONE',
        'HIGH', 'LONG', 'LOW', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ lập luận (do đó, như vậy)', 'thuật ngữ logic', 'phép loại suy'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'mối quan hệ nhân quả', 'chứng cứ đối lập', 'giả thuyết và kiểm chứng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_tam_doan_luan_diep_cau_truc_suy_luan', 'primary', 'intensity=medium; phrase=Phép tam đoạn luận & điệp cấu trúc suy luận'),
        ('phep_loai_suy', 'secondary', 'intensity=medium; phrase=Phép loại suy'),
        ('phep_liet_ke_chung_cu', 'secondary', 'intensity=medium; phrase=Phép liệt kê chứng cứ'),
        ('cau_hoi_tu_tu_boc_phat', 'avoid', 'intensity=medium; phrase=Câu hỏi tu từ bộc phát'),
        ('ngon_ngu_an_du_mo_ho', 'avoid', 'intensity=medium; phrase=Ngôn ngữ ẩn dụ mơ hồ')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: mystery.disguise_investigation | name: "Cải trang thâm nhập"
-- tone.pace.raw: "vừa (cảnh giác cao độ)"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["thật sự","thực tế"]
-- unmapped.rules.avoid_structure: ["đoạn độc thoại nội tâm dài"]
-- unmapped.rules.emotion_limit: "không biểu lộ cảm xúc thật ra mặt"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'mystery.disguise_investigation') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
        'mystery.disguise_investigation', 'mystery.disguise_investigation',
        'MODERATE', 'căng thẳng ngấm ngầm, hồi hộp', true,
        'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE',
        'LOW', 'MEDIUM', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ chỉ sự lén lút', 'miêu tả ngoại hình giả', 'động từ nhập vai'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'môi trường xung quanh', 'nguy cơ bị phát hiện', 'thu thập tin qua lời nói phiến diện'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_doi_giua_than_phan_gia_va_that', 'primary', 'intensity=medium; phrase=Phép đối giữa thân phận giả và thật'),
        ('liet_ke_hanh_vi_nhap_vai', 'secondary', 'intensity=medium; phrase=Liệt kê hành vi nhập vai'),
        ('diep_cau_truc_quan_sat_canh_giac', 'secondary', 'intensity=medium; phrase=Điệp cấu trúc quan sát cảnh giác'),
        ('doc_thoai_noi_tam_dai', 'avoid', 'intensity=medium; phrase=Độc thoại nội tâm dài'),
        ('phong_dai_kich_tinh', 'avoid', 'intensity=medium; phrase=Phóng đại kịch tính')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: mystery.interrogation | name: "Thẩm vấn"
-- tone.pace.raw: "nhanh (lời qua tiếng lại)"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["có lẽ","hình như"]
-- unmapped.rules.avoid_structure: ["đoạn văn miêu tả dài"]
-- unmapped.rules.emotion_limit: "không hài hước, không châm biếm"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'mystery.interrogation') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
        'mystery.interrogation', 'mystery.interrogation',
        'FAST', 'căng thẳng, áp lực', true,
        'MEDIUM', 'HIGH', 'LOW', 'NONE', 'NONE',
        'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'câu hỏi dồn dập', 'từ gây áp lực', 'lặp lại ý để gài bẫy'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lời nói', 'khoảng lặng', 'phản ứng của đối phương'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_diep_cau_hoi_don_dap_lap_y_gai_bay', 'primary', 'intensity=high; phrase=Phép điệp câu hỏi dồn dập & lặp ý gài bẫy'),
        ('cau_hoi_tu_tu_gay_ap_luc', 'secondary', 'intensity=high; phrase=Câu hỏi tu từ gây áp lực'),
        ('phep_doi_giua_hoi_va_tranh', 'secondary', 'intensity=high; phrase=Phép đối giữa hỏi và tránh'),
        ('doan_mieu_ta_dai', 'avoid', 'intensity=high; phrase=Đoạn miêu tả dài'),
        ('ngon_ngu_hai_huoc', 'avoid', 'intensity=high; phrase=Ngôn ngữ hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: mystery.revelation | name: "Tiết lộ bất ngờ"
-- tone.pace.raw: "rất nhanh (đổ vỡ)"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["có lẽ","không chắc"]
-- unmapped.rules.avoid_structure: ["đoạn văn kéo dài, miêu tả rời rạc"]
-- unmapped.rules.emotion_limit: "không được pha trò hay trào phúng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'mystery.revelation') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
        'mystery.revelation', 'mystery.revelation',
        'VERY_FAST', 'sốc, phẫn nộ, kinh hoàng', true,
        'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE',
        'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ mạnh, dứt khoát', 'câu nói đanh thép', 'sự tương phản (hoá ra, thì ra)'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự thật bị che giấu', 'lời thú nhận', 'phản ứng nhân vật'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_dao_ngu_nhan_hoa_ra_thi_ra', 'primary', 'intensity=high; phrase=Phép đảo ngữ & nhấn ''hoá ra / thì ra'''),
        ('cau_cut_dut_khoat', 'secondary', 'intensity=high; phrase=Câu cụt dứt khoát'),
        ('phep_liet_ke_su_that', 'secondary', 'intensity=high; phrase=Phép liệt kê sự thật'),
        ('ngon_ngu_mo_ho_do_du', 'avoid', 'intensity=high; phrase=Ngôn ngữ mơ hồ, do dự'),
        ('trao_phung', 'avoid', 'intensity=high; phrase=Trào phúng')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

