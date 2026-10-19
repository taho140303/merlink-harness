-- ==== slice_of_life_comedy ====

-- preset: slice_of_life_comedy.daily_routine | name: "Sinh hoạt đời thường"
-- tone.pace.raw: "chậm, thư thái, đều đều như nhịp thở"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["nguy hiểm","vội vã","sinh tử"]
-- unmapped.rules.avoid_structure: ["câu cộc lốc, giật gân"]
-- unmapped.rules.emotion_limit: "không bi kịch, không căng thẳng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'slice_of_life_comedy.daily_routine') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'slice_of_life_comedy.daily_routine',
       'slice_of_life_comedy.daily_routine',
       'LOW', 'ấm cúng, thoải mái, buồn cười vì thói quen', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW',
       'LOW', 'MEDIUM', 'LOW', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ chỉ hành động quen thuộc', 'tính từ đời thường (ướt, ấm, bừa bộn)', 'từ tượng thanh nhẹ nhàng'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự lặp lại êm đềm', 'những điều nhỏ nhặt đáng yêu', 'mối quan hệ giữa các nhân vật trong không gian sống'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('liet_ke_thoi_quen_lap_lai_diep_nhip_em_dem', 'primary', 'intensity=low; phrase=Liệt kê thói quen lặp lại & điệp nhịp êm đềm'),
        ('phep_doi_tuong_phan_hai_huoc', 'secondary', 'intensity=low; phrase=Phép đối tương phản hài hước'),
        ('tuong_thanh_nhe_nhang', 'secondary', 'intensity=low; phrase=Tượng thanh nhẹ nhàng'),
        ('cau_giat_gan', 'avoid', 'intensity=low; phrase=Câu giật gân'),
        ('ngon_ngu_bi_kich', 'avoid', 'intensity=low; phrase=Ngôn ngữ bi kịch')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: slice_of_life_comedy.feast_gathering | name: "Bữa tiệc & sum họp"
-- tone.pace.raw: "vui vẻ, nhộn nhịp, ấm áp"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["im lặng","cô đơn"]
-- unmapped.rules.avoid_structure: ["đoạn văn lặng lẽ dài"]
-- unmapped.rules.emotion_limit: "không buồn, không căng thẳng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'slice_of_life_comedy.feast_gathering') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'slice_of_life_comedy.feast_gathering',
       'slice_of_life_comedy.feast_gathering',
       'MODERATE', 'hạnh phúc, ồn ào, đầm ấm', true,
       'HIGH', 'HIGH', 'MEDIUM', 'HIGH', 'HIGH',
       'LOW', 'SHORT', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ miêu tả món ăn', 'từ tượng thanh vui vẻ', 'động từ mời mọc'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'món ăn', 'trò chuyện ồn ào', 'sự gắn kết'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_mon_an_diep_nhip_vui_nhon', 'primary', 'intensity=low; phrase=Phép liệt kê món ăn & điệp nhịp vui nhộn'),
        ('tuong_thanh_on_ao', 'secondary', 'intensity=low; phrase=Tượng thanh ồn ào'),
        ('phep_doi_tranh_cai_hoa_giai_vui_ve', 'secondary', 'intensity=low; phrase=Phép đối tranh cãi – hòa giải vui vẻ'),
        ('cau_trang_trong_kho_khan', 'avoid', 'intensity=low; phrase=Câu trang trọng khô khan'),
        ('ngon_ngu_bi_kich', 'avoid', 'intensity=low; phrase=Ngôn ngữ bi kịch')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: slice_of_life_comedy.festival | name: "Lễ hội"
-- tone.pace.raw: "vui tươi, nhộn nhịp, tươi sáng"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["buồn chán","vắng vẻ"]
-- unmapped.rules.avoid_structure: ["đoạn văn trầm lặng dài"]
-- unmapped.rules.emotion_limit: "không bi kịch, không lo âu"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'slice_of_life_comedy.festival') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'slice_of_life_comedy.festival',
       'slice_of_life_comedy.festival',
       'MODERATE', 'phấn khích, vui vẻ, náo nhiệt', true,
       'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM',
       'LOW', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ miêu tả lễ hội', 'từ tượng thanh sôi động', 'động từ tham gia'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'hoạt động lễ hội', 'đám đông vui vẻ', 'khoảnh khắc rực rỡ'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_tro_vui_diep_nhip_hoi', 'primary', 'intensity=medium; phrase=Phép liệt kê trò vui & điệp nhịp hội'),
        ('tuong_thanh_nao_nhiet', 'secondary', 'intensity=medium; phrase=Tượng thanh náo nhiệt'),
        ('phep_doi_mau_sac_am_thanh', 'secondary', 'intensity=medium; phrase=Phép đối màu sắc, âm thanh'),
        ('ngon_ngu_u_am', 'avoid', 'intensity=medium; phrase=Ngôn ngữ u ám'),
        ('mieu_ta_tinh_lang', 'avoid', 'intensity=medium; phrase=Miêu tả tĩnh lặng')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: slice_of_life_comedy.prank_misunderstanding | name: "Chơi khăm & hiểu lầm"
-- tone.pace.raw: "nhanh, chồng chất, hài kịch"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["nghiêm trọng","thảm kịch"]
-- unmapped.rules.avoid_structure: ["đoạn văn dài dòng nghiêm túc"]
-- unmapped.rules.emotion_limit: "không ác ý thật sự, không tổn thương thật"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'slice_of_life_comedy.prank_misunderstanding') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'slice_of_life_comedy.prank_misunderstanding',
       'slice_of_life_comedy.prank_misunderstanding',
       'FAST', 'bực bội nhẹ, buồn cười, hốt hoảng oái oăm', true,
       'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE',
       'LOW', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ hài hước', 'động từ hành động nhanh', 'từ tượng thanh chấn động'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'chuỗi hiểu lầm', 'phản ứng nhân vật', 'sự trớ trêu'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_phong_dai_tro_treu_diep_cau_truc_day_chuyen', 'primary', 'intensity=medium; phrase=Phép phóng đại trớ trêu & điệp cấu trúc dây chuyền'),
        ('liet_ke_hieu_lam_chong_chat', 'secondary', 'intensity=medium; phrase=Liệt kê hiểu lầm chồng chất'),
        ('tuong_thanh_hai_kich', 'secondary', 'intensity=medium; phrase=Tượng thanh hài kịch'),
        ('ngon_ngu_nghiem_tuc', 'avoid', 'intensity=medium; phrase=Ngôn ngữ nghiêm túc'),
        ('bi_kich_hoa', 'avoid', 'intensity=medium; phrase=Bi kịch hóa')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: slice_of_life_comedy.shopping_preparation | name: "Mua sắm & chuẩn bị"
-- tone.pace.raw: "vừa phải, vui vẻ, thong thả"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["xa xỉ hóa","bắt buộc"]
-- unmapped.rules.avoid_structure: ["đoạn văn trầm tĩnh dài"]
-- unmapped.rules.emotion_limit: "không buồn chán, không bi kịch"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'slice_of_life_comedy.shopping_preparation') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'slice_of_life_comedy.shopping_preparation',
       'slice_of_life_comedy.shopping_preparation',
       'MODERATE', 'hào hứng, thoải mái, tỉ mỉ', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW',
       'LOW', 'SHORT', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ miêu tả hàng hóa', 'từ mặc cả', 'động từ lựa chọn'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lựa chọn', 'giá cả', 'sự vui vẻ khi mua sắm'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_mon_do_doi_thoai_mac_ca_lap', 'primary', 'intensity=low; phrase=Phép liệt kê món đồ & đối thoại mặc cả lặp'),
        ('phep_doi_nguoi_ban_nguoi_mua', 'secondary', 'intensity=low; phrase=Phép đối người bán – người mua'),
        ('tuong_thanh_cho_bua', 'secondary', 'intensity=low; phrase=Tượng thanh chợ búa'),
        ('ngon_ngu_cang_thang', 'avoid', 'intensity=low; phrase=Ngôn ngữ căng thẳng'),
        ('mieu_ta_kho_khan', 'avoid', 'intensity=low; phrase=Miêu tả khô khan')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
