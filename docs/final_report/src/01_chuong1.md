# CHƯƠNG 1: TRÌNH BÀY KHẢO SÁT HIỆN TRẠNG, MÔ TẢ HỆ THỐNG VÀ CÁC QUY TRÌNH NGHIỆP VỤ LIÊN QUAN

## 1.1 PHÂN TÍCH HIỆN TRẠNG

### 1.1.1 Thực trạng ứng dụng AI trong sáng tác văn học hiện nay

Sự xuất hiện của các mô hình ngôn ngữ lớn đã thay đổi cách người viết tương tác với công cụ. Ba nhóm sản phẩm đang tồn tại song song:

**Nhóm thứ nhất — công cụ sinh văn bản chuyên biệt cho hư cấu.** NovelAI hướng tới người viết nghiệp dư với tính năng "Storyteller", cho phép gợi ý tiếp nối đoạn văn, kiểm soát phong cách bằng "memory" và "lorebook" [C1][C2]. Sudowrite tập trung vào quy trình hậu kỳ: viết lại đoạn văn, mở rộng mô tả, gợi ý ý tưởng theo "Story Bible" [C3]. Novelcrafter đẩy cấu trúc hóa xa hơn với "Codex" — một wiki nhân vật, bối cảnh và mối quan hệ mà AI có thể tra cứu trong lúc sinh [C4].

Điểm chung của nhóm này là lấy *chất lượng câu chữ của từng đoạn* làm trung tâm. Người viết mô tả bối cảnh trong vài đoạn ghi chú, AI đọc chúng như ngữ cảnh bổ sung rồi sinh tiếp. Cách tiếp cận này hiệu quả với truyện ngắn, truyện một chương, hoặc khi tác giả viết tuần tự và tự kiểm soát nhất quán.

**Nhóm thứ hai — trợ lý đa dụng.** ChatGPT, Claude, Gemini với khả năng đọc tệp và ghi nhớ dài. Người viết có thể dán hàng chục trang dàn ý và yêu cầu mô hình viết chương. Linh hoạt nhưng không có cơ chế bảo đảm: mỗi phiên chat là một "thế giới" riêng, không có nguồn sự thật duy nhất, không có kiểm tra ràng buộc giữa các chương [B2][B10].

**Nhóm thứ ba — khung phát triển agent (agent framework).** LangChain, LlamaIndex, Semantic Kernel hoặc các "harness" tự phát triển. Đây là nơi các kỹ thuật như ReAct [B1], tool calling [B2], RAG [B5] được áp dụng. Tuy nhiên phần lớn khung này phục vụ bài toán tri thức (hỏi đáp, tra cứu) chứ không phục vụ bài toán *sáng tác dài hơi*: dữ liệu là văn bản phi cấu trúc, không có ràng buộc nghiệp vụ, không có khái niệm "bản nháp chờ duyệt".

Ba nhóm sản phẩm trên cho thấy thị trường đã có đủ "nguyên liệu" (mô hình tốt, kỹ thuật gọi tool, hạ tầng desktop), nhưng còn thiếu một lớp trung gian biến ý tưởng sáng tác thành **dữ liệu có cấu trúc được kiểm chứng** trước khi AI viết.

### 1.1.2 Những khó khăn và hạn chế trong quá trình viết truyện

Qua khảo sát tài liệu và phân tích các sản phẩm hiện có, có sáu nhóm khó khăn lặp lại:

| Vấn đề | Biểu hiện thực tế | Hệ quả |
| --- | --- | --- |
| Trôi ngữ cảnh (context drift) | Càng về sau, AI càng quên hoặc hiểu sai thiết lập ban đầu | Nhân vật, thế giới mâu thuẫn; phải viết lại chương |
| Mất nhất quán nhân vật | Tính cách, ngoại hình, năng lực thay đổi không có chủ đích | Người đọc mất niềm tin vào nhân vật |
| Quá tải ngữ cảnh | Dán quá nhiều ghi chú khiến chi phí tăng, chất lượng giảm [B8][C5] | Tốn token, đầu ra loãng |
| Thiếu kiểm soát cấu trúc | AI viết lan man, tuyến phụ lấn át tuyến chính | Dàn ý bị phá vỡ sau vài chương |
| Dữ liệu phân mảnh | Ghi chú nằm ở nhiều tệp, nhiều ứng dụng | Khó tra cứu, dễ mất dữ liệu |
| Không có lịch sử thay đổi | Sửa xong không biết bản nào là bản chuẩn | Không thể quay lui khi AI sửa sai |

Trong sáu vấn đề trên, ba vấn đề đầu mang tính *kỹ thuật của mô hình* và đã có hướng giảm thiểu (cửa sổ ngữ cảnh lớn hơn, RAG, tóm tắt phân tầng). Ba vấn đề sau mang tính *tổ chức quy trình* và chỉ giải quyết được nếu hệ thống có mô hình dữ liệu và luật nghiệp vụ chặt chẽ.

### 1.1.3 Sự cần thiết của việc xây dựng hệ thống Merlink Harness

Từ phân tích trên, có bốn lý do khiến một harness chuyên biệt cho sáng tác dài hơi là cần thiết:

1. **Biến sáng tác thành quy trình có trạng thái.** Thay vì một cuộc trò chuyện không có điểm dừng, hệ thống chia thành bảy giai đoạn, mỗi giai đoạn có dữ liệu vào/ra rõ ràng và điều kiện chuyển bước. Trạng thái nghiệp vụ tồn tại độc lập với lịch sử chat.
2. **Bắt AI làm việc qua giao diện có kiểu.** Mọi thay đổi dữ liệu đi qua tool call có tham số xác định; AI không được sửa trực tiếp tệp JSON hay cơ sở dữ liệu. Cách làm này cho phép kiểm tra tham số, gán nguồn dữ liệu và từ chối thao tác sai.
3. **Bảo vệ dữ liệu do con người tạo.** Mỗi trường mang nhãn nguồn (`user`, `ai_generated`, `user_edited`). Dữ liệu người dùng chỉ bị thay đổi khi có xác nhận. Đây là cơ chế mà các công cụ hiện có chưa thực hiện được ở mức trường dữ liệu.
4. **Có nguồn sự thật duy nhất.** Sau khi người dùng bấm "Bắt đầu viết", toàn bộ dàn nháp được chuyển thành dữ liệu quan hệ trong SQLite với khóa ngoại, chỉ mục và lịch sử phiên bản. Không còn tình trạng "ba bản dữ liệu ở ba nơi".

## 1.2 MÔ TẢ HỆ THỐNG

Merlink Harness là ứng dụng desktop hỗ trợ sáng tác truyện dài, được tổ chức thành bốn phân hệ chức năng và một tầng dữ liệu dùng chung:

**Phân hệ 1 — Chat & Planning Harness.** Người dùng trò chuyện bằng ngôn ngữ tự nhiên để mô tả tác phẩm. Một vòng lặp AI dạng ReAct (suy luận — hành động) chuyển từng thông tin người dùng nêu thành tool call ghi vào bản nháp cấu trúc. Giao diện hiển thị hai chỉ số tiến độ trên 46 thuộc tính chuẩn: phần trăm dữ liệu do AI đề xuất và phần trăm dữ liệu đã được người dùng xác nhận.

**Phân hệ 2 — Draft Workspace (không gian dàn ý).** Là bảng điều khiển chia đôi màn hình, mở khóa dần sáu bước dàn ý: Arc vĩ mô, chương và cảnh thô, bối cảnh thế giới, cấu hình phân cảnh, hồ sơ nhân vật (ba nấc), và động lực quan hệ. Mỗi bước phải được người dùng xác nhận trước khi bước sau mở ra.

**Phân hệ 3 — Writing Mode.** Sau khi dàn ý được cam kết vào cơ sở dữ liệu, người dùng chuyển sang chế độ viết: chọn chương, sinh nội dung cảnh theo các khối, duyệt, sửa, lưu. Cơ chế khóa cảnh và lịch sử phiên bản cảnh bảo đảm không ghi đè nội dung đã duyệt.

**Phân hệ 4 — Thư viện và danh mục.** Gồm thư viện dự án, thư viện nhân vật, bối cảnh thế giới và khu vực cấu hình System Prompt. Đây là nơi tra cứu các bộ dữ liệu nền: thể loại, motif, chủ đề, phong cách diễn đạt, cấu hình người kể chuyện và chỉ dẫn tự sự.

**Tầng dữ liệu.** Schema quan hệ gồm 66 bảng, chia thành tám nhóm: danh mục (motif, thể loại, phong cách, người kể), dự án và phiên, nội dung (arc, chương, cảnh, phiên bản cảnh), nhân vật, thế giới và tri thức, chủ đề và luận điểm, quan hệ, và nhóm bản nháp/workflow (phiên bản draft, patch chờ duyệt, biên nhận workflow).

Về tác nhân, hệ thống có ba bên tương tác: **tác giả** (người dùng cuối), **AI Harness** (vòng lặp agent do backend điều khiển) và **nhà cung cấp LLM** (dịch vụ ngoài, giao tiếp qua HTTPS/SSE).

## 1.3 CÁC QUY TRÌNH NGHIỆP VỤ CỐT LÕI

Hệ thống có sáu quy trình nghiệp vụ cốt lõi. Mỗi quy trình được mô tả bằng mục tiêu, các bước chính và điểm kiểm soát.

Sơ đồ dòng chảy tổng thể của bảy giai đoạn được trình bày ở Chương 4 (Hình 2) cùng các điểm kiểm soát tương ứng.

### 1.3.1 Quy trình thu thập yêu cầu và cổng sẵn sàng (Readiness Gate)

**Mục tiêu:** chuyển ý tưởng tự do thành bộ mười trường tiền đề đủ để định danh tác phẩm.

| Bước | Nội dung | Kết quả |
| --- | --- | --- |
| 1 | Người dùng mô tả ý tưởng, có thể kèm lộ trình chi tiết ngay từ đầu | Lịch sử chat |
| 2 | Harness bóc tách thông tin trong từng lượt, gọi tool ghi vào bản nháp | Các trường `require` và `optional` được điền dần |
| 3 | Hệ thống đánh giá 46 thuộc tính, hiển thị hai chỉ số tiến độ | Người dùng thấy phần đã xác nhận và phần AI đề xuất |
| 4 | Khi mười trường tiền đề đầy đủ, cổng sẵn sàng mở | Nút "Xây dựng bản nháp" được kích hoạt |

Mười trường tiền đề gồm: tên tác phẩm, tóm tắt tiền đề, ngôn ngữ sáng tác, số Arc, số chương, định mức số chữ mỗi chương, danh sách thể loại, và với mỗi nhân vật khởi tạo là tên hiển thị/biệt danh, giới tính sinh học và mã motif.

**Điểm kiểm soát:** AI không tự quyết định các giá trị định lượng hoặc định danh (số Arc, số chương, tên nhân vật, thể loại). Nếu thiếu, AI phải hỏi hoặc đề xuất kèm lý do và chờ người dùng chấp thuận.

### 1.3.2 Quy trình dựng dàn ý lũy tiến sáu bước

**Mục tiêu:** đi từ dàn ý vĩ mô đến hồ sơ nhân vật chi tiết mà không làm người dùng quá tải.

| Bước | Tên bước | Dữ liệu sinh ra | Điều kiện mở |
| --- | --- | --- | --- |
| 1 | Arc Overview | Danh sách Arc, motif xung đột, khối biến cố, chủ đề và luận điểm | Cổng sẵn sàng đã mở |
| 2 | Chapter / Scene | Chương trong từng Arc, cảnh thô với tóm tắt và địa điểm dự kiến | Bước 1 đã xác nhận |
| 3 | World Building | Cây bối cảnh phân cấp và các mục tri thức (lore) | Bước 2 đã xác nhận |
| 4 | Scene Config | Góc nhìn, phong cách, địa điểm chính thức của từng cảnh | Bước 3 đã xác nhận |
| 5 | Character | Ngoại hình (5.1), tâm lý chiều sâu (5.2), ký ức và năng lực (5.3) | Bước 4 đã xác nhận |
| 6 | Relationship | Ma trận cảm xúc theo cảnh, vai trò quan hệ | Bước 5 đã xác nhận |

**Điểm kiểm soát:** mỗi lần chuyển bước phải qua hộp thoại xác nhận; AI chỉ sinh nội dung phái sinh cho bước hiện tại, không sinh trước các bước sau. Dữ liệu người dùng đã nêu trước vẫn được ghi nhận và tính vào tiến độ dù giao diện chưa mở.

### 1.3.3 Quy trình phản hồi, patch và quản lý phiên bản

**Mục tiêu:** cho phép người dùng sửa bất kỳ phần nào của dàn ý mà không phá vỡ dữ liệu đã có.

- Người dùng bật **Feedback Mode**, trỏ vào một phần tử (Arc, cảnh, nhân vật); phần tử được gắn ngữ cảnh và đưa vào ô nhập chat.
- Yêu cầu sửa được chuyển thành **patch**. Nếu trường đích là `ai_generated`, patch áp dụng ngay; nếu là `user` hoặc `user_edited`, patch chờ xác nhận.
- Cuối mỗi lượt có thay đổi được áp dụng, hệ thống **niêm phong một phiên bản** (snapshot bất biến của toàn bộ bản nháp) với khóa niêm phong duy nhất theo phiên và lượt.
- Người dùng có thể khôi phục phiên bản cũ; thao tác khôi phục đồng thời trả dữ liệu, giai đoạn và bước phụ về đúng thời điểm, đồng thời vô hiệu hóa các patch đang chờ.

**Điểm kiểm soát:** không tạo phiên bản cho lượt không ghi dữ liệu; phiên bản là nguồn phục hồi duy nhất; trạng thái làm việc trong bộ nhớ không được coi là đã lưu bền.

### 1.3.4 Quy trình workflow phân đoạn cho truyện dài

Với tác phẩm hàng trăm chương, không thể nhồi toàn bộ dàn ý vào một lượt gọi mô hình. Hệ thống chia việc thành các **lần chạy** (với Phase 1–2) và **vòng** (với Phase 3–6):

| Tham số | Mặc định | Ý nghĩa |
| --- | ---: | --- |
| `OUTLINE_CAP` | 50 | Số chương tối đa trong một lần chạy dàn ý |
| `OUTLINE_LOOKBACK` | 10 | Số chương đọc lùi từ lần chạy trước |
| `OUTLINE_PAGE` | 50 | Số chương mỗi trang tra cứu mục lục |
| `UNIT_CHAPTER` | 30 | Số chương tối đa cho một subagent |
| `CONTEXT_MARGIN` | 5 | Số chương đọc thêm mỗi bên khi chia theo chương |
| `MAX_PARALLEL` | 6 | Số subagent song song trong một vòng |
| `DRAFT_LOCK_TIMEOUT` | 30 giây | Thời gian chờ khóa ghi bản nháp |

**Ví dụ kiểm chứng:** một Arc 145 chương được chia thành bốn lần chạy: ghi 1–50; đọc lùi 41–50 và ghi 51–90; đọc lùi 81–90 và ghi 91–130; đọc lùi 121–130 và ghi 131–145. Mọi khoảng ghi phủ kín dải chương, không trùng và không hở.

**Điểm kiểm soát:** mỗi vòng có biên nhận hoàn tất bất biến (insert-once) gắn với định danh kế hoạch; nếu vòng có thay đổi dữ liệu thì việc niêm phong phiên bản và ghi biên nhận nằm trong cùng một giao dịch; hủy workflow là dừng có kiểm soát, không ngắt giữa một thao tác ghi.

### 1.3.5 Quy trình cam kết dữ liệu (Commit)

**Mục tiêu:** chuyển bản nháp JSON sang cơ sở dữ liệu quan hệ một cách nguyên tử.

- Nút "Bắt đầu viết" chỉ xuất hiện ở cuối bước 6 và mở hộp thoại xác nhận kiêm bản tóm tắt kế hoạch ghi.
- Hệ thống kiểm tra toàn bộ bất biến: tổng số chương bằng mục tiêu, số Arc bằng mục tiêu, thứ tự chương duy nhất toàn truyện, mỗi cảnh có địa điểm hợp lệ, quy tắc góc nhìn phù hợp cấu hình người kể, năng lực nhân vật gắn với cảnh.
- Nếu còn patch chờ duyệt hoặc vi phạm bất biến, commit bị chặn và lỗi được hiển thị ngay trong hộp thoại.
- Khi xác nhận, toàn bộ cây dữ liệu được ghi trong một giao dịch: sinh mã `code` cho từng thực thể, ánh xạ sang khóa `id` UUIDv7, kiểm tra trùng mã giữa các bảng, cập nhật trạng thái phiên bản nháp thành `committed`.

### 1.3.6 Quy trình sáng tác chương (Writing Mode)

Sau commit, người dùng chọn chương và từng cảnh để sinh nội dung. Nội dung đi qua chuỗi trạng thái: **sinh** (generate) → **vùng đệm** (buffer) → **duyệt** (review) → **chấp nhận** (accept) → **đánh dấu sửa** (dirty) → **lưu cấp chương** (save). Mỗi cảnh có thể khóa để tránh ghi đè; lịch sử nội dung cảnh được lưu theo từng phiên bản; biên nhận lưu chương bảo đảm thao tác lưu là idempotent.

## 1.4 YÊU CẦU HỆ THỐNG

### 1.4.1 Yêu cầu chức năng

| Mã | Nhóm chức năng | Yêu cầu |
| --- | --- | --- |
| FR-01 | Quản lý dự án | Tạo, mở, tìm kiếm, sắp xếp, xóa dự án; hiển thị thống kê số chữ, chương, nhân vật, bối cảnh |
| FR-02 | Thu thập yêu cầu | Trò chuyện tự nhiên; bóc tách thông tin thành tool call; hiển thị tiến độ hai chỉ số |
| FR-03 | Kiểm duyệt sẵn sàng | Đánh giá mười trường tiền đề; chặn mở dàn ý khi chưa đủ |
| FR-04 | Dàn ý lũy tiến | Sinh dàn ý theo sáu bước; xác nhận từng bước; mở khóa tuần tự |
| FR-05 | Phản hồi | Gắn ngữ cảnh phần tử; tạo patch; xem trước và xác nhận patch |
| FR-06 | Phiên bản | Niêm phong phiên bản cuối lượt; liệt kê, xem, khôi phục phiên bản |
| FR-07 | Workflow | Chia việc thành lần chạy/vòng; hiển thị tiến độ; hủy và tiếp tục |
| FR-08 | Commit | Kiểm tra bất biến; ghi nguyên tử; báo lỗi có cấu trúc |
| FR-09 | Nhân vật | Quản lý hồ sơ ngoại hình, tâm lý, ký ức, năng lực, quan hệ |
| FR-10 | Bối cảnh | Cây địa lý, tri thức theo loại, liên kết với cảnh |
| FR-11 | Danh mục | Tra cứu thể loại, motif, chủ đề, phong cách, người kể |
| FR-12 | Writing Mode | Sinh, duyệt, sửa, khóa cảnh; lưu cấp chương; lịch sử cảnh |

### 1.4.2 Yêu cầu phi chức năng

| Mã | Thuộc tính | Yêu cầu |
| --- | --- | --- |
| NFR-01 | Toàn vẹn dữ liệu | Giao dịch nguyên tử; khóa ngoại bật; kiểm tra schema khi khởi động |
| NFR-02 | Đồng thời | Một khóa ghi theo bản nháp; thao tác ghi tuần tự hóa; đọc từ ảnh chụp nhất quán |
| NFR-03 | Độ bền | Chế độ WAL, đồng bộ theo cấu hình; niêm phong phiên bản có khóa duy nhất |
| NFR-04 | Khả năng phục hồi | Khôi phục từ phiên bản đã niêm phong; phát hiện lượt bị gián đoạn |
| NFR-05 | Hiệu năng | Giao diện phản hồi dưới 100 ms với thao tác cục bộ; không giữ khóa khi chờ mạng |
| NFR-06 | Bảo mật | AI không đọc/ghi tệp trực tiếp; khóa API không lưu trong cơ sở dữ liệu nghiệp vụ |
| NFR-07 | Khả dụng | Giao diện tiếng Việt/tiếng Anh; hỗ trợ bàn phím; độ tương phản đạt chuẩn |
| NFR-08 | Bảo trì | Kiến trúc module rõ ràng; tài liệu là nguồn sự thật; kiểm thử tự động theo hợp đồng |
| NFR-09 | Tương thích | Chạy trên Windows, macOS, Linux thông qua Tauri; dữ liệu cục bộ, không phụ thuộc máy chủ |
