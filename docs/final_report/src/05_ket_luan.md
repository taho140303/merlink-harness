# KẾT LUẬN

Đề tài đã hoàn thành việc khảo sát, phân tích và thiết kế hệ thống Merlink Harness — một harness hỗ trợ sáng tác truyện dài bằng AI theo phương pháp cấu trúc từ trên xuống. Các kết quả chính gồm:

**Thứ nhất, hệ thống hóa được vấn đề nghiệp vụ.** Báo cáo đã chỉ ra sáu nhóm khó khăn khi dùng AI để viết truyện dài, phân loại chúng thành vấn đề kỹ thuật của mô hình (trôi ngữ cảnh, mất nhất quán, quá tải context) và vấn đề tổ chức quy trình (thiếu kiểm soát cấu trúc, dữ liệu phân mảnh, không có lịch sử). Việc phân loại này là cơ sở để thiết kế đúng: các vấn đề loại một được giảm thiểu bằng tóm tắt phân tầng và cửa sổ ngữ cảnh có kiểm soát; các vấn đề loại hai được giải quyết bằng mô hình dữ liệu và luật nghiệp vụ.

**Thứ hai, mô hình hóa được quy trình nghiệp vụ.** Bảy giai đoạn sáng tác được mô tả đầy đủ với dữ liệu vào, dữ liệu ra, điều kiện chuyển bước và bất biến. Sáu quy trình cốt lõi — thu thập và cổng sẵn sàng, dựng dàn ý lũy tiến, phản hồi và patch, workflow phân đoạn, cam kết dữ liệu, sáng tác chương — được trình bày kèm bảng tham số và ví dụ kiểm chứng, đủ chi tiết để đội phát triển triển khai mà không cần diễn giải thêm.

**Thứ ba, đặc tả được hệ thống ở mức use case.** Năm tác nhân được xác định với ranh giới quyền rõ ràng; mười use case nghiệp vụ được mô hình hóa trong sơ đồ tổng quát; tám use case chính được đặc tả theo mẫu thống nhất mười lăm trường, kèm luồng chính, luồng rẽ nhánh, luồng ngoại lệ và quy tắc nghiệp vụ. Đây là nền tảng cho việc kiểm thử chấp nhận và cho việc viết tài liệu hướng dẫn sử dụng.

**Thứ tư, đề xuất được kiến trúc khả thi.** Kiến trúc modular monolith trên Tauri v2 với backend Rust và SQLite đáp ứng đồng thời ba yêu cầu: chạy cục bộ không phụ thuộc máy chủ, kiểm soát chặt dữ liệu của AI, và duy trì tính toàn vẹn khi có nhiều nguồn thay đổi. Các cơ chế khóa ghi theo bản nháp, tuần tự hóa theo thứ tự kế hoạch, niêm phong phiên bản bất biến và biên nhận workflow là những đóng góp kỹ thuật quan trọng, giải quyết trực tiếp bài toán đồng thời của ứng dụng một người dùng có tác vụ nền.

**Thứ năm, thiết kế được mô hình dữ liệu và bộ dữ liệu nền.** Schema 66 bảng được chia thành năm nhóm theo vòng đời; cơ chế ánh xạ hai tầng định danh (mã slug khi làm việc, UUID khi cam kết) được mô tả tường minh; bộ dữ liệu danh mục gồm 50 thể loại, 19 motif và 85 chủ đề được chuẩn hóa với mã ổn định, đủ dùng cho giai đoạn đầu.

Bên cạnh kết quả, báo cáo cũng chỉ ra các giới hạn: giao diện mới được kiểm chứng ở mức web preview; backend chưa chạy đầy đủ trong môi trường nghiên cứu; chưa có đánh giá định lượng về chất lượng văn bản do AI sinh; một số luồng (Writing Mode, xuất bản) mới ở mức thiết kế.

# HƯỚNG PHÁT TRIỂN TƯƠNG LAI

1. **Hoàn thiện backend theo kế hoạch sáu pha.** Ưu tiên các sprint thuộc pha 2–3 (IPC, schema, miền bản nháp) vì đây là nền tảng cho mọi tính năng sau. Mỗi sprint cần vượt cổng kiểm thử tương ứng trước khi chuyển sang pha kế tiếp.
2. **Đánh giá định lượng chất lượng văn bản.** Xây dựng bộ tiêu chí (tính nhất quán nhân vật, mạch truyện, độ giàu chi tiết, mức độ bám dàn ý) và so sánh giữa ba cấu hình: không dùng harness, harness chỉ dùng prompt, harness đầy đủ với tool và dữ liệu cấu trúc. Kết quả sẽ là bằng chứng thuyết phục cho giá trị của phương pháp.
3. **Mở rộng khả năng cộng tác.** Hiện tại hệ thống là ứng dụng một người dùng. Hướng phát triển là xuất/nhập gói dự án có chữ ký, cho phép nhiều tác giả cùng làm việc thông qua một kho lưu trữ trung gian, kèm cơ chế giải quyết xung đột dựa trên mã thực thể thay vì văn bản.
4. **Bổ sung trợ lý biên tập.** Sau khi có nội dung chương, hệ thống có thể thêm luồng phân tích: phát hiện mâu thuẫn thiết lập, gợi ý nhịp điệu, đối chiếu với dàn ý. Đây là hướng mở rộng tự nhiên của Writing Mode.
5. **Cải thiện trải nghiệm desktop.** Kiểm thử trên cả ba hệ điều hành, tối ưu thời gian khởi động, bổ sung chế độ đọc tập trung và phím tắt cho người viết chuyên nghiệp.
6. **Tăng cường khả năng tùy biến danh mục.** Cho phép người dùng tự định nghĩa motif, chủ đề và phong cách riêng, kèm cơ chế nhập/xuất và chia sẻ bộ danh mục giữa các dự án.
7. **Mở rộng đa ngôn ngữ.** Hiện hệ thống hỗ trợ tiếng Việt và tiếng Anh ở giao diện; hướng phát triển là cho phép sáng tác bằng nhiều ngôn ngữ hơn, với bộ dữ liệu danh mục dịch tương ứng.
