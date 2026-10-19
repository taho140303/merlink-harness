-- ==== political_social ====

-- preset: political_social.conspiracy_meeting | name: "Cuộc họp bí mật"
-- tone.pace.raw: "chậm, dè chừng, thì thầm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["trung thực","ngay thẳng"]
-- unmapped.rules.avoid_structure: ["đoạn văn cởi mở, chân thành"]
-- unmapped.rules.emotion_limit: "không ngây thơ, không phấn khích"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'political_social.conspiracy_meeting') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'political_social.conspiracy_meeting',
       'political_social.conspiracy_meeting',
       'LOW', 'nguy hiểm, hoài nghi, mưu mô', true,
       'MEDIUM', 'HIGH', 'LOW', 'LOW', 'NONE',
       'HIGH', 'LONG', 'MEDIUM', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ bóng gió, ẩn ý',
        'thuật ngữ quyền lực',
        'động từ dàn xếp'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'ẩn ý trong lời nói',
        'lòng trung thành',
        'hậu quả tiềm tàng'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_bong_toi_loi_noi_bong_gio', 'primary', 'intensity=medium; phrase=Ẩn dụ bóng tối & lời nói bóng gió'),
        ('phep_doi_trung_thanh_phan_boi', 'secondary', 'intensity=medium; phrase=Phép đối trung thành – phản bội'),
        ('diep_cau_truc_trao_doi_nua_chung', 'secondary', 'intensity=medium; phrase=Điệp cấu trúc trao đổi nửa chừng'),
        ('ngon_ngu_thang_thung', 'avoid', 'intensity=medium; phrase=Ngôn ngữ thẳng thừng'),
        ('hai_huoc', 'avoid', 'intensity=medium; phrase=Hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: political_social.coronation_succession | name: "Đăng quang & kế vị"
-- tone.pace.raw: "trang trọng, chậm rãi"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["tạm bợ","vô nghĩa"]
-- unmapped.rules.avoid_structure: ["câu cụt cộc lốc"]
-- unmapped.rules.emotion_limit: "không giễu cợt, không thờ ơ"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'political_social.coronation_succession') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'political_social.coronation_succession',
       'political_social.coronation_succession',
       'LOW', 'uy nghiêm, căng thẳng ngầm, long trọng', true,
       'HIGH', 'HIGH', 'MEDIUM', 'MEDIUM', 'LOW',
       'MEDIUM', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ hoàng gia',
        'ẩn dụ quyền lực',
        'từ trang trọng'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lễ nghi',
        'quyền lực chuyển giao',
        'ẩn ý chính trị'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_tuyen_the_an_du_vuong_mien_nang_triu', 'primary', 'intensity=medium; phrase=Điệp cấu trúc tuyên thệ & ẩn dụ vương miện nặng trĩu'),
        ('phep_doi_ke_tri_nguoi_tri', 'secondary', 'intensity=medium; phrase=Phép đối kẻ trị – người trị'),
        ('liet_ke_bieu_tuong_quyen_luc', 'secondary', 'intensity=medium; phrase=Liệt kê biểu tượng quyền lực'),
        ('ngon_ngu_doi_thuong', 'avoid', 'intensity=medium; phrase=Ngôn ngữ đời thường'),
        ('hai_huoc', 'avoid', 'intensity=medium; phrase=Hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: political_social.exile_expulsion | name: "Lưu đày & trục xuất"
-- tone.pace.raw: "chậm, nặng nề, đổ vỡ"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["may mắn","thuận lợi"]
-- unmapped.rules.avoid_structure: ["đoạn văn khô khan sự kiện"]
-- unmapped.rules.emotion_limit: "không hài hước, không nhẹ nhàng"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'political_social.exile_expulsion') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'political_social.exile_expulsion',
       'political_social.exile_expulsion',
       'LOW', 'tủi nhục, phẫn uất, cô độc, mất gốc', true,
       'HIGH', 'MEDIUM', 'MEDIUM', 'LOW', 'LOW',
       'HIGH', 'LONG', 'MEDIUM', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ mất mát quê hương',
        'ẩn dụ rễ bị nhổ',
        'động từ rời bỏ'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'khoảnh khắc bị đuổi',
        'những gì để lại',
        'nỗi đau của người lưu đày'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('an_du_re_bi_nho_diep_cau_truc_ngoanh_lai', 'primary', 'intensity=high; phrase=Ẩn dụ rễ bị nhổ & điệp cấu trúc ngoảnh lại'),
        ('phep_doi_truoc_sau_khi_luu_day', 'secondary', 'intensity=high; phrase=Phép đối trước – sau khi lưu đày'),
        ('cau_cham_lung_nghen', 'secondary', 'intensity=high; phrase=Câu chấm lửng nghẹn'),
        ('ngon_ngu_vui_ve', 'avoid', 'intensity=high; phrase=Ngôn ngữ vui vẻ'),
        ('mieu_ta_ngoai_canh_dai', 'avoid', 'intensity=high; phrase=Miêu tả ngoại cảnh dài')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: political_social.negotiation_treaty | name: "Đàm phán & hiệp ước"
-- tone.pace.raw: "chậm rãi, tính toán từng lời"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["chắc chắn tuyệt đối","đơn giản"]
-- unmapped.rules.avoid_structure: ["câu cụt giận dữ"]
-- unmapped.rules.emotion_limit: "không nóng giận, không hài hước"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'political_social.negotiation_treaty') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'political_social.negotiation_treaty',
       'political_social.negotiation_treaty',
       'LOW', 'căng thẳng, khôn khéo, thận trọng', true,
       'MEDIUM', 'HIGH', 'LOW', 'NONE', 'NONE',
       'HIGH', 'LONG', 'LOW', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ đàm phán',
        'từ ngữ lập luận, nhượng bộ',
        'động từ mặc cả'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'điểm mấu chốt',
        'sự nhượng bộ',
        'con số và điều kiện'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_doi_hai_phe_lap_dieu_kien_de_nhan', 'primary', 'intensity=medium; phrase=Phép đối hai phe & lặp điều kiện để nhấn'),
        ('cau_hoi_tu_tu_tham_do', 'secondary', 'intensity=medium; phrase=Câu hỏi tu từ thăm dò'),
        ('liet_ke_dieu_khoan', 'secondary', 'intensity=medium; phrase=Liệt kê điều khoản'),
        ('ngon_ngu_cam_xuc', 'avoid', 'intensity=medium; phrase=Ngôn ngữ cảm xúc'),
        ('hai_huoc', 'avoid', 'intensity=medium; phrase=Hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: political_social.public_speech | name: "Diễn thuyết trước công chúng"
-- tone.pace.raw: "nhịp nhàng, hùng hồn, dâng cao"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["lưỡng lự","có thể là"]
-- unmapped.rules.avoid_structure: ["câu dài rối rắm"]
-- unmapped.rules.emotion_limit: "không mỉa mai, không bi quan"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'political_social.public_speech') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'political_social.public_speech',
       'political_social.public_speech',
       'MODERATE', 'đầy cảm hứng, thuyết phục, oai hùng', true,
       'MEDIUM', 'HIGH', 'LOW', 'LOW', 'NONE',
       'MEDIUM', 'SHORT', 'HIGH', 'MIXED')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'từ ngữ kêu gọi, hô hào',
        'ẩn dụ tập thể',
        'động từ hành động'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'lời kêu gọi',
        'khát vọng của đám đông',
        'cú đấm lập luận'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('diep_cau_truc_keu_goi_cau_hoi_tu_tu_thuyet_phuc', 'primary', 'intensity=high; phrase=Điệp cấu trúc kêu gọi & câu hỏi tu từ thuyết phục'),
        ('phep_liet_ke_nguyen_vong', 'secondary', 'intensity=high; phrase=Phép liệt kê nguyện vọng'),
        ('phep_doi_hom_nay_ngay_mai', 'secondary', 'intensity=high; phrase=Phép đối hôm nay – ngày mai'),
        ('ngon_ngu_rut_re', 'avoid', 'intensity=high; phrase=Ngôn ngữ rụt rè'),
        ('cau_chu_lung_cung', 'avoid', 'intensity=high; phrase=Câu chữ lủng củng')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;

-- preset: political_social.trial_court | name: "Phiên tòa"
-- tone.pace.raw: "trang trọng, căng thẳng từng lời"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["cảm tính","vô căn cứ"]
-- unmapped.rules.avoid_structure: ["đoạn văn trào phúng"]
-- unmapped.rules.emotion_limit: "không mỉa mai, không thiên vị cảm xúc"
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = 'political_social.trial_court') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       'political_social.trial_court',
       'political_social.trial_court',
       'LOW', 'nghiêm trang, hồi hộp, chờ phán quyết', true,
       'MEDIUM', 'HIGH', 'LOW', 'NONE', 'NONE',
       'HIGH', 'LONG', 'MEDIUM', 'COMPLEX')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'thuật ngữ pháp lý',
        'từ ngữ buộc tội / bào chữa',
        'lập luận chứng cứ'
      ]) AS v;

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'bằng chứng',
        'lời khai',
        'công lý'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('phep_lap_cau_hoi_doi_chat_liet_ke_chung_cu', 'primary', 'intensity=high; phrase=Phép lặp câu hỏi đối chất & liệt kê chứng cứ'),
        ('phep_doi_buoc_toi_bao_chua', 'secondary', 'intensity=high; phrase=Phép đối buộc tội – bào chữa'),
        ('cau_hoi_tu_tu_gay_ap_luc', 'secondary', 'intensity=high; phrase=Câu hỏi tu từ gây áp lực'),
        ('ngon_ngu_cam_xuc_ca_nhan', 'avoid', 'intensity=high; phrase=Ngôn ngữ cảm xúc cá nhân'),
        ('hai_huoc', 'avoid', 'intensity=high; phrase=Hài hước')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
