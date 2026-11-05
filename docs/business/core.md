# NGHIỆP VỤ CỐT LÕI: HỆ THỐNG PHÂN KỲ SÁNG TÁC LŨY TIẾN (PROGRESSIVE MULTI-PHASE BUSINESS FLOW)

> **Tài liệu này là SSOT (Single Source of Truth) của toàn bộ quy tắc nghiệp vụ Merlink Harness. Khi `tools.md`, `overview.md`, `case_newchat.md` hoặc bất kỳ tài liệu nào khác mâu thuẫn với core về nghiệp vụ, core thắng. Schema vật lý của bảng vẫn theo `docs/db.dbml`.**
> **Tài liệu tham chiếu chuẩn cho kiến trúc nghiệp vụ và lập kế hoạch kỹ thuật (Planning)**
> **Phiên bản:** 2.4
> **Phạm vi áp dụng:** Luồng Chat thu thập, Trình tính toán tiến độ (Progress Engine), Trình dựng dàn ý (Draft Building), và Mô hình dữ liệu quan hệ (Domain Database).
> **Danh mục tool:** [harness/tools.md](harness/tools.md).

---

## 1. TỔNG QUAN & TRIẾT LÝ NGHIỆP VỤ

### 1.1 Mục tiêu hệ thống

Merlink Harness được thiết kế theo phương pháp **Sáng tác có cấu trúc từ trên xuống (Top-down Architect Method)**. Hệ thống giải quyết bài toán lớn nhất của việc sáng tác truyện dài với AI: **sự trôi ngữ cảnh (context drift), mâu thuẫn nhân vật và mất phương hướng cốt truyện**.

Thay vì cho phép AI sinh thẳng văn bản thô từ vài câu lệnh mông lung, hệ thống áp dụng cơ chế:

1. **Thu thập yêu cầu có cấu trúc qua hội thoại tự nhiên (Conversational Elicitation)**.
2. **Kiểm duyệt điều kiện mở khóa (Readiness Gating)**.
3. **Dựng và kiểm duyệt dàn ý lũy tiến theo từng giai đoạn (Progressive Multi-phase Disclosure)**.
4. **Cam kết dữ liệu vào cơ sở dữ liệu quan hệ (Relational Persistence):** Khả năng đảo ngược nằm ở tầng bản nháp/phiên bản trước khi commit. Sau commit, chỉ cho phép mở lại bản nháp khi chưa có nội dung cảnh nào được viết.

### 1.2 Nguyên tắc thiết kế cốt lõi

* **Tách bạch State Hội thoại và State Nghiệp vụ:** Lịch sử chat (`ChatMessage[]`) là nhật ký trao đổi để người dùng và AI tương tác; bản nháp cấu trúc (`BusinessFlowData`) là một tài liệu JSON sống được cập nhật liên tục qua các tool call (xem mục 1.4); AI không ghi trực tiếp vào JSON.
* **Mọi trường đều Nullable trong quá trình thu thập:** Khi người dùng chưa nhắc tới một yếu tố, trường đó giữ giá trị `null` hoặc mảng rỗng. AI không tự bịa quyết định định lượng/định danh như số Arc, số chương, tên nhân vật hoặc thể loại; nội dung này tuân theo Mức 1 tại mục 1.3. Quy tắc này không cấm AI sinh nội dung phái sinh ở phase hiện tại theo Mức 2.
* **Gating trước khi Dựng (Readiness Gate):** Chỉ cho phép kích hoạt quy trình sinh dàn ý chuyên sâu (`Draft Building`) khi tập thuộc tính cốt lõi (`require`) đã được lấp đầy hoàn chỉnh.
* **Mở khóa dữ liệu lũy tiến (Progressive Disclosure):** Quy trình dựng dàn ý chia làm 6 bước kế tiếp nhau. Mỗi bước chỉ tập trung vào một tầng dữ liệu nhất định (Vĩ mô ➔ Trung mô ➔ Vi mô). Dữ liệu tầng sau chỉ được mở khóa khi người dùng duyệt tầng trước.

### 1.3 Quy tắc thu thập & sinh dữ liệu

* **Thu thập không phụ thuộc phase/gating:** Mọi thông tin người dùng nêu ra được bóc tách ngay trong lượt đó, bất kể thuộc `require` hay phase `optional`, phase hiện tại, hoặc `isRequireComplete`. Gating chỉ điều khiển việc kích hoạt nút Draft Building, không điều khiển việc ghi nhận dữ liệu.
* Nếu người dùng gửi lộ trình chi tiết ngay đầu chat, ghi nhận toàn bộ thông tin vào `businessFlowData`; `require` tự đầy khi có đủ dữ liệu. Các phase sau mở ra với phần đã có, AI chỉ bổ sung phần còn thiếu theo quy tắc ghi đè ở mục này.
* Dữ liệu người dùng cung cấp cho phase chưa mở vẫn nằm trong JSON và được tính vào progress dù UI chưa hiển thị.
* Thông tin không có trường đích trong schema được ghi vào `user_constraints` (mục 3.8).
* Nếu thông tin mới mâu thuẫn với ràng buộc hiện có (ví dụ liệt kê 7 Arc nhưng `arc_count = 5`), AI không tự sửa bên nào; ghi nhận cả hai giá trị, báo độ lệch và hỏi người dùng. Nếu liên quan trường được bảo vệ, giá trị mới được giữ trong `pending_patches` chứ không ghi đè State. Người dùng tự kiểm tra các giá trị/ràng buộc liên quan qua icon (i) trên thanh tiến độ.

#### Quy tắc ghi đè theo nguồn gốc dữ liệu

| Nguồn hiện có của trường / phần tử | AI chat / regenerate (ghi đè hoặc xóa) được làm gì |
| :--- | :--- |
| `ai_generated` | AI được ghi đè tự do; chỉ xóa tự do khi không có hậu duệ `user`/`user_edited`. Nếu có hậu duệ được bảo vệ, trả `pending_confirmation` dù cha là `ai_generated`; các thay đổi được gộp vào version chốt cuối lượt (mục 3.6) với `reason = ai_patch` hoặc `regenerate` tương ứng. |
| `user` hoặc `user_edited` | AI không được ghi đè hoặc xóa trực tiếp. AI tạo patch có scope theo `code` ổn định; người dùng xem trước và xác nhận thì patch mới được áp dụng. |

* Nếu người dùng cung cấp giá trị mới cho trường hiện có `source = ai_generated`, có thể ghi trực tiếp theo hàng đầu tiên của bảng; giá trị mới nhận `source = user_edited` vì người dùng đã sửa nội dung AI sinh.
* Quy tắc bảo vệ `user`/`user_edited` áp dụng cho cả **ghi đè** lẫn **xóa phần tử** (nhân vật, chương, cảnh, arc, lore...) trên mọi đường thực thi, kể cả khi người dùng nói trong chat; không có ngoại lệ tự áp dụng chỉ vì câu lệnh rõ ràng.
* Thông tin chat mới về trường `user`/`user_edited` được bóc tách và ghi ngay vào `businessFlowData.pending_patches`; giá trị đang có trong State chưa đổi cho đến khi người dùng duyệt.
* Patch hỗn hợp được tách theo từng trường: trường chạm `ai_generated` áp dụng ngay; mỗi trường chạm `user`/`user_edited` tạo đúng một patch `pending_confirmation` và chưa tạo version. Khi người dùng xác nhận, áp dụng patch và trường được xác nhận chuyển thành `user_edited`; mỗi thao tác xác nhận tạo đúng một version `ai_patch` nếu có ít nhất một patch được áp dụng. Khi người dùng từ chối, bỏ patch và ghi sự kiện từ chối vào event log, không tạo version.
* Patch hết hiệu lực khi `target_code` không còn tồn tại hoặc giá trị hiện tại của `field_name` khác `previous_value`; với `operation = delete`, patch hết hiệu lực khi phần tử đích không còn hoặc tập hậu duệ bị ảnh hưởng đã khác lúc đề xuất. Kiểm tra lười tại thời điểm hiển thị preview và lúc confirm; `base_version_seq` chỉ dùng cho event log. Restore làm mọi patch `pending` thành `expired` theo quy tắc hiện có. Khi target đã bị xóa hoặc đổi cấu trúc, patch chuyển `expired` (không phải `rejected`); UI hiển thị "patch hết hiệu lực"; AI chỉ tạo preview mới khi người dùng nhắc lại ý định đó.
* Tại một thời điểm chỉ tồn tại tối đa một patch `pending` cho mỗi `(draft_session_id, target_code, field_name)`. Khi có patch mới cùng đích và `field_name` với `new_value` khác, patch cũ chuyển `expired` và patch mới được tạo trong cùng một bước dưới khóa ghi draft; handler cưỡng chế, không thêm ràng buộc unique.
* Khi có nhiều patch chờ duyệt, UI hỗ trợ xác nhận từng patch và nút **Áp dụng tất cả**. Mỗi thao tác chạy dưới khóa ghi draft; **Áp dụng tất cả** duyệt patch `pending` theo `created_at` tăng dần, revalidate từng patch, bỏ qua patch lỗi (không rollback cả batch), trả danh sách `applied` và `skipped`, và chỉ tạo một version `ai_patch` khi có ít nhất một patch được áp dụng.
* Khi xóa cha, kiểm tra toàn bộ hậu duệ bị ảnh hưởng theo ma trận Cascade Delete bên dưới. Chỉ một hậu duệ có `source = user` hoặc `user_edited` cũng khiến tool trả `pending_confirmation`. Các quan hệ chưa có trong ma trận, ví dụ xóa Character đang được relationship hoặc theme argument tham chiếu, giữ ở Q1 và không tự cascade.
* Preview patch xóa liệt kê toàn bộ phần tử bị ảnh hưởng. Không lưu danh sách này kèm patch; tính lại từ State tại thời điểm hiển thị. Patch `pending` được kiểm tra lại theo điều kiện hết hiệu lực ở trên; nếu không còn hợp lệ, chuyển `expired` và tạo preview mới.
* Regenerate phase chỉ thay các trường `ai_generated`, giữ nguyên `user` và `user_edited`.
* Phase đã Confirm không bị khóa; chat và sửa tay vẫn cập nhật JSON theo các quy tắc nguồn dữ liệu trên.

#### Ba mức nội dung AI được sinh

| Mức | Ví dụ | Quy tắc |
| :--- | :--- | :--- |
| Quyết định định lượng/định danh | Số Arc, số chương, tên nhân vật, thể loại | AI không tự bịa. Nếu thiếu thì hỏi; hoặc nêu đề xuất kèm lý do và chờ người dùng chấp thuận trước khi ghi như quyết định. |
| Nội dung sáng tác phái sinh từ dữ liệu có sẵn | Tóm tắt chương/cảnh, tên Arc, lore, ngoại hình, tâm lý, ký ức | AI được sinh khi người dùng đến phase tương ứng; trường do AI tạo mang `source = ai_generated`. |
| Dữ liệu người dùng đã cung cấp | Bất kỳ thông tin người dùng đã nói | Ghi `source = user`; không ghi đè trực tiếp khi đã tồn tại, áp dụng cơ chế patch và xác nhận nêu trên. |

#### Trường `source`

* Mỗi trường/thực thể trong draft mang `source`: `user` | `ai_generated` | `user_edited`.
* Mọi tool ghi nhiều trường (`add_`, `set_`) có tham số bắt buộc `declared_sources`, là object ánh xạ tên tham số giá trị sang `user` hoặc `ai_generated`; tham số định danh (`code`, `*_code` dùng để chỉ đích) không cần khai báo nguồn. Handler kiểm tra từng khóa, gán `source` theo từng trường; trường có giá trị mà thiếu khóa nguồn hoặc nguồn sai chỉ trường đó bị `rejected`; handler không tự suy đoán nguồn. Tool xóa (`delete_*`) và tool liên kết một payload nguyên tử giữ tham số scalar `declared_source`; danh sách tool theo từng loại nằm ở [harness/tools.md](harness/tools.md) mục 1. Tool không ghi vào State nháp không có cả hai tham số. Không thêm cột DBML cho các tham số này.
* Chuyển trạng thái: người dùng nhập → `user`; AI sinh → `ai_generated`; người dùng sửa phần AI sinh hoặc xác nhận patch ghi đè → `user_edited`.
* `source` phục vụ bảo vệ dữ liệu người dùng khi AI ghi đè/regenerate, phân biệt phần AI đề xuất trên UI, và giúp event log/progress biết phần nào đã được xác nhận.
* `source` được mô tả ở cấp draft JSON; việc ánh xạ xuống các bảng quan hệ khi commit không bắt buộc (xem mục 3.7).

#### Cascade Delete trong Working State

* **Không nhầm với `ON DELETE CASCADE` của database.** Mục này mô tả cách handler xóa dữ liệu trong draft JSON và các liên kết nháp; DBML/foreign key của database không thay đổi theo mục này.
* Phân biệt hai loại quan hệ:
  * **Quan hệ sở hữu:** thực thể con thuộc vòng đời thực thể cha; xóa cha có thể xóa con theo cascade.
  * **Quan hệ tham chiếu:** thực thể chỉ dùng hoặc tham chiếu thực thể khác; xóa thực thể được tham chiếu không đồng nghĩa xóa thực thể tham chiếu.
* Ma trận đã chốt cho Working State:

| Thực thể bị xóa | Dữ liệu liên quan | Hành vi |
| :--- | :--- | :--- |
| Arc | Chapters thuộc Arc | Cascade |
| Chapter | Scenes thuộc Chapter | Cascade |
| Scene | `scene_characters` | Cascade xóa liên kết |
| Scene | `scene_lore_entries` | Cascade xóa liên kết |
| Scene | `character_competences` có `scene_id` trỏ đến scene | Cascade xóa bản ghi năng lực |
| Scene | `character_memories` có `source_scene_id` trỏ đến scene | Giữ bản ghi, đặt `source_scene_id = null` |
| Scene | Characters được tham chiếu | Giữ nguyên |
| Scene | Lore entries được tham chiếu | Giữ nguyên |
| Scene | World settings được tham chiếu | Giữ nguyên |

* Cascade có tính bắc cầu. Xóa Arc xóa Chapters và Scenes thuộc Arc, sau đó áp dụng các quy tắc phụ thuộc của Scene.
* Trước khi cascade, handler phải xác định toàn bộ dữ liệu bị ảnh hưởng. Nếu bất kỳ dữ liệu bị xóa hoặc trường bị thay đổi nào có `source = user` hoặc `source = user_edited`, AI không được tự thực hiện thay đổi; tool trả `pending_confirmation`. Quy tắc áp dụng cả khi thực thể cha có `source = ai_generated`. Ví dụ Scene do AI tạo nhưng chứa bản ghi năng lực do người dùng cung cấp: không tự xóa năng lực, không tự xóa Scene; trả `pending_confirmation` và preview liệt kê thực thể/trường bị ảnh hưởng. Việc đặt `character_memories.source_scene_id = null` cũng là mutation phải kiểm tra `source`.
* Một cascade được chấp thuận là một thao tác nghiệp vụ nguyên tử: revalidate trước khi áp dụng; nếu điều kiện không còn hợp lệ, patch chuyển `expired`; khi xác nhận và có thay đổi thực sự, tạo đúng một version `ai_patch` theo mục 3.6; không tạo version riêng cho từng hậu duệ; không bỏ qua quy tắc xác nhận patch.
* Không suy rộng ma trận này thành quyền cascade mọi foreign key. Các quan hệ chưa có quy tắc xử lý rõ ràng giữ trong CÂU HỎI MỞ Q1; không tự xóa dây chuyền, không tự đặt null nếu schema không cho phép, không tự xóa thực thể dùng chung.

### 1.4 Triết lý Harness: CLI-first, Tool-call-only

1. **Everything is a tool call.** Mọi hành động của AI lên dữ liệu (đọc State, tìm kiếm nhân vật, hỏi trường null, thêm/ghi đè/xóa giá trị) đều đi qua tool call. Tool là giao diện duy nhất giữa AI và hệ thống.
2. **Harness không có quyền đọc/ghi trực tiếp.** Harness (vòng lặp agent) không truy cập trực tiếp file, DB hay JSON; chỉ gọi tool và sinh state. Backend (Tauri/Rust) là bên duy nhất thực thi tool và thay đổi dữ liệu.
3. **AI không tự viết tài liệu JSON hay script.** Mọi thay đổi thực hiện bằng từng tool call để **thêm** giá trị mới, **ghi đè** giá trị của một trường cụ thể, hoặc **xóa** phần tử, định danh bằng slug `code` ổn định (không dùng index mảng).

   **Tầng định danh:** Bản nháp (draft JSON, chat, tool call, patch, `feedbackContext`) chỉ dùng `code` — slug định danh ổn định; với khối cố định không phải thực thể (ví dụ `require.project`), `code` là địa chỉ ổn định theo path. Tầng cơ sở dữ liệu dùng `id` UUIDv7 cho thực thể nghiệp vụ, sinh khi commit; backend giữ ánh xạ `code → id` trong bộ nhớ theo mục 3.7. Không đưa `id` DB vào tham chiếu thực thể trong draft; các khóa kỹ thuật như `patch_id`, `draft_session_id` và tên cột DB mang `id` giữ nguyên.
4. **CLI-first.** Mỗi tool như một lệnh có tham số có kiểu, gọi độc lập; UI và AI là hai client của cùng một bề mặt tool.
5. **Tool theo thực thể.** Mỗi thực thể có bộ tool riêng với tham số có kiểu, dùng slug `code` ổn định để định danh đích (không dùng index mảng hoặc một tool tổng quát theo path). Quy ước đặt tên `<động từ>_<thực thể>` với ba động từ tác động: `add_` (thêm), `set_` (ghi đè giá trị trường) và `delete_` (xóa phần tử; bảo vệ `source` cả cha lẫn hậu duệ và trả `pending_confirmation` nếu có phần tử bị ảnh hưởng thuộc `user`/`user_edited`; preview tính lại từ State theo mục 1.3). Thực thể gồm: project, character (kèm các hồ sơ ngoại hình, tâm lý, trải nghiệm sống, năng lực), arc, chapter, scene, world_setting, lore_entry, relationship (đối chiếu các phase tại mục 2).

Danh mục tool đầy đủ, nhóm ưu tiên và tham số theo DBML nằm ở [harness/tools.md](harness/tools.md). Từ nay `core.md` không liệt kê danh sách tool, chỉ dẫn link tới tài liệu này; quy ước đặt tên `<động từ>_<thực thể>` ở trên vẫn giữ nguyên.

6. **Luật nghiệp vụ nằm ở tool handler, không nằm ở prompt.** Tool handler ở backend chịu trách nhiệm: kiểm tra kiểu/enum, bảo vệ ghi đè/xóa theo `source` (mục 1.3), kiểm tra bất biến (mục 2.8), gán `source` theo `declared_sources` hoặc `declared_source` tùy loại tool, ghi vào State làm việc. Prompt chỉ hướng dẫn cách dùng tool, không phải hàng rào an toàn.
7. **Phản hồi của tool là tín hiệu có cấu trúc:** Tool `add_`/`set_` được phép cập nhật nhiều trường trong một call; xử lý và báo cáo theo từng trường. Tool ghi nhiều trường trả `{"status": <tổng>, "results": [{"field": "...", "status": "applied" | "pending_confirmation" | "rejected", "patch_id"?: "...", "error"?: "..."}]}`. `status` tổng là `applied` nếu có ít nhất một trường `applied`; nếu không, là `pending_confirmation` nếu có ít nhất một trường `pending_confirmation`; nếu không, là `rejected`. Tool ghi một trường, tool xóa và tool liên kết giữ khung cũ và có thể bỏ `results`. `field` trong `results[]` và `field_name` của patch là tên tham số phẳng của tool (ví dụ `hair_color_code`, `the_wound`, `target.chapter_count`); không dùng path lồng nhau, phần tử mảng được định danh bằng `target_code` và thêm/xóa bằng tool `add_`/`delete_` riêng. Các trường trong một call không nguyên tử với nhau: trường `rejected` không chặn các trường hợp lệ khác; bất biến cấp thực thể được validate trên kết quả gộp sau khi áp dụng các trường, vi phạm thì các trường liên quan bị `rejected` và không đổi State. `pending_confirmation` kèm `patch_id` khi ghi đè hoặc xóa chạm trường/phần tử `user`/`user_edited`, kể cả hậu duệ được bảo vệ của cha `ai_generated`; preview liệt kê toàn bộ phần tử bị ảnh hưởng và được tính lại từ State, không lưu danh sách trong patch. `rejected` kèm mã lỗi và lý do: sai kiểu, sai enum, `code` không tồn tại, vi phạm bất biến. `has_applied_write` bằng true khi có ít nhất một `results[i].status = applied`, hoặc `status` tổng là `applied` với tool không có `results`. Chỉ tool **ghi** (`add_`, `set_`, `delete_`) trả `applied` mới góp vào version; tool đọc dù trả `applied` cũng không tạo version. Khi một trường nhận `pending_confirmation`, AI không được gọi lại để lách; AI báo người dùng đúng trường đang chờ xác nhận.
8. **Tool đọc:** `get_field_null()` và `get_array_element_fields` (mục 3.1), `search_character` (mục 3.8) và tool lấy State/phần State theo thực thể thuộc nhóm tool đọc; xem [harness/tools.md](harness/tools.md). Kết quả đọc thành công có thể mang `status = applied` nhưng không thay State và không tạo version.
9. **Cấp phát tool theo flow:** Kho tool trung tâm (`tools`) không tự nạp vào context; tool chỉ được cấp qua `prompt_block_tools` khi prompt block tương ứng được nạp. Ứng dụng có nhiều flow chat, mỗi flow cần tập tool riêng; việc ghép prompt block và tập tool cho từng flow được tổ chức thành các **pipeline function ở tầng service**. Bảy flow chính (chat thu thập Phase 0 và sáu phase dựng dàn ý) đều được cấp đủ bộ tool theo thực thể (đọc, thêm, ghi đè, xóa), nên các bản ghi cấp tool trong `prompt_block_tools` của bảy flow này gần như giống nhau. Thông tin người dùng nêu ở bất kỳ flow chính nào đều được ghi nhận vào đúng trường theo mục 1.3, không phụ thuộc phase hiện tại. Các flow chính khác nhau ở nội dung prompt block hướng dẫn theo phase, không ở tập tool.

### 1.5 Hai chế độ Planning và Writing

Merlink Harness có hai luồng độc lập; không đồng nhất workflow phân đoạn của Planning với workflow sáng tác nội dung.

#### Planning Mode

* Chịu trách nhiệm: thu thập yêu cầu; xây dựng Project, Arc, Chapter, Scene outline, Character, World Setting, Lore, Memory và Competence; kiểm tra readiness; chạy workflow phân đoạn; xác nhận phase; commit plan.
* Được phép lập kế hoạch cho số lượng chương lớn, ví dụ 500 chương, nếu người dùng yêu cầu.
* Không được tự động sinh nội dung `scenes.content` của các chương. Số lượng chương trong plan không phải số lượng chương cần viết ngay.

#### Writing Mode

* Chỉ hoạt động sau khi đã có plan hợp lệ được commit.
* Một phiên Writing Mode chỉ tập trung vào một chương đang được chọn.
* Được phép: đọc plan; đọc cấu trúc chương hiện tại; đọc các cảnh thuộc chương; đọc thông tin nhân vật, lore, world setting liên quan; đọc các chương đã viết trước đó; kiểm tra continuity; viết nội dung từng cảnh thuộc chương đang chọn; chỉnh sửa nội dung thuộc chương đang chọn; lưu version nội dung theo cơ chế Writing Mode.
* Không được: tự động viết nhiều chương trong một yêu cầu; tự chuyển sang chương tiếp theo sau khi hoàn thành; tự tạo background workflow viết hàng loạt; gọi subagent để viết đồng thời nhiều chương; bỏ qua plan đã commit; tự động sinh `scenes.content` cho toàn bộ truyện.

#### Điều kiện và từ chối

* Nếu chưa có plan hợp lệ, từ chối yêu cầu viết ngay, giải thích ngắn gọn rằng cần hoàn thành Planning và hướng người dùng về Planning Mode.
* Nếu yêu cầu viết hàng loạt như "viết ngay 100 chương" hoặc "viết từ chương 1 đến chương 20", từ chối thực hiện hàng loạt; không tự chia thành nhiều job để lách giới hạn; không âm thầm chạy background task; đề nghị người dùng chọn một chương.
* Nếu người dùng yêu cầu viết một chương đã có plan, không từ chối chỉ vì chương có nhiều cảnh; có thể viết lần lượt các cảnh thuộc chương đó và dừng ở ranh giới chương.

#### Continuity

* Writing Mode sử dụng ngữ cảnh đã có để kiểm tra diễn biến nhân vật, năng lực, ký ức, quan hệ, địa điểm, lore, sự kiện đã xảy ra và các giới hạn đã thiết lập.
* Cho phép dữ liệu Memory và Competence trùng lặp giữa các cảnh; không tự coi dữ liệu trùng là lỗi. Phải phân biệt dữ liệu trùng với mâu thuẫn logic trong diễn biến truyện.

#### Pipeline độc lập

* Planning Mode và Writing Mode có điều kiện kích hoạt, context injection, prompt block khi cần, tập tool và quy tắc versioning riêng theo loại dữ liệu.
* Không tái sử dụng workflow Phase 3-6 để tự động viết nội dung nhiều chương.
* Chi tiết hợp đồng thực thi Writing Mode nằm tại mục 1.6; các điểm kỹ thuật và schema còn mở nằm tại Q4. Tài liệu này không thiết kế pipeline triển khai cụ thể.

### 1.6 Hợp đồng Writing Mode: chương-first, cảnh là block

> **Trạng thái:** APPROVED (v2.4). Đây là business contract; chi tiết giao diện và route thuộc `fe-core.md` §9. Các chi tiết kỹ thuật chưa chốt nằm tại Q4.

#### Mô hình chương và cảnh

* Một chương là một dãy cảnh có thứ tự. Trong FE, mỗi block nội dung đại diện đúng một scene; block không phải paragraph và không gộp nhiều scene.
* `scenes.content` là nội dung của một scene; `chapters` không có cột nội dung chương. Nội dung đọc được của chương là các scene thuộc chương, sắp theo `scenes.position`.
* Đọc và ghi ở cấp chương; sửa nội dung, generate, lock và version ở cấp scene.
* Sau commit, scene identity là `scenes.id`. Thứ tự hiển thị lấy từ `scenes.position`; array index, thứ tự block trong FE và id tạm không phải identity bền vững.
* Không có scene page riêng, không có mô hình block paragraph, không có `chapter_versions`.
* `sceneId` tạm trong FE preview chỉ dùng cho bản xem trước; không được ghi vào DB, không được coi là `scenes.id`.
* **[OPEN] Read-model scene identity:** backend phải trả `scenes.id` và `position` thật cho FE trước khi bỏ dữ liệu preview. Fallback `scn-{chapter}-{block}` không phải contract persistence.

#### Điều kiện mở Writing Mode

* Chỉ mở sau khi plan đã commit hợp lệ.
* Một phiên Writing Mode chỉ làm việc với một chương đã chọn.
* Không tự chuyển chương, không generate nhiều chương trong một yêu cầu, không chạy background bulk, không gọi subagent viết song song nhiều chương.
* Phải xác định được chương thuộc project đã commit và scene thuộc chương đang chọn.

#### Vòng đời sinh nội dung cảnh

```text
Generate / Regenerate
        ↓
Buffer tạm của scene
        ↓
Review inline
        ├── Discard → giữ nguyên nội dung đã có
        └── Accept
                ↓
        Working state của chương
                ↓
             Dirty
                ↓
        Save cấp chương
                ↓
        Dữ liệu bền vững
```

* Generate/Regenerate tác động đúng một scene.
* Kết quả generation đi vào buffer tạm; không ghi trực tiếp `scenes.content`.
* Accept chỉ đưa nội dung vào working state và đánh dấu dirty; Accept không persist.
* Discard và Cancel không thay đổi nội dung đã lưu.
* Tối đa một generation đang hoạt động trong một chương.
* Không tự generate scene kế tiếp, không tự chuyển chương.
* Phải review trước khi Accept.
* **[IMPLEMENTATION PENDING]** backend generation stream/cancel và buffer contract.

#### Save cấp chương

* Save là hành động cấp chương; chỉ các scene dirty được đưa vào Save.
* Backend tương lai phải thực hiện Save nguyên tử. Một Save thành công:
  * ghi nội dung cuối cùng đã Accept của các scene dirty;
  * ghi thay đổi `scenes.is_locked` áp dụng cho các scene dirty;
  * cập nhật `word_count` tương ứng;
  * tạo các bản ghi `scene_versions` cần thiết;
  * trả về trạng thái đã persist để FE cập nhật baseline.
* Một Save thất bại không được báo thành công, không xóa dirty, không tuyên bố đã tạo version, giữ nội dung đang sửa nếu có thể và không để chương ở trạng thái ghi một phần.
* Với mỗi scene có nội dung thay đổi, một Save thành công tạo đúng một bản ghi `scene_versions` mới. Nhiều lần sửa trung gian trước Save không tạo nhiều version.
* Nếu chỉ có thay đổi lock mà nội dung không đổi, Save persist `scenes.is_locked` nhưng không tạo content version. DBML hiện không có bảng audit lock; nếu sau này cần audit lock thì ghi thành `[SCHEMA FOLLOW-UP]`, không tự thêm cột/bảng.
* Nhiều scene dirty trong cùng một Save tạo nhiều version, mỗi scene tối đa một version, trong cùng transaction. Không có version cấp chương.
* `scene_versions` dùng `(parent_id = scene_id, version_number)` làm identity; `version_number` do handler cấp, unique `(parent_id, version_number)` của DB là hàng rào cuối.
* **[OPEN]** idempotency của Save, optimistic concurrency/stale write, cách persist source hiện tại của scene và audit lock.
* **[IMPLEMENTATION PENDING]** API/transaction Save và trả read-model.

#### Source của nội dung cảnh

* `scene_versions.scene_source` dùng enum `SceneSource`: `AI_GENERATED`, `HUMAN_EDITED`, `RESTORE`.
* AI tạo và được Accept mà không sửa tay → `AI_GENERATED`.
* Người dùng sửa nội dung → `HUMAN_EDITED`.
* Khôi phục version cũ và Accept/Save mà không sửa tay → `RESTORE`.
* Sau khi Restore, nếu người dùng sửa tay → `HUMAN_EDITED`.
* Source của nội dung cảnh tách biệt với `source` cấp draft (`user`, `ai_generated`, `user_edited`) tại mục 1.3. Không dùng lẫn hai mô hình.
* **[SCHEMA FOLLOW-UP]** `scenes` hiện không có cột source cho nội dung hiện tại; DBML chỉ có `scene_versions.scene_source`. Cần chốt source hiện tại được suy ra từ version mới nhất hay lưu riêng; không tự thêm cột.

#### Restore nội dung cảnh

* Restore là append-only: không sửa/xóa version cũ.
* Chọn một version lịch sử → nạp nội dung vào working state; nếu khác baseline thì đánh dấu dirty.
* Restore không persist ngay và không tạo version ngay.
* Sau Save thành công, tạo version mới với `scene_source = RESTORE`, `version_number` kế tiếp.
* Nếu nội dung restore trùng baseline thì không dirty và không tạo version.
* Không có restore cấp chương.

#### Khóa cảnh

* `ChapterBlock.locked` của FE tương ứng `scenes.is_locked`.
* Scene đang khóa: không sửa tay, không generate/regenerate; nội dung hiện có được giữ nguyên.
* Khóa một scene không ảnh hưởng scene khác.
* Lock/Unlock là thay đổi dirty và được đưa vào Save cấp chương.
* Lock-only Save không tạo content version (xem trên).
* **[IMPLEMENTATION PENDING]** persist lock qua backend; hiện FE chỉ giữ local.

#### Dirty state và điều hướng

* Dirty được theo dõi theo từng scene; gồm thay đổi nội dung và thay đổi lock.
* Đổi focus giữa các scene phải giữ working state.
* Mở Chat không được làm mất working state.
* Điều hướng rời chương khi còn thay đổi chưa lưu phải yêu cầu xác nhận.
* Không tuyên bố autosave, crash recovery hoặc durable draft nếu backend chưa có.
* **[IMPLEMENTATION PENDING]** guard dựa trên dirty thật từ backend; preview hiện dùng local state.

#### Continuity

* Continuity của Writing Mode là theo scene và chỉ mang tính advisory.
* Cảnh báo continuity không tự động chặn Save.
* Dữ liệu Memory/Competence trùng giữa các scene không phải lỗi; chỉ mâu thuẫn logic mới là cảnh báo.
* **[IMPLEMENTATION PENDING]** API continuity; không tự tạo luật mới.

#### Ràng buộc schema hiện tại

* `scenes` đã có `content`, `word_count`, `position`, `is_locked`, `chapter_id`; `chapters` không có cột nội dung.
* `scene_versions` đã có `parent_id`, `version_number`, `scene_source`, `summary`, `content`, `word_count`, `created_at`; chưa có khóa idempotency và mã tham chiếu generation (nếu cần).
* `scene_versions.parent_id` hiện nullable; contract nội dung cảnh yêu cầu gắn scene, nên cần validate/DBML follow-up nếu muốn cưỡng chế `NOT NULL`.
* Không có `chapter_versions`; không thêm bảng này trong v2.4.
* Các thay đổi schema nằm tại Q4; không sửa `docs/db.dbml` trong v2.4.

---

## 2. BẢN ĐỒ CHI TIẾT 7 PHASE NGHIỆP VỤ

Toàn bộ thông tin của một tác phẩm được biểu diễn qua cấu trúc gồm **Phase 0 Tiền đề (`require`)** và **6 Phase Triển khai (`optional`)**:

```
                       ┌────────────────────────┐
                       │   NGƯỜI DÙNG NHẬP LIỆU │
                       │    (Hội thoại chat)    │
                       └───────────┬────────────┘
                                   │
                                   ▼
                       ┌────────────────────────┐
                       │ BÓC TÁCH QUA TOOL CALL │
                       │   (require + optional) │
                       └───────────┬────────────┘
                                   │
                                   ▼
                    [Kiểm tra Gating: isRequireComplete]
                                   │
                 ┌─────────────────┴─────────────────┐
                 ▼                                   ▼
        [CHƯA ĐẠT CỐT LÕI]                   [ĐẠT ĐỦ CỐT LÕI 100%]
     AI phản hồi gợi ý & hỏi                Bật sáng nút [Draft Building]
     bổ sung các trường thiếu                            │
                                                         ▼
                                             ┌──────────────────────┐
                                             │    DRAFT WORKSPACE   │
                                             │  (Duyệt 6 Bước)      │
                                             └───────────┬──────────┘
                                                         │
   ┌─────────────────────────────────────────────────────┴─────────────────────────────────────────────────────┐
   ▼                         ▼                       ▼                      ▼                   ▼              ▼
[Phase 1]                [Phase 2]               [Phase 3]              [Phase 4]           [Phase 5]      [Phase 6]
Overview                 Chapters                World Building         Scene Config        Characters     Relationships
- Cấu trúc Arc           - Phân chia Chương      - Cây địa lý thế giới  - Phong cách kể     5.1 Ngoại hình - Ma trận cảm xúc
- Mục tiêu dung sai      - Phân cảnh (Scenes)    - Tri thức & Thần thoại- POV nhân vật      5.2 Tâm lý     - Vai trò quan hệ
- Chủ đề & Luận điểm     thô                     (Lore Entries)         - Lore áp dụng      5.3 Kỹ năng
   │                         │                       │                      │                   │              │
   └─────────────────────────┴───────────────────────┴──────────────────────┴───────────────────┴──────────────┘
                                                         │
                                                         ▼ (Người dùng ấn [Bắt đầu viết / Start])
                                             ┌──────────────────────┐
                                             │ CHUYỂN DỮ LIỆU VÀO   │
                                             │ BẢNG SQLITE QUAN HỆ  │
                                             └──────────────────────┘
```

---

### 2.1 Phase 0: Cốt lõi bắt buộc (`require` — Gating Phase)

* **Ý nghĩa:** Đây là điều kiện tiên quyết để định danh một tác phẩm. Nếu thiếu bất kỳ trường nào, tác phẩm chưa đủ thông tin nền móng để xây dựng khung dàn ý.
* **Danh mục trường kiểm tra (`progressEngine.ts`):**
  1. `require.project.display_name`: Tên tác phẩm hiển thị.
  2. `require.project.summary`: Tóm tắt tiền đề (logline/premise).
  3. `require.project.language`: Ngôn ngữ sáng tác chính (`VI`, `EN`, `JA`...).
  4. `require.project.target.arc_count`: Số lượng Arc lớn dự kiến (> 0).
  5. `require.project.target.chapter_count`: Tổng số chương mục tiêu (> 0).
  6. `require.project.target.word_per_chapter_count`: Định mức số chữ trung bình mỗi chương (> 0).
  7. `require.project.genres`: Danh sách thể loại (ít nhất 1 thể loại được gán `code`).
  8. `require.character[].display_name` / `require.character[].nickname`: Tên hiển thị hoặc tên gọi ngắn/biệt danh của nhân vật khởi tạo. Nhân vật hợp lệ không được đồng thời `null` cả `display_name` lẫn `nickname` (có thể `null` 1 trong 2 hoặc có cả 2).
  9. `require.character[].biological_sex`: Giới tính sinh học (`NONE`, `MALE`, `FEMALE`).
  10. `require.character[].motif_code`: Mã motif nhân vật khởi điểm (ví dụ: `strong_girl`, `loyal_knight`...).

**UI Mapping:** Toàn bộ `require.project` được hiển thị thành một Card có tên **Project Overview** ở Phase 0. Card này chỉ đại diện thông tin dự án; `require.character[]` là dữ liệu riêng, không gộp vào Project Overview.

`require.character[]` phải có ít nhất 1 nhân vật.

* `arc_count`, `chapter_count`, `word_per_chapter_count` là trường ẩn hoặc tùy chỉnh nâng cao, chưa được người dùng chỉnh trực tiếp; hiện chỉ AI sử dụng. `motif_code` cũng do AI dùng và tham chiếu dữ liệu motif được sinh sẵn.
* AI ghi các giá trị đã xác định vào draft qua tool call, không cần cờ `unconfirmed`. Tuy nhiên, nếu AI đề xuất lệch khỏi yêu cầu người dùng (ví dụ yêu cầu 120 chương nhưng cho rằng cần 122 chương), AI phải nêu lý do và chờ người dùng chấp thuận trước khi thay đổi target; riêng số Arc không được tự ý quyết định.

---

### 2.2 Phase 1: Tổng quan dàn ý Arc (`optional.overview_phase`)

**UI Mapping:** Phase 1 có thể được gọi là **Draft Overview** hoặc **Arc Overview** trên UI; đây là phần dàn ý Arc vĩ mô, không phải Card **Project Overview** của Phase 0.

* **Ý nghĩa:** Định hình vĩ mô toàn bộ tác phẩm: nhịp điệu phân bổ và thông điệp triết lý.
* **Cấu trúc dữ liệu:**
  * `project.target`: Yêu cầu tương đối/dự kiến cho LLM, có thể biến động và không phải giá trị cần lưu thành cột trong dự án. Các miền lệch (`arc_tolerance`, `chapter_tolerance`, `word_per_chapter_tolerance`) chỉ mang tính đề xuất, không phải ràng buộc cứng; AI đề xuất thay đổi target lệch khỏi yêu cầu và chờ người dùng chấp thuận trước khi áp dụng.
  * `arcs[]`: Danh sách Arc lớn:
    * `position`: Thứ tự của Arc (1, 2, 3...).
    * `display_name`: Tên gọi của Arc.
    * `summary`: Tóm tắt biến cố chính trong Arc.
    * `motif.code`: Motif xung đột chủ đạo của Arc.
    * `motif.event_blocks[]`: Các khối biến cố lớn trong Arc (chương bắt đầu `start_chapter`, chương kết thúc `end_chapter`, tóm tắt khối).
    * `themes`: Chủ đề triết lý của Arc:
      * `question`: Câu hỏi triết lý trung tâm mà Arc đặt ra.
      * `resolution`: Phương thức giải quyết (loại và nội dung).
      * `message`: Thông điệp nhân sinh rút ra sau khi Arc kết thúc.
      * `arguments[]`: Các nhân vật đóng vai trò đại diện cho từng luồng quan điểm đối nghịch (`role`: `PRIMARY`, `ANTITHESIS`, `CAUTIONARY`, `WITNESS`; thể hiện qua lời nói hoặc hành động `answer_mode`).

---

### 2.3 Phase 2: Dàn ý chương & cảnh thô (`optional.chapter_phase`)

* **Ý nghĩa:** Rã nhỏ từng Arc thành các đơn vị phân phối nội dung nhỏ hơn.
* **Cấu trúc dữ liệu:**
  * `arcs[].chapters[]`: Danh sách chương:
    * `position`: Số thứ tự duy nhất của chương trên toàn bộ truyện trong project; Arc chỉ nhóm nội dung theo theme và motif, không tạo phạm vi đánh số riêng. Tính duy nhất theo project được backend validate (project suy ra qua `arc.project_id`).
    * `display_name`: Tên chương.
    * `summary`: Tóm tắt nội dung chương.
    * `scenes[]`: Các cảnh quay thô:
      * `code`: Slug định danh cảnh (duy nhất nội bộ).
      * `display_name`: Tên cảnh.
      * `position`: Thứ tự cảnh trong chương.
      * `summary`: Diễn biến hành động và xung đột trong cảnh.
      * `world_setting_code`: Địa điểm chính của cảnh; chưa bắt buộc khi tạo cảnh thô ở Phase 2, được gán ở Phase 4 và bắt buộc trước commit.

---

### 2.4 Phase 3: Bối cảnh thế giới (`optional.world_building_phase`)

* **Ý nghĩa:** Dựng phông nền địa lý, quy tắc vận hành xã hội và tri thức nền để các cảnh quay diễn ra nhất quán, không mâu thuẫn lore.
* **Cấu trúc dữ liệu:**
  * `world_settings`: Cấu trúc cây thế giới phân cấp:
    * `code`, `parent_code`: Cấu trúc cha - con (Vũ trụ ➔ Giới diện ➔ Vương quốc ➔ Khu vực ➔ Địa điểm cụ thể).
    * `display_name`: Tên địa danh / bối cảnh.
    * `type`: Thể loại địa hình / môi trường.
    * `description`: Mô tả không gian, bầu không khí, luật lệ.
    * `access`: Cấp độ bảo mật hoặc điều kiện thâm nhập.
    * `lore_entries[]`: Tri thức gắn liền:
      * `code`, `display_name`: Mã và tên tri thức / thực thể (ví dụ: "Giáo phái Hư Vô", "Pháp bảo Hồn Đăng").
      * `type`: Phân loại (tổ chức, pháp bảo, kỳ nhân, lịch sử, cấm thuật).
      * `description`: Nội dung mô tả chi tiết.
      * `data`: Đối tượng JSON bổ sung theo `type`; `type` ở cột riêng nên không lặp discriminator bên trong `data`. DBML khai báo `lore_entries.data` là JSON; Business Flow/UI hiện mô tả `data` như chuỗi tự do.

Payload `lore_entries.data` theo DBML dùng shape theo `LoreType` (không thêm các cột/tables riêng cho từng loại):

| `type` | Payload gợi ý trong `data` |
| :--- | :--- |
| `CULTURE` | `traditions?: string[]`, `beliefs?: string[]`, `languages?: string[]`, `festivals?: {name, description?, timing?}[]`, `taboos?: string[]` |
| `HISTORY` | `timeline?: {date_or_period, title, description?}[]`, `causes?: string[]`, `consequences?: string[]` |
| `FACTION` | `goals?: string[]`, `methods?: string[]`, `territory?: string[]`, `members_or_structure?: string`, `allies?: string[]`, `rivals?: string[]` |
| `ITEM` | `functions?: string[]`, `powers_or_effects?: string[]`, `limitations?: string[]`, `origin?: string`, `materials?: string[]` |

* Các key đều tùy chọn; key vắng mặt nghĩa là chưa biết/chưa cung cấp. `[]` chỉ có nghĩa là đã xác nhận không có giá trị; không serialize `null`. Khi chưa có dữ liệu type-specific, dùng `{}`.
* `HISTORY.date_or_period` là chuỗi tự do để hỗ trợ niên đại giả tưởng; `FACTION.territory` là mô tả, không phải FK. Không tạo FK cho allies/rivals hoặc `ITEM.materials` khi chưa có schema ref data.
* Validate `data` phải là JSON object và đúng shape theo `type`; không ép ngày về ISO. UI nên dùng form theo type và giữ round-trip các key chưa biết để tránh mất dữ liệu.

---

### 2.5 Phase 4: Kỹ thuật phân cảnh (`optional.scene_config_phase`)

* **Ý nghĩa:** Gắn góc nhìn đạo diễn văn học vào từng phân cảnh cụ thể đã phác thảo ở Phase 2.
* **Cấu trúc dữ liệu:**
  * `arcs[].chapters[].scenes[]`:
    * `style_preset_code`: Phong cách diễn đạt (ví dụ: `dark_fantasy_grim`, `epic_battle`, `intimate_dialogue`).
    * `narrator_config_code`: Cấu hình người kể chuyện (Ngôi thứ nhất POV Minh Hoàng, Ngôi thứ ba giới hạn, Ngôi thứ ba toàn tri...).
    * `world_setting_code`: Gán hoặc đổi địa điểm chính của cảnh; bắt buộc trước commit. Cảnh là đơn vị liên tục về thời gian và địa điểm; đổi địa điểm thì là cảnh khác. Chọn nút sâu nhất trong cây `world_settings` mà `summary` của cảnh xác định được; chỉ dừng ở tầng cao hơn khi tóm tắt không nêu nơi cụ thể. Nút sâu kế thừa mô tả của nút cha, còn chọn nút quá rộng làm mất chi tiết giác quan. Nếu không có nút phù hợp, AI tạo nút con dưới nút cha phù hợp nhất qua `add_world_setting` (nội dung phái sinh), hoặc hỏi người dùng khi cảnh mơ hồ. Không tự đổi địa điểm do người dùng đã gán; thay đổi phải theo quy tắc `source` và patch chờ xác nhận tại mục 1.3.
    * `lore_entries[]`: Mỗi liên kết lore trong cảnh gồm `lore_entry_code`, `usage`, `reveal_to_reader`, `reveal_to_pov`. Ba trường `usage`, `reveal_to_reader`, `reveal_to_pov` là nội dung phái sinh (Mức 2); `source` được tính cho từng liên kết, không theo từng trường. AI sinh `usage` khi gán lore vào cảnh. Chỉ đặt `reveal_to_reader` và `reveal_to_pov` khi `usage` là `FORESHADOW`, `PAYOFF`, `REVEAL` hoặc `RED_HERRING`; các `usage` khác dùng mặc định DBML. Với ngôi kể toàn tri (0 `is_pov`), bỏ qua `reveal_to_pov`, để mặc định DBML.
    * `characters[]`: Danh sách nhân vật có mặt tại cảnh quay, tham chiếu bằng `code` ổn định thay vì `display_name`:
      * `is_pov`: Cờ xác định ai là nhân vật giữ góc nhìn trần thuật (đối với ngôi kể giới hạn phải có đúng 1 `is_pov`; đối với ngôi kể toàn tri phải có 0 `is_pov`).

---

### 2.6 Phase 5: Hồ sơ nhân vật chuyên sâu (`character` — 3 Sub-phases)

* **Ý nghĩa:** Biến những cái tên vô hồn ở Phase 0 thành những con người sống động với đầy đủ thể xác, sang chấn tâm lý và năng lực thực tế.
* **Cấu trúc chi tiết qua 3 Sub-phase:**

#### Sub-phase 5.1: Ngoại hình (`character_appearance_phase`)

* Chủng tộc (`species`), Ngày tháng sinh nhật (`birth`).
* Giới tính thể hiện (`gender_identity`, `gender_expression`).
* Thể hình: Chiều cao (`height`, `height_unit`, mặc định `CM`), Cân nặng (`weight`, `weight_unit`, mặc định `KG`).
* Tóc (`hair`: màu, kiểu tóc, kết cấu tóc, độ dài).
* Nước da (`skin`: sắc độ, đặc tính).
* Đôi mắt (`eyes`: hình dạng con ngươi, màu mống mắt, màu tròng trắng).
* Phong cách trang phục (`outfit_style`).
* Dấu hiệu nhận dạng đặc biệt (`distinguishing_marks`: sẹo, nốt ruồi, ấn ký ma thuật).
* Tóm tắt tổng thể phong thái ngoại hình (`default_summary`).
* Các nhóm `hair`, `skin`, `eyes` là cấu trúc draft ánh xạ sang các trường `*_code` tương ứng với các cột `*_id` của `character_appearances`; `distinguishing_marks[]` ánh xạ sang `character_distinguishing_marks` (`location`, `content`).

#### Sub-phase 5.2: Động cơ & Tâm lý học chiều sâu (`character_psychological_phase`)

* **Core Trauma & Belief (Vết thương & Ảo tưởng):**
  * `the_wound`: Sang chấn quá khứ định hình con người hiện tại.
  * `the_lie`: Ảo tưởng/niềm tin sai lầm mà nhân vật tự huyễn hoặc để bảo vệ bản thân.
  * `fundamental_fear`: Nỗi sợ nguyên thủy nhất khiến nhân vật dễ hoảng loạn.
* **Cognitive Moral Framework (Khung đạo đức nhận thức):**
  * `moral_alignment`: Thiên hướng đạo đức (Lawful Good, Neutral Good, Chaotic Evil...).
  * `moral_code`: Giới hạn đỏ mà nhân vật thề không bao giờ vượt qua.
  * `cognitive_bias`: Thiên kiến nhận thức có ảnh hưởng đến cách nhân vật diễn giải sự việc.
  * `breaking_point`: Điểm giới hạn mà khi bị chạm tới, nhân vật sẽ sụp đổ hoặc biến chất.
* **Narrative Engine (Cỗ máy thúc đẩy câu chuyện):**
  * `external_want`: Khát vọng vật lý bên ngoài (tiền tài, trả thù, sống sót).
  * `internal_need`: Nhu cầu tinh thần sâu thẳm thực sự cần để trưởng thành (chấp nhận tha thứ, học cách buông bỏ).
  * `internal_conflict`: Xung đột nội tâm dữ dội giữa Cái Muốn và Cái Cần.
* **Behavioral Adaptation (Thích ứng hành vi):**
  * `social_mask`: Cấu trúc gồm `persona_type` (kiểu nhân cách/mặt nạ xã hội) và `verbal_style` (phong cách ngôn từ).
  * `defense_mechanisms`: Cấu trúc gồm `primary_reaction`, `stress_trigger`, `coping_strategy` — phản ứng chính, tác nhân căng thẳng và chiến lược đối phó khi khủng hoảng.

* Business Flow dùng cấu trúc nested snake_case: `core_trauma_and_belief`, `cognitive_moral_framework`, `narrative_engine`, `behavioral_adaptation`; tên trường giữ snake_case như danh sách trên. Các nhóm tương ứng với UI hiện tại (`coreTrauma`, `cognitiveMoral`, `narrativeEngine`, `behavioralAdaptation`); khi serialize, Business Flow là contract chuẩn.
* Hồ sơ tâm lý được lưu typed trong `character_psychological`, quan hệ 1:1 với `characters` qua `character_id` PK/FK; 15 trường ở trên là cột `TEXT` snake_case, nullable để hỗ trợ draft/tiến độ từng phần. Không lưu toàn bộ hồ sơ thành JSON blob. Bảng này đã có trong DBML.

#### Sub-phase 5.3: Trải nghiệm sống & Năng lực (`character_life_experience_phase`)

* `memories[]`: Các phân cảnh ký ức quan trọng trong quá khứ gắn chặt với sự kiện hiện tại.
* `competences[]`: Năng lực thực chiến và kiến thức chuyên môn:
  * Lĩnh vực chuyên môn (`domain`), Cấp độ thuần thục (`level`).
  * Cảnh thể hiện/bộc lộ năng lực (`scene_code`, bắt buộc): tham chiếu một cảnh đã có trong `arcs[].chapters[].scenes[]`; khi commit ánh xạ sang `character_competences.scene_id` (`not null`). Nếu chưa xác định được cảnh, hỏi người dùng để làm rõ, không tự gán cảnh khác hoặc commit năng lực thiếu cảnh.
  * Khả năng làm được (`capabilities[]`).
  * Giới hạn không thể làm được (`limitations[]`).

#### Quy tắc dữ liệu phẳng cho Competence và Memory

* `character_competences` và `character_memories` lưu theo mô hình dữ liệu phẳng; không bổ sung bảng trung gian cảnh-năng lực hoặc cảnh-ký ức.
* Giữ `character_competences.scene_id NOT NULL`: mỗi bản ghi năng lực gắn với đúng một cảnh thể hiện/bộc lộ.
* Giữ `character_memories.source_scene_id` nullable: ký ức có thể có cảnh nguồn hoặc là ký ức nền/backstory.
* Cho phép nhiều bản ghi năng lực có nội dung giống hoặc tương đương nhau ở các cảnh khác nhau.
* Cho phép nhiều bản ghi ký ức có nội dung giống hoặc tương đương nhau.
* Không tự động đồng bộ những bản ghi có nội dung giống nhau.
* Không áp đặt unique constraint dựa trên nội dung năng lực hoặc ký ức.
* Khi sáng tác, AI vẫn phải kiểm tra tính nhất quán của nhân vật theo tiến trình truyện.
* Việc cho phép dữ liệu trùng không đồng nghĩa AI được phép tạo ra mâu thuẫn continuity.
* Phân biệt: trùng lặp dữ liệu là điều được cho phép; mâu thuẫn trạng thái nhân vật là vấn đề cần kiểm tra khi sáng tác; không dùng cơ chế chuẩn hóa database để thay thế việc kiểm tra continuity.

---

### 2.7 Phase 6: Động lực quan hệ (`optional.relationship_phase`)

* **Ý nghĩa:** Định nghĩa lực hút - đẩy giữa các cá nhân, giúp các màn tương tác không bị giả tạo hay một chiều.
* **Cấu trúc dữ liệu:**
  * `motif.event_blocks[]`: Biến cố xúc tác làm thay đổi cục diện quan hệ.
  * Quan hệ cặp đôi (`from_code` ➔ `to_code`):
    * Ma trận cảm xúc gắn theo Scene (`scenes[scene_code]`) được lưu thưa: chỉ ghi tại cảnh có thay đổi hoặc keyframe, không lặp lại giá trị không đổi ở mọi cảnh. Các chiều dùng enum `RelationDimensionLevel` trong DBML, không dùng thang số `-10..+10`.
      * `trust`: Độ tin cậy, dùng cùng thang `RelationDimensionLevel` như các chỉ số quan hệ (`NONE`, `LOW`, `MODERATE`, `HIGH`, `ABSOLUTE`) theo `docs/db.dbml`.
      * `affection`: Mức độ thiện cảm / tình cảm.
      * `hostility`: Sự thù địch / hiềm khích.
      * `fear`: Nỗi sợ hãi đối phương.
      * `respect`: Sự tôn trọng / nể phục.
      * `dependency`: Mức độ phụ thuộc.
      * `familiarity`: Mức độ một nhân vật hiểu/quen thuộc với nhân vật kia; trường này có trong `relationship_states` của `docs/db.dbml` và cần được phản ánh trong Business Flow.
      
    * Vai trò xã hội (`roles[]`): Thứ bậc quyền lực (`hierarchy`), phân loại (`category`), vai trò chính hay phụ (`is_primary`).

---

### 2.8 Bất biến & Validate chéo

Các quy tắc bất biến và validate chéo:

* Tổng số chương trong các Arc bằng `project.target.chapter_count`; số Arc bằng `project.target.arc_count`.
* `chapters[].position` là thứ tự duy nhất trên toàn truyện trong project, chạy xuyên Arc; backend kiểm tra tính duy nhất khi tổng hợp qua `arcs.project_id`.
* `event_blocks` phải nằm trong khoảng chương của Arc tương ứng.
* Các `*_tolerance` chỉ tạo miền gợi ý cho AI; lệch khỏi target phải được đề xuất và chờ người dùng chấp thuận, không phải lỗi validate cứng.
* Quy tắc POV phụ thuộc `narrator_config`: ngôi kể giới hạn có đúng 1 `is_pov`; ngôi kể toàn tri có 0 `is_pov`.
* Mỗi cảnh phải có `world_setting_code` trỏ tới một `world_settings` tồn tại và cùng project trước commit; thiếu hoặc không hợp lệ là lỗi validation. Handler không chặn cứng theo `LocationType`, chỉ kiểm tra nút tồn tại và thuộc cùng project.
* Một nhân vật hợp lệ không được đồng thời `null` cả `display_name` và `nickname` (`display_name IS NOT NULL OR nickname IS NOT NULL`; có thể `null` 1 trong 2 hoặc có cả 2).
* `character_competences.scene_id` là `NOT NULL`; `character_memories.source_scene_id` nullable; hai bảng giữ mô hình phẳng, không unique theo nội dung và không có bảng trung gian cảnh-năng lực/cảnh-ký ức (mục 2.6).
* Rule được thực thi tại backend (Tauri/Rust) làm nguồn duy nhất; FE chỉ hiển thị kết quả, không tự định nghĩa lại nghiệp vụ.

---

## 3. CƠ CHẾ VẬN HÀNH TRÊN GIAO DIỆN NGƯỜI DÙNG (FRONTEND MECHANICS)

### 3.1 Trình đo lường tiến độ (Progress Engine)

* Hệ thống duyệt qua **46 thuộc tính Master list** (`MASTER_ATTRIBUTES_LIST`).
* Hiển thị hai phần trăm riêng trên thanh Header, mỗi phần chia cho tổng 46 thuộc tính và mỗi thuộc tính đóng góp 0 hoặc 1 theo evaluator (giữ nguyên cách tính mảng): **Đã xác nhận** = số thuộc tính có `source = user` hoặc `user_edited` / 46; **AI đề xuất** = số thuộc tính có `source = ai_generated` / 46. Dữ liệu thuộc phase chưa mở vẫn được tính nếu có trong JSON. Thay cho một `progressPercent` tổng duy nhất.

* Người dùng có thể mở icon **(i)** trên thanh tiến độ để tự kiểm tra các trường đã có/đang thiếu và nội dung hiện ghi trong draft.
* `get_field_null()` là tool đọc (tên ví dụ, chưa chốt; backend xử lý, xem [harness/tools.md](harness/tools.md)) đọc toàn bộ `businessFlowData`, trả tối đa 3 trường `null`, mỗi trường kèm `field` và câu hỏi cố định `question` theo `system_language`. Kho câu hỏi là tập cố định theo từng ngôn ngữ do backend giữ (resource i18n theo quy ước DBML); ngôn ngữ chưa có trong kho dùng `en`. Backend tự biết ngôn ngữ, AI không truyền; không thêm tool `get_questions_by_language` để tránh một lượt gọi thừa hoặc AI truyền sai ngôn ngữ.
* `get_array_element_fields` (tên ví dụ, chưa chốt) là tool đọc cho mảng như `arcs[]`, `chapters[]`, `require.character[]`: mảng rỗng không phải giá trị `null` nên `get_field_null` không báo. Trả tối đa 3 trường kèm câu hỏi theo `system_language`. Có `code` phần tử thì trả các trường `null` của phần tử đó; không có `code` thì trả các trường ưu tiên để tạo phần tử mới theo cấu hình backend, không thêm trường ngoài core. `require.character[]` vẫn phải có ít nhất một phần tử theo mục 2.1. AI hỏi theo câu hỏi tool trả về, không tự suy luận trường còn thiếu.
* **Hộp thoại Tiến trình chi tiết (`ProcessDetailDialog`):**
  * Render Markdown trực tiếp từ cấu trúc dữ liệu `businessFlowData`.
  * Huy hiệu phân định rõ ràng: `Cốt lõi: Đã hoàn tất` (Xanh lá) hoặc `Cốt lõi: Chưa hoàn tất` (Màu hổ phách).

### 3.2 Quy chế Mở khóa lũy tiến trong Draft Workspace

Draft Workspace hiển thị chia đôi màn hình (Split Panel có thể kéo dãn độ rộng). UI mở dữ liệu tuần tự theo 6 bước. Việc bóc tách thông tin người dùng không bị giới hạn bởi phase đang mở; riêng AI chỉ sinh nội dung phái sinh cho phase hiện tại, không sinh trước toàn bộ phase sau.

1. **Bước 1 (Draft Overview / Arc Overview):** Mở xem các Arc lớn. Toàn bộ `chapters` trong Arc bị ẩn để tránh làm rối mắt. Đây là Phase 1 `optional.overview_phase`, không phải Card Project Overview của Phase 0.
2. **Bước 2 (Chapter):** Mở hiển thị chi tiết các Chương và Phân cảnh thô trong từng Arc.
3. **Bước 3 (World):** Kích hoạt hiển thị Tab **World Building** trên màn hình.
4. **Bước 4 (Scene):** Kích hoạt hiển thị Tab **Scene Config**, cho phép gán POV nhân vật.
5. **Bước 5 (Character):** Mở khóa chuyên sâu theo 3 nấc:
   * 5.1: Mở thẻ Ngoại hình.
   * 5.2: Mở thẻ Tâm lý học (Wound / Want / Need).
   * 5.3: Mở thẻ Ký ức & Năng lực.
6. **Bước 6 (Relationship):** Mở Tab **Mối quan hệ** và Ma trận cảm xúc.

### 3.3 Chuyển bước an toàn (Step Progression Guard)

* Nút **Next (Tiếp tục)** ở mỗi bước luôn mở ra `ConfirmDialog`.
* Người dùng bắt buộc phải xác nhận duyệt qua giai đoạn hiện tại thì mới kích hoạt hiệu ứng chuyển cảnh (`fade-out` ➔ `loading` ➔ `idle`) để nạp giai đoạn tiếp theo.

### 3.4 Vòng lặp phản hồi trực tiếp (Feedback Mode Cursor)

* Tại bất kỳ bước nào trong Draft Workspace, người dùng có thể kích hoạt **Feedback Mode** trên Toolbar.
* Con trỏ chuột chuyển thành hình biểu tượng trích dẫn (`MessageSquareQuote`).
* Người dùng nhấp chuột vào bất kỳ phần tử nào trên dàn ý (một Arc, một Scene, một Nhân vật).
* Phần tử đó sẽ được "gắn thẻ ngữ cảnh" (`feedbackContext = { code, tag, title }`) và ghim thẳng vào khung nhập liệu của Chat (`ChatInputBar`).
* Người dùng chỉ cần gõ nội dung chỉnh sửa (ví dụ: *"Đổi nhân vật này từ tóc đen sang tóc vàng"*), AI thực hiện thay đổi bằng tool call có scope theo `feedbackContext.code` mà không làm xáo trộn các phần khác.
* Với patch ghi đè trường `source = user` hoặc `user_edited`, scope theo `code` ổn định (không dùng index mảng) và bắt buộc cho người dùng xem trước/xác nhận trước khi áp dụng. Đây là quy tắc đã chốt; patch chỉ chờ xác nhận áp dụng các trường được bảo vệ theo mục 1.3.
* Khi người dùng attach Card **Project Overview** (Phase 0), `feedbackContext` dùng `code = "require.project"` (địa chỉ ổn định theo path), `tag = "project"`, `title = require.project.display_name`; không dùng tag `overview` để tránh nhầm Phase 1.
* `feedbackContext.content = JSON.stringify(require.project)` đóng gói toàn bộ `require.project`: `display_name`, `summary`, `language`, `target.arc_count`, `target.chapter_count`, `target.word_per_chapter_count`, và `genres[]` gồm `code`, `is_primary`, `display_name` nếu có, cùng `source` hiện tại của các trường để AI áp dụng bảo vệ ghi đè. Không gộp `require.character[]` vào payload Project Overview.
* Khi attach Header/khối **Draft Overview / Arc Overview** (Phase 1), dùng `tag = "overview"` và payload scope theo `optional.overview_phase`; các khối Arc vẫn dùng `tag = "arc"`.

### 3.5 Bắt đầu sáng tác (Start Action)

* Nút **Start** chỉ xuất hiện duy nhất ở cuối Bước 6 (Relationship).
* Khi người dùng nhấn nút này, hệ thống mở **Commit Confirmation Dialog**. Dialog đồng thời là nơi review plan: tóm tắt dữ liệu sẽ ghi, validation status và pending patches. Không có màn hình/phase review riêng.
* Commit chỉ được phép khi không còn điều kiện chặn: plan thiếu dữ liệu bắt buộc, validation thất bại hoặc còn `pending_patches`. Dialog hiển thị lỗi và không cho xác nhận khi còn điều kiện chặn.
* `phase_confirm` và `commit` là hai thao tác khác nhau:
  * `phase_confirm` là version xác nhận một phase planning (`draft_versions.reason = phase_confirm`), không chuyển dữ liệu sang bảng quan hệ.
  * `commit` là chuyển toàn bộ plan đã duyệt thành dữ liệu quan hệ bền vững theo mục 3.7.
* Khi người dùng xác nhận, hệ thống chuyển đổi toàn bộ cây JSON nháp (`businessFlowData`) thành các bản ghi thực thể quan hệ bền vững trong cơ sở dữ liệu SQLite (`projects`, `arcs`, `chapters`, `scenes`, `characters`, `project_characters`, `world_settings`, `lore_entries`). Chi tiết validation và transaction được quy định tại mục 3.7.

### 3.6 Vòng đời Draft & Version

* Mục này chỉ nói về `draft_versions` của Planning. `scene_versions` của Writing Mode nằm tại mục 1.6; không trộn hai mô hình version.
* `businessFlowData` (Super JSON gồm 6 phase) được lưu bền trong SQLite (bảng draft), không chỉ nằm trong RAM.
* **Sở hữu State và khóa ghi:** Draft là đơn vị sở hữu State. Mỗi draft có đúng một working State trong bộ nhớ backend, khóa theo `draft_session_id`, và một khóa ghi (mutex theo draft). Cùng lúc chỉ một trong các bên sau được ghi vào draft: một lượt AI, một thao tác ghi của UI/CLI (sửa tay, confirm/reject patch, restore, confirm_phase), một vòng workflow phân đoạn, hoặc một lần commit. Bên đến sau xếp hàng chờ; quá `DRAFT_LOCK_TIMEOUT` (tham số runtime, mặc định 30 giây) thì trả lỗi "draft đang bận". Nhờ vậy không tồn tại hai bản State riêng cho cùng một draft và không có ghi đè lẫn nhau. Trong workflow phân đoạn, subagent của một vòng chạy dưới khóa của vòng đó và các tool call của chúng được tuần tự hóa ở State.
* `DRAFT_LOCK_TIMEOUT` là thời gian tối đa chờ lấy khóa, không phải thời gian tối đa giữ khóa. Không áp giới hạn 30 giây cho thời gian thực thi workflow; một vòng có thể chạy lâu hơn.
* **Quan hệ Chat Session, Draft Session, Project:** Trước commit, Draft:ChatSession = 1:1; Project:ChatSession = 1:N qua `sessions.project_id`; Draft:Project = 0 hoặc 1 trước commit và 1:1 sau commit. Một tin nhắn xác định draft qua `messages.session_id` -> `draft_sessions.session_id`, lấy draft có `status` là `editing` hoặc `committing`; pipeline bơm `draft_session_id` vào `TurnContext`, không thêm định danh draft lên `messages`. Một draft không được nhiều chat session cùng ghi; cửa sổ chat thứ hai mở cùng project chỉ được xem hoặc gắn vào session đang sở hữu draft. Xung đột head version bị loại từ gốc bằng mutex, không cần merge. Trong transaction commit, cập nhật `draft_sessions.project_id`, `status = committed` và `sessions.project_id` của session sở hữu. Khi mở lại draft (chỉ khi chưa có nội dung cảnh nào được viết), dùng lại đúng draft đó, đặt `status = editing` và gắn lại `draft_sessions.session_id` sang session đang mở; một draft vẫn chỉ có một session sở hữu tại một thời điểm.
* Mỗi version là snapshot bất biến của toàn bộ JSON kèm `phase` và `subphase` tại thời điểm tạo. Khôi phục version đồng thời khôi phục dữ liệu, phase và subphase; quay về phase trước không phải thao tác độc lập với version.
* **Sub-phase đã chốt:** `draft_versions.phase` là `int`, `draft_versions.subphase` là `int` nullable; sắp xếp và restore theo cặp `(phase, subphase)`. `subphase` chỉ nhận 1, 2, 3 khi `phase = 5` (tương ứng Sub-phase 5.1/5.2/5.3), và là `null` ở các phase khác. Restore đưa người dùng về đúng phase và subphase của version được chọn. Không dùng số thực vì sai số biểu diễn nhị phân, `5.1` và `5.10` nhập nhằng, và có thể biểu diễn bước vô nghĩa như `5.5`.
* **Version theo lượt AI:** lượt có ít nhất một tool **ghi** (`add_`, `set_`, `delete_`) đạt `has_applied_write = true` thì gộp các thay đổi đã `applied` thành MỘT version, chốt khi lượt kết thúc, `reason = ai_patch`. `has_applied_write` tính theo `results[]`: có ít nhất một `results[i].status = applied`, hoặc `status` tổng là `applied` với tool không có `results`. Không tạo version cho từng tool call; tool đọc trả `applied` không góp vào version.
* Trong lượt, tool call áp dụng ngay vào State làm việc để các call sau đọc được giá trị mới; version chỉ được niêm phong cuối lượt.
* Lượt không có tool call ghi nào đạt `has_applied_write = true` thì không tạo version (bao gồm chat thuần, call `rejected`, call `pending_confirmation`).
* Trường `rejected` không làm đổi State. Khi lượt có cả trường `applied` và trường `rejected`, version chỉ chứa phần thay đổi đã `applied`; event log ghi rõ trường bị `rejected`.
* **Niêm phong cuối lượt:** Vòng lặp lượt nằm trong một hàm trong trả về kết quả lượt; hàm ngoài luôn gọi bước kết thúc lượt để niêm phong phần tool ghi `applied` trên mọi đường thoát có kiểm soát (vượt ngưỡng `consecutive_rejected_count`, vượt `MAX_ITER`, lỗi hạ tầng đã bắt). Nếu bước niêm phong lỗi DB thì thử lại một lần; xử lý lỗi theo mục "Niêm phong thất bại" bên dưới. State làm việc nằm trong bộ nhớ; khi app tắt đột ngột hoặc crash, phần ghi chưa niêm phong của lượt đó mất và khi mở lại ứng dụng, backend kiểm tra các lượt có bằng chứng bền vững về tool ghi đã trả `applied` trong `messages.payload_json` nhưng không có `draft_versions` nào cùng `turn_id`; những lượt này được đánh dấu là có khả năng bị gián đoạn và báo “lượt trước bị gián đoạn, dữ liệu chưa được ghi”. Lượt chat thuần, lượt chỉ đọc, hoặc lượt chỉ có call `rejected` hay `pending_confirmation` không bị coi là gián đoạn theo điều kiện này. Đây là cơ chế phát hiện dựa trên audit đã lưu, không bảo đảm phát hiện mọi crash: nếu ứng dụng dừng trước khi tool result được lưu bền vững, hệ thống có thể không có đủ bằng chứng để nhận diện lượt đó. Sau khi phục hồi, `pending_patches` còn `status = pending` có `target_code` không còn trong State được chuyển sang `expired`. Workflow phân đoạn (mục 3.9) niêm phong theo từng vòng và resume idempotent, nên mất mát chỉ giới hạn trong vòng đang chạy. Không cần bảng `tool_calls` khi chưa có nhu cầu thống kê; giữ chi tiết trong `messages.payload_json`.

#### Working State, `applied` và sealed version

* Working State là trạng thái đang xử lý trong bộ nhớ backend. Tool result `applied` chỉ xác nhận mutation đã áp dụng vào Working State, không đồng nghĩa mutation đã được lưu bền vững.
* Sealed Draft Version là snapshot đã được ghi bền vững trong `draft_versions`; đây là nguồn phục hồi Draft State.
* `messages.payload_json` là audit phục vụ lịch sử và phát hiện gián đoạn, không phải nguồn phục hồi Draft State.
* Không được mô tả `applied` là durable commit.

#### Niêm phong thất bại

* Cuối lượt có mutation `applied`, backend phải niêm phong version. Nếu seal thất bại, thử lại đúng một lần.
* Nếu lần thử lại thành công, lượt được xem là đã niêm phong.
* Nếu cả hai lần thất bại, trả lỗi `VersionSealFailed`; không trả kết quả thành công ban đầu, không tự báo State đã được lưu, không tự xóa Working State như một cách giả lập rollback.
* UI/API phải phân biệt lỗi lưu version với lỗi sinh phản hồi AI.

#### Idempotent sealing

* Một lượt chat thông thường chỉ được tạo tối đa một sealed version cho cùng cặp `(draft_session_id, turn_id)`.
* Với workflow phân đoạn, mỗi vòng có thể tạo một version riêng; phải phân biệt định danh lượt AI, định danh workflow và định danh vòng workflow. Không dùng một `turn_id` chung cho nhiều vòng nếu điều đó làm các version hợp lệ bị coi là trùng.
* Retry không tạo version thứ hai cho cùng một đơn vị niêm phong; nếu version đã tồn tại, trả lại kết quả version hiện có; không tạo hai snapshot khác nhau cho cùng một idempotency key.
* Kiểm tra và ghi phải có bảo đảm nguyên tử ở tầng persistence. Schema hiện tại chỉ có unique `(draft_session_id, seq)`; xem `[SCHEMA FOLLOW-UP]` bên dưới.

#### Phục hồi sau crash

1. Khôi phục Draft State từ sealed version hợp lệ cuối cùng.
2. Không phát lại tool call từ audit để dựng State.
3. Không tự áp dụng lại mutation chưa được niêm phong.
4. Không coi dữ liệu trong Working State trước crash là dữ liệu bền vững.
5. Có thể phát hiện lượt bị gián đoạn từ audit đã lưu; nếu audit chưa được lưu trước crash, chấp nhận giới hạn không thể phát hiện đầy đủ.
6. Không vì thiếu audit mà tự tạo dữ liệu hoặc tự suy luận mutation đã thành công.

#### `[SCHEMA FOLLOW-UP]` Idempotent sealing

* **Bảng:** `draft_versions`.
* **Trường/index liên quan:** hiện có `turn_id` (index thường, không unique), `seq`, `draft_session_id`; chưa có khóa idempotency cho workflow round.
* **Bất biến cần bảo đảm:** không có hai version cho cùng một lượt hoặc cùng một vòng workflow.
* **Đề xuất tối thiểu (chưa áp dụng):** thêm một cột khóa idempotency, ví dụ `seal_key varchar`, và unique index `(draft_session_id, seal_key)`; giá trị là `turn:<turn_id>` cho lượt chat và `workflow:<workflow_id>:<round_id>` cho vòng workflow. Tên cột và cách biểu diễn workflow/round cần chốt cùng Q3.
* **Vì sao chưa sửa:** nhiệm vụ này không sửa DBML; schema hiện tại chưa đủ để bảo đảm idempotency ở tầng database.
* Các sự kiện tạo version:

| Sự kiện | Tạo version? | reason |
| :--- | :--- | :--- |
| Lượt AI có `has_applied_write = true` | Có | `ai_patch` |
| Người dùng xác nhận một hoặc nhiều patch (confirm hoặc **Áp dụng tất cả**) và có ít nhất một patch được áp dụng (version riêng, không gộp vào lượt AI) | Có | `ai_patch` |
| Chỉ có tool đọc `applied`, call `rejected` hoặc `pending_confirmation` (không có tool ghi nào `applied`) | Không | — |
| Người dùng sửa tay (gộp theo debounce) | Có | `manual_edit` |
| Confirm sang phase mới | Có | `phase_confirm` |
| Regenerate phase | Có | `regenerate` |
| Restore version cũ | Có | `restore` |
| Kết thúc một vòng workflow phân đoạn có thay đổi State (mục 3.9) | Có, một version cho cả vòng | `workflow_round` |
| Chat thuần không đổi JSON; thao tác UI (đổi tab, kéo panel, bật Feedback Mode); JSON trùng hash; vòng workflow không có thay đổi | Không | — |

* Restore theo kiểu append-only: tạo version mới sao chép dữ liệu từ version được chọn, `parent_seq` trỏ về version đó; không xóa lịch sử. Nút **Quay lại** đi về `parent_seq`; select box cho phép nhảy tới version bất kỳ.
* Regenerate phase chỉ thay các trường có `source = ai_generated`; giữ nguyên `user`/`user_edited`. Phase đã Confirm không bị khóa; chat/sửa tay tiếp tục ghi theo quy tắc mục 1.3.
* Version khởi tạo cho Phase 0 được tạo tại thời điểm `isRequireComplete` chuyển sang true.
* Sau Start, draft chuyển sang `committed`; version chỉ đọc và select box bị ẩn. DB là nguồn sự thật duy nhất. Chỉ được mở lại draft nếu chưa có nội dung cảnh nào được viết.
* Một vòng workflow phân đoạn (mục 3.9) có thay đổi State được niêm phong thành đúng một version `workflow_round`, không gộp với version của lượt chat; subagent không tự tạo version. Vòng không có thay đổi (hoặc JSON trùng hash) không tạo version theo quy tắc chống trùng hiện có.
* **Lưu trữ đã có trong DBML:** `draft_sessions` quản lý phiên nháp, `session_id` 1:1 với chat session trước commit, `project_id` nullable trước commit, `head_version_seq`, trạng thái `editing`/`committing`/`committed` và thời gian tạo/cập nhật. `draft_versions` lưu snapshot toàn bộ `businessFlowData` kèm `source`, `phase`, `subphase`, hash, reason, parent_seq, `turn_id` và `chat_message_id`; `chat_message_id` trỏ tới tin nhắn user khởi phát lượt AI, `turn_id` gom message và tool result của lượt; version `manual_edit`, `restore`, `phase_confirm` có cả hai null, version `workflow_round` có cả hai null. Unique `(draft_session_id, seq)`, số thứ tự hiển thị dạng `v.03`. Giá trị `reason = workflow_round` đã có trong enum `DraftVersionReason` của DBML.
* `pending_patches` lưu bền patch với session, version cơ sở, `target_type` (loại đích, bắt buộc) và `target_code` (slug/địa chỉ ổn định trong JSON nháp, bắt buộc), trường, thao tác `set`/`delete`, giá trị cũ/mới, `target_source` (nguồn hiện có của chính phần tử đích), trạng thái và thời gian xử lý. Khi xóa cha `ai_generated` có hậu duệ `user`/`user_edited`, `target_source = ai_generated`; lý do cần xác nhận và danh sách phần tử bị ảnh hưởng được tính lại từ State lúc hiển thị preview theo mục 1.3, không lưu kèm patch. Patch chờ xác nhận còn nguyên sau khi tắt app. Patch hết hiệu lực theo quy tắc so giá trị tại mục 1.3, không theo `base_version_seq`; `base_version_seq` chỉ dùng cho event log. Tại một thời điểm chỉ tồn tại tối đa một patch `pending` cho mỗi `(draft_session_id, target_code, field_name)`; patch mới cùng đích và `field_name` với `new_value` khác làm patch cũ chuyển `expired` trong cùng bước dưới khóa ghi draft. Handler cưỡng chế giới hạn này, không thêm ràng buộc unique. Khi có patch `pending` cùng `(target_code, field_name, new_value)`, tool trả `patch_id` cũ và không tạo patch mới. **Áp dụng tất cả** chạy dưới khóa ghi draft, duyệt patch `pending` theo `created_at` tăng dần, revalidate từng patch, bỏ qua patch lỗi mà không rollback cả batch, và trả `{"applied": [patch_id...], "skipped": [{"patch_id": "...", "reason": "..."}]}`. Mỗi thao tác xác nhận tạo đúng một version `ai_patch` và chỉ tạo khi có ít nhất một patch được áp dụng; trường được xác nhận chuyển thành `user_edited`. Xác nhận hoặc từ chối từng patch độc lập nhau. Confirm và reject idempotent: cập nhật trạng thái bằng compare-and-set chỉ khi patch còn `pending`; đổi trạng thái patch, ghi State và tạo version nằm trong một transaction. Gọi lần hai trên patch đã xử lý trả kết quả hiện có (ví dụ `{"status": "already_confirmed", "seq": <version cũ>}`), không lỗi và không tạo version mới; UI khóa nút khi đang gửi. Các trạng thái gồm `pending`, `confirmed`, `rejected`, `expired`.

### 3.7 Cơ chế Commit (Start)

* Start chỉ nhận khi `draft_sessions.status = editing`. Handler giữ khóa ghi draft, chuyển `status = committing` và đọc snapshot tại `head_version_seq` ngay khi giữ khóa. Trong lúc `committing`, mọi lượt AI và thao tác ghi mới của UI bị chặn và trả thông báo "đang commit"; thao tác đọc UI vẫn được phép.
* Trước khi chuyển `committing`, nếu còn `pending_patches` ở trạng thái `pending`, Start bị chặn: không tự áp dụng patch, không tự từ chối patch, không âm thầm bỏ qua. Backend trả lỗi có cấu trúc kèm danh sách `patch_id`; UI yêu cầu người dùng xử lý patch trước khi commit. Sau khi patch đã được xử lý, người dùng có thể thử Start lại.
* Trước khi mở transaction, chạy validate trên snapshot vừa đọc bằng hàm thuần `validate(draft) → errors[]`.
* Ghi trong một transaction `BEGIN IMMEDIATE`, theo thứ tự phụ thuộc khóa ngoại: `projects` → `genres` → `characters` → `project_characters` → `arcs/themes` → `chapters` → `world_settings` (cha trước con) → `scenes` → `lore` → liên kết scene → appearance/profile/memory/competence → relationships. Giữ map `code → id` trong bộ nhớ khi tạo các bản ghi và liên kết.
* Trong transaction, handler kiểm tra lại collision `code` với dữ liệu đã commit; không dựa vào kết quả kiểm tra tại thời điểm cấp code; không dùng check-then-insert ngoài transaction làm cơ chế bảo đảm duy nhất. Unique constraint của DB theo từng bảng là hàng rào cuối cùng. Nếu collision, rollback toàn transaction, giữ draft ở trạng thái `editing` để người dùng tiếp tục xử lý, không tự đổi `code` đã được sử dụng trong draft tại thời điểm commit, trả lỗi collision có cấu trúc và không commit một phần dữ liệu.
* Trong cùng transaction, ghi `draft_sessions.project_id` bằng ID dự án vừa commit, đặt `status = committed` và gán `sessions.project_id` của session sở hữu.
* Nếu có lỗi ghi, rollback toàn transaction, đặt `status = editing`; draft được giữ nguyên và người dùng có thể thử lại.
* Bấm Start khi đang `committing` hoặc đã `committed` trả kết quả hiện có, không tạo project mới.
* Commit chạy nền, không chặn thao tác đọc UI và có hiển thị tiến độ.
* Sau commit, plan đã commit là điều kiện mở Writing Mode (mục 1.6); không tự sinh `scenes.content` cho các chương trong plan. Đóng Draft Workspace và điều hướng tới Project Detail của project vừa commit, mặc định View; người dùng chọn chương trong Creator để mở Writing Mode. Chi tiết UI và route thuộc `fe-core.md` §9.
* `source` được giữ trong snapshot JSON của `draft_versions` trong SQLite, kể cả sau commit, để truy vết/mở lại draft; không ánh xạ thành cột của các bảng thực thể quan hệ.
* Khả năng đảo ngược nằm ở tầng draft/version trước commit. Sau commit chỉ cho phép mở lại draft khi chưa có nội dung cảnh nào được viết.

#### Luồng xác nhận commit (H2)

> **Trạng thái:** APPROVED (v2.4). Triển khai FE/backend tương ứng còn `[IMPLEMENTATION PENDING]`.

* Hành động xác nhận cuối Bước 6 mở **Commit Confirmation Dialog**. Dialog đồng thời là nơi review plan: tóm tắt dữ liệu sẽ ghi, validation status và pending patches. Không thêm màn hình hoặc phase review riêng.
* Trước commit, dialog phải kiểm tra và hiển thị:
  * dữ liệu plan bắt buộc còn thiếu;
  * trạng thái readiness/validation của các phase;
  * `pending_patches` và lỗi có cấu trúc.
  Khi còn điều kiện chặn, nút xác nhận bị disable và không được gọi commit.
* Trong lúc commit:
  * chặn submit trùng;
  * giữ dialog ở trạng thái pending không thể đóng;
  * backend dùng operation idempotent;
  * request bị gián đoạn không được tự coi là thành công.
* Khi commit thất bại: giữ Draft Workspace và dữ liệu làm việc, hiển thị lỗi, cho phép thử lại an toàn; draft trở về `editing` theo mục 3.7.
* Khi commit thành công: đóng Draft Workspace, điều hướng tới Project Detail của project vừa commit, mặc định View Mode. Việc chọn chương để viết diễn ra trong Creator; không tự mở Writing Mode hoặc tự generate.
* Commit chạy nền, không chặn thao tác đọc UI và có hiển thị tiến độ.
* `phase_confirm` (xác nhận phase planning, tạo version draft) và `commit` (chuyển plan thành dữ liệu quan hệ) là hai thao tác khác nhau; không dùng lẫn thuật ngữ hoặc trạng thái.
* Mục này không thay đổi transaction, thứ tự ghi, validate, chống collision, rollback hoặc idempotency đã quy định ở trên.

### 3.8 Context cho AI

* Mỗi lượt gửi context gồm: System (vai trò, schema, phase, quy tắc bóc tách/ghi đè, ba mức sinh dữ liệu tại mục 1.3, hướng dẫn cách dùng tool; schema và mô tả tool được cấp qua function calling theo `prompt_block_tools`, không lặp lại trong prompt) + State (JSON hiện tại hoặc phần liên quan, kèm `source` của từng trường và `pending_patches`) + Event log + vài lượt chat cuối + tin nhắn hiện tại (kèm `feedbackContext`). Backend lấy `system_language` từ cài đặt ngôn ngữ giao diện mỗi lượt (hiện `vi` hoặc `en`) và pipeline bơm tên ngôn ngữ đầy đủ vào system prompt qua `{{system_language}}`. Chỉ dùng `system_language` để giao tiếp với người dùng: phản hồi, câu hỏi, giải thích, lời mời nhắc lại và mẫu trả lời ngoài nghiệp vụ; không dùng làm `project.language`. Nội dung sáng tác ghi vào State (tên, tóm tắt, mô tả...) dùng `project.language`; nếu còn `null` thì dùng ngôn ngữ tin nhắn người dùng. Chỉ định tường minh vì model có thể không tự giữ ngôn ngữ người dùng; không có ngoại lệ.
* State do hệ thống bơm vào mỗi lượt; AI không tự đọc file/DB để lấy State. Khi cần dữ liệu ngoài phần đã bơm, AI gọi tool đọc (`get_field_null()`, `get_array_element_fields` cho mảng, hoặc tool lấy State theo thực thể; xem [harness/tools.md](harness/tools.md)).
* Không bơm toàn bộ lịch sử chat; chat vẫn được lưu đầy đủ trong DB để hiển thị UI và truy vết.
* Mỗi lượt AI phải bóc tách mọi thông tin mới từ tin nhắn người dùng theo mục 1.3; system prompt nêu rõ các quy tắc thu thập, ghi đè theo `source` và ba mức sinh dữ liệu tại mục 1.3 để không bỏ sót dữ liệu phase chưa mở hoặc ghi đè trường bị bảo vệ.
* Patch đang chờ xác nhận được đưa vào State (ví dụ `pending_patches`) để AI biết người dùng chưa trả lời và không đề xuất trùng.
* Event log không có bảng riêng; do code sinh từ `draft_versions` và `pending_patches`, không do AI viết; log bao gồm cả thay đổi bị hủy/bác bỏ (ví dụ: `[v.06] Khôi phục về v.02, hủy thay đổi v.03–v.05`).
* Nếu State mâu thuẫn với event log hoặc chat thì ưu tiên State.
* Thông tin sống lâu phải nằm trong State: `user_constraints`, `decision_log`. `pending_question` chỉ là trạng thái hội thoại đang chờ; nếu người dùng đổi chủ đề thì không nhắc lại câu cũ, mà ở lượt sau dùng `get_field_null()` hoặc `get_array_element_fields` cho mảng và hỏi đúng tối đa 3 câu hỏi cố định theo `system_language` do tool trả về. AI không thêm câu khác khi hỏi bổ sung trường thiếu; quy tắc này không cản việc hỏi làm rõ tham chiếu hoặc mâu thuẫn ở mục 1.3.
* Phase 0 giữ lại nhiều lượt chat hơn vì State còn rỗng. Tối ưu token bằng cách chỉ gửi phase hiện tại và tóm tắt phase trước.

#### Phân giải tham chiếu thực thể

* Mỗi lượt chat nhận State (JSON nháp) trong context. Thực thể trong JSON nháp không dùng `id` DB: phần tử được định danh và liên kết chéo bằng `code`. Người dùng không thấy `code` trên UI, có thể gọi theo `display_name` hoặc thứ tự; AI đối chiếu tên/thứ tự (`position`) trong State để lấy `code`, rồi gọi tool theo thực thể. Không thêm tool `search_*` cho Arc, chương, cảnh; JIT retrieval chỉ phù hợp danh mục tĩnh như genres, motifs. `search_character` vẫn giữ nguyên.
* Khớp đúng một thực thể: lấy `code` và gọi tool ngay, trừ tham chiếu vị trí tương đối cần xác nhận theo quy tắc dưới đây. Khớp nhiều hơn một hoặc không khớp: AI không tự đoán, hỏi làm rõ tự nhiên; khi có nhiều ứng viên, liệt kê kèm ngữ cảnh cha (ví dụ cảnh trong Chương 2 hay Chương 15). Sau khi người dùng chọn, dùng đúng `code` để gọi tool.
* Khi người dùng gọi Arc/chương/cảnh bằng vị trí tương đối trong một phạm vi (ví dụ “chương đầu Hồi 2”, “cảnh cuối chương 41”), AI suy ra ứng viên từ `position` và quan hệ cha-con trong State, nói rõ tên/vị trí và phạm vi đã hiểu để người dùng xác nhận trước khi gọi tool ghi. Khi dựng context, backend tự tính thứ tự chương trong Arc từ `position` toàn truyện để AI hiểu cách gọi “chương 1 của Hồi 2”; không lưu thứ tự này vào DB và không đánh số lại `chapters.position`. Nếu State không đủ hoặc có nhiều cách hiểu, hỏi làm rõ thay vì đoán.
* Feedback Mode: khi người dùng bấm vào phần tử, `code` được ghim thẳng vào `feedbackContext` để không phải phân giải tên/thứ tự nữa.
* Các ca biên cần xét: trùng `display_name`; `chapters.position` là thứ tự toàn truyện nhưng người dùng gọi theo thứ tự trong Arc; State bị cắt tỉa khi truyện dài.
* **Phạm vi `code` đã chốt:** `code` duy nhất toàn cục trong draft, do handler bảo đảm; DBML chỉ ràng buộc `unique` theo từng bảng. Tool và State nháp dùng `code` ổn định; người dùng không thấy `code` trên UI.
* AI chỉ đề xuất slug gốc; tool handler bảo đảm `code` duy nhất toàn cục (thêm hậu tố nếu trùng) và trả `code` cuối trong kết quả `applied` để AI dùng cho call tiếp theo. `code` không đổi khi đổi `display_name`.
* Arc/chương/cảnh có `code` ghép từ `code` cha và đoạn số cấp bằng bộ đếm tăng dần theo **từng cha**, không tái sử dụng và độc lập với `position`. Slug theo tên chỉ gồm `[a-z0-9_]`; dấu `-` dành riêng làm dấu phân cách cấp bậc và ký hiệu loại, nên slug theo tên không thể tự sinh đoạn trùng như `-a1`. Ví dụ project `te_nu_nguu_nhan`, Arc `te_nu_nguu_nhan-a1`, chương `te_nu_nguu_nhan-a1-c15`, cảnh `te_nu_nguu_nhan-a1-c15-s3`. Bộ đếm theo cha là một phần của State và được khôi phục theo version khi restore; restore làm mọi `pending_patches` còn `pending` thành `expired` theo quy tắc patch lỗi thời, để patch cũ không nhắm nhầm thực thể mới trùng `code`. Không parse `code` để suy ra cha; dùng `parent_code`/`arc_code` làm tham chiếu. World settings và lore entries dùng tiền tố `project_code` cùng ký hiệu loại và bộ đếm; nhân vật dùng slug theo tên (thêm hậu tố khi trùng), phải gọi `search_character` để tái sử dụng trước khi tạo mới. `search_character` tìm trong project trước, nếu 0 khớp thì tìm toàn bộ thư viện nhân vật và trả cờ `in_project`; nếu tìm thấy ngoài project, AI hỏi người dùng có dùng lại không và liên kết qua `project_characters`, không tạo bản sao. AI không đoán trước `code` do handler cấp: các call phụ thuộc nhau phải gọi **tuần tự**, lấy `code` trả về từ call trước.
#### Cấp `code` và xử lý collision

* Phạm vi duy nhất: trong mỗi draft, `code` phải duy nhất toàn cục; trong database, các ràng buộc unique hiện tại vẫn có hiệu lực theo từng bảng. Handler là bên cấp `code` cuối cùng; AI không được tự quyết định `code` cuối cùng; `code` đã cấp không thay đổi khi `display_name` thay đổi.
* Khi tạo project hoặc nhân vật mới: sinh slug ứng viên; kiểm tra collision trong Working State; kiểm tra collision với dữ liệu đã commit trong DB; nếu trùng, cấp hậu tố để tạo code khác; trả code cuối cùng trong tool result `applied`.
* Với các thực thể con sử dụng namespace project, tiếp tục giữ quy tắc cấp code hiện tại; không thay đổi cấu trúc code đã chốt nếu không cần thiết.
* Hai draft chưa commit có thể tạm thời sử dụng cùng một code nếu code đó chưa tồn tại trong dữ liệu đã commit. Không triển khai reservation table; không triển khai cơ chế khóa code toàn cục xuyên suốt vòng đời draft.
* Khi commit: kiểm tra lại collision với DB; không dựa vào kết quả kiểm tra tại thời điểm cấp code; ràng buộc unique của DB là hàng rào cuối cùng; nếu collision, rollback transaction, giữ draft ở trạng thái có thể chỉnh sửa, trả lỗi collision có cấu trúc, không tự đổi code đã được sử dụng trong draft tại thời điểm commit và không commit một phần dữ liệu.
* Nhân vật thư viện: nếu người dùng sử dụng nhân vật đã tồn tại, tìm và xác minh nhân vật trong thư viện, tái sử dụng nhân vật hiện có và liên kết qua `project_characters`; không tạo bản sao chỉ vì nhân vật được sử dụng trong project khác. Nếu người dùng thực sự muốn tạo nhân vật mới có cùng tên, cho phép tạo nhân vật mới; handler cấp code khác để tránh collision; không coi trùng tên là trùng định danh.
* Bộ đếm cấp code vẫn thuộc Working State, được lưu và khôi phục theo snapshot/version.

* **Cửa sổ chương và mục lục tối giản:** Dùng cửa sổ trượt cho nội dung chi tiết (tóm tắt chương) quanh `n` là chương từ `feedbackContext` hoặc chương người dùng vừa nhắc, không cố định `n`:

```python
def get_chapter_window(n, total, t=None):
    t = t or max(3, min(round(total * 0.10), 15))
    size = 2 * t + 1
    a = max(1, n - t); c = min(total, n + t)
    if c - a + 1 < size:
        if a == 1: c = min(total, a + size - 1)
        elif c == total: a = max(1, c - size + 1)
    return a, c
```

* **Mục lục trong context:** Luôn bơm `require` (project và nhân vật). Mỗi phần tử mục lục Arc và chương có `code`, `display_name`, `position`, `code` cha và `summary`; chỉ bơm cảnh của chương neo. Nếu không có neo (`feedbackContext` và người dùng đều chưa nhắc chương), bơm tất cả Arc nếu số Arc không vượt `OUTLINE_PAGE` (nếu vượt thì phân trang Arc tương tự) và trang đầu gồm `OUTLINE_PAGE` chương đầu theo `position`. Nếu có neo, dùng `get_chapter_window` quanh chương neo. Phần còn lại hiển thị dạng đếm kèm cờ `outline_truncated`. Không khớp và cờ `false` thì hỏi người dùng; không khớp và cờ `true` thì **không** kết luận “chưa tồn tại” và **không** tạo mới.
* **Tra cứu mục lục:** Khi thứ người dùng nhắc không có trong phần đã bơm, AI gọi `lookup_outline` (tên ví dụ, chưa chốt) theo tên, vị trí hoặc mô tả nội dung. Bước 1, backend khớp xác định theo tên hoặc vị trí trên toàn mục lục, không dùng LLM, chuẩn hóa Unicode NFC như `search_character`; có kết quả thì trả. Bước 2, nếu bước 1 không có kết quả hoặc yêu cầu mô tả nội dung, backend chia mục lục thành các trang `OUTLINE_PAGE` chương bằng `plan_phase` và `split_balanced` (mỗi Arc là một item), chạy tối đa `MAX_PARALLEL` subagent song song trong mỗi vòng; các vòng tuần tự. Mỗi subagent chỉ đọc một trang kèm `code` cha và yêu cầu người dùng, trả tối đa vài ứng viên gồm `code`, `display_name`, `position`, `code` cha và lý do ngắn. Backend gộp, loại trùng, cắt theo giới hạn output tool đọc rồi trả cho AI. Subagent không có tool ghi, không tạo version, không có `declared_sources` hoặc `declared_source`. Nếu có trang lỗi, kết quả kèm `incomplete_pages` và AI không được kết luận “chưa tồn tại”. Nhiều ứng viên thì hỏi làm rõ theo quy tắc phân giải tham chiếu ở trên. `lookup_outline` là ngoại lệ giới hạn cho quy tắc không thêm `search_*` Arc/chương/cảnh.

#### Xử lý lỗi tool call

* **Lỗi kỹ thuật cấp tham số** (sai kiểu, sai enum, `code` không tồn tại, thiếu tham số bắt buộc): tool trả `rejected` kèm lý do; AI tự sửa và gọi lại trong cùng lượt; người dùng không thấy. Vì AI không sinh JSON nên không còn lỗi cú pháp JSON.
* **Lỗi ngữ nghĩa/nghiệp vụ** (lệch bất biến, chạm trường/phần tử được bảo vệ khi ghi đè hoặc xóa): AI không tự đoán; đi theo luồng hỏi người dùng hoặc `pending_patches` (mục 1.3).
* **Tra cứu và phân giải nhân vật theo tên (`search_character`, tên ví dụ):** Tìm trong phạm vi project trên `display_name` hoặc `nickname` theo thứ tự:
  1. Chuẩn hóa Unicode NFC, không phân biệt hoa thường.
  2. Ưu tiên khớp chính xác; nếu có thì chỉ xét tập khớp chính xác.
  3. Nếu không có khớp chính xác, so khớp tiền tố rồi chuỗi con.
  4. Nếu vẫn không có kết quả, thử lại một lần với so khớp không phân biệt dấu tiếng Việt (ví dụ `Thien` khớp `Thiện`).
  5. Áp dụng quy tắc số lượng bản ghi dưới đây. Ví dụ gõ `Thi`, project có `Thi` và `Thiện` thì khớp chính xác `Thi` và xác định luôn; nếu chỉ có `Thiện` và `Thiên`, khớp tiền tố trả hai bản ghi và AI hỏi lại.
  * **Trả về đúng 1 bản ghi:** Xác định chính xác nhân vật đó, không cần hỏi lại người dùng.
  * **Trả về nhiều hơn 1 bản ghi (> 1):** AI liệt kê các nhân vật khớp và hỏi người dùng muốn nói tới ai, tuyệt đối không tự đoán hoặc tự gộp.
  * **Trả về 0 bản ghi:** `search_character` tìm tiếp toàn bộ thư viện nhân vật, trả kết quả kèm cờ `in_project`; nếu thấy nhân vật ngoài project, AI hỏi người dùng có dùng lại không rồi liên kết qua `project_characters`, không tạo bản sao. Chỉ khi cả thư viện không có kết quả mới thông báo nhân vật chưa tồn tại và hỏi có muốn tạo mới.
* **Giới hạn `rejected` liên tiếp:** Backend (Tauri/Rust) giữ `consecutive_rejected_count` trong bộ nhớ của vòng lặp lượt chat hiện tại, không lưu DB. Bộ đếm chỉ tăng khi `status` tổng của tool call là `rejected` và về 0 trong các trường hợp còn lại, kể cả call có ít nhất một trường `applied` hoặc `pending_confirmation`. Khi vượt ngưỡng cấu hình runtime (giá trị mặc định ví dụ 3), dừng vòng lặp lượt, giữ State đã có; các trường `applied` trước đó vẫn được chốt vào version cuối lượt theo mục 3.6. Thông báo bằng lời tự nhiên, không lộ lỗi kỹ thuật, mời người dùng nhắc lại nội dung. Ngưỡng này độc lập với `MAX_ITER` của agent runtime. DBML chỉ lưu vết; `messages.payload_json` đã chứa chi tiết tool call và kết quả.
* **Lịch sử nạp lại khi lỗi hạ tầng:** Khi lượt kết thúc do lỗi hạ tầng, phần phản hồi có `tool_call` chưa đủ `tool_result` không được lưu vào lịch sử nạp lại; mỗi `tool_call` nạp lại phải có `tool_result` tương ứng.
* **`messages.turn_id`:** `messages.turn_id` (`uuid`, nullable) gom message và tool call cùng một lượt AI, hỗ trợ version theo lượt và truy vấn; DBML có cột và index tương ứng. Giữ chi tiết tool call trong `messages.payload_json`; chỉ cân nhắc bảng `tool_calls(id, message_id, tool_name, status, error_reason, created_at)` khi cần truy vấn thống kê.

### 3.9 Workflow phân đoạn (Segmented Workflow)

* **Mục tiêu:** Truyện dài hàng trăm chương không bị nhồi vào một lượt AI. Agent đóng gói Phase 1–6 thành workflow; **hàm thuần ở backend** tính kế hoạch chia task. Orchestrator không tự suy luận cách chia, chỉ chạy hàm kế hoạch rồi gọi tool/subagent theo kết quả.
* **Tham số cấu hình runtime:** Giá trị mặc định tập trung trong cấu hình, không hard-code trong thuật toán.

| Tham số | Mặc định | Ý nghĩa |
| :--- | :---: | :--- |
| `OUTLINE_CAP` | 50 | Sức chứa một lần chạy Phase 1–2 trong một Arc; lần đầu ghi tối đa 50 chương. |
| `OUTLINE_LOOKBACK` | 10 | Số chương đọc lùi từ lần chạy thứ hai của cùng Arc. |
| `OUTLINE_PAGE` | 50 | Số chương mỗi trang mục lục/tra cứu đọc ở mục 3.8; khác `OUTLINE_CAP` là sức chứa ngữ cảnh đọc và ghi của một lần chạy Phase 1–2. |
| `UNIT_CHAPTER` | 30 | Số chương ghi tối đa cho một subagent khi đơn vị task là chương. |
| `CONTEXT_MARGIN` | 5 | Số chương đọc thêm mỗi bên với task chia theo chương; cố định trong một lần chạy. |
| `MAX_PARALLEL` | 6 | Số subagent song song tối đa trong một vòng. |
| `DRAFT_LOCK_TIMEOUT` | 30 | Thời gian chờ khóa ghi draft, tính bằng giây; quá thời gian thì trả lỗi "draft đang bận". |

#### Khóa ghi và hủy workflow

* Mỗi draft có một khóa ghi riêng theo `draft_session_id`. Trong một vòng workflow, chỉ một writer được phép thay đổi Working State; chat làm thay đổi draft, sửa tay, commit và restore không được thực thi đồng thời với vòng đang ghi. Các thao tác đọc không làm thay đổi State vẫn được phép nhưng phải đọc từ snapshot nhất quán.
* `DRAFT_LOCK_TIMEOUT = 30s` là thời gian tối đa chờ lấy khóa, không phải thời gian tối đa giữ khóa. Không tự thêm giới hạn 30 giây cho thời gian thực thi workflow.
* Hủy workflow là yêu cầu dừng có kiểm soát:
  1. Người dùng gửi yêu cầu hủy.
  2. Backend đánh dấu workflow đang được yêu cầu hủy.
  3. Không khởi chạy task mới sau khi nhận yêu cầu hủy.
  4. Không ngắt ngang một tool mutation đang thực thi.
  5. Chờ các mutation đã bắt đầu hoàn tất hoặc trả lỗi.
  6. Nếu vòng có thay đổi `applied`, niêm phong một version cho vòng đó.
  7. Chỉ kết thúc hủy sau khi xử lý kết quả niêm phong.
  8. Không tự rollback những mutation đã `applied`.
  9. Giữ lại báo cáo tiến độ và trạng thái task để phục vụ resume.
  10. Giải phóng khóa ghi khi vòng đã kết thúc an toàn.
* Nếu niêm phong thất bại, áp dụng contract seal failure tại mục 3.6; không báo hủy thành công theo nghĩa dữ liệu đã được lưu.
* Không coi yêu cầu hủy là quyền bỏ qua các quy tắc `source`, `pending_confirmation` hoặc versioning.
* Hợp đồng UI: khi workflow đang chạy, hiển thị phase hiện tại, vòng hiện tại, số task đã hoàn thành và tổng số task nếu có; khóa các thao tác ghi draft; cho phép người dùng xem dữ liệu; hiển thị trạng thái workflow đang chạy; cung cấp thao tác yêu cầu hủy. Không bắt buộc thiết kế giao diện cụ thể, màu sắc hoặc vị trí component.

#### Phase 1–2: Dàn ý tuần tự theo cửa sổ

* Dùng thuật ngữ **lần chạy** cho Phase 1–2 để không nhầm với **vòng** song song của Phase 3–6. `OUTLINE_CAP` là sức chứa ngữ cảnh một lần chạy: tổng số chương đọc lùi cộng số chương ghi không vượt `OUTLINE_CAP`. Lần chạy đầu không đọc lùi, ghi tối đa `OUTLINE_CAP` chương; từ lần thứ hai, đọc lùi tối đa `OUTLINE_LOOKBACK` chương cuối của lần chạy trước trong cùng Arc rồi ghi tối đa `OUTLINE_CAP - OUTLINE_LOOKBACK` chương mới. Đọc lùi không vượt biên đầu Arc (không đọc sang Arc trước). Phải thỏa `0 <= OUTLINE_LOOKBACK < OUTLINE_CAP`; nếu không thì từ chối cấu hình.
* Ví dụ Arc 145 chương (`position` toàn truyện, Arc bắt đầu ở chương 1):

| Lần chạy | Đọc lùi | Ghi | Số chương trong ngữ cảnh |
| :---: | :---: | :---: | :---: |
| 1 | không | 1–50 | 50 |
| 2 | 41–50 | 51–90 | 10 + 40 = 50 |
| 3 | 81–90 | 91–130 | 10 + 40 = 50 |
| 4 | 121–130 | 131–145 | 10 + 15 = 25 |

* Arc 90 chương: (ghi 1–50), (đọc 41–50, ghi 51–90). Arc không bắt đầu ở chương 1, ví dụ `lo = 146`, `hi = 260` (115 chương): (ghi 146–195), (đọc 186–195, ghi 196–235), (đọc 226–235, ghi 236–260).
* Các lần chạy trong một Arc và các Arc với nhau chạy tuần tự. Lần đầu một Arc không đọc lùi sang Arc trước; sự liên tục lấy từ summary Arc ở Phase 1.

```python
def plan_outline_runs(lo, hi, cap=50, lookback=10):
    # lo..hi: position toàn truyện của chương đầu/cuối Arc
    # trả [{"read": (từ, đến) | None, "write": (từ, đến)}]
    assert lo <= hi and 0 <= lookback < cap
    runs, start = [], lo
    while start <= hi:
        first = not runs
        size = cap if first else cap - lookback
        end = min(hi, start + size - 1)
        read = None
        if not first and lookback > 0:
            read = (max(lo, start - lookback), start - 1)
        runs.append({"read": read, "write": (start, end)})
        start = end + 1
    return runs
```

* **Tính chất cần kiểm:** Các khoảng `write` chia đúng `lo..hi`, không trùng và không hở; mọi khoảng `read` nằm trong `lo..start-1` và dài không quá `OUTLINE_LOOKBACK`.
* Mỗi lần chạy là một vòng gồm một task; nếu có thay đổi State thì niêm phong đúng một version `workflow_round` ở cuối lần chạy.

#### Phase 3–6: Orchestrator và subagent song song

* Mỗi vòng workflow giữ khóa ghi draft trong suốt vòng; các tool call của subagent trong vòng được tuần tự hóa ở State.
* Mỗi phase đưa các items dạng `(group_key, lo, hi)` cho hàm thuần `plan_phase`. Hàm chia từng item thành các đoạn cân bằng không vượt đơn vị `unit`, rồi gom thành các **vòng**, mỗi vòng tối đa `MAX_PARALLEL` task. Vòng có thể tiếp tục lấy task của Arc kế tiếp khi Arc trước chưa lấp đầy slot; một task riêng lẻ vẫn thuộc `group_key` của item, không gộp hai Arc. Các vòng chạy tuần tự; subagent trong cùng vòng độc lập nhau.
* Với task lấy **chương làm đơn vị** (Phase 3 thăm dò, Phase 4), mỗi subagent đọc khoảng `[s - CONTEXT_MARGIN, e + CONTEXT_MARGIN]` kẹp trong `[1, tổng số chương]`; mặc định tối đa `5 + 30 + 5 = 40` chương. Với task chia theo bản ghi world/lore, nhân vật hoặc cặp quan hệ, `s..e` là thứ tự của loại bản ghi đó: đọc ngữ cảnh liên quan theo thực thể, không dùng phép cộng ±5 lên thứ tự bản ghi như thể đó là số chương.

```python
def split_balanced(lo, hi, unit):
    total = hi - lo + 1
    k = ceil(total / unit)
    base, extra = divmod(total, k)      # 55 -> 28 + 27
    out, s = [], lo
    for i in range(k):
        e = s + base + (1 if i < extra else 0) - 1
        out.append((s, e)); s = e + 1
    return out

def plan_phase(items, unit, max_parallel=6):
    tasks = [(g, s, e) for g, lo, hi in items for s, e in split_balanced(lo, hi, unit)]
    return [tasks[i:i + max_parallel] for i in range(0, len(tasks), max_parallel)]

def context_range(s, e, total, margin=5):
    return max(1, s - margin), min(total, e + margin)
```

* Ví dụ kiểm chứng: năm Arc dài 30/55/65/50/65 chương, `unit = 30` sinh `1 + 2 + 3 + 2 + 3 = 11` task. Vòng 1 có 6 task từ Arc 1, 2, 3; vòng 2 có 5 task từ Arc 4, 5.
* Đơn vị chia task theo phase (là cấu hình runtime, không cố định trong thuật toán):

| Phase / giai đoạn | Đơn vị chia task | Mặc định gợi ý |
| :--- | :--- | :--- |
| Phase 3 thăm dò | Chương để đọc/tổng hợp report | 30 chương/subagent |
| Phase 3 ghi | Bản ghi world/lore | 8–10 bản ghi/subagent |
| Phase 4 | Chương kèm cảnh, đọc thêm ±`CONTEXT_MARGIN` | 30 chương/subagent |
| Phase 5 | Nhân vật trong từng sub-phase 5.1/5.2/5.3 | 4–5 nhân vật/subagent |
| Phase 6 | Cặp quan hệ | 10 cặp/subagent |
| Tra cứu mục lục (đọc) | Trang chương | 50 chương/subagent |

* **Tra cứu mục lục:** `lookup_outline` tại mục 3.8 dùng `plan_phase` và `split_balanced` để chia trang `OUTLINE_PAGE` cho subagent chỉ đọc; không tạo version.
* **Ba giai đoạn Phase 3:** (1) thăm dò: mỗi subagent đọc phần chương của mình, ghi report địa hình/tổ chức/lore; (2) gộp: hàm thuần ở backend gộp report, loại trùng, dựng cây cha–con rồi gọi `plan_phase` chia danh sách đã gộp; (3) ghi: subagent ghi theo danh sách đã gộp và theo từng tầng của cây, **cha trước con**.
* **Phase 5:** Chạy tuần tự theo sub-phase 5.1, 5.2, 5.3; mỗi vòng thuộc đúng một sub-phase, không trộn nhiều sub-phase trong cùng vòng. Version `workflow_round` của vòng có thay đổi State ghi `phase = 5` và `subphase` tương ứng.
* **Report trung gian:** Backend tạo thư mục `./merlink-harness/tmp/<draft_session_id>/<phase>/` sau khi phase trước hoàn thành và trước khi phase này bắt đầu. Dùng `draft_session_id` để tách report giữa các draft trước commit, không lấy địa chỉ cố định `require.project` làm `project_code`. Chỉ backend đọc/ghi report qua tool (ví dụ `write_phase_report`, `read_phase_reports`, tên ví dụ chưa chốt; xem [harness/tools.md](harness/tools.md)); subagent không đụng file trực tiếp, đúng mục 1.4.2. Report là dữ liệu trung gian, **không phải State**, xóa khi workflow hoàn tất.
* **Version:** Mỗi vòng có thay đổi State chốt đúng **một** version `reason = workflow_round`, không gộp với version của lượt chat; subagent không tạo version riêng. Vòng không có thay đổi hoặc JSON trùng hash không tạo version theo mục 3.6. `workflow_round` đã có trong enum `DraftVersionReason` của DBML.
* **Sinh `code`:** Handler cấp `code` bằng bộ đếm theo từng cha phải tuần tự hóa ở backend (mutex theo cha) để hai subagent không nhận trùng số. Định dạng `code` theo quy tắc đã chốt tại mục 3.8.
* **Bảo vệ `source`:** Giữ nguyên mục 1.3. Patch `pending_confirmation` trong workflow không dừng workflow; gom lại và hiển thị cho người dùng sau khi hết vòng.
* **Giới hạn lỗi:** `consecutive_rejected_count` tính riêng cho từng subagent và chỉ tăng khi `status` tổng của tool call là `rejected`. Subagent vượt ngưỡng chỉ khiến task đó được đánh dấu failed; các trường `applied` trước đó của task vẫn góp vào version cuối vòng.
##### Resume workflow

* Resume dựa trên sealed version cuối cùng; không coi tool result `applied` chưa seal là dữ liệu bền vững.
* Workflow, mỗi vòng và mỗi task phải có định danh ổn định để phân biệt task đã hoàn thành và task cần chạy lại.
* Task hoàn thành trong vòng đã seal không tự chạy lại khi resume. Vòng chưa seal được xem là chưa hoàn thành.
* Không phát lại mutation từ audit để khôi phục State; resume phải revalidate State và điều kiện thực thi.
* Không tự coi việc có cùng tên thực thể là bằng chứng task đã hoàn thành; không tự tạo bản ghi trùng chỉ vì một task được thực thi lại.
* Các subagent có thể xử lý nội dung song song, nhưng mutation vào cùng draft vẫn phải tuân thủ khóa ghi. Không coi khóa ghi là giải pháp tự động ngăn trùng lặp ngữ nghĩa.
* Cấu trúc lưu task/checkpoint chưa được chốt; giữ trong Q3. Không tự thêm task ledger hoặc checkpoint schema.

---

## 4. BẢNG ÁNH XẠ DỮ LIỆU NHÁP SANG CƠ SỞ DỮ LIỆU QUAN HỆ (SQLITE SCHEMA)

| Khối dữ liệu trong Business Flow          | Bảng lưu trữ chính thức trong SQLite (`db.dbml`)            | Ghi chú chuyển đổi                                        |
| :------------------------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------ |
| Phiên quản lý bản nháp | `draft_sessions` | Quản lý `session_id` (1:1 với chat session trước commit), head version, trạng thái `editing`/`committing`/`committed` và liên kết project sau commit; không phải thực thể nghiệp vụ được tạo khi commit. |
| Snapshot bản nháp | `draft_versions` | Lưu toàn bộ JSON/source, phase và subphase theo version; không phải thực thể nghiệp vụ được tạo khi commit. |
| Đề xuất chờ xác nhận | `pending_patches` | Lưu bền patch và trạng thái xử lý; không phải thực thể nghiệp vụ được tạo khi commit. |
| `require.project`                          | `projects`, `project_genres`                                   | Khởi tạo dự án, lưu thể loại chính/phụ; `target`/`tolerance` là yêu cầu tương đối/runtime cho LLM, không phải dữ liệu cần lưu bền; không ánh xạ thành cột của `projects`.               |
| `require.character`                        | `characters`, `project_characters`, `character_motifs`                             | Quan hệ N-N, gắn nhân vật vào dự án qua bảng nối     |
| `optional.overview_phase.arcs`             | `arcs`, `arc_motifs`, `arc_event_blocks`, `themes`, `theme_arguments`          | Lưu danh sách Arc, chủ đề và luận điểm               |
| `optional.chapter_phase`                   | `chapters`, `scenes`                                           | Lưu cây Arc ➔ Chương ➔ Cảnh. Business Flow chốt `chapters[].position` theo thứ tự toàn truyện; DBML giữ index `(arc_id, position)` cho truy vấn theo Arc và bổ sung index `position` cho thứ tự toàn truyện; backend validate tính duy nhất theo project.                 |
| `optional.world_building_phase`            | `world_settings`, `lore_entries`                               | Cây bối cảnh và tri thức thần thoại; DBML có `lore_entries.data json`; contract payload theo `LoreType` được quy định tại 2.4. Business Flow/UI hiện vẫn mô tả `data` như chuỗi tự do và cần migrate sang contract này.                    |
| `optional.scene_config_phase`              | `scenes`, `scene_characters`, `scene_lore_entries`           | Cập nhật trường POV, gán phong cách văn học và địa điểm chính của cảnh           |
| `optional.character_appearance_phase`      | `character_appearances`, `character_distinguishing_marks`                                          | 1-1 với nhân vật                                           |
| `optional.character_psychological_phase`   | `character_psychological` (1:1 với `characters`; `character_id` là PK/FK) | Đã bổ sung vào DBML với 15 cột `TEXT` snake_case lấy từ UI/source; không dùng blob JSON.                              |
| `optional.character_life_experience_phase` | `character_memories`, `character_competences`, `competence_capabilities`, `competence_limitations`                  | Ký ức và kỹ năng của nhân vật                         |
| `optional.relationship_phase`              | `relationships`, `relationship_states`, `relationship_roles` | [Tạm treo theo xác nhận] DBML có `event_blocks` chung và `arc_event_blocks` cho Arc, nhưng chưa thiết kế bảng liên kết event block của motif quan hệ. `relationship_states` lưu các chiều `trust`, `affection`, `hostility`, `fear`, `respect`, `familiarity`, `dependency` bằng `RelationDimensionLevel`. |

**Bảng ngoài phạm vi ánh xạ từ draft:**

* **Danh mục tham chiếu (seed data, không sinh từ draft):** `motifs`, `genres`, `motif_applicable_genres`, `event_blocks`, `event_transitions`, `states`, `state_down_by_rules`, `state_rules`, `emotional_ranges`, `event_block_emotionals`, `components`, `motif_constraints`, `constraints`, `style_presets`, `rhetorical_devices`, `style_preset_rhetorical_devices`, `style_preset_vocab_items`, `style_preset_focus_items`, `narrator_configs`, `style_preset_narratives`, `narrative_directives`, `appearance_option_groups`, `appearance_options`, `competence_domains`, `domain_level`, `relationship_categories`.
* **Luồng viết sau commit (H1, xem mục 1.6):** `scene_versions`; không có `chapter_versions`. Nội dung chương đọc từ `scenes.content` theo `scenes.position`.
* **Hạ tầng chat và prompt:** `sessions`, `messages`, `prompt_blocks`, `tools`, `prompt_block_tools`.

`symbolics`, `symbolic_motifs` chưa dùng trong 7 phase (đang đánh dấu VFILL trong DBML).

---

## CHANGELOG v2.1

* Chốt quan hệ Chat Session, Draft Session và Project: thêm `draft_sessions.session_id` (1:1 với chat session trước commit); `sessions.project_id` là 1:N và được gán khi commit; tin nhắn xác định draft qua `messages.session_id`; thêm `draft_versions.turn_id`; `chat_message_id` trỏ tin nhắn user khởi phát lượt AI; mở lại draft chưa có nội dung cảnh thì dùng lại draft và gắn lại session sở hữu.
* Chốt contract ghi nhiều trường và provenance: tool ghi nhiều trường dùng `declared_sources` theo từng trường; tool xóa và tool liên kết một payload nguyên tử dùng `declared_source`; kết quả trả `results[]` theo từng trường; `has_applied_write` và `consecutive_rejected_count` tính theo `status` tổng; mỗi trường `pending_confirmation` tạo một patch và chống trùng theo từng trường.
* Chốt vòng đời patch: patch `expired` so giá trị thay vì `base_version_seq`; tối đa một patch `pending` cho mỗi `(draft_session_id, target_code, field_name)`, cưỡng chế ở handler và không thêm unique index; **Áp dụng tất cả** bỏ qua patch lỗi và tạo một version `ai_patch` nếu có patch được áp dụng; confirm/reject idempotent.
* Thêm nguyên tắc Draft sở hữu State, một working State theo `draft_session_id` và mutex ghi draft với `DRAFT_LOCK_TIMEOUT` mặc định 30 giây.
* Chốt commit/Start: thêm trạng thái `committing`; Start chỉ nhận khi `editing`; commit giữ mutex và đọc snapshot tại `head_version_seq`; chặn ghi trong lúc commit; rollback về `editing`; Start lặp trả kết quả hiện có.
* Ghi nhận xóa cha có hậu duệ được bảo vệ và State dùng chung theo draft đã chốt một phần; patch `pending` lúc Start và `characters.code` dùng chung vẫn mở. Xóa các câu hỏi đã chốt và đánh lại số câu hỏi mở còn lại.
* Cập nhật header lên phiên bản 2.1, đồng bộ `tools.md`, `overview.md`, `case_newchat.md` và `db.dbml`.

## CHANGELOG v2.2

* Làm rõ điều kiện phát hiện lượt có khả năng bị gián đoạn sau crash (mục 3.6): dựa trên bằng chứng tool ghi `applied` đã được lưu trong `messages.payload_json` nhưng chưa có version cùng `turn_id`; nêu rõ giới hạn phát hiện.
* Thêm câu hỏi mở về phạm vi duy nhất của `code` giữa các draft và toàn DB, về hành vi UI khi workflow giữ khóa ghi draft, và về contract persistence, crash recovery, idempotent sealing.
* Rút gọn câu hỏi Q3 về bộ đếm cấp `code`, tránh hỏi lại quy tắc đã chốt trong State.
* Đồng bộ ngoại lệ `lookup_outline`, quy tắc `outline_truncated`, phân loại tool ghi Draft State và xử lý seal failure trong `overview.md` và `case_newchat.md`.
* Cập nhật header lên phiên bản 2.2, đồng bộ `tools.md`.

## CHANGELOG v2.3

* Chốt mô hình dữ liệu phẳng cho `character_competences` và `character_memories`: giữ `scene_id NOT NULL`, `source_scene_id` nullable, cho phép bản ghi trùng nội dung giữa các cảnh, không thêm bảng trung gian và không unique theo nội dung.
* Chốt code collision: handler cấp code cuối cùng; code duy nhất trong draft; DB unique theo từng bảng; cho phép hai draft tạm trùng code; commit recheck trong transaction, rollback và giữ draft khi collision; tái sử dụng nhân vật thư viện qua `project_characters`.
* Chốt khóa ghi và hủy workflow: một writer tại một thời điểm; `DRAFT_LOCK_TIMEOUT` là thời gian chờ lấy khóa; hủy có kiểm soát, chờ mutation đang chạy, seal vòng có thay đổi, không rollback `applied`, giữ resume state.
* Chốt persistence và idempotent sealing: Working State khác sealed version; `applied` không phải durable; seal retry một lần; lỗi `VersionSealFailed`; phục hồi từ sealed version cuối; audit chỉ để phát hiện gián đoạn; thêm `[SCHEMA FOLLOW-UP]` cho khóa idempotency.
* Chốt Cascade Delete trong Working State: ma trận Arc/Chapter/Scene, liên kết scene, competence, memory; bảo vệ source; cascade nguyên tử; giữ các quan hệ chưa chốt trong Q1.
* Tách Planning Mode và Writing Mode tại mục 1.5: Planning được lập kế hoạch nhiều chương nhưng không sinh `scenes.content`; Writing chỉ sau plan commit, một chương, không viết hàng loạt, không tự chuyển chương.
* Chốt Start bị chặn khi còn `pending_patches`: không tự áp dụng, không tự từ chối, không bỏ qua; UI yêu cầu xử lý patch trước khi Start.
* Chốt contract resume workflow tại mục 3.9: resume từ sealed version cuối; workflow/round/task có định danh ổn định; không phát lại mutation audit; revalidate; không dựa vào trùng tên; giữ Q3 cho cấu trúc task/checkpoint.
* Đồng bộ `tools.md`, `overview.md` và `case_newchat.md` với v2.3; giữ nguyên `lookup_outline`, `outline_truncated`, `declared_sources`, `declared_source`, `pending_confirmation`.
* Không sửa DBML, `src`, backend hoặc migration; các giới hạn schema được ghi thành `[SCHEMA FOLLOW-UP]` và câu hỏi mở.

## CHANGELOG v2.4

* Chốt H1 tại mục 1.6: mô hình chương-first, cảnh là block; đọc/ghi cấp chương, sửa/generate/lock/version cấp scene; scene identity là `scenes.id`, thứ tự theo `scenes.position`; không có scene page, block paragraph hoặc `chapter_versions`.
* Chốt vòng đời Writing Mode: generation vào buffer tạm, review inline, Accept vào working state, Save cấp chương mới persist; generation không ghi thẳng `scenes.content`; tối đa một generation mỗi chương; không tự chuyển scene/chương.
* Chốt Save và version nội dung: chỉ scene dirty; một Save thành công tạo đúng một `scene_versions` cho mỗi scene có nội dung thay đổi; lock-only Save không tạo content version; không có version cấp chương; thất bại không báo thành công, không xóa dirty, không ghi một phần.
* Chốt source nội dung cảnh (`AI_GENERATED`, `HUMAN_EDITED`, `RESTORE`), restore append-only và dirty/lock/continuity theo scene; tách source này khỏi source cấp draft.
* Chốt H2 tại mục 3.5 và 3.7: Commit Confirmation Dialog đồng thời là plan review; không thêm phase review; chặn commit khi validation/pending patches; pending không thể đóng; idempotent; failure giữ draft; success đóng Draft Workspace, điều hướng Project Detail mặc định View.
* Phân biệt `phase_confirm` (version xác nhận phase planning) với `commit` (chuyển plan thành dữ liệu quan hệ bền vững); giữ nguyên transaction, validate, collision, rollback và idempotency tại mục 3.7.
* Chốt H3 ở tầng hợp đồng: Project Detail có View/Creator; Creator chỉ chọn chương, không nhúng scene editor; Chapter Detail là Writing Mode surface; route/URL và interaction chi tiết tại `fe-core.md` §9.
* Ghi các điểm kỹ thuật/schema còn mở tại Q4: idempotency Save, stale write, source hiện tại của scene, audit lock, read-model scene identity, generation stream/cancel, commit retry/recovery; không tự thêm cột/bảng và không sửa DBML.
* Ghi nhận `tools.md`, `overview.md`, `case_newchat.md` cần đồng bộ ở task riêng nếu còn mô tả H1/H2/H3 là mở; v2.4 chỉ sửa `core.md` và `fe-core.md`.

## CÂU HỎI MỞ

> Các câu hỏi số 1-4 của v2.2 đã được chốt và chuyển thành quy tắc tại mục 1.3, 2.6, 3.6, 3.7, 3.8 và 3.9. Q2 của nhóm câu hỏi mở rộng cũng đã được chốt. v2.4 chốt H1/H2/H3; các chi tiết kỹ thuật/schema còn mở của H1 nằm tại Q4.

### Q1. Cascade cho các quan hệ chưa chốt

**Bối cảnh hiện tại:**

`core.md` mục 1.3 đã chốt ma trận Cascade Delete cho các quan hệ sở hữu và tham chiếu trực tiếp của Arc, Chapter, Scene, `scene_characters`, `scene_lore_entries`, `character_competences` và `character_memories`. Mục 2.8 quy định một số bất biến và validate chéo; mục 3.7 validate toàn bộ JSON trước khi commit. DBML vẫn có các ràng buộc chưa được ma trận tại mục 1.3 bao phủ: `scenes.world_setting_id` là `not null`; `scene_lore_entries` tham chiếu `lore_entries`; `relationship_states.scene_id` tham chiếu cảnh; `theme_arguments.character_id` tham chiếu nhân vật; một số thực thể có thể dùng chung giữa nhiều project.

**Đã chốt:**

* Phân biệt quan hệ sở hữu và quan hệ tham chiếu.
* Ma trận tại mục 1.3 cho Arc → Chapter → Scene, liên kết scene, competence và memory.
* Cascade có tính bắc cầu; kiểm tra source; bảo vệ `user`/`user_edited`; preview đầy đủ; revalidate; áp dụng nguyên tử; một version `ai_patch` cho toàn cascade.
* Không tự suy rộng ma trận sang mọi foreign key.

**Vấn đề chưa chốt:**

1. Xóa `world_setting` đang được Scene tham chiếu qua `scenes.world_setting_id NOT NULL`: trả `rejected`, `pending_confirmation`, hay yêu cầu gán địa điểm khác trước?
2. Xóa `lore_entry` đang có trong `scene_lore_entries`: xóa liên kết, trả `pending_confirmation`, hay chặn?
3. Xóa Character đang được `theme_arguments` hoặc `relationships` tham chiếu: xử lý ra sao?
4. Xóa Scene đang được `relationship_states.scene_id` tham chiếu: cascade state, đặt null, hay chặn?
5. Xóa thực thể đang được nhiều project dùng chung: có cascade theo project hay giữ lại?
6. Xóa trong draft là xóa khỏi JSON, tombstone hay soft delete; ảnh hưởng thế nào đến version và `pending_patches`?
7. Các bất biến xuyên thực thể được kiểm tra ở cấp tool call, cuối lượt, cuối vòng workflow hay chỉ khi commit?

**Tình huống phát sinh:**

Người dùng xóa một `world_setting` ở Phase 3. Trong draft, Scene ở Phase 4 đang trỏ tới nó. Nếu handler cho xóa, đến commit `scenes.world_setting_id` là `NOT NULL` và transaction thất bại. Hoặc người dùng xóa một `lore_entry` đang được Scene dùng; nếu cascade xóa liên kết mà không có preview, dữ liệu người dùng có thể mất. Với `relationship_states`, xóa Scene có thể làm mất trạng thái quan hệ theo cảnh nếu không có quy tắc rõ ràng.

**Câu hỏi cần quyết định:**

1. Ma trận xử lý cho các quan hệ còn lại là gì, theo thứ tự nào?
2. Trường hợp nào `rejected`, trường hợp nào `pending_confirmation`, trường hợp nào cascade tự động?
3. Xóa trong draft là xóa khỏi JSON, tombstone hay soft delete?
4. Các bất biến xuyên thực thể được kiểm tra ở đâu và khi nào?

Các hướng như khai báo ma trận tham chiếu, validate tại handler, hay chỉ validate trước commit kèm preview đầy đủ chỉ là **phương án tham khảo, chưa được phê duyệt**.

**Phạm vi ảnh hưởng:**

Tool `delete_*`, handler, validate mục 2.8, commit tại mục 3.7, `pending_patches`, version/restore, DBML foreign key và `tools.md`.

**Trạng thái:** Chưa chốt.

### Q3. Resume workflow, task identity và checkpoint

**Bối cảnh hiện tại:**

`core.md` mục 3.9 đã chốt các nguyên tắc resume: dựa trên sealed version cuối cùng; workflow/round/task có định danh ổn định; task trong round đã seal không chạy lại; round chưa seal chưa hoàn thành; không phát lại mutation từ audit; revalidate State; không dựa vào trùng tên; không tạo bản ghi trùng; subagent chạy song song nhưng mutation tuân thủ khóa ghi. Mục 3.6 chốt idempotent sealing; mục 3.8 chốt bộ đếm `code` thuộc Working State.

**Đã chốt một phần:**

State là một bản duy nhất theo draft; các subagent của một vòng chạy dưới khóa của vòng và tool call của chúng được tuần tự hóa ở State. Các ý còn lại dưới đây vẫn chưa chốt.

**Vấn đề chưa chốt:**

Chưa xác định task được định danh bằng gì và trạng thái hoàn thành được lưu ở đâu; kế hoạch task được tính lại từ `plan_phase`/`plan_outline_runs` nên ranh giới task có thể thay đổi sau khi State thay đổi. Chưa có cơ chế chống tạo thực thể tương đương khi resume cho Arc, chương, cảnh, nhân vật, world_setting và lore; các khóa vị trí như `(parent, position)` chỉ phát hiện xung đột vị trí, không xác định được hai thực thể cùng ý nghĩa. Chưa rõ report và State lệch nhau thì ưu tiên bên nào. Hai subagent trong cùng vòng có thể cùng tạo world_setting hoặc lore trùng. Thứ tự áp dụng và xử lý xung đột khi các subagent cùng ghi vào State chung vẫn chưa xác định; cũng chưa rõ bộ đếm `code` theo cha nằm ở đâu trong snapshot để restore khôi phục đúng.

**Tình huống phát sinh:**

Subagent tạo nhân vật `code = char_001`, tool trả `applied`, rồi task lỗi trước khi hoàn tất. Khi resume, model tạo một nhân vật tương đương với `code = char_002`. Cả hai đều hợp lệ về schema; không có khóa tự nhiên nào nhận ra chúng là cùng một nhân vật. Tương tự, hai subagent trong cùng vòng có thể cùng tạo một `world_setting` hoặc `lore_entry` trùng nghĩa nhưng khác `code`.

**Câu hỏi cần quyết định:**

1. Task được định danh và đánh dấu hoàn thành bằng gì khi kế hoạch được tính lại sau thay đổi State?
2. Trạng thái task/checkpoint được lưu ở đâu nếu không thêm task ledger? Có cần schema tối thiểu hay chỉ dùng report/State?
3. Khi report và State lệch nhau, bên nào được ưu tiên để resume?
4. Hai subagent cùng tạo world_setting hoặc lore trùng trong một vòng được phát hiện và xử lý thế nào?
5. Thứ tự áp dụng và xử lý xung đột khi các subagent cùng ghi vào State chung do đâu quy định?
6. Cấu trúc serialization của bộ đếm cấp `code` trong `BusinessFlowData` được đặc tả ở đâu để bảo đảm snapshot, restore và resume sử dụng cùng một trạng thái cấp phát?

Các hướng như khóa task suy ra từ kế hoạch, report có schema cố định, hay đối chiếu State trước khi chạy lại chỉ là **phương án tham khảo, chưa được phê duyệt**.

**Phạm vi ảnh hưởng:**

Workflow orchestrator, `plan_phase`/`plan_outline_runs`, report trung gian, State store, version `workflow_round`, handler cấp `code`, restore, `tools.md`, và quy tắc resume tại mục 3.9.

**Trạng thái:** Chưa chốt.

### Q4. Hợp đồng lưu nội dung cảnh và schema follow-up

**Đã chốt (APPROVED):**

* Mô hình chương-first, cảnh là block; scene identity là `scenes.id`, thứ tự theo `scenes.position`; không có scene page, block paragraph hoặc `chapter_versions` (mục 1.6).
* Generation vào buffer tạm, review inline, Accept vào working state; Save cấp chương mới persist; tối đa một generation mỗi chương; không tự chuyển scene/chương.
* Save chỉ gồm scene dirty; một Save thành công tạo đúng một `scene_versions` cho mỗi scene có nội dung thay đổi; lock-only Save không tạo content version.
* Source nội dung cảnh dùng `AI_GENERATED` / `HUMAN_EDITED` / `RESTORE`; restore append-only; dirty và continuity theo scene.
* H2 dùng Commit Confirmation Dialog, không có phase review riêng; `phase_confirm` khác `commit`; failure giữ draft, success điều hướng Project Detail mặc định View (mục 3.5, 3.7).
* H3 dùng Project View/Creator; Creator chỉ chọn chương, Chapter Detail là Writing Mode surface; chi tiết UI tại `fe-core.md` §9.

**Còn mở (OPEN):**

1. Idempotency của Save nội dung cảnh: retry/double-submit không được tạo hai version hoặc ghi hai lần. Chưa chốt khóa idempotency ở tầng persistence.
2. Optimistic concurrency/stale write: FE gửi baseline nào, backend từ chối ghi đè khi scene đã đổi ở nơi khác, và UI xử lý conflict ra sao.
3. Source hiện tại của scene: DBML chỉ có `scene_versions.scene_source`, chưa có cột source trên `scenes`; cần chốt suy ra từ version mới nhất hay lưu riêng.
4. Audit lock: lock-only Save không tạo content version; nếu cần lịch sử lock thì phải có schema/contract riêng.
5. Read-model scene identity: backend trả `scenes.id`, `position`, `content`, `word_count`, `is_locked`; FE preview hiện dùng id tạm.
6. Generation stream/cancel: contract API, buffer và cách hủy; chưa có tool sinh nội dung scene.
7. Commit retry/recovery sau request bị gián đoạn: UI đọc trạng thái backend thay vì tự kết luận; chi tiết API còn mở.
8. `tools.md` và prompt docs còn mô tả H1/H2/H3 là mở; cần đồng bộ ở task riêng.

**SCHEMA FOLLOW-UP:**

* `scene_versions` chưa có khóa idempotency hoặc mã tham chiếu generation; unique hiện tại chỉ `(parent_id, version_number)`.
* `scene_versions.parent_id` nullable; contract nội dung cảnh cần gắn scene, nên cần validate hoặc điều chỉnh DBML nếu muốn cưỡng chế.
* `scenes` không có `created_at`/`updated_at` và không có cột source nội dung; optimistic concurrency và source hiện tại chưa có chỗ lưu chuẩn.
* Không có `chapter_versions`; v2.4 không thêm bảng này.
* Không tự thêm cột/bảng trong v2.4; tên cột/index cụ thể do DBML và backend chốt sau.

**IMPLEMENTATION PENDING:**

* API generation/stream/cancel; API Save nguyên tử và trả read-model; API version list/restore; persist lock; continuity; commit confirmation/navigation; cập nhật `tools.md`.

**Trạng thái:** Business contract của H1/H2/H3 đã chốt; các chi tiết kỹ thuật và schema nêu trên còn OPEN/SCHEMA FOLLOW-UP.

### Đã chốt trong v2.4

H1. **[APPROVED]** Hợp đồng Writing Mode tại mục 1.6: chương-first, cảnh là block; generation buffer/review/accept; Save cấp chương; source/restore/lock/dirty/continuity theo scene; không có `chapter_versions`. Chi tiết kỹ thuật còn mở tại Q4.
H2. **[APPROVED]** Luồng commit sau Phase 6 tại mục 3.5 và 3.7: Commit Confirmation Dialog đồng thời là plan review; chặn khi validation/pending patches; pending không đóng được; idempotent; failure giữ draft; success đóng Draft Workspace và điều hướng Project Detail mặc định View. Triển khai FE/backend còn `[IMPLEMENTATION PENDING]`.
H3. **[APPROVED]** Project View/Creator tại `fe-core.md` §9: Creator chỉ chọn chương, không nhúng scene editor; Chapter Detail là Writing Mode surface; route và interaction chi tiết thuộc FE contract, không phải quy tắc nghiệp vụ mới.

**Tạm treo theo xác nhận:** Chưa thiết kế bảng/liên kết cho `relationship_phase.motif.event_blocks[]`.
