# Danh mục Tool Call của Harness

> **Phiên bản:** đồng bộ với core.md v2.4.5 và hợp đồng SF-06 B+ bổ sung (đặc tả, chưa xác minh runtime)
> **Vai trò:** Nơi tổng hợp duy nhất các tool call cần có. Quy tắc vận hành nằm ở [core.md](../core.md) (mục 1.3, 1.4, 3.6, 3.8, 3.9); schema dữ liệu nằm ở [db.dbml](../../db.dbml); cơ chế ghép prompt và cấp tool nằm ở [overview.md](../prompts/overview.md). Khi `core.md` cần liệt kê tool, chỉ link tới file này, không chép lại danh sách.
> **Lưu ý:** Tên tool là tên ví dụ, chưa chốt. Tham số lấy từ `db.dbml`; trường chỉ có ở draft được ghi rõ.

---

## 1. Cách đọc tài liệu

* **Nhóm ưu tiên:** Nhóm 1 *Phải có* (chặn luồng Phase 0-2 và commit), Nhóm 2 *Cần, sẽ phát triển* (Phase 3-6 và workflow phân đoạn), Nhóm 3 *Nên có, làm sau cùng* (dự phòng, hoãn hoặc chưa có đặc tả).
* **Client:** `AI` là tool được cấp cho model qua `prompt_block_tools`; `UI/CLI` là thao tác của người dùng, **không** cấp cho AI; `AI, UI` nghĩa là cùng một bề mặt tool, UI gọi với `source = user` (đúng triết lý CLI-first, core 1.4).
* **Loại:** `Đọc` không tạo version; `Ghi` (`add_`, `set_`) và `Xóa` (`delete_`) chỉ góp vào version cuối lượt khi được `applied` (core 3.6).
* **Tham số:** `*` là bắt buộc, `?` là tùy chọn. `*_code` là tham chiếu bằng `code` ở tầng draft, được ánh xạ sang `*_id` khi commit (core 1.4). Tên trường giữ đúng cột trong `db.dbml`.
* **`declared_sources` (tool ghi nhiều trường):** tool `add_`/`set_` ghi vào State nháp có tham số bắt buộc `declared_sources`* là object ánh xạ tên tham số giá trị sang `user` hoặc `ai_generated`; tham số định danh (`code`, `*_code` dùng để chỉ đích) không cần khai báo. Handler kiểm tra từng khóa và gán `source` theo từng trường; trường có giá trị mà thiếu khóa nguồn hoặc nguồn sai chỉ trường đó bị `rejected`; handler không tự suy đoán nguồn (core 1.3). Danh sách tool: `add_project`/`set_project`, `add_character`/`set_character`, `add_arc`/`set_arc`, `add_chapter`/`set_chapter`, `add_scene`/`set_scene`, `add_world_setting`/`set_world_setting`, `add_lore_entry`/`set_lore_entry`, `set_scene_config`, `add_theme`/`set_theme`, `add_character_appearance`/`set_character_appearance`, `add_character_distinguishing_mark`, `add_character_psychological`/`set_character_psychological`, `add_character_memory`/`set_character_memory`, `add_character_competence`/`set_character_competence`, `add_competence_capability`, `add_competence_limitation`, `add_relationship`/`set_relationship`, `set_relationship_state`.
* **`declared_source` (xóa và liên kết nguyên tử):** tool `delete_*` và tool liên kết một payload nguyên tử có tham số bắt buộc `declared_source`* (`user` hoặc `ai_generated`) do AI khai báo; handler kiểm tra và gán `source` cho toàn payload, thiếu hoặc sai thì `rejected`. Danh sách tool: `add_project_genre`, `add_scene_character`, `add_scene_lore_entry`/`set_scene_lore_entry`, `set_arc_motif`, `add_arc_event_block`, `add_theme_argument`, `add_relationship_role`, `set_project_character`, `add_character_motif`, và toàn bộ tool `delete_*`. Các bảng dưới không lặp lại tham số này ở từng dòng.
* **Tool luồng viết sau commit:** `write_scene_content`, `lock_scene`, `unlock_scene`, `add_scene_version` không ghi vào State nháp, không góp version của draft và không có `declared_sources` hoặc `declared_source`; việc phân loại tool ghi State nháp dựa trên metadata của tool registry, không chỉ dựa trên tiền tố `add_`. Các thao tác này thuộc Writing Mode sau plan commit, không thuộc Planning Mode. Theo hợp đồng Writing Mode và SL-01..SL-12 (core 1.6), **không cấp cho AI bất kỳ tool nào** để đổi `is_locked`, mở khóa cảnh, vượt khóa cảnh, sửa cảnh ngoài `EditScope` của request hiện tại, hoặc persist trực tiếp `scenes.content` / `scene_versions` bỏ qua vòng đời Generate → Buffer tạm → Review → Accept → Dirty → Save cấp chương do người dùng điều khiển.
* **Chống patch trùng:** xét theo từng trường và revalidate row đang pending trước reuse. Nếu đã có patch `status = pending` còn hợp lệ cùng `(target_code, field_name, new_value)`, tool trả `patch_id` cũ cho trường đó và không tạo patch mới; row cũ không nhận origin/dependency của request mới, nếu caller cần chứng nhận origin mới thì báo conflict có cấu trúc; các trường còn lại xử lý bình thường. Tại một thời điểm chỉ tồn tại tối đa một patch `pending` cho mỗi `(draft_session_id, target_code, field_name)`, cưỡng chế ở handler, không thêm ràng buộc unique (core 1.3, 3.6).
* **Kết quả thống nhất:** tool ghi nhiều trường trả `{"status": <tổng>, "results": [{"field": "...", "status": "applied" | "pending_confirmation" | "rejected", "patch_id"?: "...", "error"?: "..."}]}`. `status` tổng là `applied` nếu có ít nhất một trường `applied`; nếu không, là `pending_confirmation` nếu có ít nhất một trường `pending_confirmation`; nếu không, là `rejected`. Tool ghi một trường, tool xóa và tool liên kết giữ khung cũ và có thể bỏ `results`; tool đọc có thể trả `applied` nhưng không đổi State (core 1.4.7).
* **Cấp `code`:** Handler cấp `code` cho thực thể mới và trả trong kết quả `applied`; các call phụ thuộc nhau gọi tuần tự (core 3.8).
* **Phạm vi code và collision:** trong mỗi draft `code` duy nhất toàn cục; DB unique theo từng bảng. Hai draft chưa commit có thể tạm trùng code; không có reservation table. Khi commit, handler recheck collision trong transaction; unique của DB là hàng rào cuối cùng; collision thì rollback, giữ draft và trả lỗi có cấu trúc (core 3.7, 3.8).
* **Cascade Delete (`Q1` — APPROVED v2.4.4):** `delete_*` áp dụng đầy đủ ma trận đã chốt tại core 1.3 (bao gồm: chặn xóa `world_setting` đang được cảnh tham chiếu với `referenced_entity_in_use`; chặn xóa `character` đang được `relationships` tham chiếu với `referenced_entity_in_use`; xóa `lore_entry` kèm liên kết `scene_lore_entries` sau khi kiểm tra provenance và tạo preview; xóa `scene` thì cascade xóa `relationship_states` thuộc cảnh đó; cấm cascade thực thể dùng chung xuyên project; xóa vật lý khỏi `WorkingState`). Cascade có tính bắc cầu, kiểm tra `source` và trả `pending_confirmation` khi chạm `user`/`user_edited`.
* **Start và pending patches:** `commit_draft` bị chặn khi còn `pending_patches`; không tự áp dụng, không tự từ chối, không bỏ qua (core 3.7).
* **Workflow cancel/resume:** không thêm tool mới. Hủy và resume là điều khiển của backend/orchestrator; tool chỉ chạy dưới khóa ghi và tuân thủ contract tại core 3.9.

## 2. Tổng quan

| Nhóm | Mục tiêu | Số tool |
| :--- | :--- | :---: |
| 1. Phải có | Thu thập Phase 0, dựng Arc/chương/cảnh thô, tra cứu bắt buộc, thao tác duyệt/commit của người dùng | 28 |
| 2. Cần, sẽ phát triển | World/lore, cấu hình cảnh, hồ sơ nhân vật 5.1-5.3, quan hệ, danh mục tra cứu, workflow phân đoạn | 50 |
| 3. Nên có, sau cùng | Dự phòng ngữ cảnh lớn, mở rộng liên kết, luồng viết sau commit (hoãn), chưa đặc tả | 8 (chưa tính tool `symbolic` chưa đặc tả) |

Số tool đếm theo từng tên (mỗi cặp `add_` / `set_` / `delete_` tính riêng). Trong mọi flow chính (Phase 0 và 6 phase), AI được cấp đủ bộ tool ghi theo thực thể (core 1.4.9); sự khác biệt giữa flow nằm ở prompt block.

---

## 3. Nhóm 1: Phải có

### 3.1 Tool đọc và tra cứu

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `get_tool_definition` | AI | Đọc | `tools`, `prompt_block_tools` của run hiện tại | `tool_name`* | Trả `description`, `input_schema`, `examples` cho tool đã được cấp. Không cấp thêm quyền, không trả placeholder chưa có metadata; không ghi State hoặc tạo version. |
| `get_field_null` | AI | Đọc | State (không có bảng) | `limit?` (tối đa 3) | Trả tối đa 3 cặp `field`, `question` cố định theo `system_language` do backend biết; AI không truyền ngôn ngữ (core 3.1, 3.8). |
| `get_array_element_fields` | AI | Đọc | State (không có bảng) | `array_path`*, `code?`, `limit?` (tối đa 3) | Mảng rỗng không phải `null`: có `code` thì trả trường `null` của phần tử, không có `code` thì trả trường ưu tiên để tạo phần tử mới; mỗi trường kèm câu hỏi theo `system_language` (core 3.1). |
| `get_state` | AI | Đọc | State (không có bảng) | `target_type?`, `code?`, `fields?` | Trả State hoặc phần State theo thực thể, kèm `source` và `pending_patches` liên quan. |
| `search_character` | AI | Đọc | `characters`, `project_characters` | `query`*, `limit?` | Chuẩn hóa NFC, ưu tiên khớp chính xác, rồi tiền tố/chuỗi con, rồi không dấu. Trả `code`, `display_name`, `nickname`. 1 kết quả: dùng luôn; nhiều: hỏi lại; 0: tìm toàn thư viện với cờ `in_project`, hỏi trước khi liên kết qua `project_characters` (core 3.8). |
| `search_genre` | AI | Đọc | `genres` | `query?`, `limit?`, `cursor?` | Danh mục tĩnh, tra theo kiểu JIT. Trả `code`, `display_name`, `description`. |
| `search_motif` | AI | Đọc | `motifs`, `motif_applicable_genres` | `category?` (`MotifsType`), `genre_code?`, `query?`, `limit?`, `cursor?` | Trả `code`, `display_name`, `category`, `description`, `min_chapter`, `max_chapter`. |

### 3.2 Tool ghi theo thực thể (Phase 0, 1, 2)

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `add_project` / `set_project` | AI, UI | Ghi | `projects` | `display_name?`, `summary?`, `language?` (`Language`), `target.arc_count?`, `target.chapter_count?`, `target.word_per_chapter_count?` | `target.*` chỉ có ở draft, không có cột trong `projects`. `code` do handler cấp từ `display_name`. `status`, `cover` chưa nằm trong Business Flow. |
| `add_project_genre` / `delete_project_genre` | AI, UI | Ghi / Xóa | `project_genres` | `genre_code`*, `is_primary?` | Cần ít nhất 1 thể loại để qua gating Phase 0. Tách tool riêng vì là trường dạng danh sách (bảo vệ `source` theo từng phần tử). [Đề xuất tách] |
| `save_approved_outline` | AI | Ghi | `decision_log` | `summary`*, `declared_sources.summary`* | Gọi khi tác giả duyệt dàn ý/quyết định; entry được workflow và chat đọc lại như bàn giao bền giữa các phase. |
| `add_character` / `set_character` / `delete_character` | AI, UI | Ghi / Xóa | `characters`, `project_characters`, `character_motifs` | `display_name?`, `nickname?`, `biological_sex?` (`gender`), `motif_code?`, `project_role?`, `project_notes?`; `set_` thêm `species?`, `birth?`, `gender_identity?`, `gender_expression?` | `display_name` và `nickname` không đồng thời `null`. `add_` tạo luôn liên kết `project_characters` (`role`, `notes`). `delete_` kéo theo hồ sơ và quan hệ nên có thể trả `pending_confirmation`. |
| `add_arc` / `set_arc` / `delete_arc` | AI, UI | Ghi / Xóa | `arcs` | `display_name?`, `summary?`, `position?` | Phase 1. Motif, event block và theme của Arc xem nhóm 2. |
| `add_chapter` / `set_chapter` / `delete_chapter` | AI, UI | Ghi / Xóa | `chapters` | `arc_code`* (khi thêm), `display_name?`, `summary?`, `position?` | `position` là thứ tự toàn truyện, duy nhất theo project (backend validate qua `arcs.project_id`). |
| `add_scene` / `set_scene` / `delete_scene` | AI, UI | Ghi / Xóa | `scenes` | `chapter_code`* (khi thêm), `display_name?`, `summary?`, `position?` | Phase 2 (cảnh thô); `position` là thứ tự trong chương. Địa điểm chính gán ở Phase 4 qua `set_scene_config`. |

### 3.3 Thao tác của người dùng (UI/CLI, không cấp cho AI)

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `confirm_patch` / `reject_patch` | UI/CLI | Ghi | `pending_patches` | `patch_id`* | Xác nhận thì áp dụng và tạo version riêng (`ai_patch`); từ chối thì hủy patch và ghi event log. Idempotent: gọi lại trên patch đã xử lý trả kết quả hiện có, không tạo version mới. Không cấp cho AI. |
| `confirm_all_patches` | UI/CLI | Ghi | `pending_patches` | `draft_session_id`* | **Áp dụng tất cả** dưới khóa ghi draft: duyệt patch `pending` theo `created_at` tăng dần, revalidate từng patch, bỏ qua patch lỗi, trả `applied`/`skipped`, tạo đúng một version `ai_patch` nếu có ít nhất một patch được áp dụng. Không cấp cho AI. |
| `restore_version` | UI/CLI | Ghi | `draft_versions` | `seq`* | Append-only; khôi phục cả `phase` và `subphase`; mọi patch `pending` chuyển `expired`. Không cấp cho AI. |
| `confirm_phase` | UI/CLI | Ghi | `draft_versions` | `phase`*, `subphase?` | Tạo version `phase_confirm`. |
| `commit_draft` | UI/CLI | Ghi | `draft_sessions` + các bảng nghiệp vụ | `draft_session_id`* | Start: chỉ nhận khi `editing`; chặn nếu còn `pending_patches`; chuyển `committing`, giữ khóa ghi draft và đọc snapshot tại `head_version_seq`; validate rồi ghi một transaction (core 3.7). Khi commit, kiểm tra lại collision `code`; collision thì rollback, giữ draft và trả lỗi có cấu trúc. Start lặp khi `committing` hoặc `committed` trả kết quả hiện có. |

---

## 4. Nhóm 2: Cần, sẽ phát triển

### 4.1 Danh mục tra cứu (đọc)

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `search_event_block` | AI | Đọc | `event_blocks`, `components`, `states` | `motif_code`*, `query?`, `limit?` | Trả `code`, `motif_code`, `component`, `state`. |
| `search_style_preset` | AI | Đọc | `style_presets` | `category?`, `query?`, `limit?`, `cursor?` | Trả `code`, `category`, `tone_pace`, `tone_emotion`, `is_system`. |
| `search_narrator_config` | AI | Đọc | `narrator_configs` | `person?` (`GrammaticalPerson`), `pov_scope?` (`PovScope`), `query?`, `limit?` | Trả `code`, `display_name`, `person`, `person_subtype`, `pov_scope`, `is_reliable`, `breaks_fourth_wall`. |
| `search_appearance_option` | AI | Đọc | `appearance_option_groups`, `appearance_options` | `group_code`*, `query?`, `limit?` | Trả `code`, `display_name`, `hex`. Nhóm gồm tóc, mắt, da... theo `appearance_option_groups`. |
| `search_competence_domain` | AI | Đọc | `competence_domains`, `domain_level` | `query?`, `limit?` | Trả `code`, `display_name`, `is_system` và các cấp độ (`domain_level.code`, `display_name`). |
| `search_relationship_category` | AI | Đọc | `relationship_categories` | `category?` (`CategoryGroup`), `query?`, `limit?` | Trả `code`, `category`, `sub_type`. |

### 4.2 Tool ghi theo thực thể (Phase 1, 3, 4, 5, 6)

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `add_world_setting` / `set_world_setting` / `delete_world_setting` | AI, UI | Ghi / Xóa | `world_settings` | `parent_code?`, `display_name`, `type` (`LocationType`), `description?`, `access?` (`LocationAccess`), `spatial_features?`, `sensory_visual?`, `sensory_auditory?`, `sensory_tactile?`, `sensory_olfactory?`, `sensory_gustatory?` | Phase 3. Ghi cha trước con. |
| `add_lore_entry` / `set_lore_entry` / `delete_lore_entry` | AI, UI | Ghi / Xóa | `lore_entries` | `world_setting_code?`, `type` (`LoreType`), `display_name`, `description?`, `data?` | `data` là JSON object theo shape của `type` (core 2.4). |
| `set_scene_config` | AI, UI | Ghi | `scenes` | `code`*, `style_preset_code?`, `narrator_config_code?`, `world_setting_code?` | Phase 4. `world_setting_code` ánh xạ `scenes.world_setting_id`; bắt buộc trước commit (core 2.5, 2.8). [Đề xuất tách khỏi `set_scene`] |
| `add_scene_character` / `delete_scene_character` | AI, UI | Ghi / Xóa | `scene_characters` | `scene_code`*, `character_code`*, `is_pov?` | Ngôi giới hạn: đúng 1 `is_pov`; ngôi toàn tri: 0 (core 2.8). |
| `add_scene_lore_entry` / `delete_scene_lore_entry` | AI, UI | Ghi / Xóa | `scene_lore_entries` | `scene_code`*, `lore_entry_code`*, `usage?` (`LoreUsageType`), `reveal_to_reader?`, `reveal_to_pov?` (`RevealDepth`) | Phase 4; AI sinh `usage` khi gán lore. Chỉ đặt `reveal_*` với `FORESHADOW`, `PAYOFF`, `REVEAL`, `RED_HERRING`; ngôi toàn tri bỏ qua `reveal_to_pov` (core 2.5). |
| `set_scene_lore_entry` | AI, UI | Ghi | `scene_lore_entries` | `scene_code`*, `lore_entry_code`*, `usage?` (`LoreUsageType`), `reveal_to_reader?`, `reveal_to_pov?` (`RevealDepth`) | Tên ví dụ, chưa chốt; cập nhật liên kết lore trong cảnh theo core 2.5. |
| `set_arc_motif` | AI, UI | Ghi | `arc_motifs` | `arc_code`*, `motif_code`*, `note?` | Mỗi Arc đúng một motif chủ đạo (`arc_id` unique). |
| `add_arc_event_block` / `delete_arc_event_block` | AI, UI | Ghi / Xóa | `arc_event_blocks` | `arc_code`*, `event_block_code`*, `start_chapter_code`*, `end_chapter_code`*, `summary?` | Khoảng chương phải nằm trong Arc (core 2.8). |
| `add_theme` / `set_theme` / `delete_theme` | AI, UI | Ghi / Xóa | `themes` | `arc_code?`, `parent_theme_code?`, `question`, `resolution_type?` (`ThemeResolutionType`), `resolution?`, `message?` | Phase 1: chủ đề triết lý của Arc. |
| `add_theme_argument` / `delete_theme_argument` | AI, UI | Ghi / Xóa | `theme_arguments` | `theme_code`*, `character_code`*, `role` (`ThemeArgumentRole`), `answer_mode?` (`ThemeAnswerMode`), `note?`, `required_knowledge?`, `required_competence?` | Luận điểm do nhân vật đại diện. |
| `add_character_appearance` / `set_character_appearance` | AI, UI | Ghi | `character_appearances` | `character_code`*, `height?`, `height_unit?`, `weight?`, `weight_unit?`, `hair_color_code?`, `hair_length_code?`, `hair_style_code?`, `hair_texture_code?`, `pupil_shape_code?`, `iris_color_code?`, `sclera_color_code?`, `skin_tone_code?`, `skin_texture_code?`, `hair_note?`, `eyes_note?`, `skin_note?`, `outfit_style?`, `default_summary?` | Sub-phase 5.1, 1:1 với nhân vật. Các `*_code` lấy từ `search_appearance_option`. |
| `add_character_distinguishing_mark` / `delete_character_distinguishing_mark` | AI, UI | Ghi / Xóa | `character_distinguishing_marks` | `character_code`*, `location?`, `content`* | Sẹo, nốt ruồi, ấn ký ma thuật. |
| `add_character_psychological` / `set_character_psychological` | AI, UI | Ghi | `character_psychological` | `character_code`*, `the_wound?`, `the_lie?`, `fundamental_fear?`, `moral_alignment?`, `moral_code?`, `cognitive_bias?`, `breaking_point?`, `external_want?`, `internal_need?`, `internal_conflict?`, `persona_type?`, `verbal_style?`, `primary_reaction?`, `stress_trigger?`, `coping_strategy?` | Sub-phase 5.2, 15 cột `TEXT`, 1:1 với nhân vật. |
| `add_character_memory` / `set_character_memory` / `delete_character_memory` | AI, UI | Ghi / Xóa | `character_memories` | `character_code`*, `content`*, `source_scene_code?` | Sub-phase 5.3; tách riêng theo từng loại bản ghi ký ức. |
| `add_character_competence` / `set_character_competence` / `delete_character_competence` | AI, UI | Ghi / Xóa | `character_competences` | `character_code`*, `domain_code`*, `level_code?`, `scene_code`*, `display_name`*, `description?`, `reliability?`, `experience?` | Sub-phase 5.3. `scene_id` là `not null` trong DBML. |
| `add_competence_capability` / `delete_competence_capability` | AI, UI | Ghi / Xóa | `competence_capabilities` | `competence_code`*, `content`* | Khả năng làm được. |
| `add_competence_limitation` / `delete_competence_limitation` | AI, UI | Ghi / Xóa | `competence_limitations` | `competence_code`*, `content`* | Giới hạn không làm được. |
| `add_relationship` / `set_relationship` / `delete_relationship` | AI, UI | Ghi / Xóa | `relationships` | `from_character_code`*, `to_character_code`*, `motif_code?` | Phase 6. Cặp `(from, to)` là duy nhất. |
| `set_relationship_state` | AI, UI | Ghi | `relationship_states` | `relationship_code`*, `scene_code`*, `trust?`, `affection?`, `hostility?`, `fear?`, `respect?`, `familiarity?`, `dependency?` | Mỗi chiều dùng `RelationDimensionLevel`. Lưu thưa: chỉ ghi tại cảnh có thay đổi (core 2.7). |
| `add_relationship_role` / `delete_relationship_role` | AI, UI | Ghi / Xóa | `relationship_roles` | `relationship_code`*, `category_code`*, `hierarchy?` (`Hierarchy`), `is_primary?` | Vai trò xã hội của quan hệ. |

### 4.3 Workflow phân đoạn

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `write_phase_report` / `read_phase_reports` | Subagent (qua backend) | Ghi / Đọc | Không có bảng; thư mục tạm `./merlink-harness/tmp/<draft_session_id>/<phase>/` | `draft_session_id`*, `phase`*, `report` | Workflow 3.9: report trung gian, không phải State, xóa khi workflow hoàn tất. Subagent không đụng file trực tiếp. Không ghi vào State nháp nên không có `declared_sources` hoặc `declared_source`. |

* Hủy workflow và resume không cần tool mới: backend đánh dấu yêu cầu hủy, chờ mutation đang chạy, seal round header + evidence và tối đa một version nếu vòng hoàn tất hợp lệ; nếu không hoàn tất, TX-04 không commit nhưng proposal TX-08 đã commit vẫn có thể tồn tại, giữ biên nhận hoàn tất bền vững để resume và giải phóng khóa (core 3.9). Report tạm không phải artifact bền; output `world_probe` được bước gộp tiêu thụ và exact merged input `world_write` tiêu thụ phải có artifact typed trong TX-04 của round tương ứng, không lưu toàn bộ LLM response.
* SF-06 tối thiểu đã `[SCHEMA APPROVED & APPLIED]` vào DBML; B+ bổ sung round header và child evidence trong đặc tả, **chưa migration/triển khai**. Input fingerprint S(i−1), output fingerprint Si; Restore phải kiểm lineage/applicability riêng. Pending proposal thuộc TX-08 độc lập, không chứng minh task/round completed. Tool trả `patch_id` sau commit TX-08; lúc confirm backend kiểm data validity và eligibility kể cả khi workflow fail, không tự áp dụng proposal.
* `commit_draft` là tool duy nhất của Start; không thêm tool commit riêng cho từng phase.

---

## 5. Nhóm 3: Nên có, làm sau cùng

| Tool (tên ví dụ) | Client | Loại | Bảng DBML | Tham số (theo DBML) | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `lookup_outline` | AI | Đọc | State (không có bảng) | `target_type?`, `query?`, `parent_code?`, `position?`, `limit?`, `cursor?` | Tên ví dụ, chưa chốt. Khi không thấy phần người dùng nhắc trong context: backend khớp xác định tên/vị trí trên toàn mục lục trước; nếu không ra hoặc tìm theo mô tả thì chia trang `OUTLINE_PAGE`, chạy subagent chỉ đọc theo vòng tối đa `MAX_PARALLEL`, gộp ứng viên có lý do ngắn. Trang lỗi trả `incomplete_pages`; không tạo version (core 3.8). |
| `set_project_character` | AI, UI | Ghi | `project_characters` | `project_code`*, `character_code`*, `role?`, `notes?` | Sửa vai trò và ghi chú sau khi đã thêm nhân vật; hiện `add_character` đã tạo liên kết lúc thêm. |
| `add_character_motif` / `delete_character_motif` | AI, UI | Ghi / Xóa | `character_motifs` | `character_code`*, `motif_code`*, `description?` | Nhiều motif cho một nhân vật; Business Flow hiện chỉ có một `motif_code`. |
| `write_scene_content` | UI/CLI (nội bộ `Save Chapter`; không cấp cho AI) | Ghi | `scenes` | `scene_code`*, `content`*, `word_count?` | Chỉ thuộc Writing Mode sau khi plan đã commit; một phiên chỉ viết một chương, không viết hàng loạt và không tự chuyển chương (core 1.5, 1.6). Không cấp cho AI: kết quả AI sinh (khi `is_locked == false` và thuộc `EditScope`) đi vào buffer tạm để người dùng Review/Accept (kể cả sau khi AI request đã kết thúc theo `SCOPE-02..04`) rồi mới được persist nguyên tử qua `Save Chapter` khi scene đang mở khóa (`LOCK-04`, `LOCK-05`). |
| `lock_scene` / `unlock_scene` | UI (chỉ người dùng; cấm cấp cho AI) | Ghi | `scenes` | `scene_code`* | Chỉ người dùng độc quyền đổi `is_locked` qua thao tác UI được ủy quyền; persist ngay lập tức xuống SQLite (`LOCK-01`), bảo toàn mọi nội dung dirty/buffer tạm (`LOCK-03`), không tạo `scene_versions` (`LOCK-07`), và nếu khóa khi đang streaming generation thì hủy stream ngay (`LOCK-04`). Cấm cấp cho AI. |
| `add_scene_version` | UI/CLI (nội bộ `Save Chapter`; không cấp cho AI) | Ghi | `scene_versions` | `scene_code`*, `scene_source`* (`SceneSource`), `summary?`, `content?` | Chỉ được tạo bên trong transaction `Save Chapter` khi nội dung scene thay đổi đã được người dùng Accept và Save (và không có scene dirty nào bị khóa theo `LOCK-05`); không cấp cho AI để ghi trực tiếp (core 1.6 `SL-06`, `SCOPE-05`). Số phiên bản `version_number` do handler cấp. |
| Tool cho `symbolic_motifs`, `symbolics` | AI, UI | Ghi | `symbolic_motifs`, `symbolics` | Chưa đặc tả | DBML có bảng nhưng 7 phase của Business Flow chưa dùng. |

### Không phải tool call (hàm thuần ở backend)

Các hàm dưới đây do backend/orchestrator chạy, **không** cấp cho AI nên không nằm trong `tools`:

| Hàm | Vai trò |
| :--- | :--- |
| `validate(draft)` | Kiểm tra bất biến trước commit; trả danh sách lỗi (core 2.8, 3.7). |
| `plan_outline_runs` | Kế hoạch các lần chạy Phase 1-2 theo `OUTLINE_CAP` và `OUTLINE_LOOKBACK` (core 3.9). |
| `plan_phase`, `split_balanced`, `context_range` | Chia task cân bằng và khoảng ngữ cảnh cho Phase 3-6; `plan_phase` và `split_balanced` còn chia trang tra cứu mục lục chỉ đọc (core 3.8, 3.9). |
| `get_chapter_window` | Chọn cửa sổ chương đưa vào context quanh `feedbackContext` (core 3.8). |
| `compose`, `get_prompt`, `get_tools_by_prompt` | Ghép prompt block và cấp tool theo flow ([overview.md](../prompts/overview.md)). |

---

## 6. Điểm chưa chốt ảnh hưởng tới danh mục tool

1. **Một hay nhiều trường mỗi lần `set_*`:** đã chốt trong core. Cho phép nhiều trường trong một call; handler trả `results[]` theo từng trường, áp dụng và báo cáo theo từng trường, không nguyên tử giữa các trường (core 1.4.7, 3.6).
2. **Trường dạng danh sách hoặc bảng nối** (thể loại, nhân vật trong cảnh, lore trong cảnh, luận điểm): đề xuất tách tool `add_`/`delete_` thay vì `set_` ghi đè cả danh sách, để bảo vệ `source` theo từng phần tử.
3. **`declared_sources`, `declared_source` và chống patch trùng:** đã chốt trong core. Tool ghi nhiều trường dùng `declared_sources`* theo từng trường; tool `delete_*` và tool liên kết một payload nguyên tử dùng `declared_source`*; danh sách tool theo từng loại ở mục 1. Chống patch trùng xét theo từng trường, tối đa một patch `pending` cho mỗi `(draft_session_id, target_code, field_name)` và cưỡng chế ở handler (core 1.3, 3.6).
4. **Event block của motif quan hệ** đang tạm treo (core), nên chưa có tool cho Phase 6 phần này.
5. **Tên lệnh UI/CLI** (`confirm_patch`, `confirm_all_patches`, `restore_version`, `commit_draft`...) chưa được thống nhất với các thao tác hiện có ở giao diện.
6. **Idempotent sealing (`SF-01`):** `draft_versions.seal_key` và unique `(draft_session_id, seal_key)` đã `[SCHEMA APPROVED & APPLIED]` trong DBML; không khẳng định migration/runtime đã chạy.
7. **Task identity và Completion Receipt (`SF-06`):** Bảng tối thiểu `workflow_task_receipts` đã `[SCHEMA APPROVED & APPLIED]` trong DBML; B+ owner-accepted mở rộng evidence/header tại core 3.9 và DBML (specification only). Không thêm tool hay `client_request_id` cho workflow V1.
8. **Cascade Delete cho toàn bộ quan hệ (`Q1` — APPROVED v2.4.4):** Đã chốt đầy đủ tại core 1.3 & Q1; các tool `delete_*` hiện có thực thi đúng ma trận và 9 bước bảo vệ mà không cần thêm tool xóa đặc thù mới.
