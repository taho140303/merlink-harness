# NGHIỆP VỤ CỐT LÕI: HỆ THỐNG PHÂN KỲ SÁNG TÁC LŨY TIẾN (PROGRESSIVE MULTI-PHASE BUSINESS FLOW)

> **Tài liệu này là SSOT (Single Source of Truth) của toàn bộ quy tắc nghiệp vụ Merlink Harness. Khi `tools.md`, `overview.md`, `case_newchat.md` hoặc bất kỳ tài liệu nào khác mâu thuẫn với core về nghiệp vụ, core thắng. Schema vật lý của bảng vẫn theo `docs/db.dbml`.**
> **Tài liệu tham chiếu chuẩn cho kiến trúc nghiệp vụ và lập kế hoạch kỹ thuật (Planning)**
> **Phiên bản:** 2.4.5
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
* Thông tin chat mới về trường `user`/`user_edited` được bóc tách và ghi ngay vào `businessFlowData.pending_patches` (góc nhìn nghiệp vụ; nơi lưu vật lý là bảng SQLite `pending_patches`, **không** nằm trong JSON sealed `BusinessFlowData`); giá trị đang có trong State chưa đổi cho đến khi người dùng duyệt.
* Patch hỗn hợp được tách theo từng trường: trường chạm `ai_generated` áp dụng ngay; mỗi trường chạm `user`/`user_edited` tạo đúng một patch `pending_confirmation` và chưa tạo version. Khi người dùng xác nhận, áp dụng patch và trường được xác nhận chuyển thành `user_edited`; mỗi thao tác xác nhận tạo đúng một version `ai_patch` nếu có ít nhất một patch được áp dụng. Khi người dùng từ chối, bỏ patch và ghi sự kiện từ chối vào event log, không tạo version.
* Patch hết hiệu lực khi `target_code` không còn tồn tại hoặc giá trị hiện tại của `field_name` khác `previous_value`; với `operation = delete`, patch hết hiệu lực khi phần tử đích không còn hoặc tập hậu duệ bị ảnh hưởng đã khác lúc đề xuất. Kiểm tra lười tại thời điểm hiển thị preview và lúc confirm; `base_version_seq` chỉ dùng cho event log. Restore làm mọi patch `pending` thành `expired` theo quy tắc hiện có. Khi target đã bị xóa hoặc đổi cấu trúc, patch chuyển `expired` (không phải `rejected`); UI hiển thị "patch hết hiệu lực"; AI chỉ tạo preview mới khi người dùng nhắc lại ý định đó.
* Tại một thời điểm chỉ tồn tại tối đa một patch `pending` cho mỗi `(draft_session_id, target_code, field_name)`. Khi có patch mới cùng đích và `field_name` với `new_value` khác, patch cũ chuyển `expired` và patch mới được tạo trong cùng một bước dưới khóa ghi draft; handler cưỡng chế, không thêm ràng buộc unique.
* Khi có nhiều patch chờ duyệt, UI hỗ trợ xác nhận từng patch và nút **Áp dụng tất cả**. Mỗi thao tác chạy dưới khóa ghi draft; **Áp dụng tất cả** duyệt patch `pending` theo `created_at` tăng dần, revalidate dữ liệu và điều kiện đủ tư cách của từng patch trên candidate tích lũy sau các patch trước đó, bỏ qua patch lỗi (không rollback cả batch), trả danh sách `applied` và `skipped`, và chỉ tạo một version `ai_patch` khi có ít nhất một patch được áp dụng.
* Khi xóa cha, kiểm tra toàn bộ hậu duệ bị ảnh hưởng theo ma trận Cascade Delete bên dưới. Chỉ một hậu duệ có `source = user` hoặc `user_edited` cũng khiến tool trả `pending_confirmation`. Các quan hệ tham chiếu còn lại (`world_setting`, `lore_entry`, `character`, `relationship_states`) tuân thủ đầy đủ ma trận đã chốt bên dưới (v2.4.4).
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
* Ma trận đã chốt cho Working State (gồm cả `Q1` — APPROVED v2.4.4):

| Thực thể bị xóa | Dữ liệu liên quan | Hành vi |
| :--- | :--- | :--- |
| Arc | Chapters thuộc Arc | Cascade (bắc cầu xuống Scenes và các phụ thuộc của Scene) |
| Chapter | Scenes thuộc Chapter | Cascade (bắc cầu xuống các phụ thuộc của Scene) |
| Scene | `scene_characters` | Cascade xóa liên kết |
| Scene | `scene_lore_entries` | Cascade xóa liên kết |
| Scene | `character_competences` có `scene_id` trỏ đến scene | Cascade xóa bản ghi năng lực |
| Scene | `character_memories` có `source_scene_id` trỏ đến scene | Giữ bản ghi, đặt `source_scene_id = null` |
| Scene | `relationship_states` có `scene_id` trỏ đến scene | Cascade xóa bản ghi trạng thái quan hệ thuộc cảnh đó; giữ nguyên thực thể `relationships` cha |
| Scene | Characters, Lore entries, World settings được tham chiếu | Giữ nguyên |
| World Setting (hoặc cây con World Setting) | Scenes đang tham chiếu qua `scenes.world_setting_id` (`world_setting_code`) tới nút đó hoặc bất kỳ hậu duệ nào trong cây con | **Chặn xóa:** trả `status = rejected`, `error = referenced_entity_in_use` (kèm danh sách `scene_code` đang tham chiếu). Không tự xóa hoặc tự gán lại địa điểm cho cảnh; người dùng phải chuyển địa điểm cho cảnh trước. Nếu không có cảnh nào tham chiếu, cho phép xóa cây con và cascade các `lore_entries` gắn trực tiếp vào cây con đó (sau khi kiểm tra provenance). |
| Lore Entry | Liên kết `scene_lore_entries` trỏ đến Lore Entry | Cho phép cascade xóa các liên kết `scene_lore_entries` phụ thuộc (không xóa Scene). Phải liệt kê toàn bộ liên kết bị ảnh hưởng trong preview và kiểm tra `source` bảo vệ của cả Lore Entry lẫn từng liên kết trước khi áp dụng nguyên tử. |
| Character | Thực thể `relationships` (`from_character_id` / `to_character_id`) | **Chặn xóa:** trả `status = rejected`, `error = referenced_entity_in_use`. Không tự động xóa quan hệ; người dùng phải dọn dẹp quan hệ trước. |
| Character | Liên kết `theme_arguments` trỏ đến Character | Chỉ cho phép xóa kèm liên kết `theme_arguments` sau khi phân tích phụ thuộc, kiểm tra `source` bảo vệ và kiểm tra bất biến của `themes` (ví dụ không làm mất luận điểm bắt buộc khiến chủ đề không hợp lệ); nếu vi phạm bất biến thì trả `rejected` yêu cầu xử lý chủ đề trước. Tuyệt đối không tự sửa nội dung triết lý hoặc tự gán nhân vật thay thế. |
| Thực thể dùng chung (Shared Entities) | Nhân vật thư viện (`project_characters` ở project khác), template `world_settings` (`project_id IS NULL`), `lore_entries` đang dùng ở project khác | **Cấm cascade xuyên ranh giới project.** Xóa trong một project/draft chỉ gỡ liên kết trong phạm vi project/draft đó; bảo toàn nguyên vẹn thực thể gốc nếu còn được project khác tham chiếu. *(Lưu ý schema: trong `docs/db.dbml`, `lore_entries` hiện gắn qua `world_setting_id` mà chưa có `project_id` hay bảng nối `project_lore_entries` riêng, nên chia sẻ lore liên dự án chỉ thực hiện khi đi qua template `world_settings` dùng chung).* |

* **Mô hình xóa trong Draft (Draft Deletion Model — APPROVED v2.4.4):** Xóa phần tử trong draft là **xóa vật lý (physical removal)** khỏi `WorkingState` (`BusinessFlowData`). Không dùng tombstone hoặc soft-delete trong JSON nháp cho V1; việc khôi phục lịch sử đã được bảo đảm đầy đủ qua các snapshot bất biến trong `draft_versions`. Khi phần tử bị xóa, mọi `pending_patches` đang chờ trên phần tử đó hoặc hậu duệ của nó tự động chuyển sang `expired`.
* **Thời điểm kiểm tra bất biến (Invariant Validation Timing — APPROVED v2.4.4):** Kiểm tra kiểu, khóa tham chiếu tồn tại trong draft và bất biến cấp thực thể/liên kết trực tiếp ngay tại mỗi tool call (trả `rejected` nếu vi phạm); kiểm tra lại toàn bộ bất biến xuyên thực thể và định lượng toàn truyện trước khi `phase_confirm` và bắt buộc trước khi `commit` (mục 2.8, 3.7).
* **9 bước bảo vệ bắt buộc trước mọi thao tác xóa (Required Safeguards — APPROVED v2.4.4):**
  1. Dựng tập phụ thuộc đầy đủ (complete dependency set), bao gồm toàn bộ cây con và liên kết bắc cầu.
  2. Phân biệt rõ quan hệ sở hữu (ownership — được phép cascade) và quan hệ tham chiếu bắt buộc (reference — phải chặn `referenced_entity_in_use`).
  3. Kiểm tra `source` của mọi bản ghi bị xóa và mọi trường bị sửa đổi (kể cả thao tác đặt `character_memories.source_scene_id = null` hay gỡ liên kết `scene_lore_entries`, `relationship_states`, `theme_arguments`).
  4. Kiểm tra hệ quả khóa ngoại (foreign-key implications) đối chiếu với `docs/db.dbml`.
  5. Kiểm tra các bất biến nghiệp vụ bị ảnh hưởng.
  6. Nếu bất kỳ phần tử đích, hậu duệ hoặc liên kết bị ảnh hưởng nào có `source = user` hoặc `user_edited` (hoặc AI đề xuất xóa thực thể do người dùng tạo), trả `pending_confirmation` kèm preview liệt kê đầy đủ (tính động từ State hiện tại).
  7. Revalidate toàn bộ điều kiện ngay trước thời điểm áp dụng thực tế (nếu tập phụ thuộc hoặc trạng thái đã đổi, chuyển patch sang `expired`).
  8. Áp dụng toàn bộ thao tác xóa được duyệt một cách nguyên tử (atomic) dưới khóa ghi draft.
  9. Niêm phong version theo đúng quy tắc tại mục 3.6 (tạo đúng một version `ai_patch` khi xác nhận patch hoặc cuối lượt AI có thay đổi `applied`, không tạo version rời rạc cho từng hậu duệ).

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
* Chi tiết hợp đồng thực thi Writing Mode nằm tại mục 1.6 và mục Q4 (đã chốt toàn bộ quy tắc nghiệp vụ ở v2.4.4). Tài liệu này không thiết kế pipeline triển khai cụ thể.

### 1.6 Hợp đồng Writing Mode: chương-first, cảnh là block

> **Trạng thái:** APPROVED (v2.4..v2.4.4). Đây là business contract đã chốt đầy đủ; chi tiết giao diện và route thuộc `fe-core.md` §9; các đề xuất bổ sung cột schema tối thiểu nằm tại `[SCHEMA FOLLOW-UP]` (chờ phê duyệt riêng trước khi sửa `docs/db.dbml`).

#### Mô hình chương và cảnh và Read-Model Identity (`Q4.5` — APPROVED v2.4.4)

* Một chương là một dãy cảnh có thứ tự. Trong FE, mỗi block nội dung đại diện đúng một scene; block không phải paragraph và không gộp nhiều scene.
* `scenes.content` là nội dung của một scene; `chapters` không có cột nội dung chương. Nội dung đọc được của chương là các scene thuộc chương, sắp theo `scenes.position`.
* Đọc và ghi ở cấp chương; sửa nội dung, generate, lock và version ở cấp scene.
* Sau commit, scene identity là `scenes.id` (UUIDv7 thật trong cơ sở dữ liệu). Thứ tự hiển thị lấy từ `scenes.position`; array index, thứ tự block trong FE và id tạm không phải identity bền vững.
* Không có scene page riêng, không có mô hình block paragraph, không có `chapter_versions`.
* `sceneId` tạm trong FE preview (`scn-{chapter}-{block}`) chỉ dùng cho bản xem trước; không được ghi vào DB, không được coi là `scenes.id`.
* **Read-model scene identity (`Q4.5` — APPROVED v2.4.4):** Backend read-model bắt buộc trả về định danh và trạng thái thực từ cơ sở dữ liệu cho từng cảnh thuộc chương đang mở, bao gồm tối thiểu: `id` (`scenes.id`), `chapter_id`, `position`, `display_name`, `summary`, `content` (baseline), `word_count`, `is_locked` (trạng thái khóa bền vững hiện tại), `head_version_number` (`COALESCE(MAX(scene_versions.version_number), 0)` dùng làm baseline OCC và hiển thị version), và `current_scene_source` (suy ra từ bản ghi `scene_versions` mới nhất theo `Q4.3`, hoặc `null` nếu cảnh chưa có version nào). Tập trường này cung cấp đủ thông tin cho: hiển thị (rendering), trạng thái khóa, so sánh dirty baseline, kiểm tra OCC, hiển thị nguồn gốc (source badge), lịch sử version và Save. (**[IMPLEMENTATION PENDING]** IPC endpoint đọc chương cho FE).

#### Điều kiện mở Writing Mode

* Chỉ mở sau khi plan đã commit hợp lệ.
* Một phiên Writing Mode chỉ làm việc với một chương đã chọn.
* Không tự chuyển chương, không generate nhiều chương trong một yêu cầu, không chạy background bulk, không gọi subagent viết song song nhiều chương.
* Phải xác định được chương thuộc project đã commit và scene thuộc chương đang chọn.

#### Vòng đời sinh nội dung cảnh và Streaming Contract (`Q4.6` — APPROVED v2.4.4)

```text
Generate / Regenerate
        ↓
started → delta* → completed | cancelled | failed
        ↓
Buffer tạm của scene (kèm generation metadata)
        ↓
Review inline
        ├── Discard → giữ nguyên nội dung đã có
        └── Accept (chỉ khi generation hợp lệ & scene đang mở khóa)
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
* **Chu trình streaming khái niệm (`Q4.6` — APPROVED v2.4.4):** Một lần sinh nội dung cảnh đi qua các trạng thái `started -> delta* -> completed | cancelled | failed`. Đầu ra chỉ là tạm thời trong buffer cho đến khi người dùng bấm Accept.
* **Generation metadata bắt buộc (`Q4.6` — APPROVED v2.4.4):** Mỗi phiên sinh nội dung cảnh phải gắn metadata phân biệt rõ:
  1. `request_id`: định danh AI request (gắn với `EditScope`).
  2. `generation_id`: định danh duy nhất của lần sinh cảnh đó.
  3. `scene_id`: định danh `scenes.id` đích.
  4. `baseline_version_number`: số version nội dung cơ sở của cảnh lúc bắt đầu sinh.
  5. `authorization_valid`: cờ xác nhận cảnh đang mở khóa và thuộc `EditScope` hợp lệ trong suốt quá trình sinh.
  6. `status`: `streaming | completed | cancelled | invalidated_by_lock | failed`.
* **Vô hiệu hóa khi Lock giữa chừng (`LOCK-04`, `Q4.6`):** Nếu người dùng bấm Lock cảnh trong khi đang streaming (`started`/`delta*`), thao tác khóa có hiệu lực ngay lập tức và luồng sinh bị hủy/đánh dấu `invalidated_by_lock` (`authorization_valid = false`). Phần văn bản tạm đã hiển thị vẫn được giữ trên màn hình để người dùng xem tham khảo, nhưng **tuyệt đối không thể bấm Accept** — kể cả sau đó người dùng có bấm Unlock lại cảnh đó (một generation đã bị vô hiệu hóa bởi Lock vẫn giữ trạng thái không thể Accept sau khi Unlock; muốn có nội dung AI mới phải bấm Generate/Regenerate lại). Ngược lại, một generation đã hoàn tất hợp lệ (`completed` và `authorization_valid = true`) không bị vô hiệu hóa chỉ vì `EditScope` của request hết hạn (`SCOPE-02`).
* Accept chỉ đưa nội dung từ một generation hợp lệ (`completed` và `authorization_valid == true`) vào working state khi cảnh hiện đang mở khóa (`is_locked == false`), và đánh dấu dirty; Accept không persist.
* Discard và Cancel không thay đổi nội dung đã lưu.
* Tối đa một generation đang hoạt động trong một chương.
* Không tự generate scene kế tiếp, không tự chuyển chương.
* Phải review trước khi Accept.
* **Regenerate note (APPROVED v2.4.1):** Regenerate hỗ trợ ghi chú tùy chọn từ người dùng. Ghi chú rỗng hoặc chỉ khoảng trắng được chuẩn hóa thành `null`. Ghi chú khác `null` chỉ áp dụng cho regeneration request hiện tại, cung cấp hướng dẫn về thay đổi mong muốn ở scene đã chọn. Ghi chú không trực tiếp sửa nội dung scene, không được persist như ràng buộc dự án vĩnh viễn và không bỏ qua vòng Review → Accept → Save.
* **[IMPLEMENTATION PENDING]** backend generation stream/cancel và buffer contract.

#### Save cấp chương, OCC, Idempotency và Retry Recovery (`Q4.1`, `Q4.2`, `Q4.4`, `Q4.7` — APPROVED v2.4.4)

* Save là hành động cấp chương do người dùng điều khiển; chỉ các scene có thay đổi nội dung chưa lưu (dirty content) được đưa vào Save. (Trạng thái khóa `scenes.is_locked` đã được persist ngay khi bấm Lock/Unlock theo `LOCK-01`, không chờ Save chương).
* Backend tương lai phải thực hiện Save nguyên tử trong một SQLite transaction (`BEGIN IMMEDIATE`) (`LOCK-05`, `SCOPE-05`, `Q4.2`). Save không yêu cầu còn một AI request đang hoạt động và không yêu cầu kích hoạt lại `EditScope` đã hết hạn.
* **Kiểm tra Optimistic Concurrency (OCC) trên lịch sử version hiện có (`Q4.2` — APPROVED v2.4.4):** Sử dụng lịch sử `scene_versions` hiện có làm chuẩn OCC mà không cần thêm cột `scenes.revision` cho V1. Với mỗi scene có nội dung dirty trong yêu cầu Save, bên trong cùng một transaction nguyên tử, backend bắt buộc xác minh:
  1. Danh tính thực `scenes.id` và quyền sở hữu đúng `chapter_id` thuộc project đã commit.
  2. Quyền thực thi hợp lệ của người dùng (user authorization).
  3. Trạng thái khóa bền vững hiện tại trong DB phải đang mở (`scenes.is_locked == false`).
  4. Phiên bản nội dung kỳ vọng khớp với DB: `expected_version_number == COALESCE(MAX(scene_versions.version_number), 0)`. (Vì `scene_versions` là append-only, mọi lần lưu nội dung — kể cả sửa từ A sang B rồi sửa ngược về A — đều làm tăng `MAX(version_number)` thêm 1, nên chống ghi đè cũ chính xác mà không bị lỗi ABA nội dung. Việc người dùng bấm `Lock -> Unlock` thuần túy không đổi nội dung nên không tạo xung đột OCC nội dung; riêng generation bị hủy bởi Lock đã bị chặn từ bước Accept theo `Q4.6`).
  5. Tính hợp lệ của nội dung và `word_count`.
* Một Save thành công:
  * ghi nội dung cuối cùng đã Accept/chỉnh sửa của các scene có nội dung dirty vào `scenes.content`;
  * cập nhật `scenes.word_count` tương ứng;
  * tạo đúng một bản ghi `scene_versions` mới (`version_number = MAX + 1`) cho mỗi scene có nội dung thay đổi;
  * ghi nhận kết quả bền vững gắn với `client_save_id` của yêu cầu Save;
  * trả về trạng thái đã persist (`head_version_number`, `word_count`, `current_scene_source`, `is_locked`) để FE cập nhật baseline.
* Nếu bất kỳ scene nào có thay đổi nội dung đang ở trạng thái `is_locked == true`, hoặc không khớp `expected_version_number` (stale write / OCC conflict), hoặc không hợp lệ: toàn bộ Save cấp chương bị từ chối nguyên tử (`LOCK-05`, `Q4.2`) — rollback toàn bộ transaction, không ghi một phần, không xóa dirty, không tạo version, và bảo toàn nguyên vẹn toàn bộ nội dung dirty để người dùng xử lý.
* Một scene đang `is_locked == true` nhưng không có thay đổi nội dung chờ lưu (clean) **không** chặn việc Save các scene đang mở khóa khác trong cùng chương (`LOCK-05`) và không tạo `scene_versions`.
* **Idempotency bền vững ở cấp request cho Save Chapter (`Q4.1` — APPROVED v2.4.4):** Mỗi thao tác bấm Save của người dùng mang một định danh ổn định `client_save_id` (tạo mới cho mỗi lần người dùng chủ động lưu một tập thay đổi mới; giữ nguyên khi retry cùng một lần lưu đó). Hệ thống bắt buộc bảo đảm 5 tính chất bền vững (kể cả khi ứng dụng crash ngay sau khi SQLite commit nhưng trước khi trả phản hồi cho UI):
  1. *Cùng `client_save_id`, cùng payload:* trả về nguyên vẹn kết quả đã commit ban đầu, tuyệt đối không tạo thêm `scene_versions` lần thứ hai.
  2. *Cùng `client_save_id`, khác payload:* từ chối với lỗi `idempotency_conflict`.
  3. *Khác `client_save_id`, Save mới hợp lệ:* xử lý bình thường dưới kiểm tra OCC (`expected_version_number`).
  4. *Crash sau khi DB commit nhưng trước khi trả response:* khi retry với cùng `client_save_id`, phát hiện bản ghi đã commit và trả lại kết quả đó mà không nhân đôi version.
  5. *Retry của một Save cũ đến trễ sau khi đã có một Save khác thành công:* nếu Save cũ chưa từng được commit trước đó thì `expected_version_number` đã cũ nên bị từ chối bởi OCC (không bao giờ ghi đè làm mất nội dung mới hơn); nếu Save cũ thực chất đã commit trước đó thì trả lại biên nhận lịch sử của lần commit đó kèm trạng thái hiện hành mà không ghi đè ngược vào `scenes.content`.
  * *Lưu trữ Schema đã phê duyệt (`[SCHEMA APPROVED: SF-02]` — v2.4.5):* Bảng `chapter_save_receipts` trong `docs/db.dbml` (`id`, `chapter_id`, `client_save_id`, `payload_hash`, `response_json`, `created_at`, unique `(chapter_id, client_save_id)`) lưu biên nhận bất biến (insert-once: `ON CONFLICT DO NOTHING` + verify `payload_hash`) trong cùng một SQLite transaction `BEGIN IMMEDIATE` với việc cập nhật `scenes` và tạo `scene_versions`. Đồng thời `scene_versions.parent_id` được siết `NOT NULL` (`SF-07`).
* **Khôi phục và Retry sau kết quả Save mập mờ (`Q4.7` — APPROVED v2.4.4):** Khi một yêu cầu Save gặp lỗi mạng IPC, timeout hoặc gián đoạn có kết quả mập mờ:
  1. FE **không** được xóa trạng thái dirty và **không** được âm thầm tự sinh một `client_save_id` mới.
  2. FE truy vấn kết quả bền vững của `client_save_id` ban đầu từ backend (hoặc gửi lại yêu cầu retry mang nguyên `client_save_id` và `expected_version_number` ban đầu).
  3. Nếu backend xác nhận `client_save_id` đó đã commit, FE đồng bộ baseline theo kết quả đã commit; nếu chưa commit và baseline DB vẫn khớp `expected_version_number`, retry được phép thực thi; nếu DB đã tiến sang version khác do tác động khác, backend trả lỗi conflict để người dùng quyết định, không tự ghi đè.
  4. Hợp đồng không hứa hẹn tự khôi phục các buffer/dirty state chỉ nằm trong RAM của FE nếu người dùng tắt hẳn ứng dụng khi chưa Save (vì V1 chưa có tính năng durable draft/autosave cho Writing Mode).
* **Không tạo lịch sử Audit Lock riêng cho V1 (`Q4.4` — APPROVED v2.4.4):** Thao tác Lock/Unlock chỉ cập nhật trạng thái hiện hành `scenes.is_locked` và tuyệt đối không tạo `scene_versions` (`LOCK-01`, `LOCK-07`). V1 không thêm bảng hay cột lịch sử khóa cảnh.
* Nhiều scene dirty trong cùng một Save tạo nhiều version, mỗi scene tối đa một version, trong cùng transaction. Không có version cấp chương.
* `scene_versions` dùng `(parent_id = scene_id, version_number)` làm identity; `version_number` do handler cấp, unique `(parent_id, version_number)` của DB là hàng rào cuối.
* **[IMPLEMENTATION PENDING]** API/transaction Save và trả read-model.

#### Source của nội dung cảnh (`Q4.3` — APPROVED v2.4.4)

* `scene_versions.scene_source` dùng enum `SceneSource`: `AI_GENERATED`, `HUMAN_EDITED`, `RESTORE`.
* AI tạo và được Accept mà không sửa tay → `AI_GENERATED`.
* Người dùng sửa nội dung → `HUMAN_EDITED`.
* Khôi phục version cũ và Accept/Save mà không sửa tay → `RESTORE`.
* Sau khi Restore, nếu người dùng sửa tay → `HUMAN_EDITED`.
* Source của nội dung cảnh tách biệt với `source` cấp draft (`user`, `ai_generated`, `user_edited`) tại mục 1.3. Không dùng lẫn hai mô hình.
* **Cách lấy `scene_source` hiện tại (`Q4.3` — APPROVED v2.4.4):** Không thêm cột `scenes.current_source` cho V1. Nguồn gốc của nội dung cảnh đã lưu hiện tại luôn được suy ra trực tiếp từ bản ghi `scene_versions` có `version_number` lớn nhất của cảnh đó (`ORDER BY version_number DESC LIMIT 1`, tận dụng unique index `(parent_id, version_number)` sẵn có trong `docs/db.dbml`). Nếu cảnh chưa có bản ghi `scene_versions` nào (`content IS NULL`), `current_scene_source` là `null`.

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
* **[APPROVED v2.4.2 & v2.4.3 — Decision 1B] Ý nghĩa và hiệu lực tức thời của Scene Lock (`SL-01..SL-06`, `LOCK-01..LOCK-07`):** `scenes.is_locked` là ràng buộc sửa do người dùng độc quyền điều khiển, nhằm ngăn AI sửa cảnh mà người dùng chưa mở cho phép sửa. Khóa không phải khóa đồng thời của database, không phải token optimistic concurrency, không phải bất biến vĩnh viễn: người dùng khóa/mở khóa bất cứ lúc nào. Các quy tắc dưới đây tích hợp với vòng đời Generate → Buffer → Review → Accept/Discard → Dirty → Save cấp chương:
  * **LOCK-01 / SL-01 — Người dùng độc quyền & Persist tức thời:** chỉ người dùng, qua tương tác frontend được ủy quyền, mới được đổi `is_locked`. Khi người dùng bật/tắt khóa cảnh: (1) FE gọi thao tác backend chuyên biệt dành riêng cho người dùng, (2) backend xác thực caller và danh tính scene, (3) backend cập nhật `scenes.is_locked` ngay lập tức vào SQLite (không chờ `Save Chapter`), (4) backend trả về trạng thái khóa bền vững đã xác nhận, và (5) FE chỉ hiển thị trạng thái khóa mới sau khi backend xác nhận (nếu lỗi, FE không được báo đã khóa/mở khóa thành công giả).
  * **LOCK-02 / SL-02 — Cấm AI truy cập hoặc vượt khóa:** AI không được gọi thao tác Lock/Unlock, không được đổi `is_locked` qua tool sửa scene hay tác dụng phụ, không có `force_unlock`/`override_locked`. Khi `is_locked == true`, AI bị cấm sửa scene đó kể cả khi người dùng nêu tên cảnh trong chat, AI cho là cần thiết, người dùng đồng ý bằng lời, AI từng sửa cảnh đó, hoặc cảnh từng mở khóa trước đó trong cùng hội thoại.
  * **SL-03 & SL-04 — Mở khóa thủ công tách biệt với đồng ý sửa:** khi AI cần sửa cảnh đang khóa, AI không sửa mà giải thích lý do và xin phép; người dùng phải tự mở khóa thủ công trên UI; backend kiểm tra lại trạng thái khóa đã persist trước khi cho sửa. Tin nhắn "đồng ý" hoặc duyệt mở rộng scope không tự mở khóa cảnh.
  * **LOCK-03 — Bảo toàn nội dung chưa lưu khi khóa:** việc khóa một scene **không** tự động lưu nội dung dirty, **không** hủy nội dung dirty, **không** xóa buffer generation tạm, **không** xóa nội dung đã Accept, **không** reset dirty state của chương và **không** tạo `scene_versions`. Người dùng được phép khóa một scene đang có chỉnh sửa chưa lưu.
  * **LOCK-04 & SL-05 — Hạn chế mutation khi đang khóa & Kiểm tra lại tại mọi ranh giới:** khi `is_locked == true`, cấm AI Generate, cấm AI Regenerate, cấm Accept nội dung AI mới vào scene đó, và cấm persist mọi thay đổi nội dung (cả AI lẫn sửa tay chưa lưu) vào scene đó khi Save. Nếu người dùng khóa cảnh trong lúc AI đang streaming generation (**Case A**): thao tác khóa thành công ngay lập tức; backend hủy luồng generation đang chạy (và đánh dấu kết quả `invalidated_by_lock` không thể Accept kể cả khi mở khóa lại sau đó); phần văn bản tạm đã hiện trên UI không bị tự động xóa để người dùng vẫn xem được, nhưng không thể Accept hoặc Save chừng nào cảnh còn khóa.
  * **LOCK-05 & SL-06 — Giữ nguyên vòng đời & Save cấp chương nguyên tử:** Accept không persist; sinh văn bản vào buffer tạm không phải quyền persist. Khi bấm `Save Chapter`, nếu bất kỳ scene nào có thay đổi nội dung đang bị khóa (`is_locked == true`), toàn bộ Save bị từ chối nguyên tử và giữ nguyên mọi buffer dirty (**Case B, Case E**); ngược lại, các scene đang khóa nhưng không có thay đổi nội dung (clean) không chặn việc Save các scene mở khóa khác (**Case F**).
  * **LOCK-06 — Hành vi khi Unlock:** mở khóa có hiệu lực ngay sau khi backend xác nhận persist, không tự động Save, không tự động Generate, không tự động Accept, và không tự cấp quyền cho AI sửa ngoài `EditScope` đang hoạt động (nhưng cho phép người dùng Accept một generation đã hoàn tất hợp lệ trước đó hoặc Save nội dung dirty hợp lệ đang có của cảnh đó; các generation từng bị hủy do Lock vẫn không được Accept).
  * **LOCK-07 — Lịch sử phiên bản:** thao tác chỉ đổi khóa (Lock/Unlock) không tạo bản ghi `scene_versions`.
* **[IMPLEMENTATION PENDING]** IPC persist lock tức thời và kiểm tra khóa tại backend; hiện FE preview chỉ giữ local.

#### Phạm vi sửa của AI trong một request (EditScope) và vòng đời nội dung

> **Trạng thái:** APPROVED v2.4.2..v2.4.4 (**Decision 2A**: `SL-07..SL-12`, `SCOPE-01..SCOPE-06`, và UX contract `Q4.9`). Vòng đời ủy quyền của AI tách biệt với vòng đời của nội dung đã sinh hợp lệ. Dạng dữ liệu của `EditScope` và provenance là hợp đồng khái niệm, không thêm bảng phân quyền vĩnh viễn trong DBML.

* **SCOPE-01 / SL-07 & SL-08 — Ủy quyền theo request và hai điều kiện độc lập:** mỗi request sửa nội dung của AI có một `EditScope` tường minh (`request_id`, `chapter_id`, `allowed_scene_ids`) xuất phát từ hành động/ủy quyền của người dùng. AI chỉ được phép bắt đầu hoặc tiếp tục generate/regenerate cho một scene khi thỏa đồng thời: `scene.is_locked == false` **VÀ** `scene.id ∈ active_request.EditScope`. Scene đang mở khóa nhưng nằm ngoài `EditScope` vẫn bị cấm (**Case G**); scene nằm trong `EditScope` nhưng đang khóa vẫn bị cấm (**Case H**).
* **SL-09 & SL-10 — Mở rộng scope cần người dùng duyệt và không vượt khóa:** nếu AI cần sửa scene ngoài `EditScope` hiện tại, AI phải nêu lý do, chỉ rõ scene đích, mô tả thay đổi dự kiến và xin phép mở rộng scope; người dùng phải duyệt tường minh. Backend không tự suy ra phê duyệt từ lập luận của model. Ngay cả khi đã duyệt mở rộng scope, nếu scene đích đang khóa thì người dùng vẫn phải tự mở khóa thủ công (**Case H**).
* **SCOPE-02 / SL-11 & SL-12 — Hết hạn quyền khi kết thúc request nhưng bảo toàn nội dung hợp lệ:** khi AI request kết thúc (hoàn tất hoặc dừng), quyền sửa của AI theo `EditScope` đó hết hạn ngay lập tức, không chuyển sang request kế tiếp, và không có quyền sửa toàn cục ngầm từ lịch sử chat hay context. Tuy nhiên, việc request kết thúc **không** làm vô hiệu hóa nội dung đã được sinh hợp lệ trước đó, buffer generation tạm, trạng thái chờ người dùng review, nội dung đã Accept trước đó, hay trạng thái dirty chưa lưu của chương.
* **SCOPE-03 — Review sau khi request kết thúc:** sau khi AI request kết thúc, người dùng vẫn có toàn quyền xem lại (review) nội dung đã sinh, bấm Accept hoặc Discard, tiếp tục xem các thay đổi đã Accept, và bấm Save chương vào bất kỳ thời điểm nào sau đó mà **không** cần mở AI request mới, **không** cần cấp lại `EditScope` đã hết hạn, và **không** phải regenerate lại nội dung (**Case C, Case D**).
* **SCOPE-04 — Quyền thực hiện Accept:** bấm Accept một kết quả đã sinh sau khi request kết thúc là **hành động của người dùng**, không phải một mutation mới của AI. Hệ thống phân biệt rõ: (1) nội dung có được sinh hoàn tất hợp lệ (`status == completed` và `authorization_valid == true`) dưới một `EditScope` và trạng thái mở khóa hợp lệ tại thời điểm sinh hay không (nếu từng bị hủy giữa chừng bởi Lock thì bị vô hiệu vĩnh viễn, không được Accept kể cả sau khi Unlock), (2) người dùng hiện đang chủ động bấm Accept, và (3) scene đích hiện tại có đang mở khóa (`is_locked == false`) hay không. Request hết hạn không làm hỏng kết quả hợp lệ, nhưng người dùng **không** được Accept nội dung vào một scene hiện đang khóa (`LOCK-04`).
* **SCOPE-05 — Quyền thực hiện Save:** `Save Chapter` là thao tác do người dùng điều khiển, không cần AI request đang hoạt động và không cần kích hoạt lại `EditScope` cũ (**Case D**). Save chỉ kiểm tra: hành động hợp lệ của người dùng, danh tính scene, trạng thái khóa bền vững hiện tại (`is_locked == false` cho mọi scene có thay đổi nội dung), tính hợp lệ của nội dung, điều kiện optimistic concurrency và tính nguyên tử cấp chương.
* **SCOPE-06 — Provenance tối thiểu của kết quả sinh:** kết quả generation trong buffer tạm giữ metadata nguồn gốc tối thiểu trong phiên làm việc (`request_id`, `generation_id`, `scene_id`, `baseline_version_number`, `authorization_valid`, `status`, và `scene_source = AI_GENERATED`) để cho phép Accept về sau mà không cần bảng ledger phân quyền bền vững trong SQLite. Vì buffer generation và dirty state của Writing Mode vốn là trạng thái làm việc trong phiên (chưa có autosave/durable draft theo mục Dirty state), metadata này sống cùng vòng đời buffer trong phiên và chuyển thành `scene_versions.scene_source = AI_GENERATED` khi Save thành công.
* **Hợp đồng UX tối thiểu cho `EditScope` (`Q4.9` — APPROVED v2.4.4):**
  1. Khi người dùng bấm nút `Generate` hoặc `Regenerate` trực tiếp trên một cảnh đang mở khóa, hành động đó chính là sự ủy quyền tường minh khởi tạo `EditScope = {scene_id}` cho request đó.
  2. Khi người dùng gửi yêu cầu trong khung Chat của chương: cảnh đang được chọn/focus (hoặc cảnh được đính kèm) có thể được gợi ý làm `EditScope` đề xuất, nhưng **người dùng phải xác nhận/ủy quyền tường minh phạm vi cảnh được phép sửa** trước khi AI thực thi sinh nội dung cảnh.
  3. Nếu tin nhắn Chat không có `EditScope` nào được xác định/ủy quyền, yêu cầu đó chỉ được chạy ở chế độ **tư vấn/trò chuyện thuần túy (advisory-only)** — AI trả lời, gợi ý hoặc kiểm tra continuity trong khung chat nhưng không được ghi vào buffer generation của bất kỳ cảnh nào.
  4. Khi AI đề xuất mở rộng `EditScope` sang cảnh khác, UI hiển thị lời nhắc xác nhận tường minh (nêu rõ cảnh đích, lý do, thay đổi dự kiến và nút Đồng ý / Từ chối). Việc bấm Đồng ý mở rộng scope chỉ áp dụng cho request hiện tại và **không** tự động mở khóa các cảnh đang bị khóa (`SL-10`). Chi tiết bố cục giao diện thuộc `fe-core.md` §9.
* **Tách bạch khái niệm:** `is_locked` (hạn chế sửa, chỉ người dùng, persist tức thời theo `LOCK-01`) ≠ `EditScope` (cảnh AI được sửa trong lúc request đang chạy, do người dùng ủy quyền) ≠ token optimistic concurrency (phát hiện stale write khi Save dựa trên `MAX(scene_versions.version_number)`, do backend kiểm soát; không dùng `is_locked` làm token này) ≠ draft operation lease (tuần tự hóa thao tác draft ở Planning Mode) ≠ Save cấp chương (persist nguyên tử các thay đổi nội dung đã duyệt kèm `client_save_id`).
* **[IMPLEMENTATION PENDING]** Giao diện chọn/duyệt mở rộng `EditScope` và nối IPC backend (xem `fe-core.md` §9).

#### Dirty state và điều hướng

* Dirty được theo dõi theo từng scene dựa trên sự khác biệt giữa nội dung working state và nội dung baseline đã lưu. (Theo `LOCK-01` và `LOCK-03`, thao tác Lock/Unlock được persist ngay lập tức xuống SQLite, không làm mất dirty content hiện có và không tạo content version).
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

* Hệ thống duyệt qua **46 thuộc tính Master list** (`MASTER_ATTRIBUTES_LIST` — chuẩn thẩm quyền của Business Core; không tự đổi thành 50 theo mã nguồn FE prototype).
* Hiển thị hai phần trăm riêng trên thanh Header, mỗi phần chia cho tổng 46 thuộc tính và mỗi thuộc tính đóng góp 0 hoặc 1 theo evaluator (giữ nguyên cách tính mảng: một thuộc tính mảng đạt 1 khi có ít nhất một phần tử hợp lệ trong mảng tương ứng, và phân loại theo `source` của giá trị đã điền): **Đã xác nhận** = số thuộc tính đạt có `source = user` hoặc `user_edited` / 46; **AI đề xuất** = số thuộc tính đạt có `source = ai_generated` / 46. Dữ liệu thuộc phase chưa mở vẫn được tính nếu có trong JSON. Thay cho một `progressPercent` tổng duy nhất.
* **Quy tắc đối chiếu 46 thuộc tính chuẩn, Composite Provenance và `world_setting_code` (`APPROVED v2.4.4..v2.4.5`):**
  1. Một thuộc tính tiến độ (progress attribute) là chỉ báo hoàn thiện dàn ý ở tầng Business Flow, **không đồng nhất 1:1 với cột vật lý trong DBML** (không xóa cột DBML và không xóa trường khỏi kiểu dữ liệu runtime chỉ vì không nằm trong 46 thuộc tính tiến độ).
  2. **Loại trừ chính xác 4 mục khỏi danh sách 50 của FE prototype (`50 - 4 = 46`):**
     * Ba trường dung sai gợi ý runtime: `optional.overview_phase.project.target.arc_tolerance`, `optional.overview_phase.project.target.chapter_tolerance`, `optional.overview_phase.project.target.word_per_chapter_tolerance` (vẫn giữ trong cấu hình runtime/gợi ý cho AI, không tính vào mẫu số 46).
     * Một trường đang tạm treo: `optional.relationship_phase.relationship.motif.event_blocks` (chưa thiết kế bảng trong `docs/db.dbml`, giữ trạng thái tạm treo và không tính vào mẫu số 46).
  3. **Quy tắc Provenance cho thuộc tính tổng hợp / mảng (`Composite Provenance — APPROVED v2.4.5`):**
     * Một phần tử (element) chỉ đạt hoàn chỉnh khi thỏa mãn đầy đủ `completion_rule` của thuộc tính đó (thiếu hoặc sai trường bắt buộc thì phần tử đó là `incomplete`).
     * Một phần tử hoàn chỉnh được phân loại là **confirmed (`Đã xác nhận`)** khi và chỉ khi **tất cả** các trường bắt buộc đóng góp vào điều kiện hoàn thành của phần tử đó đều có `source = user` hoặc `source = user_edited`.
     * Một phần tử hoàn chỉnh nhưng có **bất kỳ** trường bắt buộc nào mang `source = ai_generated` thì được phân loại là **AI suggested (`AI đề xuất`)**.
     * Với thuộc tính dạng mảng: thuộc tính đạt hoàn chỉnh khi có ít nhất một phần tử hợp lệ thỏa mãn `completion_rule`. Nếu có ít nhất một phần tử hợp lệ đạt mức `confirmed` thì toàn bộ thuộc tính Master đó được tính vào **Đã xác nhận**; ngược lại, nếu có ít nhất một phần tử hợp lệ đạt mức `ai_suggested` thì tính vào **AI đề xuất**; ngược lại là chưa hoàn thành (`incomplete`).
     * Mỗi thuộc tính Master đóng góp tối đa 1 đơn vị vào mẫu số 46 và không bao giờ được tính đồng thời vào cả hai chỉ số. Thứ tự ưu tiên phân loại tiến độ là tất định: `confirmed > ai_suggested > incomplete` (chỉ áp dụng cho phân loại tiến độ hiển thị, không thay đổi quyền ghi đè hay quy tắc bảo vệ trường tại mục 1.3).
  4. **Quy tắc cho `world_setting_code` (`APPROVED v2.4.5`):** `world_setting_code` là trường bắt buộc cho tính hợp lệ của cấu hình cảnh và bắt buộc phải có trước khi `commit_draft` (`scenes.world_setting_id NOT NULL` theo mục 2.3, 2.5, 2.8), nhưng **không** phải là thuộc tính thứ 47 và **không** thay thế bất kỳ thuộc tính nào trong 46 thuộc tính Master của Progress Engine.
  5. Các lỗi đường dẫn/kiểu dữ liệu còn lại trong `src/features/business-flow/progressEngine.ts` (sai chính tả `charactar_psychological_phase`, cấu trúc `world_settings` đơn lẻ thay vì mảng cây, `lore_entries_code` scalar thay vì mảng `lore_entries[]`, biểu thức `typeof ch?.is_pov === 'boolean'` luôn đúng) là sai lệch của tầng triển khai FE (`[IMPLEMENTATION PENDING]`), sẽ được đồng bộ theo đúng 46 thuộc tính chuẩn khi nối backend.

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
* Kiểm tra và ghi phải có bảo đảm nguyên tử ở tầng persistence qua cột `draft_versions.seal_key` và unique index `(draft_session_id, seal_key)` (`[SCHEMA APPROVED: SF-01]`).

#### Phục hồi sau crash

1. Khôi phục Draft State từ sealed version hợp lệ cuối cùng.
2. Không phát lại tool call từ audit để dựng State.
3. Không tự áp dụng lại mutation chưa được niêm phong.
4. Không coi dữ liệu trong Working State trước crash là dữ liệu bền vững.
5. Có thể phát hiện lượt bị gián đoạn từ audit đã lưu; nếu audit chưa được lưu trước crash, chấp nhận giới hạn không thể phát hiện đầy đủ.
6. Không vì thiếu audit mà tự tạo dữ liệu hoặc tự suy luận mutation đã thành công.

#### `[SCHEMA APPROVED: SF-01]` Idempotent sealing trên `draft_versions`

* **Bảng:** `draft_versions` (`docs/db.dbml`).
* **Trường & Unique Index đã bổ sung (`v2.4.5`):** cột `seal_key varchar` (nullable cho các bản ghi legacy trước khi có `seal_key`; bắt buộc khác null cho mọi thao tác niêm phong mới) và unique index `(draft_session_id, seal_key)`.
* **Giao thức kiểm tra & niêm phong nguyên tử:** Trong transaction `BEGIN IMMEDIATE`, backend kiểm tra `(draft_session_id, seal_key)` trước khi tạo dòng mới. Nếu đã tồn tại dòng có cùng `(draft_session_id, seal_key)`: đối chiếu ngữ nghĩa thao tác và payload thông qua bộ cột đã lưu (`data_hash`, `reason`, `phase`, `subphase`, `parent_seq`, `turn_id`, `chat_message_id`) — nếu khớp hoàn toàn thì trả lại kết quả version gốc mà không tạo dòng thứ hai; nếu bất kỳ thông tin ngữ nghĩa hoặc `data_hash` nào không khớp thì từ chối với lỗi conflict có cấu trúc (`seal_key_payload_mismatch`).
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
* Patch đang chờ xác nhận được chiếu từ bảng SQLite `pending_patches` vào **context prompt** cạnh State (không ghi vào sealed Business State) để AI biết người dùng chưa trả lời và không đề xuất trùng.
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
* **Sinh `code` và bảo toàn bộ đếm `_counters` (`Q3.6` — APPROVED v2.4.4):** Handler cấp `code` bằng bộ đếm đơn điệu tăng dần theo từng cha phải tuần tự hóa ở backend (mutex theo cha) để hai subagent không nhận trùng số. Định dạng `code` theo quy tắc đã chốt tại mục 3.8. Bộ đếm theo cha (`_counters: { "<parent_code>": <next_int> }`) được lưu trực tiếp trong snapshot của `BusinessFlowData` (`draft_versions.data_json`) và bắt buộc nhất quán với các thực thể đã tạo; khi `restore` một version cũ, `_counters` được khôi phục nguyên tử cùng với State của version đó (và luôn kiểm tra chống đụng độ với các `code` hiện có). Không tạo bảng cơ sở dữ liệu riêng cho bộ đếm.
* **Điều phối đồng thời giữa các subagent (`Q3.5` — APPROVED v2.4.4):** Các subagent trong một vòng được phép tính toán/gọi LLM song song (không giữ khóa cơ sở dữ liệu hay khóa ghi trong lúc chờ mạng LLM), nhưng mọi mutation ghi vào `WorkingState` dùng chung phải có phạm vi ghi tường minh (explicit write scope) và được áp dụng theo **thứ tự tất định (deterministic application order)** theo thứ tự chỉ mục task trong kế hoạch vòng (`task_index`), tuyệt đối không phụ thuộc vào thứ tự trả kết quả mạng ngẫu nhiên (network completion order) để giải quyết xung đột.
* **Ngăn chặn trùng lặp kỹ thuật và ngữ nghĩa (`Q3.4` — APPROVED v2.4.4):** Phân biệt rõ hai loại trùng lặp:
  1. *Trùng lặp kỹ thuật (Technical duplicates):* trùng `code`, trùng vị trí `(parent, position)` hoặc vi phạm ràng buộc định danh/khóa ngoại — bị từ chối ngay tại handler (`status = rejected`).
  2. *Trùng lặp ngữ nghĩa (Semantic duplicates):* hai subagent đề xuất hai thực thể tương đương về ý nghĩa nhưng khác `code` (ví dụ cùng một địa danh hoặc lore) — được xử lý bằng bước gộp/đối chiếu tường minh (như giai đoạn 2 của Phase 3) trước khi ghi. Tuyệt đối **không** tự động gộp (auto-merge) hoặc ghi đè dữ liệu của người dùng đã được bảo vệ (`source = user | user_edited`) chỉ dựa trên sự tương đồng về tên gọi.
* **Bảo vệ `source`:** Giữ nguyên mục 1.3. Patch `pending_confirmation` trong workflow không dừng workflow; gom lại và hiển thị cho người dùng sau khi hết vòng.
* **Giới hạn lỗi:** `consecutive_rejected_count` tính riêng cho từng subagent và chỉ tăng khi `status` tổng của tool call là `rejected`. Subagent vượt ngưỡng chỉ khiến task đó được đánh dấu failed; các trường `applied` trước đó của task vẫn góp vào version cuối vòng.

##### Resume workflow, Định danh Task và Biên nhận Hoàn tất Bền vững (`Q3.1..Q3.5` — APPROVED v2.4.4)

* **Tách biệt 5 khái niệm lưu trữ (`Q3.2`):**
  1. *Business State (`WorkingState`):* trạng thái nghiệp vụ đang làm việc trong bộ nhớ.
  2. *Sealed Draft Version (`draft_versions`):* nguồn sự thật duy nhất để khôi phục Business State.
  3. *Workflow Completion Receipt (Biên nhận hoàn tất):* bằng chứng bền vững ghi nhận một round/task đã hoàn tất hợp lệ trên một trạng thái đầu vào xác định; dùng để điều phối resume, không bao giờ thay thế Sealed Draft Version.
  4. *Temporary Task Report (`./merlink-harness/tmp/<draft_session_id>/<phase>/`):* báo cáo trung gian tạm thời.
  5. *Runtime Execution Status:* trạng thái đang chạy trong RAM.
* **Định danh Task ổn định và Thế hệ kế hoạch (`Q3.1` — APPROVED v2.4.4..v2.4.5):**
  * Mỗi lần chạy workflow có một `workflow_run_id` duy nhất; mỗi khi kế hoạch phân đoạn được tính lại do thay đổi cấu trúc đầu vào (số Arc/chương...) hoặc khi chạy lại một task đã bị stale sau Restore/chỉnh sửa, hệ thống tạo một `plan_generation` mới và đối chiếu tường minh phần việc đã hoàn tất.
  * Danh tính đầy đủ của một biên nhận hoàn tất gồm 4 thành phần: `(draft_session_id, workflow_run_id, plan_generation, task_identity)`, trong đó `task_identity` là định danh chuẩn hóa của `(phase, subphase, task_kind, assigned_scope)` (không nhúng lặp `workflow_run_id` vào bên trong `task_identity`). Định danh của một task không được âm thầm thay đổi khi task đó được retry. Tuyệt đối không dùng `display_name` hoặc array index làm định danh bền vững.
* **Tính nguyên tử, Biên nhận bất biến (Insert-Once) và Kiểm tra trước khi tạo Version (`Q3.2`, `Q3.3` — APPROVED v2.4.5):**
  * *Bất biến bắt buộc:* **Một workflow task không bao giờ được coi là đã hoàn tất bền vững nếu các thay đổi Business State bắt buộc của nó không thể khôi phục được từ Sealed Draft Version.**
  * *Kiểm tra biên nhận trước khi tạo Draft Version (Retry Before Version Creation):* Khi thực thi hoặc retry một workflow task/round trong `BEGIN IMMEDIATE`, backend bắt buộc truy vấn biên nhận hiện có trong `workflow_task_receipts` theo `(draft_session_id, workflow_run_id, plan_generation, task_identity)` **trước khi** tạo `draft_versions` mới. Nếu biên nhận đã tồn tại và còn hiệu lực (khớp `plan_generation`, `input_scope_fingerprint`, `output_scope_fingerprint` và phả hệ snapshot hiện tại), trả về ngay kết quả hoàn tất trước đó và **tạo 0 `draft_versions` mới**. Nếu biên nhận đã tồn tại nhưng không khớp ngữ nghĩa hoặc đã stale, không được âm thầm ghi đè biên nhận cũ mà trả về trạng thái `stale`/`conflict` có cấu trúc để kích hoạt chính sách chạy lại với `plan_generation` (hoặc `workflow_run_id`) mới.
  * *Biên nhận bất biến (Insert-Once Semantics):* Bảng `workflow_task_receipts` chỉ dùng cơ chế chèn một lần (`INSERT ... ON CONFLICT DO NOTHING` kèm truy vấn xác minh), **tuyệt đối cấm dùng `INSERT OR REPLACE`**.
  * *Với vòng có thay đổi State (mutation-producing round):* việc niêm phong `draft_versions` (`reason = workflow_round`, `had_mutations = true`, `resulting_version_seq = new_seq`) và việc chèn biên nhận vào `workflow_task_receipts` **bắt buộc nằm trong cùng một ranh giới SQLite transaction nguyên tử (`BEGIN IMMEDIATE`)**. Nếu xảy ra xung đột hoặc lỗi ở bất kỳ bước nào, rollback toàn bộ transaction (không bao giờ để lọt `draft_versions` mồ côi).
  * *Với vòng hoàn tất hợp lệ nhưng không có thay đổi State (zero-mutation / no-op round):* **không** tạo bản ghi `draft_versions` giả (tuân thủ quy tắc chống trùng `data_hash == head_hash` tại mục 3.6); thay vào đó, ghi bền vững một Completion Receipt với `had_mutations = false`, `resulting_version_seq = NULL`, `base_version_seq = head_version_seq`, gắn chặt với `input_scope_fingerprint` và `output_scope_fingerprint` đã xác thực.
* **Kiểm tra hiệu lực khi Restore và Resume (`Q3.4`):**
  * **Cấm** kiểm tra hiệu lực của Completion Receipt chỉ bằng phép so sánh số thứ tự version (`current_seq >= receipt_seq`). Ví dụ: chuỗi `V1 -> V2 -> V3 -> V4` (task hoàn tất ở `V4`), sau đó người dùng Restore `V2` tạo ra `V5` (`V5 > V4` nhưng nội dung quay về `V2`) — việc `5 > 4` không chứng minh task ở `V4` còn hiệu lực trên `V5`.
  * Một Completion Receipt chỉ còn hiệu lực khi kiểm tra đồng thời: `task_identity`, `plan_generation`, `assigned_scope`, dấu vân tay đầu vào lịch sử của task trên S(i−1), dấu vân tay đầu ra lịch sử trên Si ngay sau task, và khả năng áp dụng độc lập trên head hiện tại theo lineage/phụ thuộc/phạm vi; không được đồng nhất Si với snapshot cuối vòng Sn. Các fingerprint lịch sử không tự chứng minh hiệu lực hiện tại. Khi người dùng Restore hoặc sửa dữ liệu làm thay đổi đầu vào/đầu ra của một task, biên nhận cũ được giữ nguyên trong lịch sử nhưng được suy dẫn là hết hiệu lực (stale) đối với trạng thái hiện tại và task đó phải được chạy lại dưới `plan_generation` / lần chạy mới; ngược lại, các task không liên quan có đầu vào và kết quả trong snapshot hiện tại vẫn khớp nguyên vẹn fingerprint thì được bảo toàn, không bắt chạy lại vô ích.
* **Ưu tiên khi Report và State lệch nhau (`Q3.3`):**
  * **Sealed Business State trong `draft_versions` luôn thắng tuyệt đối** so với các file report tạm trong `./merlink-harness/tmp/`.
  * Report tạm chỉ được tái sử dụng khi cả `task_identity`, `plan_generation` và `input_scope_fingerprint` của report khớp với checkpoint hợp lệ hiện tại; report bị thiếu hoặc cũ (stale) bị bỏ qua/tạo lại và tuyệt đối không được làm hỏng State.
* **Lưu trữ Schema cho Completion Receipt (`[SCHEMA APPROVED: SF-06]` — v2.4.5):** Bảng `workflow_task_receipts` đã được bổ sung vào `docs/db.dbml` với unique index `(draft_session_id, workflow_run_id, plan_generation, task_identity)`. Phê duyệt SF-06 tối thiểu này không đồng nghĩa B+ đã được triển khai.

##### SF-06 B+ — hợp đồng bổ sung theo quyết định owner (đặc tả, chưa kiểm chứng triển khai)

* **Identity và niêm phong:** Mỗi round hợp lệ có header `workflow_round_receipts` bất biến theo giao thức backend, định danh `(draft_session_id, workflow_run_id, plan_generation, round_index)` và `round_seal_key` backend sinh cả với zero-net. Trong một TX-04, ghi header, đủ biên nhận task và evidence; nếu canonical State cuối vòng khác baseline thì ghi đúng một Draft Version và cập nhật head; nếu bằng nhau, vẫn ghi header/receipts nhưng không tạo version. `had_mutations` ở biên nhận legacy là bit ghép `effectiveStateChanged && resulting_version_seq != NULL`, **không** phải số lần tool ghi; evidence mới ghi riêng `hadAppliedWrites` và `effectiveStateChanged` kể cả hai task triệt tiêu nhau. Không thay đổi các biên nhận cũ.
* **Ranh giới task:** Với task thứ i, fingerprint đầu vào tính trên S(i−1), đầu ra trên Si; evidence có pure state patch để phục hồi S0→…→Sn; fingerprint mới dùng phiên bản `sf06:v2:`, không diễn giải lại v1. Hash/bytes canonical của State loại `meta.updated_at`, nhưng bao gồm provenance, `_counters` và thứ tự mảng có nghĩa. Patch không chứa proposal đang chờ duyệt. Codec không hỗ trợ transition hợp lệ thì từ chối seal, không tái gọi tool/LLM để lấp evidence.
* **Tin cậy D09-A:** Backend là writer ứng dụng đã kiểm quyền khi thực thi/seal; SQLite cục bộ tin cậy trong V1. Receipt đã commit là attestation lịch sử, không phải bằng chứng mật mã chống người có quyền ghi trực tiếp DB hay yêu cầu chạy lại evaluator chính sách cũ. Tồn tại lịch sử, nhất quán nội tại, attestation ủy quyền, cùng intent request và khả năng áp dụng hiện tại là năm phép kiểm riêng. Restore giữ nguyên lịch sử nhưng không cho skip chỉ vì `seq` tăng; thiếu/không hỗ trợ evidence ⇒ `Indeterminate`/`RequiresReplan`, không tự sửa và không suy hoàn tất từ audit/report tạm. Chỉ round đã commit được phục hồi từ SQLite; không hứa exactly-once với hiệu ứng mạng/filesystem ngoài DB.
* **Độc lập D10-B:** Proposal được commit ngay trong TX-08 của tool executor dưới khóa draft, trước khi trả `patch_id`; không stage vào TX-04 và có thể còn sau khi workflow fail. UI có thể đọc proposal đã commit ngay, còn workflow vẫn có thể trì hoãn trình bày tới cuối vòng. Dedupe cùng giá trị trả ID của đề xuất cũ còn hợp lệ nhưng không tự nhận origin/dependency mới; nếu origin mới là prerequisite thì phải báo conflict thay vì chứng nhận sai. Proposal không chứng minh round hoàn tất và không được replay như Working State. Khi confirm, ngoài tính còn tồn tại/giá trị trước/provenance/expiry, phải kiểm quyền người duyệt, đúng draft, không bị Restore vô hiệu, không phụ thuộc state chưa seal đã biến mất và các dependency nghiệp vụ thực tế; không tự expire mọi proposal chỉ vì round fail. TX-03 Restore tiếp tục expire mọi pending; thay thế và expiry tuân thủ §1.3; confirm dùng TX-02.
* **Artifact tối thiểu (OC-1B):** Chỉ persist output có nội dung thực sự được task downstream tiêu thụ mà không thể dựng lại thuần từ State/evidence (từng output report Phase 3 `world_probe` được bước gộp tiêu thụ, và exact merged input mà `world_write` tiêu thụ nếu có; mỗi artifact gắn với evidence của round tạo/tiêu thụ nó). Trước khi vòng write seal, backend có thể gộp thuần các probe artifact đã seal hoặc replan nếu thiếu; sau khi write seal, verifier dùng input artifact bền thay vì report tạm/merger lịch sử. Không lưu mọi LLM response; artifact có schema/codec version, bytes canonical, hash và giới hạn kích thước bằng cấu hình runtime; ghi nguyên tử với evidence TX-04. Nếu thiếu/quá budget, không seal completion cần artifact hoặc trả `RequiresReplan` cho lịch sử; report tạm không thay thế được.
* **Baseline legacy (OC-3A):** `isRequireComplete=false, head_version_seq=NULL` là draft chưa ready bình thường. `isRequireComplete=true, head_version_seq=NULL` là baseline workflow không hợp lệ: trả `WorkflowBaselineUnavailable`, không sinh version `seq=0`, không dựng genesis từ RAM và không backfill tự động. Cho phép xem dữ liệu còn đọc được; sửa thủ công riêng chỉ sau khi xác định dữ liệu nguồn tin cậy, quyền owner, snapshot nhất quán và kế hoạch bảo toàn lịch sử. Không đưa `client_request_id` vào workflow V1 khi chưa có phản ví dụ correctness cụ thể.


---

## 4. BẢNG ÁNH XẠ DỮ LIỆU NHÁP SANG CƠ SỞ DỮ LIỆU QUAN HỆ (SQLITE SCHEMA)

| Khối dữ liệu trong Business Flow          | Bảng lưu trữ chính thức trong SQLite (`db.dbml`)            | Ghi chú chuyển đổi                                        |
| :------------------------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------ |
| Phiên quản lý bản nháp | `draft_sessions` | Quản lý `session_id` (1:1 với chat session trước commit), head version, trạng thái `editing`/`committing`/`committed` và liên kết project sau commit; không phải thực thể nghiệp vụ được tạo khi commit. |
| Snapshot bản nháp | `draft_versions` | Lưu toàn bộ JSON/source, phase, subphase và `seal_key` (`SF-01`) theo version; không phải thực thể nghiệp vụ được tạo khi commit. |
| Biên nhận hoàn tất workflow | `workflow_task_receipts` | Lưu bền biên nhận hoàn tất bất biến của từng task/round trong workflow phân đoạn (`SF-06`); không phải thực thể nghiệp vụ được tạo khi commit. |
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
* **Luồng viết sau commit (H1, xem mục 1.6):** `scene_versions` (`parent_id NOT NULL` theo `SF-07`), `chapter_save_receipts` (`SF-02`); không có `chapter_versions`. Nội dung chương đọc từ `scenes.content` theo `scenes.position`.
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

## CHANGELOG v2.4.1

* Bổ sung quy tắc Regenerate note tại mục 1.6: ghi chú tùy chọn, rỗng/khoảng trắng chuẩn hóa `null`, chỉ áp dụng cho request hiện tại, không sửa trực tiếp nội dung scene, không persist như ràng buộc dự án, không bỏ qua Review → Accept → Save.
* Không thay đổi versioning, Save, source, lock hoặc continuity của v2.4; không sửa DBML/backend.

## CHANGELOG v2.4.2

* Chốt quy tắc Scene Lock và phân quyền sửa theo request (SL-01..SL-12) tại mục 1.6: `scenes.is_locked` là hạn chế sửa do người dùng độc quyền điều khiển; AI không đổi `is_locked`, không vượt khóa và phải chờ người dùng tự mở khóa thủ công trên frontend; backend kiểm tra lại trạng thái khóa tại ranh giới mutation.
* Bổ sung `EditScope` theo từng request sửa của AI: AI chỉ được sửa scene khi đồng thời đang mở khóa và nằm trong `EditScope`; mở rộng scope cần người dùng duyệt tường minh và không thay thế việc mở khóa thủ công; quyền sửa hết hạn theo request.
* Tách biệt rõ `is_locked`, `EditScope`, token optimistic concurrency, draft operation lease và Save cấp chương; ghi nhận các điểm còn mở về hiệu lực Unlock và UI mở rộng scope tại Q4 (mục 9, 10).

## CHANGELOG v2.4.3

* Chốt **Decision 1B (`LOCK-01..LOCK-07`)** tại mục 1.6: thao tác Lock/Unlock của người dùng persist `scenes.is_locked` ngay lập tức xuống SQLite (không chờ `Save Chapter`), chỉ cập nhật trạng thái xác nhận trên UI khi persist thành công, không tạo `scene_versions`, và bảo toàn nguyên vẹn mọi buffer generation / nội dung dirty chưa lưu. Khi `is_locked == true`, chặn AI Generate/Regenerate, chặn Accept nội dung mới và chặn Save nội dung dirty của scene đó (nếu một scene có nội dung dirty bị khóa thì toàn bộ `Save Chapter` bị từ chối nguyên tử và giữ nguyên dirty; scene khóa nhưng clean không chặn Save các scene khác).
* Chốt **Decision 2A (`SCOPE-01..SCOPE-06`)** tại mục 1.6: tách biệt thời hạn ủy quyền của `EditScope` (hết hạn ngay khi AI request kết thúc) với vòng đời của nội dung đã sinh hợp lệ (vẫn xem lại, Accept, Discard và Save được sau khi request đã kết thúc mà không cần mở request mới hay cấp lại `EditScope`, miễn là scene đang mở khóa tại thời điểm Accept/Save).
* Đóng mục Q4.10 (hiệu lực của Unlock đã được giải quyết bằng `LOCK-01`/`LOCK-06`) và cập nhật Q4.8, Q4.9.

## CHANGELOG v2.4.4

* **Đóng toàn bộ câu hỏi nghiệp vụ còn mở (`Q1`, `Q3`, `Q4`) theo chỉ thị Option B (Correctness First, Minimal Schema Changes When Necessary):**
  * **Chốt Q1 (Cascade Delete — mục 1.3):** Hoàn thiện ma trận xóa cho toàn bộ quan hệ tham chiếu và sở hữu: chặn xóa `world_setting` (và cây con) khi đang được `scenes.world_setting_id` tham chiếu (`referenced_entity_in_use`); cho phép xóa `lore_entry` kèm các liên kết `scene_lore_entries` phụ thuộc sau khi phân tích phụ thuộc, tạo preview đầy đủ và kiểm tra provenance bảo vệ; chặn xóa `character` đang được `relationships` tham chiếu (`referenced_entity_in_use`); chỉ cho phép xóa `character` kèm `theme_arguments` khi vượt qua kiểm tra provenance và không vi phạm bất biến chủ đề; xóa `scene` thì cascade xóa các bản ghi `relationship_states` thuộc cảnh đó nhưng giữ nguyên thực thể `relationships`; cấm cascade thực thể dùng chung xuyên project; chốt xóa vật lý (physical removal) khỏi `WorkingState` (không dùng tombstone cho V1); chuẩn hóa 9 bước bảo vệ bắt buộc trước khi xóa.
  * **Chốt Q3 (Workflow Recovery & Resume — mục 3.9):** Chốt định danh task ổn định suy ra từ `(workflow_run_id, plan_generation, phase, subphase, task_kind, assigned_scope)`; tách biệt 5 tầng lưu trữ (`Business State`, `Sealed Draft Version`, `Workflow Completion Receipt`, `Temporary Task Report`, `Runtime Execution Status`); bắt buộc tính nguyên tử giữa mutation và completion receipt; không tạo version giả cho vòng có 0 mutation nhưng phải ghi biên nhận hoàn tất bền vững; kiểm tra hiệu lực receipt khi Restore/Resume dựa trên `input_scope_fingerprint`, `plan_generation` và `task_identity` (cấm chỉ so sánh `V_current > V_receipt`); Sealed Business State luôn thắng report tạm; phân biệt trùng lặp kỹ thuật và trùng lặp ngữ nghĩa (cấm tự gộp dữ liệu người dùng được bảo vệ); áp dụng mutation của subagent theo thứ tự tất định (không phụ thuộc thứ tự hoàn tất mạng); bảo toàn `_counters` trong snapshot.
  * **Chốt Q4 (Writing Mode — mục 1.6):** Chốt idempotency bền vững ở cấp request cho `Save Chapter` theo `client_save_id` (`Q4.1`); chốt OCC dựa trên lịch sử `scene_versions` (`expected_version_number == COALESCE(MAX(version_number), 0)`) kết hợp kiểm tra `scenes.is_locked == false` trong một transaction nguyên tử mà không cần `scenes.revision` (`Q4.2`); chốt suy xuất `current_scene_source` từ bản ghi `scene_versions` mới nhất mà không cần `scenes.current_source` (`Q4.3`); chốt không tạo bảng lịch sử audit lock cho V1 (`Q4.4`); chốt hợp đồng read-model cảnh trả `scenes.id` thật và đủ thông tin baseline/OCC/source (`Q4.5`); chốt chu trình streaming `started -> delta* -> completed | cancelled | failed` và quy tắc vô hiệu hóa vĩnh viễn generation bị hủy bởi Lock ngay cả sau khi Unlock (`Q4.6`); chốt giao thức retry/khôi phục khi kết quả Save mập mờ (`Q4.7`); đồng bộ tài liệu dẫn xuất (`Q4.8`); và chốt hợp đồng UX tối thiểu cho `EditScope` (`Q4.9`).
  * **Chuẩn hóa danh mục `[SCHEMA FOLLOW-UP]`:** Phân tách rõ các quy tắc nghiệp vụ đã chốt (`APPROVED`) với 3 đề xuất bổ sung schema tối thiểu cần phê duyệt riêng trước khi sửa `docs/db.dbml` (`draft_versions.seal_key`, bảng biên nhận hoàn tất workflow `workflow_task_receipts`, và bảng/cột idempotency cho Save Chapter). Không tự ý sửa `docs/db.dbml`.

## CHANGELOG v2.4.5

* **Chốt các làm rõ nghiệp vụ cuối cùng và áp dụng 4 bổ sung Schema tối thiểu (`SF-01`, `SF-02`, `SF-06`, `SF-07`) vào `docs/db.dbml` sau khi vượt qua toàn bộ cổng kiểm chứng:**
  * **Chuẩn hóa 46 Master Attributes, Composite Provenance và `world_setting_code` (mục 3.1):** Loại trừ chính xác 4 mục khỏi danh sách 50 của FE (`arc_tolerance`, `chapter_tolerance`, `word_per_chapter_tolerance`, và `relationship.motif.event_blocks` đang tạm treo); chốt quy tắc `Composite Provenance` tất định (`confirmed > ai_suggested > incomplete`, không tính điểm trọng số, không tính đồng thời vào cả hai chỉ số); chốt giữ `world_setting_code` làm yêu cầu bắt buộc trước `commit_draft` mà không làm tăng mẫu số 46.
  * **Áp dụng `SF-01` (`draft_versions.seal_key`):** Bổ sung cột `seal_key varchar` (nullable cho dữ liệu legacy) và unique index `(draft_session_id, seal_key)` trong `docs/db.dbml`, đối chiếu ngữ nghĩa thao tác và `data_hash` khi retry.
  * **Áp dụng `SF-02` (`chapter_save_receipts`):** Bổ sung bảng biên nhận `chapter_save_receipts` với unique index `(chapter_id, client_save_id)`, lưu cùng transaction với `scenes` và `scene_versions` theo cơ chế insert-once.
  * **Áp dụng `SF-06` (`workflow_task_receipts`):** Bổ sung bảng biên nhận hoàn tất bất biến `workflow_task_receipts` với unique index `(draft_session_id, workflow_run_id, plan_generation, task_identity)` (không lặp `workflow_run_id` bên trong `task_identity`), cơ chế insert-once (`ON CONFLICT DO NOTHING` + verify), bắt buộc kiểm tra biên nhận trước khi tạo `draft_versions` mới, và xác thực an toàn qua Restore bằng `input_scope_fingerprint` + `output_scope_fingerprint`.
  * **Áp dụng `SF-07` (`scene_versions.parent_id NOT NULL`):** Siết ràng buộc `not null` cho `scene_versions.parent_id` trong `docs/db.dbml`.

## CÁC QUYẾT ĐỊNH NGHIỆP VỤ ĐÃ CHỐT (`Q1..Q4`) & TRẠNG THÁI SCHEMA ĐÃ PHÊ DUYỆT

> **Trạng thái tổng thể (v2.4.5):** Toàn bộ các câu hỏi nghiệp vụ (`Q1`, `Q2`, `Q3`, `Q4`, `H1..H3`) và 4 hạng mục schema tối thiểu (`SF-01`, `SF-02`, `SF-06`, `SF-07`) đã được phê duyệt và cập nhật đồng bộ vào `docs/business/core.md` cùng `docs/db.dbml`. Các phần việc còn lại thuộc tầng đặc tả và triển khai kỹ thuật (`[IMPLEMENTATION PENDING]`).

### Q1. Cascade Delete cho toàn bộ quan hệ trong Working State — `[RESOLVED / APPROVED v2.4.4]`

* **Quyết định nghiệp vụ đã chốt (đã tích hợp vào mục 1.3):**
  1. **`world_setting` đang được Scene tham chiếu (`scenes.world_setting_id NOT NULL`):** Không tự xóa và không tự gán lại địa điểm cho cảnh. Trả `status = rejected`, `error = referenced_entity_in_use`. Với cây con `world_settings`, kiểm tra tham chiếu trên toàn bộ các nút hậu duệ trong cây con.
  2. **`lore_entry` đang được `scene_lore_entries` tham chiếu:** Cho phép xóa kèm các liên kết `scene_lore_entries` phụ thuộc (không xóa Scene). Phải liệt kê toàn bộ liên kết bị ảnh hưởng, kiểm tra provenance bảo vệ (`user`/`user_edited`), tạo preview đầy đủ, yêu cầu xác nhận khi chạm dữ liệu được bảo vệ, và áp dụng nguyên tử.
  3. **`character` đang được `theme_arguments` tham chiếu:** Chỉ cho phép gỡ liên kết `theme_arguments` phụ thuộc sau khi phân tích phụ thuộc, kiểm tra provenance bảo vệ và xác minh không vi phạm bất biến nghiệp vụ của `themes`; nếu vi phạm bất biến, trả `rejected` yêu cầu người dùng xử lý chủ đề trước. Không tự ý sửa nội dung luận điểm hay gán nhân vật thay thế.
  4. **`character` đang được `relationships` tham chiếu:** Không tự xóa thực thể quan hệ. Trả `status = rejected`, `error = referenced_entity_in_use` và yêu cầu dọn dẹp quan hệ trước.
  5. **`scene` đang được `relationship_states.scene_id` tham chiếu:** Cascade xóa các bản ghi `relationship_states` thuộc cảnh đó (sau khi kiểm tra provenance của mọi bản ghi bị ảnh hưởng); giữ nguyên thực thể `relationships` cha.
  6. **Thực thể dùng chung (Shared entities):** Không cascade xuyên ranh giới project; bảo toàn thực thể còn được project khác tham chiếu.
  7. **Mô hình xóa trong Draft & Kiểm tra bất biến:** Xóa vật lý (physical removal) khỏi `WorkingState` (không dùng tombstone cho V1; khôi phục qua `draft_versions`). Áp dụng đầy đủ 9 bước bảo vệ bắt buộc tại mục 1.3.
* **Trạng thái triển khai:** `[IMPLEMENTATION PENDING]` trong các tool handler `delete_*` của backend. Không yêu cầu sửa `docs/db.dbml`.

### Q3. Resume workflow, định danh task, biên nhận hoàn tất và bộ đếm `_counters` — `[RESOLVED / APPROVED v2.4.4..v2.4.5]`

* **Quyết định nghiệp vụ đã chốt (đã tích hợp vào mục 3.9):**
  1. **Định danh Task ổn định (`5.1`):** Mỗi lần chạy có `workflow_run_id`; danh tính biên nhận gồm `(draft_session_id, workflow_run_id, plan_generation, task_identity)` với `task_identity` chuẩn hóa từ `(phase, subphase, task_kind, assigned_scope)` (không nhúng lặp `workflow_run_id` vào `task_identity`). Không đổi định danh khi retry; khi kế hoạch phân đoạn tính lại hoặc chạy lại task stale thì tạo `plan_generation` mới; không dùng `display_name` hay array index làm định danh bền vững.
  2. **Biên nhận hoàn tất bền vững & Tách biệt 5 tầng (`5.2`):** Tách biệt `Business State`, `Draft Version`, `Workflow Completion Receipt`, `Temporary Task Report`, và `Runtime Execution Status`. Nguồn sự thật để khôi phục Business State luôn là `draft_versions`.
  3. **Tính nguyên tử, Insert-Once & Kiểm tra trước khi tạo Version (`5.3`):** Kiểm tra biên nhận hiện có trong `workflow_task_receipts` trước khi tạo `draft_versions` mới; dùng insert-once (`ON CONFLICT DO NOTHING` + verify, cấm `INSERT OR REPLACE`). Với vòng có mutation: niêm phong `draft_versions` và ghi Completion Receipt có chung ranh giới commit nguyên tử. Với vòng có 0 mutation: không tạo `draft_versions` giả, nhưng ghi bền vững Completion Receipt (`had_mutations = false`, `resulting_version_seq = NULL`).
  4. **Kiểm tra hiệu lực khi Restore và Resume (`5.4`):** Không dùng phép so sánh `V_current > V_receipt` đơn thuần. Xác thực biên nhận dựa trên `assigned_scope`, `input_scope_fingerprint`, `output_scope_fingerprint`, `plan_generation` và `task_identity`.
  5. **Ưu tiên khi Report và State lệch nhau (`5.5`):** Sealed Business State trong `draft_versions` thắng tuyệt đối; report tạm chỉ dùng lại khi khớp định danh và fingerprint đầu vào.
  6. **Chống trùng lặp & Đồng thời (`5.6`, `5.7`, `5.8`):** Từ chối trùng lặp kỹ thuật bằng `code` và ràng buộc hiện có; xử lý trùng lặp ngữ nghĩa bằng bước gộp tường minh (không tự gộp dữ liệu người dùng được bảo vệ); áp dụng mutation của các subagent theo thứ tự tất định (không phụ thuộc thứ tự hoàn tất mạng; không giữ khóa DB qua lệnh chờ mạng LLM); lưu `_counters` trực tiếp trong snapshot `BusinessFlowData` và khôi phục cùng State khi Restore.
* **Trạng thái Schema (`[SCHEMA APPROVED & APPLIED v2.4.5]`):** Đã bổ sung `draft_versions.seal_key` (`SF-01`) và bảng `workflow_task_receipts` (`SF-06`) vào `docs/db.dbml`.

### Q4. Hợp đồng thực thi Writing Mode — `[RESOLVED / APPROVED v2.4.4..v2.4.5]`

* **Quyết định nghiệp vụ đã chốt (đã tích hợp vào mục 1.6):**
  1. **Idempotency bền vững ở cấp request cho Save Chapter (`Q4.1`):** Gắn mỗi lần Save của người dùng với `client_save_id` ổn định; bảo đảm 5 tính chất bền vững (cùng ID cùng payload trả kết quả gốc; cùng ID khác payload trả `idempotency_conflict`; khác ID xử lý theo OCC; crash sau commit trước response khi retry không tạo trùng version; retry trễ sau một Save khác không bao giờ ghi đè nội dung mới hơn).
  2. **Optimistic Concurrency Control (`Q4.2`):** Dùng lịch sử `scene_versions` hiện có (`expected_version_number == COALESCE(MAX(version_number), 0)`) kết hợp kiểm tra `scenes.is_locked == false`, danh tính `scenes.id`, quyền sở hữu chương và quyền người dùng trong một SQLite transaction nguyên tử. Từ chối toàn bộ Save cấp chương và giữ nguyên dirty content nếu bất kỳ cảnh dirty nào không đạt. `Lock -> Unlock` thuần túy không tạo xung đột nội dung; generation từng bị hủy bởi Lock giữ nguyên trạng thái vô hiệu hóa sau khi Unlock. Không cần thêm `scenes.revision` cho V1.
  3. **Nguồn gốc cảnh hiện tại (`Q4.3`):** Suy ra `current_scene_source` trực tiếp từ bản ghi `scene_versions` mới nhất của cảnh; không thêm `scenes.current_source` cho V1.
  4. **Audit Lock (`Q4.4`):** Không tạo bảng/lịch sử audit lock cho V1; chỉ lưu `scenes.is_locked` hiện hành; thao tác khóa không tạo `scene_versions`.
  5. **Read-model Scene Identity (`Q4.5`):** Trả `scenes.id` và `position` thật cùng `content`, `word_count`, `is_locked`, `head_version_number` và `current_scene_source`.
  6. **Generation Streaming (`Q4.6`):** Chu trình `started -> delta* -> completed | cancelled | failed`; metadata phân biệt rõ `request_id`, `generation_id`, `scene_id`, `baseline_version_number`, `authorization_valid` và `status`; Lock khi đang stream làm vô hiệu hóa generation đó vĩnh viễn (giữ text tạm để xem nhưng cấm Accept kể cả sau khi Unlock); hết hạn `EditScope` không làm hỏng kết quả đã hoàn tất hợp lệ.
  7. **Save Retry & Recovery (`Q4.7`):** Khi kết quả Save mập mờ, giữ nguyên dirty state, không tự sinh `client_save_id` mới, truy vấn kết quả bền vững của ID gốc trước khi retry; trả conflict nếu không còn an toàn; không hứa hẹn khôi phục buffer RAM qua tắt/mở app khi chưa có durable draft/autosave.
  8. **Đồng bộ tài liệu (`Q4.8`):** `core.md`, `fe-core.md`, `tools.md`, `overview.md`, `case_newchat.md` đã đồng bộ hoàn toàn với `H1..H3`, `Decision 1B` và `Decision 2A`.
  9. **Hợp đồng UX tối thiểu cho `EditScope` (`Q4.9`):** Cảnh đang chọn có thể được gợi ý làm scope nhưng người dùng phải ủy quyền tường minh; không có scope thì chỉ chạy chế độ tư vấn (advisory-only); mở rộng scope cần xác nhận tường minh; duyệt scope không mở khóa cảnh; quyền chỉ có hiệu lực trong request hiện tại.
* **Trạng thái Schema (`[SCHEMA APPROVED & APPLIED v2.4.5]`):**
  * **`SF-02` (`chapter_save_receipts`):** Đã bổ sung bảng `chapter_save_receipts` với unique index `(chapter_id, client_save_id)` vào `docs/db.dbml`.
  * **`SF-07` (`scene_versions.parent_id NOT NULL`):** Đã siết `not null` cho `scene_versions.parent_id` trong `docs/db.dbml`.
* **Trạng thái triển khai (`[IMPLEMENTATION PENDING]`):** Các IPC command/channel của Writing Mode, H2 Commit transaction và continuity API sẽ được đặc tả kỹ thuật chi tiết trong `docs/business/be-core.md`.

### Đã chốt trong v2.4..v2.4.4

H1. **[APPROVED]** Hợp đồng Writing Mode tại mục 1.6 (bao gồm đầy đủ `SL-01..12`, `LOCK-01..07`, `SCOPE-01..06`, và `Q4.1..Q4.9`).
H2. **[APPROVED]** Luồng commit sau Phase 6 tại mục 3.5 và 3.7: Commit Confirmation Dialog đồng thời là plan review; chặn khi validation/pending patches; pending không đóng được; idempotent; failure giữ draft; success đóng Draft Workspace và điều hướng Project Detail mặc định View. Triển khai FE/backend còn `[IMPLEMENTATION PENDING]`.
H3. **[APPROVED]** Project View/Creator tại `fe-core.md` §9: Creator chỉ chọn chương, không nhúng scene editor; Chapter Detail là Writing Mode surface; route và interaction chi tiết thuộc FE contract, không phải quy tắc nghiệp vụ mới.

**Tạm treo theo xác nhận:** Chưa thiết kế bảng/liên kết cho `relationship_phase.motif.event_blocks[]` (không tính vào 46 thuộc tính Master list của Progress Engine).
