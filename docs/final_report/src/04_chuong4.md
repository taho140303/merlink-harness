# CHƯƠNG 4: ĐỀ XUẤT GIẢI PHÁP VÀ THIẾT KẾ HỆ THỐNG

## 4.1 TỔNG QUAN GIẢI PHÁP HỆ THỐNG

Giải pháp đề xuất gồm năm thành phần phối hợp theo một chiều duy nhất: giao diện **không bao giờ** đọc hoặc ghi dữ liệu nghiệp vụ trực tiếp, mà luôn gửi yêu cầu qua lớp IPC của Tauri tới backend Rust; backend là bên duy nhất thực thi tool, thay đổi trạng thái và ghi cơ sở dữ liệu.

1. **Lớp trình diễn (React 19 + React Router 8 + Tailwind CSS 4 + Zustand 5).** Chịu trách nhiệm hiển thị và thu nhận tương tác. Giao diện giữ trạng thái cục bộ cho các thành phần tạm thời (nội dung đang gõ, chế độ xem), nhưng mọi dữ liệu nghiệp vụ đều lấy từ backend.
2. **Lớp cầu nối IPC (Tauri v2).** Cung cấp các lệnh có kiểu (command) và kênh truyền sự kiện (channel) để giao diện nhận phản hồi theo luồng. Không có HTTP server nội bộ, giảm bề mặt tấn công và tránh xung đột cổng.
3. **Lớp ứng dụng (Rust).** Chứa toàn bộ logic: vòng lặp harness, thực thi tool, điều phối workflow, đo tiến độ, niêm phong phiên bản, commit, kiểm soát đồng thời, khởi động và migration.
4. **Lớp lưu trữ (SQLite + hệ thống tệp).** Cơ sở dữ liệu quan hệ cho dữ liệu nghiệp vụ và dữ liệu điều phối; hệ thống tệp cho báo cáo trung gian của workflow, ảnh bìa dự án và tệp xuất bản.
5. **Dịch vụ ngoài (nhà cung cấp LLM).** Giao tiếp HTTPS/SSE, cấu hình theo từng nhà cung cấp; khóa API không nằm trong cơ sở dữ liệu nghiệp vụ.

![Dòng chảy nghiệp vụ bảy giai đoạn và các điểm kiểm soát](images/fig_business_flow.png){width=100%}

Luồng end-to-end điển hình: tác giả gửi tin nhắn → giao diện gọi lệnh IPC → backend ghép prompt và gọi mô hình → mô hình trả về các tool call → backend kiểm tra và áp dụng vào trạng thái làm việc → cuối lượt niêm phong phiên bản → giao diện nhận sự kiện, cập nhật tiến độ và thẻ dữ liệu.

## 4.2 KIẾN TRÚC HỆ THỐNG ĐỀ XUẤT

![Kiến trúc tổng thể của Merlink Harness](images/fig_architecture.png){width=100%}

### 4.2.1 Kiến trúc modular monolith trên Tauri v2

Backend tổ chức thành các module Rust độc lập về trách nhiệm, nhưng biên dịch thành một tiến trình duy nhất — lựa chọn "modular monolith" thay vì microservice vì ứng dụng chạy trên máy cá nhân, không cần ranh giới mạng. Các module chính:

| Module | Trách nhiệm | Không được làm |
| --- | --- | --- |
| `harness` | Vòng lặp suy luận một lượt, ghép prompt, phân tách ngôn ngữ | Không truy cập cơ sở dữ liệu trực tiếp |
| `tools` | Đăng ký tool, kiểm tra quyền gọi, kiểm tra tham số, thực thi | Không tự quyết định nguồn dữ liệu thay AI |
| `workflow` | Chia kế hoạch thành lần chạy/vòng, điều phối subagent | Không giữ khóa khi chờ mạng |
| `draft` | Trạng thái làm việc, patch, phiên bản, bộ đếm mã | Không ghi trực tiếp khi chưa qua tool |
| `commit` | Ánh xạ bản nháp sang bảng quan hệ, kiểm tra bất biến | Không bỏ qua kiểm tra bất biến |
| `db` | Kết nối, migration, kiểm tra schema, giao dịch | Không chứa luật nghiệp vụ |
| `progress` | Đánh giá 46 thuộc tính, tính hai chỉ số | Không sửa dữ liệu bản nháp |
| `projection` | Đọc dữ liệu cho giao diện, phân trang, tìm kiếm | Không ghi dữ liệu |

**Chiều phụ thuộc một chiều:** giao diện → IPC → module ứng dụng → module lưu trữ. Không có phụ thuộc ngược từ tầng lưu trữ lên tầng ứng dụng. Nhờ vậy có thể kiểm thử từng module bằng dữ liệu giả, không cần chạy toàn bộ ứng dụng.

### 4.2.2 Mô hình đồng thời

Đây là phần phức tạp nhất của hệ thống vì phải phối hợp bốn nguồn thay đổi: lượt chat của AI, thao tác ghi của người dùng, vòng workflow chạy nền và thao tác commit. Hệ thống áp dụng ba nguyên tắc:

1. **Không giữ giao dịch hoặc khóa cơ sở dữ liệu trong lúc chờ mạng.** Mọi lời gọi mô hình diễn ra ngoài giao dịch; chỉ khi có kết quả mới mở giao dịch ngắn để ghi.
2. **Tuần tự hóa ghi theo bản nháp.** Mỗi bản nháp có một khóa ghi (mutex) theo `draft_session_id`. Chỉ một trong các bên — một lượt AI, một thao tác ghi của giao diện/CLI, một vòng workflow hoặc một lần commit — được ghi tại một thời điểm. Bên đến sau xếp hàng chờ; quá thời gian chờ (`DRAFT_LOCK_TIMEOUT`, mặc định 30 giây) thì nhận lỗi "bản nháp đang bận".
3. **Thứ tự áp dụng tất định.** Các subagent trong một vòng được phép tính toán và gọi mô hình song song, nhưng mọi thay đổi vào trạng thái dùng chung được áp dụng theo thứ tự chỉ mục kế hoạch, không theo thứ tự trả kết quả mạng. Cách làm này loại bỏ tranh chấp mà không cần khóa trong lúc chờ.

Hủy workflow cũng là dừng có kiểm soát: đánh dấu yêu cầu hủy, không khởi chạy tác vụ mới, không ngắt giữa một thao tác ghi, chờ thao tác đang chạy kết thúc, niêm phong phiên bản nếu vòng đã có thay đổi, rồi mới giải phóng khóa.

### 4.2.3 Lưu trữ và khôi phục

- **Cấu hình kết nối:** `foreign_keys = ON`, `journal_mode = WAL`, `synchronous` theo cấu hình, thời gian chờ khóa hợp lý.
- **Migration:** mỗi lần khởi động so sánh `PRAGMA user_version` với phiên bản mã nguồn; chạy tuần tự các bước migration chưa áp dụng; ghi lại phiên bản sau khi thành công.
- **Kiểm tra toàn vẹn:** xác minh sự tồn tại của bảng, cột, chỉ mục thiết yếu; nếu thiếu thì chặn khởi động và báo lỗi rõ ràng thay vì chạy với schema sai.
- **Phiên bản bất biến:** mỗi phiên bản là ảnh chụp toàn bộ bản nháp kèm giai đoạn, bước phụ và băm dữ liệu. Khóa niêm phong gồm mã phiên và mã lượt, kèm chỉ mục duy nhất để bảo đảm idempotent.
- **Phục hồi sau sự cố:** khi mở lại ứng dụng, backend đối chiếu nhật ký tin nhắn với danh sách phiên bản; lượt có bằng chứng ghi nhưng không có phiên bản được đánh dấu "gián đoạn" và thông báo cho người dùng, không tự ý dựng lại dữ liệu.

### 4.2.4 Bảo mật và phân quyền

- AI Harness chỉ tương tác qua tool; không có đường dẫn tệp, không có kết nối cơ sở dữ liệu.
- Mỗi tool có cổng kiểm tra người gọi (AI hay giao diện/CLI) và danh sách tham số cho phép.
- Khóa API và cấu hình nhà cung cấp lưu ngoài cơ sở dữ liệu nghiệp vụ, không ghi vào tệp log.
- Nhật ký thực thi tool ghi lại tên tool, tham số đã lọc, kết quả và mã lỗi; không ghi nội dung nhạy cảm.

## 4.3 THIẾT KẾ GIAO DIỆN

Giao diện được xây dựng theo một design system riêng với hai tầng bề mặt: **tầng vỏ (shell)** dùng hiệu ứng neumorphic nhẹ cho thanh điều hướng, thanh tiêu đề và các vùng chứa; **tầng nội dung** dùng phẳng, viền mảnh để bảo đảm độ tương phản cho văn bản dài. Hệ thống dùng chung bộ token về màu, khoảng cách, bo góc, đổ bóng; typography hỗ trợ tiếng Việt và tiếng Anh với cùng hệ đo; mọi thành phần tương tác có trạng thái focus rõ ràng và nhãn trợ năng.

Sáu màn hình dưới đây được chụp từ bản chạy web preview với dữ liệu mẫu của dự án. Ở chế độ preview, giao diện hiển thị nhãn "Backend Unavailable" và dải thông báo không lưu dữ liệu — đây là hành vi đúng thiết kế khi backend Rust chưa chạy, không phải lỗi của giao diện.

![Màn hình Chat Workspace với thanh điều hướng, vùng hội thoại và ô nhập liệu](images/fig_chat.png){width=100%}

**Chat Workspace (Hình 4).** Bố cục hai vùng: thanh điều hướng bên trái và vùng hội thoại chiếm phần còn lại. Cụm tiêu đề hiển thị tên dự án, thanh tiến độ hai chỉ số và nút "Xây dựng bản nháp". Ô nhập liệu tích hợp nút đính kèm, chọn nguồn ngữ cảnh, chọn mô hình và chỉ báo mức sử dụng ngữ cảnh. Đây là bề mặt chính của giai đoạn thu thập yêu cầu.

![Thư viện dự án với thẻ dự án, ảnh bìa, trạng thái và thống kê](images/fig_project.png){width=100%}

**Thư viện dự án (Hình 5).** Mỗi dự án là một thẻ gồm ảnh bìa, tên, nhãn trạng thái, mô tả ngắn và liên kết xem nhanh. Thanh công cụ phía trên hỗ trợ tìm kiếm và sắp xếp; thẻ có thao tác xóa với xác nhận. Cách trình bày này giúp người dùng nắm nhanh tiến độ nhiều tác phẩm song song.

![Trang chi tiết dự án với thống kê và danh sách Arc, chương](images/fig_project_detail.png){width=100%}

**Chi tiết dự án (Hình 6).** Phần đầu là thông tin dự án và bốn chỉ số: số chữ, số chương, số nhân vật, ngày sửa. Phần dưới nhóm chương theo Arc, mỗi Arc có ảnh đại diện riêng và danh sách chương kèm ngày. Đây là nơi người dùng chuyển giữa chế độ Xem và chế độ Sáng tác.

![Trình soạn chương với các khối nội dung và thanh điều hướng chương](images/fig_chapter.png){width=100%}

**Trình soạn chương (Hình 7).** Nội dung chương hiển thị theo khối, mỗi khối là một đơn vị cảnh có thể chỉnh sửa, khóa hoặc sinh lại. Thanh bên phải cho phép nhảy nhanh giữa các khối; nút "Chỉnh sửa" chuyển sang chế độ soạn thảo. Đây là bề mặt của Writing Mode.

![Thư viện nhân vật với bộ lọc và thẻ hồ sơ nhân vật](images/fig_characters.png){width=100%}

**Thư viện nhân vật (Hình 8).** Mỗi thẻ nhân vật hiển thị chữ cái đại diện, tên, biệt danh, chủng tộc, dự án trực thuộc, giới tính sinh học và giới tính thể hiện, cùng số mối quan hệ. Bộ lọc theo dự án và giới tính giúp tra cứu nhanh khi dàn nhân vật lớn.

![Khu vực cấu hình System Prompt với các tab danh mục Motif, Style Preset, Narrative Directive, Theme, Thể loại](images/fig_config.png){width=100%}

**Khu vực System Prompt (Hình 9).** Giao diện đọc danh mục nền gồm năm tab: Motif, Style Preset, Narrative Directive, Theme và Thể loại. Mỗi thẻ danh mục cho biết mã, phiên bản, mô tả, nguồn gốc (hệ thống hay tùy chỉnh) và các thể loại áp dụng. Đây là nơi kiểm tra tính nhất quán của bộ dữ liệu seed trước khi dùng cho sáng tác.

## 4.4 THIẾT KẾ DỮ LIỆU VÀ TÍCH HỢP

### 4.4.1 Mô hình dữ liệu quan hệ

![Bản đồ nhóm bảng dữ liệu và quan hệ chính](images/fig_erd.png){width=100%}

Schema gồm 66 bảng (Hình 10), chia thành năm nhóm theo vòng đời dữ liệu:

| Nhóm | Bảng tiêu biểu | Vai trò |
| --- | --- | --- |
| Bản nháp và workflow | `draft_sessions`, `draft_versions`, `pending_patches`, `workflow_task_receipts` | Dữ liệu tồn tại trước commit: trạng thái làm việc, phiên bản, patch, biên nhận |
| Nội dung tác phẩm | `projects`, `arcs`, `chapters`, `scenes`, `scene_versions`, `chapter_save_receipts` | Dữ liệu sau commit và nội dung chương đã viết |
| Nhân vật và quan hệ | `characters`, `project_characters`, `character_appearances`, `character_psychological`, `character_memories`, `character_competences`, `relationships`, `relationship_states` | Hồ sơ nhân vật nhiều tầng và trạng thái quan hệ theo cảnh |
| Thế giới và chủ đề | `world_settings`, `lore_entries`, `themes`, `theme_arguments` | Cây bối cảnh, tri thức theo loại, chủ đề và luận điểm |
| Danh mục seed | `motifs`, `genres`, `style_presets`, `narrator_configs`, `narrative_directives`, `competence_domains`, `relationship_categories`, `appearance_options` | Dữ liệu tra cứu, không sinh từ bản nháp |

Ba đặc điểm thiết kế đáng chú ý:

**Thứ nhất — tách bạch vòng đời.** Nhóm bản nháp và nhóm nội dung có vòng đời khác nhau: bản nháp chỉ tồn tại trước commit và được dọn sau khi hoàn tất; dữ liệu nội dung tồn tại lâu dài. Việc tách bảng giúp tránh nhầm lẫn giữa "trạng thái đang duyệt" và "dữ liệu đã chốt".

**Thứ hai — lưu trạng thái quan hệ theo cảnh ở dạng thưa.** Bảng `relationship_states` chỉ ghi tại cảnh có thay đổi hoặc mốc then chốt, thay vì lặp giá trị ở mọi cảnh. Các chiều trạng thái dùng enum bậc (`NONE`, `LOW`, `MODERATE`, `HIGH`, `ABSOLUTE`) thay cho thang số, tránh tranh luận về giá trị trung gian và dễ kiểm tra hơn.

**Thứ ba — dữ liệu bán cấu trúc có kiểm soát.** `lore_entries.data` là JSON nhưng phải tuân theo hình dạng tương ứng với `type` (văn hóa, lịch sử, tổ chức, vật phẩm). Backend kiểm tra cấu trúc và giữ nguyên các khóa chưa biết khi ghi lại, tránh mất dữ liệu khi mở rộng.

### 4.4.2 Ánh xạ từ bản nháp sang cơ sở dữ liệu

Điểm khó của thiết kế là hai tầng định danh: bản nháp dùng **mã slug** (`code`) ổn định trong khi người dùng đang chỉnh sửa; cơ sở dữ liệu dùng **khóa UUIDv7** sinh khi commit. Bảng ánh xạ dưới đây tóm tắt quan hệ giữa hai tầng:

| Khối dữ liệu bản nháp | Bảng đích | Ghi chú chuyển đổi |
| --- | --- | --- |
| `require.project` | `projects`, `project_genres` | Thể loại chính/phụ; mục tiêu số chữ là tham số runtime, không lưu thành cột |
| `require.character` | `characters`, `project_characters`, `character_motifs` | Quan hệ nhiều-nhiều với dự án qua bảng nối |
| `overview_phase.arcs` | `arcs`, `arc_motifs`, `arc_event_blocks`, `themes`, `theme_arguments` | Arc, motif, khối biến cố, chủ đề |
| `chapter_phase` | `chapters`, `scenes` | Thứ tự chương duy nhất toàn truyện, backend kiểm tra |
| `world_building_phase` | `world_settings`, `lore_entries` | Cây phân cấp; tri thức theo loại |
| `scene_config_phase` | `scenes`, `scene_characters`, `scene_lore_entries` | Góc nhìn, phong cách, địa điểm, liên kết tri thức |
| `character_appearance_phase` | `character_appearances`, `character_distinguishing_marks` | Quan hệ một-một với nhân vật |
| `character_psychological_phase` | `character_psychological` | Mười lăm cột văn bản, không dùng blob JSON |
| `character_life_experience_phase` | `character_memories`, `character_competences`, `competence_capabilities`, `competence_limitations` | Năng lực bắt buộc gắn cảnh; ký ức được phép không có cảnh nguồn |
| `relationship_phase` | `relationships`, `relationship_states`, `relationship_roles` | Trạng thái thưa theo cảnh, enum bậc |

Quy tắc chuyển đổi cốt lõi: handler cấp `code` cho thực thể mới và trả trong kết quả; khi commit, backend dựng bảng ánh xạ `code → id` trong bộ nhớ, kiểm tra trùng mã trong giao dịch, sinh UUIDv7 cho từng thực thể và ghi theo thứ tự phụ thuộc (dự án → Arc → chương → cảnh → nhân vật → bối cảnh → quan hệ). Nếu phát hiện trùng mã giữa hai bản nháp đã commit, toàn bộ giao dịch rollback và trả lỗi có cấu trúc; bản nháp được giữ nguyên để người dùng xử lý.

### 4.4.3 Hợp đồng tích hợp

Hệ thống tích hợp với ba bên: giao diện, nhà cung cấp LLM và hệ thống tệp. Mỗi tích hợp có hợp đồng rõ ràng:

| Kênh tích hợp | Hình thức | Hợp đồng chính |
| --- | --- | --- |
| Giao diện ↔ Backend | Lệnh IPC có kiểu, kênh sự kiện | Tham số theo DTO; lỗi có mã; luồng sự kiện cho sinh nội dung |
| Backend ↔ Nhà cung cấp LLM | HTTPS, phản hồi dạng luồng | Cấu hình theo nhà cung cấp; ánh xạ tool call; giới hạn token; thời gian chờ |
| AI ↔ Tool | Gọi hàm có schema | Bắt buộc khai báo nguồn dữ liệu; kết quả từng trường; lỗi phân loại |
| Backend ↔ Hệ thống tệp | API hệ thống | Báo cáo trung gian theo mã phiên; xóa khi hoàn tất; ảnh bìa theo dự án |

Nguyên tắc "everything is a tool call" được thể hiện ngay trong hợp đồng tích hợp: mười hai lệnh IPC chính đều là các thao tác nghiệp vụ có kiểm tra quyền, không có lệnh nào cho phép giao diện ghi trực tiếp tệp JSON. Cách làm này bảo đảm dù giao diện có lỗi, dữ liệu vẫn không thể hỏng ngoài các luật nghiệp vụ.

### 4.4.4 Dữ liệu danh mục ban đầu

Hệ thống đi kèm bộ dữ liệu seed đã được chuẩn hóa, nạp một lần khi khởi tạo cơ sở dữ liệu:

| Bộ dữ liệu | Số lượng | Nội dung |
| --- | ---: | --- |
| Thể loại (`genres`) | 50 | Thể loại, bối cảnh, sắc thái, nhóm độc giả, mỗi nhãn có mô tả xác định phạm vi |
| Motif (`motifs`) | 19 | Mẫu cốt truyện kèm trạng thái, thành phần, khối biến cố và ràng buộc |
| Chủ đề (`themes`) | 85 | 12 chủ đề mẹ và 73 chủ đề gắn tác phẩm, kèm lập trường nhân vật mẫu |
| Cấu hình người kể | Theo ngôi kể | Ngôi thứ nhất, ngôi thứ ba giới hạn, ngôi thứ ba toàn tri; quy tắc góc nhìn |
| Chỉ dẫn tự sự | Theo phong cách | Chỉ dẫn viết, phong cách diễn đạt, từ vựng và trọng tâm |

Bộ dữ liệu này được xây dựng từ nghiên cứu thị trường và lý luận văn học (xem Tài liệu tham khảo, nhóm D), sau đó chắt lọc để bảo đảm mỗi nhãn có phạm vi rõ ràng, không chồng lấn và có thể tra cứu bằng mã ổn định. Cách tổ chức danh mục theo mã giúp AI có thể tham chiếu nhất quán qua nhiều dự án mà không phụ thuộc tên hiển thị.
