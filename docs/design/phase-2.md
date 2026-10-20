# Phase 2 - kiến trúc pipeline dàn ý

Tài liệu cho người đọc repo. Nguồn: `docs/plan/sprint-17.md` mục 3 đến 8.
Phạm vi: phase 2 (dàn ý). Phase 3 (viết văn) chỉ được nhắc ở phần lưu trữ.

## 0. Tóm tắt

Phase 1 điền 12 unit `require` qua chat. Phase 2 AI viết phần còn lại của state
qua 6 stage có gate, người dùng review từng stage rồi mới chốt. Phase 3 mới viết văn.

Ba tầng, không lẫn:

| Tầng | Là gì | Ai gọi |
| --- | --- | --- |
| Chat agent | agent loop model -> tool -> model, tối đa 5 round | người dùng chat |
| Merlink subagent | 1 call provider không tool, vào JSON ra JSON | backend Merlink |
| Backend apply | validate + ghi state theo `STATE_ORDER` | lệnh `commit` |

## 1. Sơ đồ stage và gate (ASCII)

```
require đủ 12/12
        |
      start          tạo run, seed anchor, stage = seed
        |
      seed           backend, không LLM
        |
      arcs           arc_smith, 1 call
        |
    chapters         chapter_smith, ceil(N/10) call
        |
   +----+----+
   |         |
   cast      world   cast_smith, world_smith (song song)
   |         |
   +----+----+
        |
     scenes          scene_binder, 1 call mỗi chương
        |
      ready          không ghi state, mở phase 3
```

Gate:

- `commit(stage)` chỉ chạy khi stage trước đã `committed`. Riêng `seed` không cần.
- `scenes` cần cả `cast` và `world` committed. Thiếu một bên là lỗi gate.
- Không có cờ override. Bỏ qua gate là lỗi, không phải cảnh báo.
- Mỗi `commit` là một transaction logic: validate hết, ghi hết, fsync.
  Không ghi một phần rồi bỏ dở.

## 2. Ai làm gì

| Việc | Chat agent | Merlink subagent | Backend apply |
| --- | --- | --- | --- |
| Chạy stage | gọi tool `outline` | không | có |
| Sinh đề xuất | không | 1 JSON mỗi call | không |
| Lưu đề xuất | không | không | `outline_items` |
| Ghi state | không | không | `apply` khi `commit` |
| Giữ thứ tự ghi | không | không | `STATE_ORDER` |
| Đổi trạng thái item | nhận lệnh tác giả | không | `accepted` hoặc `rejected` |
| Retry lỗi | không | không | 2 lần mỗi lỗi |
| Phát tiến độ | không | không | `OutlineEvent` |

Chat agent không tham gia fan-out. Nó chỉ gọi `outline` rồi đọc tóm tắt,
nhờ vậy giới hạn 5 tool round vẫn đúng.
Playbook `outline_playbook.md` được chèn khi `filled_required == required_total`.

## 3. Lane

Sáu lane của phase 2. Mỗi lane là 1 system prompt + 1 input JSON + 1 output JSON.
Không lane nào có tool loop. `prose_writer` là lane phase 3, không thuộc bảng này.

| Lane | Stage | Input | Output | Số call |
| --- | --- | --- | --- | --- |
| `arc_smith` | 2.1a | anchor + plan counts | `arcs[]`: `code`, `goal`, `chapters` | 1 |
| `chapter_smith` | 2.1b | anchor + arc + range | `chapters[]`: `code`, `scene_count` | ceil(N/10) |
| `cast_smith` | 2.2a | anchor + chapters | `characters[]`: `ref`, `memories` | ceil(M/6) |
| `world_smith` | 2.2c | anchor + chapters + genre | `settings`, `lore`, `power_systems` | 1-2 |
| `bond_smith` | 2.2b | cast đã commit | `relationships[]`: `from`, `to`, `states` | 1-2 |
| `scene_binder` | 2.3 | anchor + chapter + cast + settings | `scenes[]`: `cast`, `code` | 1/ch |

N = `plan.chapter.count`, M = số nhân vật cần sinh.

Thứ tự chạy:

1. `arc_smith` chạy trước, 1 call. Output chia `plan.chapter.count` cho từng arc.
2. `chapter_smith` chỉ chạy sau khi tác giả chốt arc. Shard mặc định 10 chương.
3. `cast_smith` và `world_smith` chạy song song sau khi chapters committed.
4. `bond_smith` chạy sau cast, vì quan hệ cần character đã commit.
5. `scene_binder` chạy sau cùng, mỗi chương 1 call, tối đa 5 chương song song.

Chi tiết input:

- Anchor là 6 đoạn từ `outline/anchor.rs`, trần 1.500 token.
- `chapter_smith` nhận tóm tắt chương trước để mạch liền giữa các shard.
- `cast_smith` nhận danh sách motif và competence domain từ catalog.
- `scene_binder` nhận catalog style preset, narrator, directive đã rút gọn.
- Tổng bundle của `scene_binder` không quá 6.000 token.

## 4. STATE_ORDER 25 bước

`outline/stage.rs::STATE_ORDER` là hằng số. `apply` đi theo danh sách này,
không theo thứ tự model trả về.

| # | Entity | Vì sao phải trước |
| --- | --- | --- |
| 1 | catalogs | tra `code` ra name/description khi build prompt |
| 2 | `project` leaves | phase 1 đã có, phase 2 không ghi |
| 3 | `project_genres` | `genre_code` phải tồn tại trong catalog genres |
| 4 | `arcs` | `chapters.arc_id` trỏ tới index của arc |
| 5 | `chapters` | `scenes.chapter_id` và `chapters.arc_id` |
| 6 | `characters` | mọi bảng `*_character` trỏ tới đây |
| 7 | `world_settings` | cha trước con: REALM -> ... -> ROOM |
| 8 | `lore_entries` | `world_setting_id` trỏ setting đã ghi |
| 9 | `power_systems` -> `power_tiers` -> `power_abilities` | tier cần system |
| 10 | `character_appearances`, `character_distinguishing_marks` | FK character |
| 11 | `character_motifs` | `motif_code` phải có trong catalog motifs |
| 12 | `competence_domains` -> `character_competences` | rồi capabilities |
| 13 | `memories` | `owner_id`; `source_scene_id` null ở bước này |
| 14 | `memory_emotions`, `memory_references`, `memory_connections` | FK memory |
| 15 | `relationships` | from/to trỏ character đã ghi |
| 16 | `relationship_states` | `parent_id` trỏ relationship |
| 17 | `relationship_roles` | `category_code` phải có trong catalog |
| 18 | `relationship_motifs` | cần relationship và motif |
| 19 | `themes` -> `theme_arguments` | argument trỏ character, theme trỏ arc |
| 20 | `project_motifs` | phải có >= 1 row `scope=project` |
| 21 | `scenes` | chapters + world_settings + catalog style/narrator |
| 22 | `scene_characters` | FK scene + character; cần >= 1 `is_pov=true` |
| 23 | `scenes.relationships` (patch) | relationship + scene_characters trước |
| 24 | `memories.source_scene_id` (patch) | giải vòng phụ thuộc với scenes |
| 25 | `scenes.content_path`, `word_count`, `status` | phase 3, văn bản sau cùng |

Ghi chú bước 12: sau `character_competences` mới tới `competence_capabilities`
và `competence_limitations`. Bước 9 tương tự: system -> tier -> ability.
Bước 10 gộp hai bảng vì cùng phụ thuộc `characters`.

### 4.1 Vòng phụ thuộc `memories.source_scene_id`

- `memories` sinh ở 2.2, `scenes` sinh ở 2.3. Chưa thể biết scene nguồn ở 2.2.
- Bước 13 ghi memory với `source_scene_id = null` (draft mode cho phép).
- Bước 24 là link pass: `apply` dùng `add_field` để gán `source_scene_id_json_index`
  cho từng memory, sau khi scene đã có index.
- Không đảo thứ tự: scene chưa tồn tại thì memory không có scene nguồn.
- Binder không xác định được scene nguồn thì để null và ghi warning. Không bịa.

Hai ghi chú thứ tự khác:

- `themes` có `arc_id`, nhưng `theme_arguments.character_id` cần cast.
  Vì vậy themes ghi ở bước 19, sau cast, dù nội dung đến từ `require`.
- `project_motifs` có `arc_id`/`chapter_id` optional. Ghi sau chapters nếu có ref.

## 5. Nơi lưu dữ liệu

| Nơi | Chứa | Ghi bởi | Ghi chú |
| --- | --- | --- | --- |
| `projects/<id>/super.json` | state committed | `apply` khi `commit` | v3, 34 bảng, 81 path |
| `merlink.db` `outline_runs` | 1 run mỗi stage | backend | stage, status, calls |
| `merlink.db` `outline_items` | đề xuất và duyệt | backend | payload, status, error |
| `projects/<id>/content/.../*.md` | văn bản phase 3 | phase 3 | 1 file mỗi scene |

Quy ước:

- `scenes.content` luôn null trong phase 2. Văn bản phase 3 nằm ở `content_path`.
- `content_path` dạng `content/arc-<a>/chapter-<c>/<position>-<slug>.md`.
- Bản cũ khi viết lại: `.../versions/<position>-v<n>.md`. Không có bảng SQLite
  cho version, chỉ file.
- `outline_items.payload_json` là JSON của lane, không phải state. Nó không được
  coi là nguồn sự thật.
- SQLite không có bảng entity. Không port `docs/schema/system.dbml` sang SQLite.
- `outline_runs.calls`, `tokens_in`, `tokens_out` dùng để chặn chi phí.

## 6. Lỗi thường gặp và cách xử lý

| Lỗi | Dấu hiệu | Xử lý |
| --- | --- | --- |
| Contract sai | `parse` fail, item `failed` | retry 1 lần kèm lỗi cũ |
| Lệch số lượng | `commit` báo `3 arc, plan 4` | xin range nhỏ hơn |
| Code ngoài catalog | `style_preset_code` không có | chọn code trong catalog |
| Ghi sai thứ tự | `chương ch-004 chưa commit` | commit chapters trước |
| Thiếu POV | scene không có `is_pov=true` | regenerate scene đó |
| Ref chết | `cast[0].character` không có | regenerate với cast đã commit |
| Gate chặn | stage trước chưa `committed` | chạy stage trước, không override |
| Prose vi phạm | có emoji hoặc U+2013 | retry, rồi sanitize, đánh dấu |

Chi tiết:

- Contract sai: `contract::parse` chấp nhận fence và sửa trailing comma. Nếu vẫn
  sai, retry đúng 1 lần với error feedback trong user message. Vẫn sai thì item
  `failed`, các shard khác vẫn chạy.
- Lệch số lượng: `commit` in ra lệch bao nhiêu. Không tự tăng `max_calls`,
  không tự đổi `plan.*.count`. Đề xuất range nhỏ hơn cho tác giả chọn.
- Code ngoài catalog: lỗi nêu đúng field và code. Chọn code khác có trong catalog,
  không đoán code gần giống.
- Ghi sai thứ tự: message có path dạng `scenes[3].chapter_id: chương ch-004 chưa
  commit`. Đọc đúng path, commit stage thiếu, rồi chạy lại `commit` cũ.
- Lỗi lặp 2 lần cho cùng một item: dừng, báo tác giả, xin hướng khác. Không thử
  lần thứ ba.

## 7. Vòng chạy một stage

1. Chat gọi `outline {action: "run", stage}`.
2. Backend dựng `Plan` gồm các `Shard`, mỗi shard 1 lane, 1 key, 1 budget.
3. `JoinSet` chạy tối đa 5 shard. Mỗi shard: `complete` -> `parse` -> `validate`.
4. Item hợp lệ ghi vào `outline_items` với `status = pending`. Không ghi state.
5. Chat đọc `items`, tóm tắt cho tác giả. Tác giả duyệt hoặc sửa.
6. `commit` validate toàn bộ rồi ghi theo `STATE_ORDER`.
7. `status` trả `next_stage`. Gate mở stage kế tiếp.

Concurrency mặc định 5, chỉnh được 1..8 bằng hằng số. Không đổi contract.

Hủy và resume:

- `cancel` set cờ `AtomicBool`. Shard xong muộn bị bỏ, không ghi.
- Resume theo key: shard đã `done` trong lần trước thì skip.
- `regenerate` chạy lại đúng 1 item với `note`, không chạy lại cả stage.

## 8. Nợ kỹ thuật đã biết

- `catalog` từng có `#![allow(dead_code)]`. Gỡ trong 17.2. Consumer thật là seed,
  prompt builder và commit validator.
- Prompt `arcs` và `chapters` còn tối giản. 17.3 làm đầy budget, tone, cấm emoji.
- UI outline chưa có. 17.3 mới có `OutlineWorkspace` và board 2.1.
- `scene_binder` và `write` chưa có prompt thật. 17.5 và 17.6 mới có.
- `regenerate` mới ở mức backend. UI gọi được từ 17.3.
- `tokio` cần feature `rt` cho `JoinSet`. Bật trên crate có sẵn, không thêm
  dependency. Fallback là `std::thread::scope`.
- `scene_versions` không có bảng SQLite. Quyết định mục 15 của plan: chỉ file.
- Chưa có test runner frontend, nên acceptance UI là `pnpm build` và kiểm tay.

## 9. Đọc thêm

- `docs/plan/sprint-17.md` mục 3 (quyết định), 5 (lane), 7 (STATE_ORDER), 8 (tool).
- `docs/design/super-json.md` cho 81 path và các field v3.
- `src-tauri/prompts/outline_playbook.md` cho luật vận hành của model.
