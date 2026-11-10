# Báo cáo phân tích và thiết kế hệ thống Merlink Harness

Thư mục này chứa báo cáo cuối kỳ ở hai định dạng và toàn bộ nguồn để tái tạo báo cáo.

## Cấu trúc

```
final_report/
├── MERLINK_HARNESS_BAO_CAO.md      # Bản Markdown tự chứa (có mục lục liên kết)
├── MERLINK_HARNESS_BAO_CAO.pdf     # Bản PDF (58 trang, khổ A4, XeLaTeX)
├── images/                          # Hình 1–10: sơ đồ và ảnh giao diện
├── src/                             # Nguồn Markdown theo chương
│   ├── 00_title.md                  # Trang bìa + metadata
│   ├── 00_mo_dau.md                 # PHẦN MỞ ĐẦU (6 mục)
│   ├── 01_chuong1.md                # Hiện trạng, mô tả hệ thống, quy trình, yêu cầu
│   ├── 02_chuong2.md                # Công nghệ và công cụ
│   ├── 03_chuong3.md                # Tác nhân, use case, bảng đặc tả
│   ├── 04_chuong4.md                # Giải pháp, kiến trúc, giao diện, dữ liệu
│   ├── 05_ket_luan.md               # Kết luận + hướng phát triển
│   └── 06_tai_lieu_tham_khao.md     # Tài liệu tham khảo (A–D)
└── build/
    ├── build.sh                     # Script sinh MD + PDF
    ├── hdr.tex                      # Header LaTeX (font chữ, nhãn Hình/Bảng, CJK)
    └── br.lua                       # Chuyển <br> trong ô bảng thành ngắt dòng LaTeX
```

## Tái tạo báo cáo

Yêu cầu: `pandoc`, `xelatex` (TeX Live, có gói `xeCJK`), font `Noto Serif`, `Noto Sans`, `JetBrains Mono`.

```bash
cd docs/final_report
./build/build.sh
```

Script sẽ ghép `src/*.md`, sinh lại `MERLINK_HARNESS_BAO_CAO.md` và biên dịch `MERLINK_HARNESS_BAO_CAO.pdf`.

## Ghi chú về nguồn dữ liệu

- Nội dung nghiệp vụ: `docs/business/core.md` (SSOT), `be-core.md`, `fe-core.md`,
  `harness/tools.md`, `docs/bussiness-flow.md`, `docs/plan/*`.
- Hình 1–3, 10: sơ đồ dựng từ DBML và mô tả nghiệp vụ.
- Hình 4–9: ảnh chụp giao diện ở chế độ web preview với dữ liệu mẫu.
- Tài liệu tham khảo: nhóm A–C đã xác minh bằng công cụ tìm kiếm web (HTTP 200, có nội dung);
  nhóm D trích từ `docs/temp_seed/report/*.md` và đã kiểm tra khả năng truy cập.
