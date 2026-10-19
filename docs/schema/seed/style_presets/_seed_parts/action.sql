-- ==== action ====
-- preset: action.assassination | name: "Hành động ám sát"
-- tone.pace.raw: "rất chậm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không bộc lộ cảm xúc thừa thãi"
-- unmapped.genre: "action"
-- unmapped.settings: "không gian kín, bóng tối, bí mật (Để sau)"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.assassination') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.assassination',
       'action.assassination',
       'VERY_LOW', 'căng thẳng nghẹt thở', true,
       'HIGH', 'LOW', 'HIGH', 'MEDIUM', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ tĩnh',
        'từ ám chỉ',
        'từ mang tính ẩn dụ nhẹ'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'thời điểm',
        'góc nhìn',
        'bóng tối',
        'sự im lặng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du', 'primary', 'intensity=medium; phrase=Ẩn dụ'),
        ('diep_cau_truc_cau_cut_mo_ho', 'secondary', 'intensity=medium; phrase=Điệp cấu trúc câu cụt mơ hồ'),
        ('dao_ngu_nhan_khoanh_khac_tiep_xuc', 'secondary', 'intensity=medium; phrase=Đảo ngữ nhấn khoảnh khắc tiếp xúc'),
        ('tuong_thanh_on', 'avoid', 'intensity=medium; phrase=Tượng thanh ồn'),
        ('cau_hoi_tu_tu_boc_phat', 'avoid', 'intensity=medium; phrase=Câu hỏi tu từ bộc phát')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.battle | name: "Hành động chiến trận"
-- tone.pace.raw: "nhanh, dồn dập"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không tập trung vào cá nhân, mang tính đám đông"
-- unmapped.genre: "action"
-- unmapped.settings: "chiến trường đông đúc, hỗn loạn, nhiều bên tham chiến"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.battle') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.battle',
       'action.battle',
       'FAST', 'hỗn loạn, khẩn trương', true,
       'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ mạnh',
        'từ chỉ số lượng',
        'từ tượng thanh hỗn tạp'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'tương tác nhóm',
        'hỗn loạn',
        'lợi thế chiến thuật tổng thể'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_doi_liet_ke_hon_tap', 'primary', 'intensity=high; phrase=Phép đối & liệt kê hỗn tạp'),
        ('tuong_thanh_don_dap', 'secondary', 'intensity=high; phrase=Tượng thanh dồn dập'),
        ('diep_tu_chi_so_luong_dam_dong', 'secondary', 'intensity=high; phrase=Điệp từ chỉ số lượng đám đông'),
        ('tap_trung_vao_ca_nhan', 'avoid', 'intensity=high; phrase=Tập trung vào cá nhân'),
        ('nhip_van_cham', 'avoid', 'intensity=high; phrase=Nhịp văn chậm')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.chase | name: "Hành động truy đuổi"
-- tone.pace.raw: "rất nhanh"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không suy tư dài dòng"
-- unmapped.genre: "action"
-- unmapped.settings: "không gian mở, tốc độ, liên tục"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.chase') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.chase',
       'action.chase',
       'VERY_FAST', 'gấp gáp, khẩn cấp', true,
       'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'VERY_SHORT', 'MEDIUM', 'SIMPLE')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ chuyển động',
        'từ chỉ tốc độ',
        'từ tượng thanh'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'quãng đường',
        'vật cản',
        'khoảng cách',
        'hơi thở'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('tuong_thanh_nhip_do_diep_khuc_xa_gan', 'primary', 'intensity=high; phrase=Tượng thanh nhịp độ & điệp khúc ''xa/gần'''),
        ('liet_ke_chuong_ngai', 'secondary', 'intensity=high; phrase=Liệt kê chướng ngại'),
        ('cau_cut_lien_tiep_khong_lien_tu', 'secondary', 'intensity=high; phrase=Câu cụt liên tiếp không liên từ'),
        ('cau_ghep_dai', 'avoid', 'intensity=high; phrase=Câu ghép dài'),
        ('an_du_tru_tinh', 'avoid', 'intensity=high; phrase=Ẩn dụ trữ tình')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.combat | name: "Hành động đối kháng"
-- tone.pace.raw: "rất nhanh"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không châm biếm"
-- unmapped.genre: "action"
-- unmapped.settings: "chiến đấu cận chiến, đối đầu trực tiếp"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.combat') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.combat',
       'action.combat',
       'VERY_FAST', 'căng thẳng', true,
       'HIGH', 'HIGH', 'HIGH', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'SIMPLE')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ mạnh',
        'từ tượng thanh'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'chuyển động',
        'va chạm',
        'phản xạ'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_dong_tu_manh_tuong_thanh_va_cham', 'primary', 'intensity=high; phrase=Điệp động từ mạnh & tượng thanh va chạm'),
        ('cau_cut_lien_tiep', 'secondary', 'intensity=high; phrase=Câu cụt liên tiếp'),
        ('phep_liet_ke_don_danh', 'secondary', 'intensity=high; phrase=Phép liệt kê đòn đánh'),
        ('an_du_tru_tinh', 'avoid', 'intensity=high; phrase=Ẩn dụ trữ tình'),
        ('cau_ghep_dai', 'avoid', 'intensity=high; phrase=Câu ghép dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.duel | name: "Hành động đấu tay đôi"
-- tone.pace.raw: "chậm"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không châm biếm, tập trung vào tâm lý đấu trí"
-- unmapped.genre: "action"
-- unmapped.settings: "không gian hẹp, đối thủ ngang tài"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.duel') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.duel',
       'action.duel',
       'LOW', 'căng thẳng âm ỉ', true,
       'HIGH', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ chính xác',
        'thuật ngữ kỹ thuật',
        'từ miêu tả chuyển động nhỏ'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'thế đứng',
        'ánh mắt',
        'hơi thở',
        'nhịp độ'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_the_dung_nhip_tho', 'primary', 'intensity=high; phrase=Điệp cấu trúc thế đứng & nhịp thở'),
        ('phep_doi_giua_hai_doi_thu', 'secondary', 'intensity=high; phrase=Phép đối giữa hai đối thủ'),
        ('so_sanh_chuyen_dong_chinh_xac', 'secondary', 'intensity=high; phrase=So sánh chuyển động chính xác'),
        ('tuong_thanh_on_ao', 'avoid', 'intensity=high; phrase=Tượng thanh ồn ào'),
        ('mieu_ta_ngoai_canh_dai', 'avoid', 'intensity=high; phrase=Miêu tả ngoại cảnh dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.escape | name: "Hành động chạy trốn"
-- tone.pace.raw: "rất nhanh"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không triết lý dài dòng"
-- unmapped.genre: "action"
-- unmapped.settings: "không gian bị kìm kẹp, áp lực thời gian"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.escape') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.escape',
       'action.escape',
       'VERY_FAST', 'hoảng loạn, khẩn cấp', true,
       'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'NONE', 'LOW', 'SHORT', 'HIGH', 'SIMPLE')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ chạy, ẩn nấp',
        'từ chỉ thời gian'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lối thoát',
        'sự truy đuổi',
        'chướng ngại',
        'thời gian'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('liet_ke_don_dap_cau_cut_lien_tiep_tao_hieu_ung_ngop_tho', 'primary', 'intensity=high; phrase=Liệt kê dồn dập & câu cụt liên tiếp (tạo hiệu ứng ngộp thở)'),
        ('tuong_thanh_tieng_dong_duoi_theo', 'secondary', 'intensity=high; phrase=Tượng thanh tiếng động đuổi theo'),
        ('dao_ngu_nhan_loi_thoat', 'secondary', 'intensity=high; phrase=Đảo ngữ nhấn lối thoát'),
        ('cau_dai', 'avoid', 'intensity=high; phrase=Câu dài'),
        ('an_du_triet_ly', 'avoid', 'intensity=high; phrase=Ẩn dụ triết lý')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.execution | name: "Hành động kết liễu"
-- tone.pace.raw: "cực chậm"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không cảm xúc, tập trung vào sự kiện"
-- unmapped.genre: "action"
-- unmapped.settings: "khoảnh khắc cuối, ít chuyển động"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.execution') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.execution',
       'action.execution',
       'VERY_LOW', 'lạnh lùng, dứt khoát', true,
       'HIGH', 'LOW', 'HIGH', 'LOW', 'NONE', 'LOW', 'VERY_SHORT', 'LOW', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ dứt khoát',
        'từ ngắn gọn',
        'từ mang tính kết thúc'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'khoảnh khắc',
        'điểm tiếp xúc',
        'kết thúc',
        'hệ quả'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_tu', 'primary', 'intensity=low; phrase=Điệp từ'),
        ('dao_ngu_dua_tan_ngu_trang_ngu_len_truoc', 'secondary', 'intensity=low; phrase=Đảo ngữ (đưa tân ngữ/trạng ngữ lên trước)'),
        ('cau_rut_gon_chu_ngu', 'secondary', 'intensity=low; phrase=Câu rút gọn chủ ngữ'),
        ('an_du_hoa_my', 'avoid', 'intensity=low; phrase=Ẩn dụ hoa mỹ'),
        ('so_sanh_dai_dong', 'avoid', 'intensity=low; phrase=So sánh dài dòng'),
        ('cau_hoi_tu_tu', 'avoid', 'intensity=low; phrase=Câu hỏi tu từ')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.infiltration | name: "Hành động xâm nhập"
-- tone.pace.raw: "chậm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không bộc lộ cảm xúc"
-- unmapped.genre: "action"
-- unmapped.settings: "khu vực địch, yêu cầu ngụy trang, bí mật"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.infiltration') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.infiltration',
       'action.infiltration',
       'LOW', 'thận trọng, căng thẳng ngầm', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'NONE', 'MEDIUM', 'MEDIUM', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ chỉ quan sát',
        'động từ lén lút',
        'thuật ngữ trinh sát'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'quan sát',
        'ngụy trang',
        'đường đi',
        'lịch trình'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('so_sanh_ngam_an_du_chuyen_doi_cam_giac', 'primary', 'intensity=medium; phrase=So sánh ngầm (Ẩn dụ chuyển đổi cảm giác)'),
        ('cau_rut_gon_canh_bao', 'secondary', 'intensity=medium; phrase=Câu rút gọn cảnh báo'),
        ('liet_ke_huong_dan_di_chuyen_re_tranh_luot', 'secondary', 'intensity=medium; phrase=Liệt kê hướng dẫn di chuyển (rẽ, tránh, lướt)'),
        ('diep_cau_truc_qua_deu_dan', 'avoid', 'intensity=medium; phrase=Điệp cấu trúc quá đều đặn'),
        ('tuong_thanh_on_ao', 'avoid', 'intensity=medium; phrase=Tượng thanh ồn ào')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.showdown | name: "Hành động đối đầu cuối cùng"
-- tone.pace.raw: "chậm đến cực chậm"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "cho phép cảm xúc bộc phát có kiểm soát"
-- unmapped.genre: "action"
-- unmapped.settings: "thời điểm quyết định, không gian tượng trưng"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.showdown') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.showdown',
       'action.showdown',
       'VERY_LOW', 'cực kỳ căng thẳng, xúc cảm dâng trào', true,
       'HIGH', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'HIGH', 'LONG', 'HIGH', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'động từ nặng',
        'từ gợi cảm xúc',
        'ẩn dụ'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'ánh mắt',
        'khoảnh khắc',
        'hậu quả',
        'quyết định'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_doi_canh_nguoi_qua_khu_hien_tai_ben_trong_ben_ngoai', 'primary', 'intensity=high; phrase=Phép đối (cảnh - người, quá khứ - hiện tại, bên trong - bên ngoài)'),
        ('cau_hoi_tu_tu_ve_so_phan', 'secondary', 'intensity=high; phrase=Câu hỏi tu từ về số phận'),
        ('an_du_manh_anh_sang_bong_toi_song_nui', 'secondary', 'intensity=high; phrase=Ẩn dụ mạnh (ánh sáng/bóng tối, sông/núi)'),
        ('diep_tu_nhan_thoi_khac', 'secondary', 'intensity=high; phrase=Điệp từ nhấn thời khắc'),
        ('mia_mai_cham_biem', 'avoid', 'intensity=high; phrase=Mỉa mai châm biếm'),
        ('cau_cut_vo_cam', 'avoid', 'intensity=high; phrase=Câu cụt vô cảm')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.survival | name: "Hành động sinh tồn"
-- tone.pace.raw: "chậm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không lạc quan thái quá"
-- unmapped.genre: "action"
-- unmapped.settings: "môi trường khắc nghiệt, nguồn lực hạn chế"
-- unmapped.rhetorical_devices.examples: []
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.survival') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.survival',
       'action.survival',
       'LOW', 'khắc khoải, tuyệt vọng nhẹ', true,
       'HIGH', 'MEDIUM', 'HIGH', 'HIGH', 'MEDIUM', 'LOW', 'MEDIUM', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ cảm giác cơ thể',
        'từ chỉ thời tiết, địa hình',
        'động từ hành động cụ thể'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'môi trường',
        'cơ thể',
        'mệt mỏi',
        'vật dụng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('so_sanh_cam_giac', 'primary', 'intensity=medium; phrase=So sánh cảm giác'),
        ('diep_tu_chi_su_met_moi_kiet_suc', 'secondary', 'intensity=medium; phrase=Điệp từ chỉ sự mệt mỏi, kiệt sức'),
        ('cau_hoi_tu_tu_tham_thi_trong_suy_nghi', 'secondary', 'intensity=medium; phrase=Câu hỏi tu từ thầm thì trong suy nghĩ'),
        ('lac_quan_thai_qua', 'avoid', 'intensity=medium; phrase=Lạc quan thái quá'),
        ('tu_ngu_hanh_dong_hon_ho', 'avoid', 'intensity=medium; phrase=Từ ngữ hành động hớn hở')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.training | name: "Hành động luyện tập"
-- tone.pace.raw: "chậm đến trung bình"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không kịch tính, mang tính hướng dẫn"
-- unmapped.genre: "action"
-- unmapped.settings: "không gian an toàn, bài bản, có hướng dẫn"
-- unmapped.rhetorical_devices.examples: [""]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.training') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.training',
       'action.training',
       'LOW', 'tập trung, quyết tâm', true,
       'MEDIUM', 'MEDIUM', 'HIGH', 'LOW', 'NONE', 'MEDIUM', 'LONG', 'MEDIUM', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ kỹ thuật',
        'động từ hướng dẫn',
        'từ chỉ động tác'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'kỹ thuật',
        'sửa sai',
        'lặp lại',
        'cơ thể'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('liet_ke_trinh_tu_cau_menh_lenh', 'primary', 'intensity=low; phrase=Liệt kê trình tự & câu mệnh lệnh'),
        ('lap_cau_truc_dieu_kien_neu_thi', 'secondary', 'intensity=low; phrase=Lặp cấu trúc điều kiện (Nếu... thì...)'),
        ('so_sanh_doi_chieu_sua_sai', 'secondary', 'intensity=low; phrase=So sánh đối chiếu sửa sai'),
        ('an_du', 'avoid', 'intensity=low; phrase=Ẩn dụ'),
        ('diep_tu_cam_xuc', 'avoid', 'intensity=low; phrase=Điệp từ cảm xúc'),
        ('cau_hoi_tu_tu', 'avoid', 'intensity=low; phrase=Câu hỏi tu từ')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: action.warfare | name: "Hành động chiến tranh"
-- tone.pace.raw: "chậm đến trung bình"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: []
-- unmapped.rules.avoid_structure: []
-- unmapped.rules.emotion_limit: "không cảm xúc cá nhân, thiên về báo cáo"
-- unmapped.genre: "action"
-- unmapped.settings: "chiến trường rộng lớn, nhiều giai đoạn, hậu cần"
-- unmapped.rhetorical_devices.examples: []
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'action.warfare') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'action.warfare',
       'action.warfare',
       'LOW', 'lạnh lùng, tính toán', true,
       'HIGH', 'MEDIUM', 'LOW', 'LOW', 'NONE', 'HIGH', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ quân sự',
        'từ chỉ chiến thuật',
        'từ chỉ quy mô'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'chiến lược',
        'hậu cần',
        'quy mô',
        'thời gian'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('liet_ke_toan_canh_diep_cau_truc_so_sanh_luc_luong', 'primary', 'intensity=low; phrase=Liệt kê toàn cảnh & điệp cấu trúc so sánh lực lượng'),
        ('cau_ghep_dai_mang_tinh_thong_ke', 'secondary', 'intensity=low; phrase=Câu ghép dài mang tính thống kê'),
        ('an_du_ve_co_vua_hoac_ban_co', 'secondary', 'intensity=low; phrase=Ẩn dụ về cờ vua hoặc bàn cờ'),
        ('cam_xuc_ca_nhan', 'avoid', 'intensity=low; phrase=Cảm xúc cá nhân'),
        ('tuong_thanh_hon_loan', 'avoid', 'intensity=low; phrase=Tượng thanh hỗn loạn')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

