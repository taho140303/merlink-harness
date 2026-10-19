-- ==== transition ====
-- preset: transition.recovery_rest | name: "Hồi phục & nghỉ ngơi"
-- tone.pace.raw: "chậm, êm dịu, dễ chịu"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["vội vã","khẩn cấp"]
-- unmapped.rules.avoid_structure: ["câu cụt giật gân"]
-- unmapped.rules.emotion_limit: "không căng thẳng, không lo âu"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'transition.recovery_rest') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'transition.recovery_rest',
       'transition.recovery_rest',
       'LOW', 'thư thái, hồi phục, bình yên', true,
       'MEDIUM', 'LOW', 'MEDIUM', 'MEDIUM', 'LOW',
       'MEDIUM', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ êm dịu', 'ẩn dụ chữa lành', 'động từ buông lỏng'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'cảm giác hồi phục', 'thời gian trôi chậm', 'sự trở về cân bằng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_chua_lanh_diep_nhip_em_diu', 'primary', 'intensity=low; phrase=Ẩn dụ chữa lành & điệp nhịp êm dịu'),
        ('phep_doi_truoc_sau', 'secondary', 'intensity=low; phrase=Phép đối trước – sau'),
        ('tuong_thanh_nhe_nhang', 'secondary', 'intensity=low; phrase=Tượng thanh nhẹ nhàng'),
        ('ngon_ngu_gap_gap', 'avoid', 'intensity=low; phrase=Ngôn ngữ gấp gáp'),
        ('kich_tinh', 'avoid', 'intensity=low; phrase=Kịch tính')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: transition.time_skip | name: "Nhảy thời gian"
-- tone.pace.raw: "nhanh khi lướt qua, chậm ở điểm nhấn"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["ngày qua ngày","nhàm chán"]
-- unmapped.rules.avoid_structure: ["đoạn văn kể lể chi tiết"]
-- unmapped.rules.emotion_limit: "không tiếc nuối thái quá, không phấn khích quá đà"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'transition.time_skip') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'transition.time_skip',
       'transition.time_skip',
       'FAST', 'hoài niệm, mới mẻ, trăn trở về thay đổi', true,
       'MEDIUM', 'LOW', 'LOW', 'LOW', 'NONE',
       'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ chỉ sự trôi chảy thời gian', 'ẩn dụ quá khứ – hiện tại', 'động từ chuyển giao'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự thay đổi', 'điều giữ nguyên', 'mất mát thời gian'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_mua_thang_phep_doi_truoc_sau', 'primary', 'intensity=medium; phrase=Phép liệt kê mùa / tháng & phép đối trước – sau'),
        ('an_du_dong_song_thoi_gian', 'secondary', 'intensity=medium; phrase=Ẩn dụ dòng sông thời gian'),
        ('cau_lung_hoai_niem', 'secondary', 'intensity=medium; phrase=Câu lửng hoài niệm'),
        ('mieu_ta_chi_tiet_tung_ngay', 'avoid', 'intensity=medium; phrase=Miêu tả chi tiết từng ngày'),
        ('cau_chu_dai_dong', 'avoid', 'intensity=medium; phrase=Câu chữ dài dòng')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: transition.travel_montage | name: "Montage hành trình"
-- tone.pace.raw: "nhanh, cắt cảnh liên tục, nhịp lướt"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["dừng lại","trì trệ"]
-- unmapped.rules.avoid_structure: ["đoạn văn tĩnh tại dài"]
-- unmapped.rules.emotion_limit: "không chán nản, không mệt mỏi tuyệt vọng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'transition.travel_montage') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'transition.travel_montage',
       'transition.travel_montage',
       'FAST', 'phiêu lưu, phấn khích, hân hoan', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'MEDIUM', 'LOW',
       'LOW', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ di chuyển', 'từ miêu tả địa danh', 'từ tượng thanh chuyển cảnh'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự thay đổi khung cảnh', 'chặng đường', 'cảm giác rộng mở'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_liet_ke_chang_duong_diep_cau_truc_cat_canh', 'primary', 'intensity=medium; phrase=Phép liệt kê chặng đường & điệp cấu trúc cắt cảnh'),
        ('phep_doi_vung_dat_nay_vung_dat_kia', 'secondary', 'intensity=medium; phrase=Phép đối vùng đất này – vùng đất kia'),
        ('tuong_thanh_nhip_di_chuyen', 'secondary', 'intensity=medium; phrase=Tượng thanh nhịp di chuyển'),
        ('mieu_ta_tinh_lang_keo_dai', 'avoid', 'intensity=medium; phrase=Miêu tả tĩnh lặng kéo dài'),
        ('suy_tu_noi_tam_dai', 'avoid', 'intensity=medium; phrase=Suy tư nội tâm dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: transition.waiting_anticipation | name: "Chờ đợi & hồi hộp"
-- tone.pace.raw: "chậm, kéo dài, từng giây nặng nề"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["nhanh","ngay lập tức"]
-- unmapped.rules.avoid_structure: ["đoạn văn hành động gấp gáp"]
-- unmapped.rules.emotion_limit: "không tuyệt vọng quá đà, không hài hước"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'transition.waiting_anticipation') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'transition.waiting_anticipation',
       'transition.waiting_anticipation',
       'LOW', 'hồi hộp, sốt ruột, lo lắng, hy vọng', true,
       'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'NONE',
       'MEDIUM', 'LONG', 'MEDIUM', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ chỉ sự trì hoãn', 'động từ chờ đợi', 'ẩn dụ thời gian chậm lại'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'sự trôi qua của thời gian', 'phản ứng cơ thể', 'khoảnh khắc sắp vỡ'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_chua_an_du_thoi_gian_cham_lai', 'primary', 'intensity=medium; phrase=Điệp cấu trúc ''chưa...'' & ẩn dụ thời gian chậm lại'),
        ('liet_ke_dau_hieu_mong_manh', 'secondary', 'intensity=medium; phrase=Liệt kê dấu hiệu mong manh'),
        ('cau_hoi_tu_tu_lo_lang', 'secondary', 'intensity=medium; phrase=Câu hỏi tu từ lo lắng'),
        ('ngon_ngu_on_ao', 'avoid', 'intensity=medium; phrase=Ngôn ngữ ồn ào'),
        ('hanh_dong_don_dap', 'avoid', 'intensity=medium; phrase=Hành động dồn dập')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

