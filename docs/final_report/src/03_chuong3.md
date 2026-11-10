# CHƯƠNG 3: PHÂN TÍCH HỆ THỐNG VÀ ĐẶC TẢ USE CASE

## 3.1 XÁC ĐỊNH CÁC TÁC NHÂN (ACTORS) TRONG HỆ THỐNG

| Tác nhân | Loại | Vai trò | Quyền tương tác |
| --- | --- | --- | --- |
| Tác giả (người dùng) | Chính | Người sáng tác, chủ sở hữu dữ liệu truyện | Tạo dự án, trò chuyện, duyệt dàn ý, phản hồi, xác nhận patch, khôi phục phiên bản, commit, viết chương |
| AI Harness (agent) | Phụ (tác nhân hệ thống) | Vòng lặp suy luận — hành động do backend điều khiển | Chỉ đọc/ghi dữ liệu thông qua tool call được cấp; không truy cập tệp hay cơ sở dữ liệu trực tiếp |
| Tool Executor (backend) | Phụ (nội bộ) | Kiểm tra quyền gọi, kiểm tra tham số, áp dụng thay đổi vào trạng thái làm việc | Thực thi tool, cấp mã định danh, ghi nhật ký, trả kết quả có cấu trúc |
| Nhà cung cấp LLM | Phụ (dịch vụ ngoài) | Sinh phản hồi ngôn ngữ và quyết định gọi tool | Nhận prompt qua HTTPS, trả về luồng sự kiện (SSE) |
| Hệ điều hành / hệ thống tệp | Phụ (hạ tầng) | Lưu tệp cơ sở dữ liệu, báo cáo trung gian, ảnh bìa dự án | Được backend gọi qua API hệ thống, không lộ cho AI |

**Ghi chú về phạm vi quyền của AI Harness.** Đây là điểm khác biệt cốt lõi so với các trợ lý viết văn thông thường: AI không có kênh nào để ghi dữ liệu ngoài danh mục tool được cấp. Mỗi tool có một bộ tham số có kiểu, một tham số bắt buộc khai báo nguồn dữ liệu (`declared_sources` hoặc `declared_source`), và một bộ luật kiểm tra ở backend. Nếu AI bỏ trống hoặc khai báo sai nguồn, chỉ trường đó bị từ chối `rejected`; toàn bộ lượt không bị hủy để tránh mất các phần đúng.

## 3.2 SƠ ĐỒ USE CASE TỔNG QUÁT

![Sơ đồ use case tổng quát của hệ thống Merlink Harness](images/fig_usecase.png){width=100%}

Sơ đồ thể hiện mười use case nghiệp vụ chính trong ranh giới hệ thống. Tác giả kết nối với toàn bộ các use case vì đây là tác nhân duy nhất khởi tạo luồng nghiệp vụ. AI Harness kết nối với bốn use case có sinh nội dung: trò chuyện, duyệt dàn ý, phản hồi và sáng tác chương. Ba quan hệ `«include»` thể hiện ràng buộc bắt buộc: duyệt dàn ý luôn kiểm tra cổng sẵn sàng; phản hồi luôn tạo phiên bản khi có thay đổi; commit luôn niêm phong phiên bản cuối trước khi chuyển dữ liệu. Nhà cung cấp LLM kết nối với use case sáng tác và trò chuyện qua kênh HTTPS/SSE.

## 3.3 ĐẶC TẢ CHI TIẾT CÁC USE CASE CHÍNH

Các bảng dưới đây theo mẫu đặc tả use case thống nhất của dự án, gồm mười lăm trường: mã, tên, module, tác nhân, mô tả, trigger, tiền điều kiện, hậu điều kiện, luồng chính, luồng rẽ nhánh, luồng ngoại lệ, quy tắc nghiệp vụ, độ ưu tiên và ghi chú.

### 3.3.1 UC_PROJ_01 — Quản lý dự án và thư viện

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_PROJ_01 |
| **Tên Use Case** | Quản lý dự án và thư viện |
| **Module/Phân hệ** | Thư viện dự án |
| **Actor chính** | Tác giả |
| **Actor phụ** | Backend Rust, SQLite |
| **Mô tả ngắn** | Tác giả tạo, tìm kiếm, mở và xóa dự án; xem thống kê tổng quan của từng dự án. |
| **Trigger** | Tác giả chọn mục "Dự án" trên thanh điều hướng hoặc bấm "Xem nhanh" trên một thẻ dự án. |
| **Precondition** | Ứng dụng đã khởi động; cơ sở dữ liệu đã được kiểm tra schema và mở thành công. |
| **Postcondition** | Danh sách dự án phản ánh trạng thái mới nhất; dự án được chọn mở ở chế độ xem tương ứng. |
| **Main Flow** | 1. Hệ thống nạp danh sách dự án kèm ảnh bìa, trạng thái, mô tả, số chữ, số chương.<br>2. Tác giả lọc theo từ khóa hoặc sắp xếp theo tên/ngày sửa.<br>3. Tác giả chọn một dự án.<br>4. Hệ thống mở trang chi tiết, hiển thị danh sách Arc và chương.<br>5. Tác giả chuyển giữa chế độ Xem và chế độ Sáng tác. |
| **Alternative Flow** | 2a. Không có dự án nào khớp từ khóa → hiển thị trạng thái rỗng kèm gợi ý tạo mới.<br>5a. Dự án chưa có Arc → chế độ Sáng tác chuyển sang luồng tạo dàn ý mới. |
| **Exception Flow** | E1. Cơ sở dữ liệu không mở được → hiển thị lỗi khởi động, chặn thao tác ghi.<br>E2. Xóa dự án thất bại do ràng buộc → hiển thị lỗi, giữ nguyên dự án. |
| **Business Rules** | BR1: Xóa dự án là xóa mềm, dữ liệu liên quan giữ lại theo ma trận cascade.<br>BR2: Thứ tự chương là duy nhất trên toàn truyện, không theo từng Arc.<br>BR3: Trạng thái dự án thuộc tập `PLANNING`, `ONGOING`, `IN REVIEW`, `COMPLETED`. |
| **Priority** | High |
| **Ghi chú** | Phụ thuộc UC_COMMIT_01 để có dự án sau khi cam kết dàn ý. |

### 3.3.2 UC_CHAT_01 — Trò chuyện thu thập yêu cầu

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_CHAT_01 |
| **Tên Use Case** | Trò chuyện thu thập yêu cầu |
| **Module/Phân hệ** | Chat & Planning Harness |
| **Actor chính** | Tác giả |
| **Actor phụ** | AI Harness, Tool Executor, Nhà cung cấp LLM |
| **Mô tả ngắn** | Tác giả mô tả ý tưởng bằng ngôn ngữ tự nhiên; AI bóc tách và ghi thông tin vào bản nháp cấu trúc bằng tool call. |
| **Trigger** | Tác giả gửi tin nhắn trong khung chat. |
| **Precondition** | Có phiên chat và bản nháp đang ở trạng thái `editing`. |
| **Postcondition** | Bản nháp được cập nhật; hai chỉ số tiến độ được tính lại; nếu có thay đổi ghi thì một phiên bản mới được niêm phong. |
| **Main Flow** | 1. Tác giả nhập nội dung.<br>2. Hệ thống ghép prompt gồm System Prompt, danh mục tool, trạng thái bản nháp hiện tại.<br>3. Gọi mô hình qua HTTPS/SSE, nhận chuỗi sự kiện.<br>4. Với mỗi tool call, backend kiểm tra quyền, tham số và nguồn dữ liệu.<br>5. Áp dụng thay đổi vào trạng thái làm việc; trả kết quả từng trường.<br>6. Cuối lượt, niêm phong một phiên bản nếu có trường `applied`.<br>7. Giao diện cập nhật tin nhắn, thẻ trạng thái và thanh tiến độ. |
| **Alternative Flow** | 4a. AI gọi tool đọc (tra cứu thể loại, motif, nhân vật) → trả dữ liệu, không tạo phiên bản.<br>5a. Trường đích thuộc dữ liệu người dùng → tạo patch chờ xác nhận thay vì ghi đè. |
| **Exception Flow** | E1. Mô hình trả lỗi hoặc quá thời gian → hiển thị lỗi, giữ nguyên bản nháp.<br>E2. Tool call sai định dạng → từ chối riêng trường đó, ghi nhật ký.<br>E3. Số lần từ chối liên tiếp vượt ngưỡng → dừng lượt, niêm phong phần đã ghi.<br>E4. Không có khóa API hợp lệ → chặn gọi, hướng dẫn cấu hình. |
| **Business Rules** | BR1: AI không tự quyết định số Arc, số chương, tên nhân vật, thể loại.<br>BR2: Mọi thông tin người dùng nêu được ghi nhận ngay, không phụ thuộc giai đoạn đang mở.<br>BR3: Mỗi trường mang nhãn `user`/`ai_generated`/`user_edited`; thiếu nhãn thì bị từ chối. |
| **Priority** | High |
| **Ghi chú** | Phụ thuộc UC_GATE_01 để phản ánh mức sẵn sàng. |

### 3.3.3 UC_GATE_01 — Xem tiến độ và kiểm duyệt sẵn sàng

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_GATE_01 |
| **Tên Use Case** | Xem tiến độ và kiểm duyệt sẵn sàng |
| **Module/Phân hệ** | Chat & Planning Harness |
| **Actor chính** | Tác giả |
| **Actor phụ** | Tool Executor, Progress Engine |
| **Mô tả ngắn** | Hệ thống đánh giá 46 thuộc tính chuẩn và hiển thị hai chỉ số: dữ liệu đã xác nhận và dữ liệu AI đề xuất. |
| **Trigger** | Mỗi khi bản nháp thay đổi, hoặc khi tác giả mở chi tiết tiến trình. |
| **Precondition** | Có bản nháp đang làm việc. |
| **Postcondition** | Chỉ số tiến độ và danh sách trường còn thiếu được cập nhật; trạng thái nút "Xây dựng bản nháp" được xác định. |
| **Main Flow** | 1. Engine duyệt 46 thuộc tính.<br>2. Mỗi thuộc tính đạt 1 điểm nếu có ít nhất một giá trị hợp lệ, và được phân loại theo nguồn.<br>3. Hiển thị phần trăm đã xác nhận và phần trăm AI đề xuất.<br>4. Tác giả mở chi tiết để xem trường nào còn thiếu và câu hỏi gợi ý.<br>5. Khi đủ mười trường tiền đề, hệ thống mở khóa nút "Xây dựng bản nháp". |
| **Alternative Flow** | 3a. Dữ liệu thuộc giai đoạn chưa mở vẫn được tính → chỉ số tăng nhưng giao diện chưa hiển thị chi tiết. |
| **Exception Flow** | E1. Bản nháp không đọc được do lỗi lưu trữ → hiển thị lỗi, giữ chỉ số lần trước. |
| **Business Rules** | BR1: Tổng điểm luôn chia cho 46, không đổi theo số giai đoạn đã mở.<br>BR2: Một thuộc tính dạng mảng chỉ đạt 1 điểm khi có ít nhất một phần tử hợp lệ.<br>BR3: Không gộp hai chỉ số thành một phần trăm duy nhất. |
| **Priority** | High |
| **Ghi chú** | Là điều kiện của UC_DRAFT_01. |

### 3.3.4 UC_DRAFT_01 — Duyệt dàn ý lũy tiến

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_DRAFT_01 |
| **Tên Use Case** | Duyệt dàn ý lũy tiến sáu bước |
| **Module/Phân hệ** | Draft Workspace |
| **Actor chính** | Tác giả |
| **Actor phụ** | AI Harness, Workflow Orchestrator, Tool Executor |
| **Mô tả ngắn** | Tác giả duyệt dàn ý từ vĩ mô đến vi mô; mỗi bước phải xác nhận trước khi bước sau mở khóa. |
| **Trigger** | Tác giả bấm "Xây dựng bản nháp" khi cổng sẵn sàng đã mở. |
| **Precondition** | Mười trường tiền đề đã đầy đủ; không có patch chờ duyệt ở bước hiện tại. |
| **Postcondition** | Bước được xác nhận, một phiên bản `phase_confirm` được niêm phong, bước kế tiếp mở khóa. |
| **Main Flow** | 1. Mở bảng điều khiển chia đôi: chat bên trái, không gian dàn ý bên phải.<br>2. Hiển thị bước 1 (Arc Overview) với các Arc, motif, chủ đề.<br>3. Tác giả trao đổi để chỉnh sửa; AI cập nhật qua tool call.<br>4. Tác giả bấm "Tiếp tục"; hệ thống mở hộp thoại xác nhận.<br>5. Xác nhận → niêm phong phiên bản, mở khóa bước 2.<br>6. Lặp lại cho các bước 2 đến 6. |
| **Alternative Flow** | 3a. Tác giả yêu cầu tạo lại (regenerate) → chỉ các trường AI sinh bị thay, dữ liệu người dùng giữ nguyên.<br>4a. Tác giả hủy hộp thoại → ở lại bước hiện tại. |
| **Exception Flow** | E1. Bước trước chưa xác nhận → nút chuyển bước bị vô hiệu.<br>E2. Thiếu dữ liệu bắt buộc của bước → chặn xác nhận, chỉ rõ trường thiếu.<br>E3. Workflow đang chạy chiếm khóa ghi → hiển thị "bản nháp đang bận". |
| **Business Rules** | BR1: Mở khóa tuần tự, không nhảy bước.<br>BR2: Mỗi bước chỉ nhận nội dung phái sinh cho chính bước đó.<br>BR3: Xác nhận bước là phiên bản `phase_confirm`, không ghi sang cơ sở dữ liệu quan hệ. |
| **Priority** | High |
| **Ghi chú** | Sáu bước tương ứng Phase 1 đến Phase 6 trong mô hình nghiệp vụ. |

### 3.3.5 UC_FEED_01 — Phản hồi và xử lý patch

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_FEED_01 |
| **Tên Use Case** | Phản hồi và xử lý patch |
| **Module/Phân hệ** | Chat & Draft Workspace |
| **Actor chính** | Tác giả |
| **Actor phụ** | AI Harness, Tool Executor |
| **Mô tả ngắn** | Tác giả trỏ vào một phần tử dàn ý, yêu cầu chỉnh sửa; hệ thống tạo patch và xử lý theo nguồn dữ liệu. |
| **Trigger** | Tác giả bật Feedback Mode và bấm vào phần tử, hoặc gõ yêu cầu sửa trong chat. |
| **Precondition** | Bản nháp đang mở; phần tử đích tồn tại. |
| **Postcondition** | Patch được áp dụng (dữ liệu AI) hoặc chờ xác nhận (dữ liệu người dùng); phiên bản được niêm phong nếu có thay đổi. |
| **Main Flow** | 1. Tác giả bật Feedback Mode; con trỏ đổi thành biểu tượng trích dẫn.<br>2. Bấm vào Arc/cảnh/nhân vật → gắn ngữ cảnh vào ô nhập.<br>3. Tác giả mô tả thay đổi mong muốn.<br>4. AI sinh patch có phạm vi theo mã định danh ổn định.<br>5. Hệ thống phân loại: dữ liệu AI → áp dụng ngay; dữ liệu người dùng → chờ xác nhận.<br>6. Tác giả xem trước, xác nhận hoặc từ chối.<br>7. Niêm phong phiên bản nếu có ít nhất một patch được áp dụng. |
| **Alternative Flow** | 6a. Tác giả chọn "Áp dụng tất cả" → các patch được duyệt theo thứ tự tạo, patch lỗi bị bỏ qua.<br>6b. Tác giả từ chối → patch bị loại, không tạo phiên bản. |
| **Exception Flow** | E1. Phần tử đích bị xóa trước khi xác nhận → patch chuyển `expired`.<br>E2. Giá trị hiện tại khác giá trị lúc đề xuất → patch hết hiệu lực, yêu cầu tạo lại.<br>E3. Ghi đồng thời với workflow → xếp hàng chờ khóa. |
| **Business Rules** | BR1: Mỗi thời điểm chỉ một patch chờ cho mỗi cặp (đích, trường).<br>BR2: Patch mới cùng đích và trường sẽ vô hiệu patch cũ.<br>BR3: Xóa phần tử là xóa vật lý trong bản nháp; lịch sử vẫn phục hồi được từ phiên bản. |
| **Priority** | High |
| **Ghi chú** | Liên kết với UC_VER_01 qua quan hệ `«include»`. |

### 3.3.6 UC_VER_01 — Quản lý phiên bản bản nháp

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_VER_01 |
| **Tên Use Case** | Xem và khôi phục phiên bản bản nháp |
| **Module/Phân hệ** | Draft Workspace |
| **Actor chính** | Tác giả |
| **Actor phụ** | Backend Rust, SQLite |
| **Mô tả ngắn** | Hệ thống niêm phong phiên bản bất biến theo lượt; tác giả có thể xem lịch sử và khôi phục. |
| **Trigger** | Kết thúc một lượt có thay đổi dữ liệu, hoặc tác giả mở hộp thoại lịch sử phiên bản. |
| **Precondition** | Có bản nháp; cơ sở dữ liệu ở trạng thái ghi được. |
| **Postcondition** | Phiên bản mới được ghi (khi niêm phong) hoặc dữ liệu, giai đoạn và bước phụ được khôi phục (khi restore). |
| **Main Flow** | 1. Cuối lượt có trường `applied`, hệ thống tạo khóa niêm phong duy nhất.<br>2. Ghi snapshot toàn bộ bản nháp kèm giai đoạn, bước phụ, băm dữ liệu.<br>3. Cập nhật con trỏ phiên bản đầu (head).<br>4. Tác giả mở danh sách phiên bản, xem nhãn thời gian và lý do.<br>5. Tác giả chọn một phiên bản để khôi phục.<br>6. Hệ thống xác nhận, khôi phục dữ liệu và đặt lại vị trí giai đoạn. |
| **Alternative Flow** | 1a. Lượt không có thay đổi hoặc dữ liệu trùng băm → không tạo phiên bản.<br>5a. Tác giả chỉ xem chi tiết mà không khôi phục. |
| **Exception Flow** | E1. Ghi phiên bản thất bại → thử lại một lần; vẫn lỗi thì trả lỗi niêm phong và không báo thành công.<br>E2. Phiên bản bị hỏng hoặc thiếu → từ chối khôi phục, giữ trạng thái hiện tại. |
| **Business Rules** | BR1: Mỗi lượt chỉ một phiên bản cho cùng khóa niêm phong.<br>BR2: Phiên bản là nguồn phục hồi duy nhất; trạng thái trong bộ nhớ không thay thế được.<br>BR3: Khôi phục làm mọi patch đang chờ chuyển sang `expired`. |
| **Priority** | High |
| **Ghi chú** | Không áp dụng cho nội dung chương; lịch sử cảnh dùng bảng phiên bản cảnh riêng. |

### 3.3.7 UC_COMMIT_01 — Cam kết dàn ý sang cơ sở dữ liệu

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_COMMIT_01 |
| **Tên Use Case** | Commit dàn ý sang cơ sở dữ liệu quan hệ |
| **Module/Phân hệ** | Draft Workspace → Cơ sở dữ liệu |
| **Actor chính** | Tác giả |
| **Actor phụ** | Commit Engine, SQLite |
| **Mô tả ngắn** | Chuyển toàn bộ bản nháp đã duyệt thành dữ liệu quan hệ bền vững, trong một giao dịch nguyên tử. |
| **Trigger** | Tác giả bấm "Bắt đầu viết" ở bước cuối và xác nhận trong hộp thoại. |
| **Precondition** | Sáu bước đã xác nhận; không còn patch chờ; toàn bộ bất biến vượt kiểm tra. |
| **Postcondition** | Dự án và các thực thể được tạo trong cơ sở dữ liệu; phiên bản nháp chuyển `committed`; người dùng được điều hướng sang Writing Mode. |
| **Main Flow** | 1. Hệ thống dựng kế hoạch ghi: đếm thực thể, kiểm tra trùng mã.<br>2. Hiển thị hộp thoại xác nhận kèm tóm tắt và trạng thái kiểm tra.<br>3. Tác giả xác nhận.<br>4. Mở giao dịch `BEGIN IMMEDIATE`.<br>5. Ghi dự án, Arc, chương, cảnh theo thứ tự phụ thuộc.<br>6. Ghi nhân vật, bối cảnh, tri thức, chủ đề, quan hệ.<br>7. Cập nhật trạng thái phiên bản nháp và phiên chat.<br>8. Commit giao dịch, trả kết quả. |
| **Alternative Flow** | 3a. Tác giả quay lại chỉnh dàn ý → đóng hộp thoại, giữ nguyên bản nháp. |
| **Exception Flow** | E1. Thiếu trường bắt buộc hoặc vi phạm bất biến → chặn commit, liệt kê lỗi.<br>E2. Còn patch chờ → chặn commit, hướng dẫn xử lý patch.<br>E3. Trùng mã giữa hai bản nháp đã commit trước → rollback, trả lỗi có cấu trúc.<br>E4. Lỗi ghi giữa chừng → rollback toàn bộ, bản nháp giữ nguyên. |
| **Business Rules** | BR1: Commit là nguyên tử: hoặc toàn bộ, hoặc không gì cả.<br>BR2: Tổng chương trong các Arc phải bằng mục tiêu; mỗi cảnh phải có địa điểm hợp lệ.<br>BR3: Mã thực thể sinh từ bộ đếm theo cha, không dùng chỉ số mảng. |
| **Priority** | High |
| **Ghi chú** | Phụ thuộc UC_VER_01 và UC_GATE_01. |

### 3.3.8 UC_WRITE_01 — Sáng tác chương

| Trường | Nội dung |
| --- | -------- |
| **Mã UC** | UC_WRITE_01 |
| **Tên Use Case** | Sáng tác chương và quản lý nội dung cảnh |
| **Module/Phân hệ** | Writing Mode |
| **Actor chính** | Tác giả |
| **Actor phụ** | AI Harness, Nhà cung cấp LLM, Backend Rust |
| **Mô tả ngắn** | Tác giả chọn chương, sinh nội dung từng cảnh theo khối, duyệt, sửa, khóa và lưu. |
| **Trigger** | Tác giả mở một chương ở chế độ sáng tác và bấm "Sinh nội dung". |
| **Precondition** | Dự án đã commit; chương và cảnh tồn tại; cảnh chưa bị khóa. |
| **Postcondition** | Nội dung cảnh được cập nhật hoặc lưu thành phiên bản; biên nhận lưu chương được ghi. |
| **Main Flow** | 1. Hiển thị chương dưới dạng các khối cảnh.<br>2. Tác giả chọn cảnh, xem cấu hình góc nhìn, phong cách, địa điểm.<br>3. Hệ thống ghép ngữ cảnh: tóm tắt chương, hồ sơ nhân vật, tri thức, cảnh trước.<br>4. Gọi mô hình, hiển thị nội dung vào vùng đệm.<br>5. Tác giả duyệt, sửa trực tiếp nếu cần.<br>6. Tác giả chấp nhận → cảnh chuyển trạng thái đã sửa, có thể khóa.<br>7. Tác giả lưu chương → ghi nội dung và biên nhận. |
| **Alternative Flow** | 4a. Sinh lại với hướng dẫn khác → tạo vùng đệm mới, giữ bản trước.<br>6a. Bỏ qua chấp nhận → quay lại nội dung đã lưu gần nhất. |
| **Exception Flow** | E1. Cảnh bị khóa → chặn sửa, hiển thị trạng thái khóa.<br>E2. Lỗi gọi mô hình → giữ vùng đệm, cho thử lại.<br>E3. Lưu thất bại → giữ trạng thái đã sửa, đánh dấu chưa lưu, cho thử lại. |
| **Business Rules** | BR1: AI không có tool đổi trạng thái khóa cảnh.<br>BR2: Chỉ người dùng được chấp nhận và lưu nội dung.<br>BR3: Nội dung chương đọc từ các cảnh theo thứ tự; không có bảng phiên bản chương. |
| **Priority** | Medium |
| **Ghi chú** | Thuộc giai đoạn sau commit; chưa triển khai đầy đủ backend tại thời điểm báo cáo. |

### 3.3.9 Bảng tổng hợp các use case còn lại

| Mã UC | Tên use case | Tác nhân chính | Ưu tiên | Trạng thái |
| --- | --- | --- | --- | --- |
| UC_CHAR_01 | Quản lý thư viện nhân vật | Tác giả | High | Giao diện hiện có, dữ liệu mẫu |
| UC_CHAR_02 | Tra cứu hồ sơ nhân vật theo dự án, giới tính | Tác giả | Medium | Giao diện hiện có |
| UC_WORLD_01 | Quản lý cây bối cảnh và tri thức | Tác giả | Medium | Đang xây dựng |
| UC_CFG_01 | Cấu hình System Prompt và danh mục | Tác giả | Medium | Giao diện đọc danh mục |
| UC_PROV_01 | Cấu hình nhà cung cấp và khóa API | Tác giả | High | Một phần (chọn mô hình trong chat) |
| UC_HIST_01 | Xem lịch sử phiên trò chuyện | Tác giả | Low | Giao diện hiện có |
| UC_EXP_01 | Xuất bản thảo ra định dạng ngoài | Tác giả | Low | Chưa triển khai |
