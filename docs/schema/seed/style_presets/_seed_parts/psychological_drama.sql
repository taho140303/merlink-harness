-- ==== psychological_drama ====
-- preset: psychological_drama.betrayal | name: "Phản bội"
-- tone.pace.raw: "đột ngột, nhanh khi vỡ lẽ, sau đó đóng băng"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["tất nhiên","đúng như dự đoán"]
-- unmapped.rules.avoid_structure: ["đoạn miêu tả cảnh vật dài"]
-- unmapped.rules.emotion_limit: "không hài hước, không bình thản"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'psychological_drama.betrayal') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'psychological_drama.betrayal',
       'psychological_drama.betrayal',
       'FAST', 'sốc, đau đớn tột cùng, phẫn uất, hoang mang', true,
       'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'LOW',
       'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ đối lập (tôi đã tin – hóa ra)',
        'động từ mạnh chỉ sự đâm thủng',
        'tính từ về sự sụp đổ niềm tin'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự thay đổi trong nhận thức nhân vật',
        'lời nói của kẻ phản bội',
        'khoảng lặng nặng nề sau cú sốc'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_doi_toi_da_tin_hoa_ra_diep_khoang_lang', 'primary', 'intensity=high; phrase=Phép đối ''tôi đã tin – hoá ra'' & điệp khoảng lặng'),
        ('cau_cham_lung_nghen_lai', 'secondary', 'intensity=high; phrase=Câu chấm lửng nghẹn lại'),
        ('an_du_vet_thuong', 'secondary', 'intensity=high; phrase=Ẩn dụ vết thương'),
        ('ngon_ngu_binh_than', 'avoid', 'intensity=high; phrase=Ngôn ngữ bình thản'),
        ('mieu_ta_ngoai_canh_dai', 'avoid', 'intensity=high; phrase=Miêu tả ngoại cảnh dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: psychological_drama.identity_crisis | name: "Khủng hoảng danh tính"
-- tone.pace.raw: "chậm, trầm lắng, có những cơn xáo trộn"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["đơn giản","chỉ là"]
-- unmapped.rules.avoid_structure: ["câu khẳng định quá dứt khoát"]
-- unmapped.rules.emotion_limit: "không mỉa mai, không hài hước"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'psychological_drama.identity_crisis') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'psychological_drama.identity_crisis',
       'psychological_drama.identity_crisis',
       'LOW', 'mơ hồ, lo lắng, mất phương hướng', true,
       'MEDIUM', 'LOW', 'LOW', 'NONE', 'NONE',
       'HIGH', 'LONG', 'HIGH', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ về sự phân mảnh, tan rã',
        'câu hỏi tu từ về bản thân',
        'ẩn dụ cho lốt và thực chất'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự nghi ngờ về quá khứ hoặc tương lai',
        'đối thoại giữa các phần của bản thân',
        'khoảnh khắc không nhận ra gương mặt mình'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('cau_hoi_tu_tu_ve_ban_than_an_du_lot_thuc_chat', 'primary', 'intensity=high; phrase=Câu hỏi tu từ về bản thân & ẩn dụ lốt/thực chất'),
        ('diep_cau_truc_phan_manh', 'secondary', 'intensity=high; phrase=Điệp cấu trúc phân mảnh'),
        ('phep_doi_giua_cac_phan_cua_ban_than', 'secondary', 'intensity=high; phrase=Phép đối giữa các phần của bản thân'),
        ('cau_khang_dinh_dut_khoat', 'avoid', 'intensity=high; phrase=Câu khẳng định dứt khoát'),
        ('ngon_ngu_mia_mai', 'avoid', 'intensity=high; phrase=Ngôn ngữ mỉa mai')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: psychological_drama.inner_monologue | name: "Độc thoại nội tâm - bước ngoặt"
-- tone.pace.raw: "chậm, đong đếm từng suy nghĩ"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["dễ dàng","chắc chắn"]
-- unmapped.rules.avoid_structure: ["đối thoại với người khác (trừ khi là tưởng tượng)"]
-- unmapped.rules.emotion_limit: "không hài hước, không mỉa mai"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'psychological_drama.inner_monologue') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'psychological_drama.inner_monologue',
       'psychological_drama.inner_monologue',
       'LOW', 'dằn vặt, đấu tranh, cô độc', true,
       'LOW', 'LOW', 'LOW', 'NONE', 'NONE',
       'HIGH', 'LONG', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ triết lý, nội quan',
        'so sánh ẩn dụ về trọng lượng (gánh nặng)',
        'động từ chỉ sự đấu tranh tinh thần'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'dòng ý thức',
        'mâu thuẫn giữa lý trí và cảm xúc',
        'hậu quả tưởng tượng của từng lựa chọn'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_ve_trong_luong_ganh_nang_cau_hoi_tu_tu_dau_tranh', 'primary', 'intensity=medium; phrase=Ẩn dụ về trọng lượng (gánh nặng) & câu hỏi tu từ đấu tranh'),
        ('dong_y_thuc_lap_di_lap_lai', 'secondary', 'intensity=medium; phrase=Dòng ý thức lặp đi lặp lại'),
        ('phep_doi_ly_tri_cam_xuc', 'secondary', 'intensity=medium; phrase=Phép đối lý trí – cảm xúc'),
        ('doi_thoai_voi_nguoi_khac', 'avoid', 'intensity=medium; phrase=Đối thoại với người khác'),
        ('ngon_ngu_hai_huoc', 'avoid', 'intensity=medium; phrase=Ngôn ngữ hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: psychological_drama.nightmare_flashback | name: "Ác mộng – hồi ức xâm nhập"
-- tone.pace.raw: "rất nhanh, hỗn loạn, rồi đột ngột băng giá"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["bình tĩnh","từ từ"]
-- unmapped.rules.avoid_structure: ["đoạn văn mạch lạc, logic"]
-- unmapped.rules.emotion_limit: "không hài hước, không nhẹ nhàng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'psychological_drama.nightmare_flashback') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'psychological_drama.nightmare_flashback',
       'psychological_drama.nightmare_flashback',
       'VERY_FAST', 'hoảng loạn, kinh hoàng, bất lực', true,
       'HIGH', 'HIGH', 'HIGH', 'MEDIUM', 'LOW',
       'MEDIUM', 'SHORT', 'HIGH', 'SIMPLE')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ gây ám ảnh',
        'tính từ về sự mất kiểm soát',
        'từ tượng thanh cho âm thanh ám ảnh'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự xâm nhập của quá khứ vào hiện tại',
        'cảm giác không thể thoát',
        'phản ứng cơ thể (mồ hôi, tim đập)'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('tuong_thanh_am_anh_diep_hinh_anh_qua_khu_xam_nhap', 'primary', 'intensity=high; phrase=Tượng thanh ám ảnh & điệp hình ảnh quá khứ xâm nhập'),
        ('phep_phong_dai_su_mat_kiem_soat', 'secondary', 'intensity=high; phrase=Phép phóng đại sự mất kiểm soát'),
        ('cau_cut_hon_loan_khong_dau_phay', 'secondary', 'intensity=high; phrase=Câu cụt hỗn loạn không dấu phẩy'),
        ('mach_lac_logic', 'avoid', 'intensity=high; phrase=Mạch lạc logic'),
        ('ngon_ngu_binh_tinh', 'avoid', 'intensity=high; phrase=Ngôn ngữ bình tĩnh')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: psychological_drama.sacrifice | name: "Hy sinh"
-- tone.pace.raw: "chậm, trang trọng, mỗi giây là vĩnh hằng"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["đáng tiếc","vô ích"]
-- unmapped.rules.avoid_structure: ["câu hỏi tu từ mỉa mai"]
-- unmapped.rules.emotion_limit: "không giễu cợt, không bi quan"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'psychological_drama.sacrifice') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'psychological_drama.sacrifice',
       'psychological_drama.sacrifice',
       'LOW', 'bi tráng, tĩnh lặng, bao dung, đau nhưng thanh thản', true,
       'HIGH', 'HIGH', 'HIGH', 'LOW', 'NONE',
       'HIGH', 'VERY_SHORT', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ về sự chấp nhận',
        'ẩn dụ về ánh sáng, sự tự do, sự chuộc lỗi',
        'động từ chỉ sự từ bỏ và buông tay'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự chuyển giao (vật chất hoặc tinh thần)',
        'lời trăn trối hoặc ánh mắt đầy đủ',
        'cảm giác của người ở lại'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_anh_sang_tu_do_diep_cau_truc_can_xung', 'primary', 'intensity=medium; phrase=Ẩn dụ ánh sáng / tự do & điệp cấu trúc cân xứng'),
        ('phep_doi_buong_tay_nam_giu', 'secondary', 'intensity=medium; phrase=Phép đối buông tay – nắm giữ'),
        ('nhip_dieu_tram_deu', 'secondary', 'intensity=medium; phrase=Nhịp điệu trầm đều'),
        ('cau_hoi_tu_tu_mia_mai', 'avoid', 'intensity=medium; phrase=Câu hỏi tu từ mỉa mai'),
        ('ngon_ngu_bi_quan', 'avoid', 'intensity=medium; phrase=Ngôn ngữ bi quan')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
