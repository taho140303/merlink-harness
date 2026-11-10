# PHẦN MỞ ĐẦU

## 1. LÝ DO CHỌN ĐỀ TÀI

Trong khoảng năm năm trở lại đây, các mô hình ngôn ngữ lớn (Large Language Model — LLM) đã trở thành công cụ quen thuộc của người viết: từ gợi ý tên nhân vật, mở rộng một đoạn văn, đến viết trọn một chương truyện. Các sản phẩm thương mại như NovelAI, Sudowrite hay Novelcrafter đã chứng minh nhu cầu thực tế của thị trường: người viết sẵn sàng trả phí cho một trợ lý AI hiểu được "story bible" của họ [C1][C3][C4]. Tuy nhiên, phần lớn công cụ hiện nay tối ưu cho việc *sinh văn bản ngắn* hoặc *gợi ý cục bộ*; khi độ dài tác phẩm tăng lên hàng trăm chương, chất lượng đầu ra suy giảm rõ rệt vì ba lý do kỹ thuật đã được ghi nhận trong nghiên cứu: hiện tượng "lost in the middle" khi thông tin nằm giữa ngữ cảnh dài [B8], hiện tượng "context rot" khi số token đầu vào tăng [C6], và độ phức tạp tự sự của truyện dài vượt quá khả năng giữ mạch của một lần sinh [C5].

Hệ quả là người viết phải tự đóng vai "người quản lý dự án" cho câu chuyện của mình: ghi chú nhân vật trong một tệp, dàn ý trong một tệp khác, dán lại ngữ cảnh mỗi lần trò chuyện với AI. Cách làm thủ công này không chỉ tốn thời gian mà còn tạo ra ba loại lỗi hệ thống: dữ liệu phân mảnh (mỗi công cụ giữ một bản khác nhau), mâu thuẫn nhân vật (tính cách, ngoại hình thay đổi giữa các chương), và mất phương hướng cốt truyện (tuyến phụ lấn át tuyến chính). Đây chính là khoảng trống mà một **harness** — lớp điều phối chặt chẽ giữa AI và dữ liệu có cấu trúc — có thể lấp đầy.

Merlink Harness được chọn làm đối tượng nghiên cứu vì hệ thống này đặt vấn đề ngược với số đông: thay vì để AI tự do sinh văn bản, hệ thống **bắt buộc AI làm việc qua tool call** trên một mô hình dữ liệu quan hệ, chia quá trình sáng tác thành bảy giai đoạn, và chỉ cho phép chuyển dữ liệu sang cơ sở dữ liệu khi người dùng đã duyệt xong dàn ý. Cách tiếp cận "sáng tác có cấu trúc từ trên xuống" (Top-down Architect Method) này biến những yêu cầu về tính nhất quán thành ràng buộc kỹ thuật, thay vì phó mặc cho chất lượng prompt.

## 2. MỤC TIÊU NGHIÊN CỨU

Đề tài hướng tới năm mục tiêu cụ thể:

1. Hệ thống hóa hiện trạng ứng dụng AI trong sáng tác văn học và các hạn chế của cách làm hiện nay, làm cơ sở lý luận cho việc thiết kế hệ thống.
2. Mô hình hóa quy trình nghiệp vụ cốt lõi của Merlink Harness thành bảy giai đoạn (Phase 0 đến Phase 6), mô tả rõ dữ liệu vào, dữ liệu ra, điều kiện chuyển bước và các bất biến cần bảo toàn.
3. Xác định tác nhân, đặc tả các use case chính bằng bảng đặc tả theo mẫu thống nhất, làm cơ sở cho thiết kế và kiểm thử.
4. Đề xuất kiến trúc hệ thống dựa trên Tauri v2, Rust và SQLite, kèm cơ chế kiểm soát đồng thời, lưu trữ phiên bản và duy trì tính toàn vẹn dữ liệu.
5. Thiết kế mô hình dữ liệu quan hệ (66 bảng), cơ chế ánh xạ từ bản nháp sang cơ sở dữ liệu, và bộ dữ liệu danh mục ban đầu (thể loại, motif, chủ đề, cấu hình kể chuyện) phục vụ quá trình sáng tác.

## 3. ĐỐI TƯỢNG NGHIÊN CỨU

Đối tượng nghiên cứu gồm hai phần gắn bó với nhau:

- **Quy trình nghiệp vụ sáng tác có cấu trúc**: cách một tác phẩm được hình thành qua chuỗi giai đoạn thu thập — kiểm duyệt — dựng dàn ý — kiểm chứng — cam kết dữ liệu, cùng các quy tắc bảo vệ dữ liệu người dùng.
- **Hệ thống Merlink Harness**: tập tài liệu đặc tả (Business Core, Backend Core, Frontend Core, danh mục tool, schema DBML) và mã nguồn hiện có của dự án.

Bên cạnh đó, đề tài khảo sát một số sản phẩm cùng loại trên thị trường để làm nền so sánh.

## 4. PHẠM VI NGHIÊN CỨU

- **Phạm vi chức năng**: bảy giai đoạn của Planning Mode (thu thập yêu cầu đến commit dàn ý), luồng phản hồi và quản lý phiên bản bản nháp, luồng workflow phân đoạn cho truyện dài, và giai đoạn đầu của Writing Mode (sinh nội dung cảnh theo mô hình chương-first).
- **Phạm vi kỹ thuật**: giai đoạn phân tích và thiết kế — đặc tả nghiệp vụ, use case, kiến trúc, mô hình dữ liệu; giao diện được kiểm chứng ở mức web preview (chưa có backend Rust chạy thực tế trong môi trường nghiên cứu).
- **Ngoài phạm vi**: huấn luyện hoặc tinh chỉnh mô hình ngôn ngữ; đánh giá định lượng chất lượng văn bản do AI sinh; hạ tầng triển khai nhiều người dùng, đồng bộ đám mây và thanh toán.

## 5. PHƯƠNG PHÁP NGHIÊN CỨU

Đề tài sử dụng kết hợp bốn phương pháp:

1. **Nghiên cứu tài liệu**: đọc và đối chiếu hệ thống tài liệu đặc tả của dự án, trong đó Business Core là nguồn sự thật cao nhất về nghiệp vụ, DBML là nguồn sự thật về schema vật lý.
2. **Phân tích và thiết kế hướng chức năng**: phân rã hệ thống thành phân hệ, chức năng, tác nhân; mô hình hóa bằng sơ đồ use case và bảng đặc tả.
3. **Thực nghiệm giao diện**: chạy giao diện ở chế độ web preview với dữ liệu mẫu, quan sát hành vi từng màn hình, chụp ảnh minh họa và đối chiếu với đặc tả.
4. **Đối chiếu mã nguồn**: kiểm tra sự tương thích giữa tài liệu và mã nguồn (đường dẫn tính năng, route, thành phần giao diện) để bảo đảm báo cáo phản ánh đúng hiện trạng.

## 6. KẾT CẤU ĐỀ TÀI

Ngoài phần mở đầu, kết luận, hướng phát triển và tài liệu tham khảo, báo cáo gồm bốn chương:

- **Chương 1** trình bày khảo sát hiện trạng, mô tả hệ thống, các quy trình nghiệp vụ cốt lõi và yêu cầu hệ thống.
- **Chương 2** giới thiệu các công nghệ, công cụ và phương pháp được sử dụng trong phân tích và thiết kế.
- **Chương 3** phân tích hệ thống: xác định tác nhân, sơ đồ use case tổng quát và đặc tả chi tiết các use case chính.
- **Chương 4** đề xuất giải pháp và thiết kế hệ thống: kiến trúc, giao diện, dữ liệu và tích hợp.
