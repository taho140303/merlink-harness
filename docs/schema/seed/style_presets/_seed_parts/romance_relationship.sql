-- ==== romance_relationship ====
-- preset: romance_relationship.confession | name: "Tỏ tình"
-- tone.pace.raw: "chậm đến trung bình (có cao trào)"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["chắc chắn","rõ ràng"]
-- unmapped.rules.avoid_structure: ["đoạn miêu tả ngoại cảnh dài"]
-- unmapped.rules.emotion_limit: "không hài hước, không mỉa mai"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'romance_relationship.confession') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'romance_relationship.confession',
       'romance_relationship.confession',
       'LOW', 'mong manh, dũng cảm, hồi hộp', true,
       'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ ẩn dụ tình cảm',
        'động từ bộc lộ cảm xúc',
        'từ chỉ sự do dự và quyết tâm'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lời nói trực tiếp',
        'phản ứng không lời',
        'khoảng lặng đầy ẩn ý'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_tinh_cam_cau_cham_lung_do_du', 'primary', 'intensity=medium; phrase=Ẩn dụ tình cảm & câu chấm lửng do dự'),
        ('diep_dong_tu_boc_lo_cam_xuc', 'secondary', 'intensity=medium; phrase=Điệp động từ bộc lộ cảm xúc'),
        ('phep_doi_khoang_lang_va_loi_noi', 'secondary', 'intensity=medium; phrase=Phép đối khoảng lặng và lời nói'),
        ('ngon_ngu_kho_khan_lanh_lung', 'avoid', 'intensity=medium; phrase=Ngôn ngữ khô khan, lạnh lùng'),
        ('cau_hoi_tu_tu_mia_mai', 'avoid', 'intensity=medium; phrase=Câu hỏi tu từ mỉa mai')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: romance_relationship.date | name: "Hẹn hò"
-- tone.pace.raw: "chậm, nhẹ nhàng"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["nguy hiểm","bắt buộc"]
-- unmapped.rules.avoid_structure: ["câu cộc lốc, câu cảm thán giận dữ"]
-- unmapped.rules.emotion_limit: "không căng thẳng, không bi kịch"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'romance_relationship.date') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'romance_relationship.date',
       'romance_relationship.date',
       'LOW', 'ấm áp, vui vẻ, thoải mái', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'MEDIUM', 'LOW', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ miêu tả sinh hoạt',
        'tính từ nhẹ nhàng',
        'câu hỏi quan tâm'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'tương tác vui vẻ',
        'những điều nhỏ nhặt',
        'sự đồng điệu'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_vui_ve_liet_ke_dieu_nho_nhat', 'primary', 'intensity=low; phrase=Điệp cấu trúc vui vẻ & liệt kê điều nhỏ nhặt'),
        ('cau_hoi_quan_tam', 'secondary', 'intensity=low; phrase=Câu hỏi quan tâm'),
        ('phep_doi_giua_hai_nhan_vat', 'secondary', 'intensity=low; phrase=Phép đối giữa hai nhân vật'),
        ('cam_than_gian_du', 'avoid', 'intensity=low; phrase=Cảm thán giận dữ'),
        ('ngon_ngu_cang_thang', 'avoid', 'intensity=low; phrase=Ngôn ngữ căng thẳng')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: romance_relationship.heartbreak | name: "Chia tay – đau lòng"
-- tone.pace.raw: "chậm, nặng nề"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["vui vẻ","hy vọng"]
-- unmapped.rules.avoid_structure: ["câu hỏi tu từ mỉa mai"]
-- unmapped.rules.emotion_limit: "không châm biếm, không hài hước"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'romance_relationship.heartbreak') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'romance_relationship.heartbreak',
       'romance_relationship.heartbreak',
       'LOW', 'đau buồn, tuyệt vọng, lặng lẽ', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE', 'HIGH', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ về nỗi đau',
        'động từ chỉ sự rạn vỡ',
        'tính từ u ám'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'nội tâm nhân vật',
        'lời chia tay cuối cùng',
        'hình ảnh tượng trưng cho mất mát'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_mat_mat_diep_tu_u_am', 'primary', 'intensity=medium; phrase=Ẩn dụ mất mát & điệp từ u ám'),
        ('cau_cham_lung_hut_hoi', 'secondary', 'intensity=medium; phrase=Câu chấm lửng hụt hơi'),
        ('phep_doi_ky_uc_hien_tai', 'secondary', 'intensity=medium; phrase=Phép đối ký ức – hiện tại'),
        ('ngon_ngu_vui_ve', 'avoid', 'intensity=medium; phrase=Ngôn ngữ vui vẻ'),
        ('cau_hoi_tu_tu_mia_mai', 'avoid', 'intensity=medium; phrase=Câu hỏi tu từ mỉa mai')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: romance_relationship.jealousy_confrontation | name: "Ghen tuông – đối chất"
-- tone.pace.raw: "nhanh, dồn dập"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["bình tĩnh","từ từ"]
-- unmapped.rules.avoid_structure: ["đoạn miêu tả dài"]
-- unmapped.rules.emotion_limit: "không buồn thảm, không bi lụy (dù có thể tiếc nuối)"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'romance_relationship.jealousy_confrontation') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'romance_relationship.jealousy_confrontation',
       'romance_relationship.jealousy_confrontation',
       'FAST', 'nóng giận, tủi thân, bực bội', true,
       'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'SIMPLE')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'câu cáo buộc',
        'từ ngữ sắc bén',
        'tính từ so sánh (anh ta hơn tôi?)'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lời nói xoắn xuýt',
        'sự thiếu tin tưởng',
        'những điều không nói ra'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('cau_cao_buoc_lap_cau_hoi_tu_tu_sac_ben', 'primary', 'intensity=high; phrase=Câu cáo buộc lặp & câu hỏi tu từ sắc bén'),
        ('diep_giong_cao_don_dap', 'secondary', 'intensity=high; phrase=Điệp giọng cao dồn dập'),
        ('phep_doi_anh_ta_toi', 'secondary', 'intensity=high; phrase=Phép đối ''anh ta – tôi'''),
        ('doan_mieu_ta_dai', 'avoid', 'intensity=high; phrase=Đoạn miêu tả dài'),
        ('ngon_ngu_buon_tham', 'avoid', 'intensity=high; phrase=Ngôn ngữ buồn thảm')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: romance_relationship.reunion | name: "Tái ngộ"
-- tone.pace.raw: "chậm khi nhận ra, vỡ òa khi tiếp cận"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["lạnh lùng","thờ ơ"]
-- unmapped.rules.avoid_structure: ["đoạn kể toàn sự kiện khô khan"]
-- unmapped.rules.emotion_limit: "không mỉa mai, không giễu cợt"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'romance_relationship.reunion') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'romance_relationship.reunion',
       'romance_relationship.reunion',
       'LOW', 'xúc động, nghẹn ngào, mừng mừng tủi tủi', true,
       'HIGH', 'HIGH', 'HIGH', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ gợi nhớ quá khứ',
        'tính từ chỉ cảm xúc dâng trào',
        'so sánh với ký ức'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự thay đổi theo thời gian',
        'câu nói dang dở',
        'cử chỉ thể hiện sự gắn bó'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('so_sanh_voi_ky_uc_diep_cau_noi_dang_do', 'primary', 'intensity=medium; phrase=So sánh với ký ức & điệp câu nói dang dở'),
        ('phep_doi_truoc_sau_theo_thoi_gian', 'secondary', 'intensity=medium; phrase=Phép đối ''trước – sau'' theo thời gian'),
        ('cau_cham_lung_nghen_ngao', 'secondary', 'intensity=medium; phrase=Câu chấm lửng nghẹn ngào'),
        ('ngon_ngu_tho_o', 'avoid', 'intensity=medium; phrase=Ngôn ngữ thờ ơ'),
        ('ke_su_kien_kho_khan', 'avoid', 'intensity=medium; phrase=Kể sự kiện khô khan')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: romance_relationship.wedding_ceremony | name: "Lễ cưới / Đính hôn"
-- tone.pace.raw: "trang trọng, chậm rãi"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["tạm bợ","chia ly"]
-- unmapped.rules.avoid_structure: ["câu cụt, thiếu chủ ngữ"]
-- unmapped.rules.emotion_limit: "không bi quan, không giễu cợt"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'romance_relationship.wedding_ceremony') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'romance_relationship.wedding_ceremony',
       'romance_relationship.wedding_ceremony',
       'LOW', 'hạnh phúc, thiêng liêng, xúc động', true,
       'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'MEDIUM', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ về sự gắn kết',
        'ẩn dụ cho tương lai',
        'tính từ long trọng, tôn kính'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lời thề và cam kết',
        'khoảnh khắc chuyển giao',
        'cảm xúc của nhân vật và khách mời'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_gan_ket_diep_cau_truc_trang_trong_can_xung', 'primary', 'intensity=medium; phrase=Ẩn dụ gắn kết & điệp cấu trúc trang trọng cân xứng'),
        ('phep_doi_loi_the', 'secondary', 'intensity=medium; phrase=Phép đối lời thề'),
        ('liet_ke_khoanh_khac_chuyen_giao', 'secondary', 'intensity=medium; phrase=Liệt kê khoảnh khắc chuyển giao'),
        ('cau_cut_thieu_chu_ngu', 'avoid', 'intensity=medium; phrase=Câu cụt thiếu chủ ngữ'),
        ('ngon_ngu_bi_quan', 'avoid', 'intensity=medium; phrase=Ngôn ngữ bi quan')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
