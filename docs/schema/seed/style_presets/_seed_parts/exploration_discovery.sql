-- ==== exploration_discovery ====

-- preset: exploration_discovery.first_contact | name: "Lần đầu tiếp xúc"
-- tone.pace.raw: "chậm rãi, dò dẫm, từng khoảnh khắc"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["bình thường","quen thuộc"]
-- unmapped.rules.avoid_structure: ["đoạn văn liệt kê máy móc dài"]
-- unmapped.rules.emotion_limit: "không châm biếm sự khác biệt"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'exploration_discovery.first_contact') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'exploration_discovery.first_contact',
       'exploration_discovery.first_contact',
       'LOW', 'kinh ngạc, thận trọng, hồi hộp', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'NONE',
       'MEDIUM', 'MEDIUM', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ miêu tả sự kỳ lạ', 'ẩn dụ so sánh với điều quen thuộc', 'động từ quan sát'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự khác biệt', 'phản ứng nhân vật', 'nỗ lực giao tiếp'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_so_sanh_la_quen_an_du_kham_pha', 'primary', 'intensity=medium; phrase=Phép so sánh lạ – quen & ẩn dụ khám phá'),
        ('liet_ke_chi_tiet_ky_la', 'secondary', 'intensity=medium; phrase=Liệt kê chi tiết kỳ lạ'),
        ('cau_hoi_tu_tu_kinh_ngac', 'secondary', 'intensity=medium; phrase=Câu hỏi tu từ kinh ngạc'),
        ('ngon_ngu_qua_doi_thuong', 'avoid', 'intensity=medium; phrase=Ngôn ngữ quá đỗi thường'),
        ('mieu_ta_kho_khan', 'avoid', 'intensity=medium; phrase=Miêu tả khô khan')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: exploration_discovery.map_unlocking | name: "Mở khóa bản đồ"
-- tone.pace.raw: "vừa phải, hào hứng"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["nhàm chán","tẻ nhạt"]
-- unmapped.rules.avoid_structure: ["đoạn văn miêu tả tĩnh lặng dài"]
-- unmapped.rules.emotion_limit: "không bi quan, không thất vọng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'exploration_discovery.map_unlocking') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'exploration_discovery.map_unlocking',
       'exploration_discovery.map_unlocking',
       'MODERATE', 'tò mò, phấn khích, kỳ vọng', true,
       'HIGH', 'MEDIUM', 'LOW', 'LOW', 'NONE',
       'MEDIUM', 'SHORT', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ địa hình', 'từ chỉ sự khai mở, hiển lộ', 'miêu tả màu sắc bản đồ'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'vùng đất mới', 'manh mối dẫn đường', 'cảm giác chinh phục'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_dia_danh_diep_cau_truc_khai_mo', 'primary', 'intensity=medium; phrase=Phép liệt kê địa danh & điệp cấu trúc khai mở'),
        ('an_du_buc_man_duoc_ven_len', 'secondary', 'intensity=medium; phrase=Ẩn dụ bức màn được vén lên'),
        ('cau_cam_than_hao_hung', 'secondary', 'intensity=medium; phrase=Câu cảm thán hào hứng'),
        ('ngon_ngu_cham_chap_nang_ne', 'avoid', 'intensity=medium; phrase=Ngôn ngữ chậm chạp, nặng nề'),
        ('mieu_ta_chi_tiet_may_moc', 'avoid', 'intensity=medium; phrase=Miêu tả chi tiết máy móc')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: exploration_discovery.navigation_journey | name: "Hành trình dẫn đường"
-- tone.pace.raw: "trung bình, bền bỉ"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["bất ngờ","đột ngột"]
-- unmapped.rules.avoid_structure: ["câu cụt giật gân"]
-- unmapped.rules.emotion_limit: "không hoảng loạn, không nản chí"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'exploration_discovery.navigation_journey') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'exploration_discovery.navigation_journey',
       'exploration_discovery.navigation_journey',
       'MODERATE', 'kiên nhẫn, cảnh giác, quyết tâm', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW',
       'LOW', 'MEDIUM', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ di chuyển', 'thuật ngữ định hướng', 'từ miêu tả địa hình'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'phương hướng', 'cột mốc', 'cảm giác tiến bước'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_buoc_tien_liet_ke_cot_moc', 'primary', 'intensity=medium; phrase=Điệp cấu trúc bước tiến & liệt kê cột mốc'),
        ('phep_doi_giua_duong_di_va_can_tro', 'secondary', 'intensity=medium; phrase=Phép đối giữa đường đi và cản trở'),
        ('tuong_thanh_nhip_buoc', 'secondary', 'intensity=medium; phrase=Tượng thanh nhịp bước'),
        ('ngon_ngu_gap_gap', 'avoid', 'intensity=medium; phrase=Ngôn ngữ gấp gáp'),
        ('mieu_ta_noi_tam_dai', 'avoid', 'intensity=medium; phrase=Miêu tả nội tâm dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: exploration_discovery.ruin_delving | name: "Thám hiểm di tích"
-- tone.pace.raw: "chậm, dè chừng"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["sạch sẽ","hiện đại"]
-- unmapped.rules.avoid_structure: ["đoạn văn vui vẻ nhẹ nhàng"]
-- unmapped.rules.emotion_limit: "không hài hước, không thản nhiên"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'exploration_discovery.ruin_delving') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'exploration_discovery.ruin_delving',
       'exploration_discovery.ruin_delving',
       'LOW', 'bí ẩn, sợ hãi pha tò mò', true,
       'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'NONE',
       'MEDIUM', 'LONG', 'MEDIUM', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ cổ học', 'từ chỉ sự xuống cấp, hoang tàn', 'ẩn dụ bí mật chôn vùi'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'không gian cũ kỹ', 'dấu vết người trước', 'nguy hiểm tiềm ẩn'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_bi_mat_chon_vui_diep_cau_truc_do_duong', 'primary', 'intensity=medium; phrase=Ẩn dụ bí mật chôn vùi & điệp cấu trúc dò đường'),
        ('liet_ke_co_vat', 'secondary', 'intensity=medium; phrase=Liệt kê cổ vật'),
        ('cau_hoi_tu_tu_ve_qua_khu', 'secondary', 'intensity=medium; phrase=Câu hỏi tu từ về quá khứ'),
        ('ngon_ngu_hien_dai_tuc', 'avoid', 'intensity=medium; phrase=Ngôn ngữ hiện đại, tục'),
        ('tuong_thanh_on_ao', 'avoid', 'intensity=medium; phrase=Tượng thanh ồn ào')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: exploration_discovery.treasure_hunt | name: "Săn kho báu"
-- tone.pace.raw: "nhanh, hồi hộp"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["chán nản","bỏ cuộc"]
-- unmapped.rules.avoid_structure: ["đoạn văn suy tư dài"]
-- unmapped.rules.emotion_limit: "không bi kịch, không buồn thảm"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'exploration_discovery.treasure_hunt') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'exploration_discovery.treasure_hunt',
       'exploration_discovery.treasure_hunt',
       'FAST', 'háo hức, căng thẳng, vui sướng', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW',
       'LOW', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ săn tìm', 'thuật ngữ giải đố', 'động từ hành động nhanh'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'manh mối', 'cuộc đua', 'phần thưởng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_manh_moi_diep_cau_truc_duoi_theo', 'primary', 'intensity=high; phrase=Phép liệt kê manh mối & điệp cấu trúc đuổi theo'),
        ('cau_hoi_tu_tu_cang_thang', 'secondary', 'intensity=high; phrase=Câu hỏi tu từ căng thẳng'),
        ('phep_doi_nguoi_thang_ke_thua', 'secondary', 'intensity=high; phrase=Phép đối người thắng – kẻ thua'),
        ('ngon_ngu_cham_rai', 'avoid', 'intensity=high; phrase=Ngôn ngữ chậm rãi'),
        ('mieu_ta_noi_tam_dai', 'avoid', 'intensity=high; phrase=Miêu tả nội tâm dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
