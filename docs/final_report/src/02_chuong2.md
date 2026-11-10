# CHƯƠNG 2: GIỚI THIỆU CÁC CÔNG NGHỆ ĐƯỢC SỬ DỤNG TRONG QUÁ TRÌNH PHÂN TÍCH VÀ THIẾT KẾ HỆ THỐNG

## 2.1 NGÔN NGỮ MÔ HÌNH HÓA (Lược bỏ)

Mục này được lược bỏ theo yêu cầu của đề tài. Phần mô hình hóa trong báo cáo sử dụng sơ đồ use case, sơ đồ kiến trúc và sơ đồ nhóm bảng dữ liệu trình bày trực tiếp bằng hình vẽ (xem Chương 3 và Chương 4), không gắn với một ngôn ngữ mô hình hóa hình thức cụ thể.

## 2.2 CÔNG CỤ THIẾT KẾ SƠ ĐỒ

Ba công cụ được sử dụng ở các mức khác nhau:

**DBML trên dbdiagram.io.** Toàn bộ schema quan hệ của dự án được đặc tả bằng DBML — một ngôn ngữ mô tả lược đồ dạng văn bản, có cú pháp cho bảng, cột, kiểu dữ liệu, chỉ mục, khóa ngoại và ghi chú [A10]. Ưu điểm của DBML là *lược đồ nằm trong kho mã nguồn* (`docs/db.dbml`), cho phép so sánh phiên bản (diff), rà soát trong pull request và sinh sơ đồ trực quan khi cần. Báo cáo dùng DBML làm nguồn sự thật cho phần thiết kế dữ liệu; các sơ đồ trong báo cáo được dựng lại từ tệp này.

**Mermaid.** Với các sơ đồ luồng và sơ đồ tuần tự có tính thay đổi cao, Mermaid cho phép viết sơ đồ bằng cú pháp văn bản, dễ lưu trong tài liệu và dễ cập nhật [A14]. Mermaid phù hợp với các sơ đồ nhỏ, ít yêu cầu bố cục chính xác.

**Sinh sơ đồ trực tiếp bằng SVG.** Với ba sơ đồ lớn trong báo cáo — use case tổng quát, kiến trúc hệ thống, dòng chảy nghiệp vụ — nhóm thực hiện dùng một script Python sinh SVG với bố cục xác định, sau đó raster hóa bằng trình duyệt Chromium ở hệ số phóng 1,2 để bảo đảm độ nét khi in. Cách này cho phép kiểm soát hoàn toàn vị trí, nhãn tiếng Việt có dấu và kích thước chữ, đồng thời tái tạo được sơ đồ khi nội dung thay đổi.

| Công cụ | Ưu điểm | Hạn chế | Dùng cho |
| --- | --- | --- | --- |
| DBML / dbdiagram.io | Lược đồ dạng văn bản, diff được, sinh SQL | Không trực quan bằng công cụ kéo thả | Thiết kế cơ sở dữ liệu |
| Mermaid | Nhanh, nhúng được vào Markdown | Khó canh bố cục phức tạp | Sơ đồ nhỏ, luồng đơn giản |
| Script sinh SVG | Kiểm soát chính xác, chữ Việt rõ | Cần viết mã cho từng sơ đồ | Ba sơ đồ lớn của báo cáo |

## 2.3 HỆ QUẢN TRỊ CƠ SỞ DỮ LIỆU

Merlink Harness là ứng dụng **một người dùng, chạy cục bộ**, nên tiêu chí chọn hệ quản trị khác với ứng dụng web nhiều người dùng. Nhóm thực hiện so sánh ba phương án:

| Tiêu chí | SQLite | PostgreSQL | MySQL/MariaDB |
| --- | --- | --- | --- |
| Mô hình triển khai | Thư viện nhúng trong tiến trình | Máy chủ riêng | Máy chủ riêng |
| Cài đặt cho người dùng cuối | Không cần | Cần | Cần |
| Kích thước | Dưới 1 MB | Hàng trăm MB | Hàng trăm MB |
| Ghi đồng thời | Một writer, WAL cho phép đọc song song | Đa writer | Đa writer |
| Giao dịch ACID | Có | Có | Có |
| Ràng buộc, khóa ngoại | Có (bật bằng PRAGMA) | Có | Có |
| JSON | Hàm JSON1 | JSONB mạnh | JSON |
| Phù hợp desktop offline | Rất phù hợp | Không | Không |

Hệ thống chọn **SQLite** vì bốn lý do: (1) không yêu cầu cài đặt hay vận hành máy chủ, phù hợp ứng dụng desktop đóng gói bằng Tauri; (2) toàn bộ dữ liệu nằm trong một tệp, dễ sao lưu và di chuyển; (3) chế độ WAL cho phép đọc trong khi ghi, đủ cho mô hình một người dùng với nhiều tác vụ nền [A7]; (4) hỗ trợ đầy đủ giao dịch, khóa ngoại và chỉ mục cần cho nghiệp vụ.

Hạn chế cố hữu của SQLite là **chỉ một writer tại một thời điểm**. Hệ thống biến hạn chế này thành giả định thiết kế: mọi ghi dữ liệu đi qua backend Rust và được tuần tự hóa theo khóa của từng bản nháp; giao dịch dùng `BEGIN IMMEDIATE` để giành quyền ghi sớm. Cách làm này được trình bày chi tiết ở Chương 4.

## 2.4 NGÔN NGỮ TRUY VẤN SQL

Hệ thống dùng **dialect SQLite**, với các đặc điểm cần lưu ý khi thiết kế câu lệnh:

- **Kiểu dữ liệu động nhưng có khai báo**: cột khai báo kiểu để tài liệu hóa và kiểm tra ở tầng ứng dụng; SQLite áp dụng quy tắc chuyển đổi lỏng. Vì vậy backend phải kiểm tra kiểu trước khi ghi, không dựa hoàn toàn vào cơ sở dữ liệu.
- **Khóa ngoại phải bật tường minh** bằng `PRAGMA foreign_keys = ON` cho từng kết nối; nếu không, ràng buộc tham chiếu sẽ không được thực thi.
- **Chế độ WAL và đồng bộ**: `PRAGMA journal_mode = WAL` cho phép đọc song song với ghi; `PRAGMA synchronous` được chọn theo mức đánh đổi giữa độ bền và tốc độ [A7].
- **Quản lý phiên bản schema** qua `PRAGMA user_version`: mỗi lần khởi động, backend so sánh số phiên bản trong tệp với phiên bản mã nguồn và chạy migration theo thứ tự.
- **Giao dịch ngắn, không chờ mạng**: không giữ giao dịch hay khóa cơ sở dữ liệu trong lúc chờ phản hồi LLM — đây là nguyên tắc bắt buộc để tránh khóa dây chuyền.
- **Truy vấn có tham số** (prepared statement) cho mọi dữ liệu đến từ người dùng hoặc mô hình.

Một số câu lệnh tiêu biểu của hệ thống:

```sql
-- Bật ràng buộc và chế độ ghi an toàn
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;

-- Niêm phong phiên bản: chỉ chèn nếu chưa có khóa niêm phong
INSERT INTO draft_versions (id, draft_session_id, seq, data_hash, seal_key, data_json, phase, subphase)
VALUES (?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT (draft_session_id, seal_key) DO NOTHING;

-- Tra cứu cảnh kèm địa điểm và số nhân vật xuất hiện
SELECT s.code, s.position, ws.display_name AS location, COUNT(sc.character_id) AS cast
FROM scenes s
LEFT JOIN world_settings ws ON ws.id = s.world_setting_id
LEFT JOIN scene_characters sc ON sc.scene_id = s.id
WHERE s.chapter_id = ?
GROUP BY s.id
ORDER BY s.position;
```

Với dữ liệu bán cấu trúc (payload tri thức theo loại, tham số tool), hệ thống dùng cột `TEXT` chứa JSON và kiểm tra cấu trúc ở backend thay vì dựa vào cơ sở dữ liệu.

## 2.5 PHƯƠNG PHÁP PHÁT TRIỂN HỆ THỐNG

Dự án áp dụng hai nguyên tắc song song:

**Thứ nhất — phát triển hợp đồng trước (contract-first).** Tài liệu được xếp theo thứ tự thẩm quyền: quyết định của chủ dự án > Business Core > DBML > Frontend Core > Backend Core > hợp đồng tool/prompt. Mọi thay đổi hành vi phải bắt đầu từ tài liệu có thẩm quyền cao hơn; mã nguồn không được tự định nghĩa lại nghiệp vụ. Cách làm này giúp đội phát triển và các agent tự động có cùng "nguồn sự thật", tránh tình trạng giao diện và backend hiểu khác nhau.

**Thứ hai — triển khai theo pha kỹ thuật có cổng kiểm soát.** Kế hoạch triển khai chia thành sáu pha kỹ thuật (khác với bảy giai đoạn nghiệp vụ), mỗi pha gồm các sprint có phụ thuộc rõ ràng và điều kiện nghiệm thu riêng:

| Pha kỹ thuật | Nội dung | Số sprint |
| --- | --- | ---: |
| 1 | Kiểm chứng hợp đồng, kiểm kê hiện trạng | 3 |
| 2 | Nền tảng và giao diện đóng băng (IPC, schema, migration) | 6 |
| 3 | Miền bản nháp và workflow bền vững | 10 |
| 4 | Dịch vụ backend và Writing Mode | 5 |
| 5 | Danh mục, hồ sơ nhân vật, bối cảnh | 6 |
| 6 | Đóng gói, kiểm thử tích hợp, phát hành | 3 |

Mỗi sprint chỉ được đánh dấu hoàn tất khi vượt cổng kiểm thử tương ứng; sprint kiểm chứng hợp đồng có thể kết thúc ở trạng thái "bị chặn hợp đồng" thay vì tự sửa tài liệu nguồn. Cách tổ chức này phù hợp với dự án có nhiều tác nhân thực thi (người và agent) cùng làm việc trên một kho mã.

**Công cụ hỗ trợ:** Git cho quản lý phiên bản; CI kiểm tra kiểu TypeScript, lint, kiểm thử Rust và kiểm thử Node; các script xác minh hợp đồng chạy trong CI (kiểm tra Dockerfile, workflow, biến môi trường, đường dẫn dữ liệu thử nghiệm).

## 2.6 CÔNG CỤ SOẠN THẢO VÀ TRÌNH BÀY BÁO CÁO

Báo cáo được viết bằng **Markdown** — định dạng văn bản thuần, dễ so sánh phiên bản, dễ chuyển đổi. Chuỗi công cụ xuất bản gồm:

1. **Markdown** cho nội dung, với bảng, danh sách, hình và công thức đơn giản.
2. **Pandoc** chuyển Markdown sang LaTeX và gọi **XeLaTeX** biên dịch PDF [A11]. XeLaTeX được chọn vì hỗ trợ Unicode đầy đủ, cần thiết cho tiếng Việt có dấu; bộ font Noto Serif (nội dung) và Noto Sans (tiêu đề) cho kết quả ổn định trên Linux.
3. **Một tệp header LaTeX** tùy biến: đổi nhãn "Figure"/"Table" sang "Hình"/"Bảng", định dạng chú thích, thụt đầu dòng, bổ sung font dự phòng cho các ký tự mũi tên.
4. **Chromium** raster hóa sơ đồ SVG và chụp ảnh giao diện; **ImageMagick** chuyển định dạng và thu nhỏ ảnh trước khi nhúng.
5. **Script Python** sinh sơ đồ và tổng hợp danh mục tài liệu tham khảo.

Toàn bộ quy trình được đóng gói trong một script build, cho phép tái tạo PDF từ nguồn Markdown bất kỳ lúc nào. Cách làm này cũng bảo đảm báo cáo luôn đồng bộ với kho tài liệu của dự án: khi đặc tả thay đổi, chỉ cần sửa Markdown và chạy lại build.

## 2.7 NHẬN XÉT VỀ VIỆC LỰA CHỌN CÔNG NGHỆ

**Điểm mạnh.** Bộ công nghệ được chọn có tính nhất quán cao: TypeScript ở giao diện, Rust ở backend, SQLite ở tầng lưu trữ, Markdown–Pandoc–XeLaTeX ở tầng tài liệu. Mọi thành phần đều là mã nguồn mở, chạy cục bộ, không phụ thuộc dịch vụ đám mây, phù hợp với yêu cầu bảo mật dữ liệu sáng tác của người dùng. Tauri cho phép đóng gói ứng dụng nhỏ gọn (không kèm trình duyệt nhúng đầy đủ như Electron), trong khi vẫn dùng được hệ sinh thái web.

**Điểm hạn chế.** Thứ nhất, SQLite chỉ cho một writer, nên thiết kế phải tuân thủ kỷ luật khóa rất chặt; mọi sai sót về thứ tự lấy khóa có thể gây nghẽn. Thứ hai, việc phụ thuộc WebKit trên Linux khiến hành vi giao diện có thể khác nhau giữa các nền tảng, cần kiểm thử trên từng hệ điều hành. Thứ ba, chuỗi Pandoc–XeLaTeX đòi hỏi cài đặt TeX Live đầy đủ (khoảng vài trăm MB) và có thể lỗi font với một số ký tự đặc biệt; nhóm thực hiện đã khắc phục bằng font dự phòng và kiểm tra danh sách ký tự thiếu.

**Kết luận lựa chọn.** Với mục tiêu là ứng dụng desktop một người dùng, ưu tiên tính toàn vẹn dữ liệu và khả năng tái tạo tài liệu, bộ công nghệ trên là phù hợp. Các hạn chế đã được nhận diện và có phương án giảm thiểu cụ thể, được trình bày trong Chương 4.
