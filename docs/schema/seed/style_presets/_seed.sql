-- ============================================================
-- STYLE_PRESETS SEED
-- Generated: 2026-10-18T23:00:58+07:00
-- Source: docs/library/style_presets/<folder>/*.json
-- Spec: docs/schema/system.dbml (SYSTEM_PROMPT.STYLE_PRESET)
-- ============================================================
-- Usage:
--   psql -d <db> -f docs/library/style_presets/_seed.sql
-- Requirements:
--   - PostgreSQL 13+ (gen_random_uuid)
--   - Empty style_presets / rhetorical_devices tables
--   - Idempotent: re-run skips existing presets/devices
-- Notes:
--   - 'code' and 'category' both = '<folder>.<file>' (Sprint 4 may split)
--   - Human-readable name preserved in per-preset comments only
--   - Unmapped JSON fields (rules.avoid_words, etc.) in per-preset comments
--   - Root example.json intentionally skipped per user decision
-- ============================================================
BEGIN;

-- rhetorical_devices — 243 unique devices (merged from 9 sources)
INSERT INTO rhetorical_devices (id, code, description)
SELECT gen_random_uuid(), v.code, v.description
FROM (VALUES
  ('an_du', 'Ẩn dụ'),
  ('an_du_anh_sang_tu_do_diep_cau_truc_can_xung', 'Ẩn dụ ánh sáng / tự do & điệp cấu trúc cân xứng'),
  ('an_du_bi_mat_chon_vui_diep_cau_truc_do_duong', 'Ẩn dụ bí mật chôn vùi & điệp cấu trúc dò đường'),
  ('an_du_bong_toi_loi_noi_bong_gio', 'Ẩn dụ bóng tối & lời nói bóng gió'),
  ('an_du_buc_man_duoc_ven_len', 'Ẩn dụ bức màn được vén lên'),
  ('an_du_cat_chay_nguoc', 'Ẩn dụ cát chảy ngược'),
  ('an_du_chua_lanh_diep_nhip_em_diu', 'Ẩn dụ chữa lành & điệp nhịp êm dịu'),
  ('an_du_dong_song_thoi_gian', 'Ẩn dụ dòng sông thời gian'),
  ('an_du_gan_ket_diep_cau_truc_trang_trong_can_xung', 'Ẩn dụ gắn kết & điệp cấu trúc trang trọng cân xứng'),
  ('an_du_hoa_my', 'Ẩn dụ hoa mỹ'),
  ('an_du_lua_song', 'Ẩn dụ lửa / sóng'),
  ('an_du_manh_anh_sang_bong_toi_song_nui', 'Ẩn dụ mạnh (ánh sáng/bóng tối, sông/núi)'),
  ('an_du_mat_mat_diep_tu_u_am', 'Ẩn dụ mất mát & điệp từ u ám'),
  ('an_du_re_bi_nho_diep_cau_truc_ngoanh_lai', 'Ẩn dụ rễ bị nhổ & điệp cấu trúc ngoảnh lại'),
  ('an_du_tinh_cam_cau_cham_lung_do_du', 'Ẩn dụ tình cảm & câu chấm lửng do dự'),
  ('an_du_triet_ly', 'Ẩn dụ triết lý'),
  ('an_du_tru_tinh', 'Ẩn dụ trữ tình'),
  ('an_du_ve_co_vua_hoac_ban_co', 'Ẩn dụ về cờ vua hoặc bàn cờ'),
  ('an_du_ve_trong_luong_ganh_nang_cau_hoi_tu_tu_dau_tranh', 'Ẩn dụ về trọng lượng (gánh nặng) & câu hỏi tu từ đấu tranh'),
  ('an_du_vet_thuong', 'Ẩn dụ vết thương'),
  ('bi_kich_hoa', 'Bi kịch hóa'),
  ('cam_than_gian_du', 'Cảm thán giận dữ'),
  ('cam_xuc_ca_nhan', 'Cảm xúc cá nhân'),
  ('cau_cam_than_bung_no', 'Câu cảm thán bùng nổ'),
  ('cau_cam_than_hai_hung', 'Câu cảm thán hãi hùng'),
  ('cau_cam_than_hao_hung', 'Câu cảm thán hào hứng'),
  ('cau_cam_than_ngan', 'Câu cảm thán ngắn'),
  ('cau_cao_buoc_lap_cau_hoi_tu_tu_sac_ben', 'Câu cáo buộc lặp & câu hỏi tu từ sắc bén'),
  ('cau_cham_lung_hut_hoi', 'Câu chấm lửng hụt hơi'),
  ('cau_cham_lung_nghen', 'Câu chấm lửng nghẹn'),
  ('cau_cham_lung_nghen_lai', 'Câu chấm lửng nghẹn lại'),
  ('cau_cham_lung_nghen_ngao', 'Câu chấm lửng nghẹn ngào'),
  ('cau_chu_dai_dong', 'Câu chữ dài dòng'),
  ('cau_chu_dut_doan_hon_loan', 'Câu chữ đứt đoạn hỗn loạn'),
  ('cau_chu_lung_cung', 'Câu chữ lủng củng'),
  ('cau_cut_coc_loc', 'Câu cụt cộc lốc'),
  ('cau_cut_dut_khoat', 'Câu cụt dứt khoát'),
  ('cau_cut_hon_loan_khong_dau_phay', 'Câu cụt hỗn loạn không dấu phẩy'),
  ('cau_cut_lien_tiep', 'Câu cụt liên tiếp'),
  ('cau_cut_lien_tiep_khong_lien_tu', 'Câu cụt liên tiếp không liên từ'),
  ('cau_cut_thieu_chu_ngu', 'Câu cụt thiếu chủ ngữ'),
  ('cau_cut_vo_cam', 'Câu cụt vô cảm'),
  ('cau_dai', 'Câu dài'),
  ('cau_ghep_dai', 'Câu ghép dài'),
  ('cau_ghep_dai_mang_tinh_thong_ke', 'Câu ghép dài mang tính thống kê'),
  ('cau_giat_gan', 'Câu giật gân'),
  ('cau_hoi_quan_tam', 'Câu hỏi quan tâm'),
  ('cau_hoi_tu_tu', 'Câu hỏi tu từ'),
  ('cau_hoi_tu_tu_boc_phat', 'Câu hỏi tu từ bộc phát'),
  ('cau_hoi_tu_tu_cang_thang', 'Câu hỏi tu từ căng thẳng'),
  ('cau_hoi_tu_tu_gay_ap_luc', 'Câu hỏi tu từ gây áp lực'),
  ('cau_hoi_tu_tu_kinh_ngac', 'Câu hỏi tu từ kinh ngạc'),
  ('cau_hoi_tu_tu_lo_lang', 'Câu hỏi tu từ lo lắng'),
  ('cau_hoi_tu_tu_mia_mai', 'Câu hỏi tu từ mỉa mai'),
  ('cau_hoi_tu_tu_tham_do', 'Câu hỏi tu từ thăm dò'),
  ('cau_hoi_tu_tu_tham_thi_trong_suy_nghi', 'Câu hỏi tu từ thầm thì trong suy nghĩ'),
  ('cau_hoi_tu_tu_ve_ban_than_an_du_lot_thuc_chat', 'Câu hỏi tu từ về bản thân & ẩn dụ lốt/thực chất'),
  ('cau_hoi_tu_tu_ve_dinh_menh', 'Câu hỏi tu từ về định mệnh'),
  ('cau_hoi_tu_tu_ve_qua_khu', 'Câu hỏi tu từ về quá khứ'),
  ('cau_hoi_tu_tu_ve_so_phan', 'Câu hỏi tu từ về số phận'),
  ('cau_khang_dinh_dut_khoat', 'Câu khẳng định dứt khoát'),
  ('cau_lung_hoai_niem', 'Câu lửng hoài niệm'),
  ('cau_rut_gon_canh_bao', 'Câu rút gọn cảnh báo'),
  ('cau_rut_gon_chu_ngu', 'Câu rút gọn chủ ngữ'),
  ('cau_trang_trong_kho_khan', 'Câu trang trọng khô khan'),
  ('dao_ngu_dua_tan_ngu_trang_ngu_len_truoc', 'Đảo ngữ (đưa tân ngữ/trạng ngữ lên trước)'),
  ('dao_ngu_nhan_khoanh_khac_tiep_xuc', 'Đảo ngữ nhấn khoảnh khắc tiếp xúc'),
  ('dao_ngu_nhan_loi_thoat', 'Đảo ngữ nhấn lối thoát'),
  ('diep_cau_truc_buoc_tien_liet_ke_cot_moc', 'Điệp cấu trúc bước tiến & liệt kê cột mốc'),
  ('diep_cau_truc_cau_cut_mo_ho', 'Điệp cấu trúc câu cụt mơ hồ'),
  ('diep_cau_truc_chua_an_du_thoi_gian_cham_lai', 'Điệp cấu trúc ''chưa...'' & ẩn dụ thời gian chậm lại'),
  ('diep_cau_truc_keu_goi_cau_hoi_tu_tu_thuyet_phuc', 'Điệp cấu trúc kêu gọi & câu hỏi tu từ thuyết phục'),
  ('diep_cau_truc_phan_manh', 'Điệp cấu trúc phân mảnh'),
  ('diep_cau_truc_qua_deu_dan', 'Điệp cấu trúc quá đều đặn'),
  ('diep_cau_truc_quan_sat', 'Điệp cấu trúc quan sát'),
  ('diep_cau_truc_quan_sat_canh_giac', 'Điệp cấu trúc quan sát cảnh giác'),
  ('diep_cau_truc_tan_cong_tung_lop_an_du_dot_nhap', 'Điệp cấu trúc tấn công từng lớp & ẩn dụ đột nhập'),
  ('diep_cau_truc_than_chu_an_du_ranh_gioi_the_gioi', 'Điệp cấu trúc thần chú & ẩn dụ ranh giới thế giới'),
  ('diep_cau_truc_the_dung_nhip_tho', 'Điệp cấu trúc thế đứng & nhịp thở'),
  ('diep_cau_truc_trao_doi_nua_chung', 'Điệp cấu trúc trao đổi nửa chừng'),
  ('diep_cau_truc_tuyen_the_an_du_vuong_mien_nang_triu', 'Điệp cấu trúc tuyên thệ & ẩn dụ vương miện nặng trĩu'),
  ('diep_cau_truc_vui_ve_liet_ke_dieu_nho_nhat', 'Điệp cấu trúc vui vẻ & liệt kê điều nhỏ nhặt'),
  ('diep_cau_truc_xac_minh', 'Điệp cấu trúc xác minh'),
  ('diep_dong_tu_boc_lo_cam_xuc', 'Điệp động từ bộc lộ cảm xúc'),
  ('diep_dong_tu_manh_tuong_thanh_va_cham', 'Điệp động từ mạnh & tượng thanh va chạm'),
  ('diep_giong_cao_don_dap', 'Điệp giọng cao dồn dập'),
  ('diep_tu', 'Điệp từ'),
  ('diep_tu_cam_xuc', 'Điệp từ cảm xúc'),
  ('diep_tu_chi_so_luong_dam_dong', 'Điệp từ chỉ số lượng đám đông'),
  ('diep_tu_chi_su_met_moi_kiet_suc', 'Điệp từ chỉ sự mệt mỏi, kiệt sức'),
  ('diep_tu_nhan_thoi_khac', 'Điệp từ nhấn thời khắc'),
  ('doan_mieu_ta_dai', 'Đoạn miêu tả dài'),
  ('doc_thoai_noi_tam_dai', 'Độc thoại nội tâm dài'),
  ('doi_thoai_voi_nguoi_khac', 'Đối thoại với người khác'),
  ('dong_y_thuc_lap_di_lap_lai', 'Dòng ý thức lặp đi lặp lại'),
  ('hai_huoc', 'Hài hước'),
  ('hanh_dong_don_dap', 'Hành động dồn dập'),
  ('ke_su_kien_kho_khan', 'Kể sự kiện khô khan'),
  ('kich_tinh', 'Kịch tính'),
  ('lac_quan_thai_qua', 'Lạc quan thái quá'),
  ('lap_cau_truc_dieu_kien_neu_thi', 'Lặp cấu trúc điều kiện (Nếu... thì...)'),
  ('liet_ke_bieu_tuong', 'Liệt kê biểu tượng'),
  ('liet_ke_bieu_tuong_quyen_luc', 'Liệt kê biểu tượng quyền lực'),
  ('liet_ke_chi_tiet_ky_la', 'Liệt kê chi tiết kỳ lạ'),
  ('liet_ke_chuong_ngai', 'Liệt kê chướng ngại'),
  ('liet_ke_co_vat', 'Liệt kê cổ vật'),
  ('liet_ke_dau_hieu_mong_manh', 'Liệt kê dấu hiệu mong manh'),
  ('liet_ke_dieu_khoan', 'Liệt kê điều khoản'),
  ('liet_ke_don_dap_cau_cut_lien_tiep_tao_hieu_ung_ngop_tho', 'Liệt kê dồn dập & câu cụt liên tiếp (tạo hiệu ứng ngộp thở)'),
  ('liet_ke_hanh_vi_nhap_vai', 'Liệt kê hành vi nhập vai'),
  ('liet_ke_hieu_lam_chong_chat', 'Liệt kê hiểu lầm chồng chất'),
  ('liet_ke_huong_dan_di_chuyen_re_tranh_luot', 'Liệt kê hướng dẫn di chuyển (rẽ, tránh, lướt)'),
  ('liet_ke_khoanh_khac_chuyen_giao', 'Liệt kê khoảnh khắc chuyển giao'),
  ('liet_ke_thoi_quen_lap_lai_diep_nhip_em_dem', 'Liệt kê thói quen lặp lại & điệp nhịp êm đềm'),
  ('liet_ke_toan_canh_diep_cau_truc_so_sanh_luc_luong', 'Liệt kê toàn cảnh & điệp cấu trúc so sánh lực lượng'),
  ('liet_ke_trinh_tu_cau_menh_lenh', 'Liệt kê trình tự & câu mệnh lệnh'),
  ('mach_lac_logic', 'Mạch lạc logic'),
  ('mia_mai_cham_biem', 'Mỉa mai châm biếm'),
  ('mieu_ta_cham_chap', 'Miêu tả chậm chạp'),
  ('mieu_ta_chi_tiet_may_moc', 'Miêu tả chi tiết máy móc'),
  ('mieu_ta_chi_tiet_tung_ngay', 'Miêu tả chi tiết từng ngày'),
  ('mieu_ta_kho_khan', 'Miêu tả khô khan'),
  ('mieu_ta_ngoai_canh_dai', 'Miêu tả ngoại cảnh dài'),
  ('mieu_ta_noi_tam_dai', 'Miêu tả nội tâm dài'),
  ('mieu_ta_tinh_lang', 'Miêu tả tĩnh lặng'),
  ('mieu_ta_tinh_lang_keo_dai', 'Miêu tả tĩnh lặng kéo dài'),
  ('ngon_ngu_an_du_mo_ho', 'Ngôn ngữ ẩn dụ mơ hồ'),
  ('ngon_ngu_bi_kich', 'Ngôn ngữ bi kịch'),
  ('ngon_ngu_bi_quan', 'Ngôn ngữ bi quan'),
  ('ngon_ngu_binh_than', 'Ngôn ngữ bình thản'),
  ('ngon_ngu_binh_tinh', 'Ngôn ngữ bình tĩnh'),
  ('ngon_ngu_buon_tham', 'Ngôn ngữ buồn thảm'),
  ('ngon_ngu_cam_xuc', 'Ngôn ngữ cảm xúc'),
  ('ngon_ngu_cam_xuc_ca_nhan', 'Ngôn ngữ cảm xúc cá nhân'),
  ('ngon_ngu_cam_xuc_manh', 'Ngôn ngữ cảm xúc mạnh'),
  ('ngon_ngu_cam_xuc_thai_qua', 'Ngôn ngữ cảm xúc thái quá'),
  ('ngon_ngu_cang_thang', 'Ngôn ngữ căng thẳng'),
  ('ngon_ngu_cham_chap_nang_ne', 'Ngôn ngữ chậm chạp, nặng nề'),
  ('ngon_ngu_cham_rai', 'Ngôn ngữ chậm rãi'),
  ('ngon_ngu_doi_thuong', 'Ngôn ngữ đời thường'),
  ('ngon_ngu_gap_gap', 'Ngôn ngữ gấp gáp'),
  ('ngon_ngu_hai_huoc', 'Ngôn ngữ hài hước'),
  ('ngon_ngu_hien_dai_tuc', 'Ngôn ngữ hiện đại, tục'),
  ('ngon_ngu_kho_khan_lanh_lung', 'Ngôn ngữ khô khan, lạnh lùng'),
  ('ngon_ngu_mia_mai', 'Ngôn ngữ mỉa mai'),
  ('ngon_ngu_mo_ho_do_du', 'Ngôn ngữ mơ hồ, do dự'),
  ('ngon_ngu_nghiem_tuc', 'Ngôn ngữ nghiêm túc'),
  ('ngon_ngu_on_ao', 'Ngôn ngữ ồn ào'),
  ('ngon_ngu_qua_doi_thuong', 'Ngôn ngữ quá đỗi thường'),
  ('ngon_ngu_rut_re', 'Ngôn ngữ rụt rè'),
  ('ngon_ngu_thang_thung', 'Ngôn ngữ thẳng thừng'),
  ('ngon_ngu_tho_o', 'Ngôn ngữ thờ ơ'),
  ('ngon_ngu_thong_tuc_ngoai_le', 'Ngôn ngữ thông tục ngoài lề'),
  ('ngon_ngu_tuyet_vong_tuyet_doi', 'Ngôn ngữ tuyệt vọng tuyệt đối'),
  ('ngon_ngu_u_am', 'Ngôn ngữ u ám'),
  ('ngon_ngu_vui_ve', 'Ngôn ngữ vui vẻ'),
  ('ngon_ngu_yeu_ot', 'Ngôn ngữ yếu ớt'),
  ('nhip_dieu_tram_deu', 'Nhịp điệu trầm đều'),
  ('nhip_van_cham', 'Nhịp văn chậm'),
  ('phep_dao_ngu_nhan_hoa_ra_thi_ra', 'Phép đảo ngữ & nhấn ''hoá ra / thì ra'''),
  ('phep_diep_cau_hoi_don_dap_lap_y_gai_bay', 'Phép điệp câu hỏi dồn dập & lặp ý gài bẫy'),
  ('phep_diep_toan_canh_lap_lai_phep_doi_giua_cac_lan_thu', 'Phép điệp toàn cảnh lặp lại & phép đối giữa các lần thử'),
  ('phep_doi_anh_ta_toi', 'Phép đối ''anh ta – tôi'''),
  ('phep_doi_buoc_toi_bao_chua', 'Phép đối buộc tội – bào chữa'),
  ('phep_doi_buong_tay_nam_giu', 'Phép đối buông tay – nắm giữ'),
  ('phep_doi_canh_nguoi_qua_khu_hien_tai_ben_trong_ben_ngoai', 'Phép đối (cảnh - người, quá khứ - hiện tại, bên trong - bên ngoài)'),
  ('phep_doi_dau_vet', 'Phép đối dấu vết'),
  ('phep_doi_giua_cac_phan_cua_ban_than', 'Phép đối giữa các phần của bản thân'),
  ('phep_doi_giua_duong_di_va_can_tro', 'Phép đối giữa đường đi và cản trở'),
  ('phep_doi_giua_hai_doi_thu', 'Phép đối giữa hai đối thủ'),
  ('phep_doi_giua_hai_nhan_vat', 'Phép đối giữa hai nhân vật'),
  ('phep_doi_giua_hoi_va_tranh', 'Phép đối giữa hỏi và tránh'),
  ('phep_doi_giua_than_phan_gia_va_that', 'Phép đối giữa thân phận giả và thật'),
  ('phep_doi_hai_phe_lap_dieu_kien_de_nhan', 'Phép đối hai phe & lặp điều kiện để nhấn'),
  ('phep_doi_hom_nay_ngay_mai', 'Phép đối hôm nay – ngày mai'),
  ('phep_doi_ke_tri_nguoi_tri', 'Phép đối kẻ trị – người trị'),
  ('phep_doi_khoang_lang_va_loi_noi', 'Phép đối khoảng lặng và lời nói'),
  ('phep_doi_ky_uc_hien_tai', 'Phép đối ký ức – hiện tại'),
  ('phep_doi_liet_ke_hon_tap', 'Phép đối & liệt kê hỗn tạp'),
  ('phep_doi_loi_the', 'Phép đối lời thề'),
  ('phep_doi_ly_tri_cam_xuc', 'Phép đối lý trí – cảm xúc'),
  ('phep_doi_mau_sac_am_thanh', 'Phép đối màu sắc, âm thanh'),
  ('phep_doi_nguoi_ban_nguoi_mua', 'Phép đối người bán – người mua'),
  ('phep_doi_nguoi_thang_ke_thua', 'Phép đối người thắng – kẻ thua'),
  ('phep_doi_toi_da_tin_hoa_ra_diep_khoang_lang', 'Phép đối ''tôi đã tin – hoá ra'' & điệp khoảng lặng'),
  ('phep_doi_tranh_cai_hoa_giai_vui_ve', 'Phép đối tranh cãi – hòa giải vui vẻ'),
  ('phep_doi_trung_thanh_phan_boi', 'Phép đối trung thành – phản bội'),
  ('phep_doi_truoc_sau', 'Phép đối trước – sau'),
  ('phep_doi_truoc_sau_khi_luu_day', 'Phép đối trước – sau khi lưu đày'),
  ('phep_doi_truoc_sau_theo_thoi_gian', 'Phép đối ''trước – sau'' theo thời gian'),
  ('phep_doi_tuong_phan_hai_huoc', 'Phép đối tương phản hài hước'),
  ('phep_doi_vung_dat_nay_vung_dat_kia', 'Phép đối vùng đất này – vùng đất kia'),
  ('phep_lap_cau_hoi_doi_chat_liet_ke_chung_cu', 'Phép lặp câu hỏi đối chất & liệt kê chứng cứ'),
  ('phep_liet_ke_chang_duong_diep_cau_truc_cat_canh', 'Phép liệt kê chặng đường & điệp cấu trúc cắt cảnh'),
  ('phep_liet_ke_chi_tiet_nhan_chi_tiet_bat_thuong', 'Phép liệt kê chi tiết & nhấn chi tiết bất thường'),
  ('phep_liet_ke_chung_cu', 'Phép liệt kê chứng cứ'),
  ('phep_liet_ke_cong_doan_diep_cau_truc_nang_cap', 'Phép liệt kê công đoạn & điệp cấu trúc nâng cấp'),
  ('phep_liet_ke_dia_danh_diep_cau_truc_khai_mo', 'Phép liệt kê địa danh & điệp cấu trúc khai mở'),
  ('phep_liet_ke_doi_chieu_thoi_gian', 'Phép liệt kê & đối chiếu thời gian'),
  ('phep_liet_ke_don_danh', 'Phép liệt kê đòn đánh'),
  ('phep_liet_ke_ma_lenh', 'Phép liệt kê mã lệnh'),
  ('phep_liet_ke_manh_moi_diep_cau_truc_duoi_theo', 'Phép liệt kê manh mối & điệp cấu trúc đuổi theo'),
  ('phep_liet_ke_mon_an_diep_nhip_vui_nhon', 'Phép liệt kê món ăn & điệp nhịp vui nhộn'),
  ('phep_liet_ke_mon_do_doi_thoai_mac_ca_lap', 'Phép liệt kê món đồ & đối thoại mặc cả lặp'),
  ('phep_liet_ke_mua_thang_phep_doi_truoc_sau', 'Phép liệt kê mùa / tháng & phép đối trước – sau'),
  ('phep_liet_ke_nguyen_vong', 'Phép liệt kê nguyện vọng'),
  ('phep_liet_ke_su_that', 'Phép liệt kê sự thật'),
  ('phep_liet_ke_tro_vui_diep_nhip_hoi', 'Phép liệt kê trò vui & điệp nhịp hội'),
  ('phep_loai_suy', 'Phép loại suy'),
  ('phep_phong_dai_su_mat_kiem_soat', 'Phép phóng đại sự mất kiểm soát'),
  ('phep_phong_dai_suc_manh_diep_cau_truc_vuot_gioi_han', 'Phép phóng đại sức mạnh & điệp cấu trúc vượt giới hạn'),
  ('phep_phong_dai_tro_treu_diep_cau_truc_day_chuyen', 'Phép phóng đại trớ trêu & điệp cấu trúc dây chuyền'),
  ('phep_so_sanh_la_quen_an_du_kham_pha', 'Phép so sánh lạ – quen & ẩn dụ khám phá'),
  ('phep_so_sanh_mau_thuan', 'Phép so sánh mâu thuẫn'),
  ('phep_so_sanh_truoc_sau', 'Phép so sánh trước – sau'),
  ('phep_tam_doan_luan_diep_cau_truc_suy_luan', 'Phép tam đoạn luận & điệp cấu trúc suy luận'),
  ('phong_dai', 'Phóng đại'),
  ('phong_dai_kich_tinh', 'Phóng đại kịch tính'),
  ('so_sanh_cam_giac', 'So sánh cảm giác'),
  ('so_sanh_chuyen_dong_chinh_xac', 'So sánh chuyển động chính xác'),
  ('so_sanh_dai_dong', 'So sánh dài dòng'),
  ('so_sanh_doi_chieu_sua_sai', 'So sánh đối chiếu sửa sai'),
  ('so_sanh_khong_gian_mang_thuc_dia', 'So sánh không gian mạng – thực địa'),
  ('so_sanh_ngam_an_du_chuyen_doi_cam_giac', 'So sánh ngầm (Ẩn dụ chuyển đổi cảm giác)'),
  ('so_sanh_voi_ky_uc_diep_cau_noi_dang_do', 'So sánh với ký ức & điệp câu nói dang dở'),
  ('suy_tu_noi_tam_dai', 'Suy tư nội tâm dài'),
  ('tap_trung_vao_ca_nhan', 'Tập trung vào cá nhân'),
  ('trao_phung', 'Trào phúng'),
  ('tu_ngu_hanh_dong_hon_ho', 'Từ ngữ hành động hớn hở'),
  ('tuong_thanh_am_anh_diep_hinh_anh_qua_khu_xam_nhap', 'Tượng thanh ám ảnh & điệp hình ảnh quá khứ xâm nhập'),
  ('tuong_thanh_che_tac', 'Tượng thanh chế tác'),
  ('tuong_thanh_cho_bua', 'Tượng thanh chợ búa'),
  ('tuong_thanh_don_dap', 'Tượng thanh dồn dập'),
  ('tuong_thanh_hai_kich', 'Tượng thanh hài kịch'),
  ('tuong_thanh_hon_loan', 'Tượng thanh hỗn loạn'),
  ('tuong_thanh_nao_nhiet', 'Tượng thanh náo nhiệt'),
  ('tuong_thanh_nhe_nhang', 'Tượng thanh nhẹ nhàng'),
  ('tuong_thanh_nhip_buoc', 'Tượng thanh nhịp bước'),
  ('tuong_thanh_nhip_di_chuyen', 'Tượng thanh nhịp di chuyển'),
  ('tuong_thanh_nhip_do_diep_khuc_xa_gan', 'Tượng thanh nhịp độ & điệp khúc ''xa/gần'''),
  ('tuong_thanh_on', 'Tượng thanh ồn'),
  ('tuong_thanh_on_ao', 'Tượng thanh ồn ào'),
  ('tuong_thanh_tieng_dong_duoi_theo', 'Tượng thanh tiếng động đuổi theo')
) AS v(code, description)
ON CONFLICT (code) DO NOTHING;

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

-- ==== exploration_discovery ====

-- preset: exploration_discovery.first_contact | name: "Lần đầu tiếp xúc"
-- tone.pace.raw: "chậm rãi, dò dẫm, từng khoảnh khắc"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["bình thường","quen thuộc"]
-- unmapped.rules.avoid_structure: ["đoạn văn liệt kê máy móc dài"]
-- unmapped.rules.emotion_limit: "không châm biếm sự khác biệt"
-- unmapped.genre: "exploration_discovery"
-- unmapped.settings: "khoảnh khắc gặp gỡ nền văn minh / thực thể lạ lần đầu, vùng biên giới chưa khám phá"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "exploration_discovery"
-- unmapped.settings: "thu thập manh mối, kích hoạt bản đồ, mở rộng vùng đất chưa biết"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "exploration_discovery"
-- unmapped.settings: "di chuyển qua nhiều vùng đất, theo dấu, đọc bản đồ, vượt địa hình"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "exploration_discovery"
-- unmapped.settings: "khu di tích cổ, hầm mộ, ngôi đền bỏ hoang, bí ẩn cổ xưa"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "exploration_discovery"
-- unmapped.settings: "truy tìm kho báu, giải đố, bản đồ cũ, cạnh tranh với kẻ khác"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== fantasy_scifi_mechanics ====

-- preset: fantasy_scifi_mechanics.crafting_enhancement | name: "Chế tạo & nâng cấp"
-- tone.pace.raw: "chậm, tỉ mỉ, cao trào khi thành phẩm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["tùy tiện","ngẫu nhiên"]
-- unmapped.rules.avoid_structure: ["đoạn văn hỗn loạn"]
-- unmapped.rules.emotion_limit: "không hài hước lệch lạc, không sốt ruột"
-- unmapped.genre: "fantasy_scifi_mechanics"
-- unmapped.settings: "xưởng rèn, phòng thí nghiệm, kết hợp nguyên liệu, nâng cấp trang bị"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "fantasy_scifi_mechanics"
-- unmapped.settings: "khoảnh khắc biến đổi sức mạnh, phá vỡ giới hạn, nguồn năng lượng mới"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "fantasy_scifi_mechanics"
-- unmapped.settings: "không gian mạng, màn hình mã lệnh, phá tường lửa, giải thuật mật mã"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "fantasy_scifi_mechanics"
-- unmapped.settings: "vòng triệu hồi, nghi lễ cổ, gọi thực thể siêu nhiên, linh hồn, quái vật"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "fantasy_scifi_mechanics"
-- unmapped.settings: "lặp lại khoảnh khắc, vòng lặp thời gian, sửa sai, ngày tận thế"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== mystery ====
-- preset: mystery.alibi_check | name: "Xác minh chứng ngoại phạm"
-- tone.pace.raw: "chậm rãi, máy móc"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["chắc hẳn","có lẽ"]
-- unmapped.rules.avoid_structure: ["câu cảm thán, câu hỏi tu từ"]
-- unmapped.rules.emotion_limit: "không phẫn nộ, không mỉa mai"
-- unmapped.genre: "mystery"
-- unmapped.settings: "lục soát giấy tờ, phỏng vấn nhân chứng, so sánh thời gian"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "mystery"
-- unmapped.settings: "hiện trường, phòng tài liệu, khám xét"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "mystery"
-- unmapped.settings: "nội tâm nhân vật, phòng làm việc, bảng phân tích"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "mystery"
-- unmapped.settings: "khu vực công cộng, đám đông, buổi tiệc, khu ổ chuột"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "mystery"
-- unmapped.settings: "phòng thẩm vấn, đối chất giữa các nhân vật"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "mystery"
-- unmapped.settings: "điểm cao trào, đối chất trực diện"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== political_social ====

-- preset: political_social.conspiracy_meeting | name: "Cuộc họp bí mật"
-- tone.pace.raw: "chậm, dè chừng, thì thầm"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["trung thực","ngay thẳng"]
-- unmapped.rules.avoid_structure: ["đoạn văn cởi mở, chân thành"]
-- unmapped.rules.emotion_limit: "không ngây thơ, không phấn khích"
-- unmapped.genre: "political_social"
-- unmapped.settings: "phòng tối, hội kín, bàn mưu tính, âm mưu chính trị"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "political_social"
-- unmapped.settings: "nghi lễ đăng quang, triều đình, tranh giành ngai vàng, lễ tôn vương"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "political_social"
-- unmapped.settings: "không gian biên giới, trục xuất khỏi quê hương, lưu đày nơi hoang vắng"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "political_social"
-- unmapped.settings: "phòng họp, bàn đàm phán, hai phe đối lập, hiệp ước"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "political_social"
-- unmapped.settings: "quảng trường, khán đài, đám đông, tuyên bố chính trị"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "political_social"
-- unmapped.settings: "phòng xử án, bục thẩm phán, bồi thẩm, tòa án công bằng / không công bằng"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== psychological_drama ====
-- preset: psychological_drama.betrayal | name: "Phản bội"
-- tone.pace.raw: "đột ngột, nhanh khi vỡ lẽ, sau đó đóng băng"
-- rhetorical_devices.intensity: "cao"
-- unmapped.rules.avoid_words: ["tất nhiên","đúng như dự đoán"]
-- unmapped.rules.avoid_structure: ["đoạn miêu tả cảnh vật dài"]
-- unmapped.rules.emotion_limit: "không hài hước, không bình thản"
-- unmapped.genre: "psychological_drama"
-- unmapped.settings: "khoảnh khắc sự thật được phơi bày (bất kỳ không gian nào – tấm lưng quay đi, lưỡi dao đâm sau lưng)"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "psychological_drama"
-- unmapped.settings: "không gian cô lập hoặc nơi đối diện với hình chiếu của bản thân"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "psychological_drama"
-- unmapped.settings: "không gian tĩnh, trong đầu nhân vật (khi đứng trước lựa chọn hệ trọng)"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "psychological_drama"
-- unmapped.settings: "trong giấc mơ hoặc ký ức bất chợt, thường là nơi có ký ức chấn thương"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "psychological_drama"
-- unmapped.settings: "thời điểm mất mát (chiến trận, bệnh tật, chia ly vì lý tưởng)"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== romance_relationship ====
-- preset: romance_relationship.confession | name: "Tỏ tình"
-- tone.pace.raw: "chậm đến trung bình (có cao trào)"
-- rhetorical_devices.intensity: "trung bình"
-- unmapped.rules.avoid_words: ["chắc chắn","rõ ràng"]
-- unmapped.rules.avoid_structure: ["đoạn miêu tả ngoại cảnh dài"]
-- unmapped.rules.emotion_limit: "không hài hước, không mỉa mai"
-- unmapped.genre: "romance_relationship"
-- unmapped.settings: "không gian riêng tư hoặc bối cảnh bất ngờ, hai nhân vật đối diện"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "romance_relationship"
-- unmapped.settings: "không gian thư giãn (quán cà phê, công viên, bữa tối, dạo phố…)"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "romance_relationship"
-- unmapped.settings: "nơi hai người từng gắn bó hoặc không gian cô độc"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "romance_relationship"
-- unmapped.settings: "nơi diễn ra hiểu lầm, thường là không gian kín hoặc bữa tiệc đông người"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "romance_relationship"
-- unmapped.settings: "nơi công cộng bất ngờ hoặc điểm hẹn đã định, thời gian cách xa"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "romance_relationship"
-- unmapped.settings: "không gian nghi lễ, có sự hiện diện của tập thể (nhà thờ, sảnh tiệc, thiên nhiên)"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== slice_of_life_comedy ====

-- preset: slice_of_life_comedy.daily_routine | name: "Sinh hoạt đời thường"
-- tone.pace.raw: "chậm, thư thái, đều đều như nhịp thở"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["nguy hiểm","vội vã","sinh tử"]
-- unmapped.rules.avoid_structure: ["câu cộc lốc, giật gân"]
-- unmapped.rules.emotion_limit: "không bi kịch, không căng thẳng"
-- unmapped.genre: "slice_of_life_comedy"
-- unmapped.settings: "không gian thân thuộc, hàng ngày"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "slice_of_life_comedy"
-- unmapped.settings: "bàn tiệc, nhà bếp, gia đình / bạn bè sum họp, ăn uống ồn ào"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "slice_of_life_comedy"
-- unmapped.settings: "quảng trường, lễ hội truyền thống, đèn hoa, đám đông vui vẻ"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "slice_of_life_comedy"
-- unmapped.settings: "không gian sinh hoạt thường ngày, trò đùa, hiểu lầm dây chuyền"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "slice_of_life_comedy"
-- unmapped.settings: "chợ, siêu thị, phố mua sắm, chuẩn bị đồ đạc, mặc cả"
-- unmapped.rhetorical_devices.examples: [""]
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

-- ==== transition ====
-- preset: transition.recovery_rest | name: "Hồi phục & nghỉ ngơi"
-- tone.pace.raw: "chậm, êm dịu, dễ chịu"
-- rhetorical_devices.intensity: "thấp"
-- unmapped.rules.avoid_words: ["vội vã","khẩn cấp"]
-- unmapped.rules.avoid_structure: ["câu cụt giật gân"]
-- unmapped.rules.emotion_limit: "không căng thẳng, không lo âu"
-- unmapped.genre: "transition"
-- unmapped.settings: "chốn bình yên, sau trận chiến, giường bệnh, thiên nhiên tĩnh lặng"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "transition"
-- unmapped.settings: "trôi qua ngày tháng, năm tháng, thay đổi lớn trong khoảnh khắc"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "transition"
-- unmapped.settings: "di chuyển qua nhiều cảnh, chuyến đi dài, đổi khung cảnh liên tục"
-- unmapped.rhetorical_devices.examples: [""]
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
-- unmapped.genre: "transition"
-- unmapped.settings: "khoảng trống trước sự kiện, chờ tin tức, chờ đối thủ / người thân, giây phút căng thẳng"
-- unmapped.rhetorical_devices.examples: [""]
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

COMMIT;
