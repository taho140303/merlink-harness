-- ==== fantasy_scifi_mechanics ====

-- preset: fantasy_scifi_mechanics.crafting_enhancement | name: "Chế tạo & nâng cấp"
-- tone.pace.raw: "chậm, tỉ mỉ, cao trào khi thành phẩm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["tùy tiện","ngẫu nhiên"]
-- unmapped.rules.avoid_structure: ["đoạn văn hỗn loạn"]
-- unmapped.rules.emotion_limit: "không hài hước lệch lạc, không sốt ruột"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'fantasy_scifi_mechanics.crafting_enhancement') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'fantasy_scifi_mechanics.crafting_enhancement',
       'fantasy_scifi_mechanics.crafting_enhancement',
       'LOW', 'tập trung, kiên nhẫn, thỏa mãn', true,
       'HIGH', 'MEDIUM', 'HIGH', 'MEDIUM', 'NONE',
       'LOW', 'MEDIUM', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ nguyên liệu', 'động từ chế tác', 'từ chỉ sự biến đổi'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'quy trình từng bước', 'nguyên liệu', 'khoảnh khắc hoàn thành'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_cong_doan_diep_cau_truc_nang_cap', 'primary', 'intensity=medium; phrase=Phép liệt kê công đoạn & điệp cấu trúc nâng cấp'),
        ('phep_so_sanh_truoc_sau', 'secondary', 'intensity=medium; phrase=Phép so sánh trước – sau'),
        ('tuong_thanh_che_tac', 'secondary', 'intensity=medium; phrase=Tượng thanh chế tác'),
        ('cau_cut_coc_loc', 'avoid', 'intensity=medium; phrase=Câu cụt cộc lốc'),
        ('ngon_ngu_cam_xuc_thai_qua', 'avoid', 'intensity=medium; phrase=Ngôn ngữ cảm xúc thái quá')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: fantasy_scifi_mechanics.evolution_awakening | name: "Tiến hóa & thức tỉnh"
-- tone.pace.raw: "nhanh dần, đỉnh điểm, vỡ òa"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["yếu đuối","nhỏ nhoi"]
-- unmapped.rules.avoid_structure: ["đoạn văn trầm lắng dài"]
-- unmapped.rules.emotion_limit: "không bi quan, không hèn nhát"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'fantasy_scifi_mechanics.evolution_awakening') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'fantasy_scifi_mechanics.evolution_awakening',
       'fantasy_scifi_mechanics.evolution_awakening',
       'FAST', 'hào hứng, kinh ngạc, quyền lực', true,
       'HIGH', 'HIGH', 'HIGH', 'LOW', 'LOW',
       'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ biến đổi', 'ẩn dụ sức mạnh', 'từ chỉ vượt giới hạn'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'quá trình biến hóa', 'giới hạn bị phá vỡ', 'cảm giác quyền năng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_phong_dai_suc_manh_diep_cau_truc_vuot_gioi_han', 'primary', 'intensity=high; phrase=Phép phóng đại sức mạnh & điệp cấu trúc vượt giới hạn'),
        ('an_du_lua_song', 'secondary', 'intensity=high; phrase=Ẩn dụ lửa / sóng'),
        ('cau_cam_than_bung_no', 'secondary', 'intensity=high; phrase=Câu cảm thán bùng nổ'),
        ('ngon_ngu_yeu_ot', 'avoid', 'intensity=high; phrase=Ngôn ngữ yếu ớt'),
        ('mieu_ta_cham_chap', 'avoid', 'intensity=high; phrase=Miêu tả chậm chạp')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: fantasy_scifi_mechanics.hacking_decryption | name: "Hack & giải mã"
-- tone.pace.raw: "nhanh, căng thẳng nhịp phím"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["chậm rãi","thong thả"]
-- unmapped.rules.avoid_structure: ["đoạn văn mượt mà du dương"]
-- unmapped.rules.emotion_limit: "không hài hước, không lo lắng thái quá"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'fantasy_scifi_mechanics.hacking_decryption') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'fantasy_scifi_mechanics.hacking_decryption',
       'fantasy_scifi_mechanics.hacking_decryption',
       'FAST', 'tập trung cao độ, hồi hộp, chiến thắng', true,
       'HIGH', 'MEDIUM', 'LOW', 'NONE', 'NONE',
       'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ công nghệ', 'động từ xâm nhập', 'từ chỉ tốc độ xử lý'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'các lớp phòng thủ', 'lỗ hổng', 'khoảnh khắc phá mã'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_tan_cong_tung_lop_an_du_dot_nhap', 'primary', 'intensity=high; phrase=Điệp cấu trúc tấn công từng lớp & ẩn dụ đột nhập'),
        ('phep_liet_ke_ma_lenh', 'secondary', 'intensity=high; phrase=Phép liệt kê mã lệnh'),
        ('so_sanh_khong_gian_mang_thuc_dia', 'secondary', 'intensity=high; phrase=So sánh không gian mạng – thực địa'),
        ('ngon_ngu_thong_tuc_ngoai_le', 'avoid', 'intensity=high; phrase=Ngôn ngữ thông tục ngoài lề'),
        ('mieu_ta_ngoai_canh_dai', 'avoid', 'intensity=high; phrase=Miêu tả ngoại cảnh dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: fantasy_scifi_mechanics.summoning_ritual | name: "Nghi thức triệu hồi"
-- tone.pace.raw: "chậm, trang trọng, tăng dần nghẹt thở"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["bình thường","hiện đại"]
-- unmapped.rules.avoid_structure: ["đoạn văn sinh hoạt thường nhật"]
-- unmapped.rules.emotion_limit: "không giễu cợt, không thản nhiên"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'fantasy_scifi_mechanics.summoning_ritual') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'fantasy_scifi_mechanics.summoning_ritual',
       'fantasy_scifi_mechanics.summoning_ritual',
       'LOW', 'thiêng liêng, sợ hãi, cuốn hút', true,
       'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'NONE',
       'HIGH', 'LONG', 'MEDIUM', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ nghi thức cổ', 'ẩn dụ giới hạn giữa hai thế giới', 'động từ triệu gọi'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'quy trình nghi lễ', 'cái giá phải trả', 'sự xuất hiện'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_than_chu_an_du_ranh_gioi_the_gioi', 'primary', 'intensity=high; phrase=Điệp cấu trúc thần chú & ẩn dụ ranh giới thế giới'),
        ('liet_ke_bieu_tuong', 'secondary', 'intensity=high; phrase=Liệt kê biểu tượng'),
        ('cau_cam_than_hai_hung', 'secondary', 'intensity=high; phrase=Câu cảm thán hãi hùng'),
        ('ngon_ngu_doi_thuong', 'avoid', 'intensity=high; phrase=Ngôn ngữ đời thường'),
        ('hai_huoc', 'avoid', 'intensity=high; phrase=Hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: fantasy_scifi_mechanics.time_leap_loop | name: "Nhảy thời gian & vòng lặp"
-- tone.pace.raw: "lặp lại, căng thẳng dần, vỡ ra khi phá vòng"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["một lần duy nhất","cuối cùng"]
-- unmapped.rules.avoid_structure: ["đoạn văn tuyến tính thông thường"]
-- unmapped.rules.emotion_limit: "không hoảng loạn mất trí, không hài hước"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'fantasy_scifi_mechanics.time_leap_loop') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'fantasy_scifi_mechanics.time_leap_loop',
       'fantasy_scifi_mechanics.time_leap_loop',
       'MODERATE', 'bối rối, quyết tâm, mệt mỏi, hy vọng', true,
       'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'NONE',
       'HIGH', 'MEDIUM', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ chỉ sự lặp lại', 'ẩn dụ bàn tay thời gian', 'từ ngữ số phận'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'điểm khác biệt giữa các vòng lặp', 'sự mệt mỏi tinh thần', 'khe cửa phá vỡ'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_diep_toan_canh_lap_lai_phep_doi_giua_cac_lan_thu', 'primary', 'intensity=medium; phrase=Phép điệp toàn cảnh lặp lại & phép đối giữa các lần thử'),
        ('cau_hoi_tu_tu_ve_dinh_menh', 'secondary', 'intensity=medium; phrase=Câu hỏi tu từ về định mệnh'),
        ('an_du_cat_chay_nguoc', 'secondary', 'intensity=medium; phrase=Ẩn dụ cát chảy ngược'),
        ('cau_chu_dut_doan_hon_loan', 'avoid', 'intensity=medium; phrase=Câu chữ đứt đoạn hỗn loạn'),
        ('ngon_ngu_tuyet_vong_tuyet_doi', 'avoid', 'intensity=medium; phrase=Ngôn ngữ tuyệt vọng tuyệt đối')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
