# Super JSON — đặc tả hoàn chỉnh (Sprint 8.5)

> **Trạng thái:** Chốt. Thay thế phần "Siêu JSON có dạng" trong `docs/design/idea.md` (dòng 9-49).
> **Artifact máy đọc:** [`docs/schema/runtime/super.json`](../schema/runtime/super.json)
> **Nguồn đối chiếu:** `docs/schema/system.dbml` (1149 dòng) + `docs/design/process.md` + `docs/design/chatgpt-note.md`

---

## 1. Super JSON là gì và không là gì

**Là:** tập quyết định khởi tạo của một project — những thứ phải chốt *trước khi* AI viết được dàn ý.
Tồn tại dưới dạng 1 file JSON toàn cục của project: `~/.merlink/projects/{project_id}/super.json`.

**Không là:**
- Không phải system prompt. Các thư viện `SYSTEM_PROMPT.*` trong dbml (motif, style preset, narrative
  directive, narrator config) là **nguyên liệu inject cho AI**, không phải lựa chọn của project.
- Không phải nội dung truyện. `characters`, `world_settings`, `lore_entries`, `power_systems`, `arcs`,
  `chapters`, `scenes`, `memories`, `relationships` là **output sinh ra sau** khi super json đã đủ.
- Không phải nơi lưu dữ liệu runtime. Message/chat nằm trong SQLite.

**Quy tắc require vs optional:** require = chặn nút "viết dàn ý" (đúng luồng UX trong `idea.md`).
Optional = điền được lúc nào cũng được, kể cả sau khi đã viết xong vài chương.

---

## 2. Bốn nhóm bị LOẠI (thuộc SYSTEM_PROMPT, không vào super json)

| # | Nhóm (comment trong dbml) | Dòng | Bảng bị loại |
|---|---|---|---|
| 1 | `// SYSTEM_PROMPT.MOTIF` | 1–146 | `motifs`, `genres`, `motif_applicable_genres`, `event_blocks`, `event_transitions`, `states`, `state_down_by_rules`, `state_rules`, `emotional_ranges`, `event_block_emotionals`, `components`, `motif_constraints`, `constraints` |
| 2 | `// SYSTEM_PROMPT.STYLE_PRESET` | 148–244 | `style_presets`, `rhetorical_devices`, `style_preset_rhetorical_devices`, `style_preset_vocab_items`, `style_preset_focus_items` |
| 3 | `// SYSTEM_PROMPT.NARRATIVE_DIRECTIVES` | 246–289 | `narrator_configs` + enum `GrammaticalPerson`, `PersonSubtype`, `PovScope` |
| 4 | `// SYSTEM_PROMPT.{PROSE_PRINCIPLES, INFORMATION_CONTROL}` | 291–321 | `narrative_directives`, `style_preset_narratives`, enum `DirectiveCategory` |

Loại thêm (không phải init, cũng không phải system prompt — là dữ liệu tính sau):
`// VFILL (TÍNH SAU)` dòng 324–346 (`character_motifs`, `relationship_motifs`) và `symbolics`,
`symbolic_motifs` dòng 1140–1148.

> **Lưu ý ranh giới:** `genres` (thư viện) bị loại, nhưng `project_genres` (lựa chọn của project) thì vào
> super json. Cùng logic với `motifs` vs `project_motifs` — xem mục 6.

---

## 3. REQUIRE — 11 leaf path

Điền hết 11 path này thì nút viết dàn ý mới sáng.

| # | Path | Nguồn trong dbml | Vì sao bắt buộc |
|---|---|---|---|
| 1 | `require.overview.name` | `projects.name` | Không có tên thì không tạo được project, không đặt được tiêu đề chat. (`idea.md` đã để require) |
| 2 | `require.overview.logline.trigger-event` | — (init-only) | 5 mảnh logline là động cơ truyện. (`idea.md` require) |
| 3 | `require.overview.logline.main-character` | — (init-only) | Đầu vào để sinh `characters` ở bước sau (`idea.md` dòng 140) |
| 4 | `require.overview.logline.action` | — (init-only) | `idea.md` require |
| 5 | `require.overview.logline.risk` | — (init-only) | `idea.md` require |
| 6 | `require.overview.logline.major-loss` | — (init-only) | `idea.md` require — mất mát lớn là thứ buộc theme phải trả lời |
| 7 | `require.overview.plan.arc.count` | `arcs` (đếm dự kiến) | `process.md` dòng 14: "Dung lượng dự kiến: số Arc/Volume" |
| 8 | `require.overview.plan.chapter.count` | `chapters` (đếm dự kiến) | `process.md` dòng 14: "số Chapter" |
| 9 | `require.overview.plan.word.count` | — (init-only) | `process.md` dòng 14: "target word count". **Mới so với idea.md** |
| 10 | `require.genre.primary` | `project_genres.is_primary = true` → `genres.code` | Thể loại quyết định `motif_applicable_genres` và preset nào được phép chọn. Không có genre thì mọi bước sau đều mù. **Mới** |
| 11 | `require.theme.question` | `themes.question` — dbml đánh `[not null]` | dbml tự khẳng định NOT NULL; `process.md` gọi theme là "mỏ neo toàn bộ dự án". **Mới** |

**Vì sao 3 field mới (9, 10, 11):** `idea.md` mới chỉ có `overview` (name + logline + arc/chapter count).
Ba field này là phần "còn lại" của schema mà mày yêu cầu xét — chúng thuộc nhóm high-level anchor chứ
không phải nội dung sinh sau.

---

## 4. OPTIONAL — 14 leaf path

| # | Path | Nguồn trong dbml | Vì sao không bắt buộc |
|---|---|---|---|
| 1 | `optional.overview.description` | `projects.summarize` | Tóm tắt toàn truyện chỉ viết trôi chảy được **sau** khi có dàn ý |
| 2 | `optional.overview.author` | `projects.author` | Không ảnh hưởng generation |
| 3 | `optional.overview.language` | `projects.language` default `VI` | dbml đã có default |
| 4 | `optional.overview.target-audience` | — (init-only) | `idea.md` để optional |
| 5 | `optional.overview.age-rating` | — (init-only) | `idea.md` để optional |
| 6 | `optional.overview.plan.arc.tolerance` | — (init-only) | `idea.md` để optional |
| 7 | `optional.overview.plan.chapter.tolerance` | — (init-only) | `idea.md` để optional |
| 8 | `optional.overview.plan.word.tolerance` | — (init-only) | Thêm cho đối xứng với arc/chapter |
| 9 | `optional.genre.secondary` | `project_genres.is_primary = false` | Thể loại phụ là tinh chỉnh, không chặn gì |
| 10 | `optional.theme.resolution-type` | `themes.resolution_type` default `OPEN` | dbml đã có default |
| 11 | `optional.theme.resolution` | `themes.resolution` | "Câu trả lời cuối cùng của truyện" — **không thể biết trước khi viết** |
| 12 | `optional.theme.message` | `themes.message` — note "có thể null" | Chính dbml ghi rõ field này nullable |
| 13 | `optional.world.has-power-system` | gate cho `power_systems` | Init-only: quyết định bước Ưu tiên 2 có sinh hệ thống sức mạnh hay không |
| 14 | `optional.world.setting-scale` | `world_settings.type` (`LocationType`) | Init-only: độ sâu cây không gian cần dựng (`process.md` Ưu tiên 2) |

> **Xung đột đã gặp và cách xử lý:** `process.md` dòng 15-17 liệt kê "Thông điệp cốt lõi (Premise /
> Controlling Idea)" như một phần của `theme.json`. Nhưng `themes.message` trong dbml có note
> *"Thông điệp rút ra, có thể null"*. Tao theo **dbml** (nguồn chặt hơn) → `message` là optional.
> Nếu mày muốn nó bắt buộc, đổi 1 dòng trong `super.json` và doc này.

---

## 5. Luật validate cho `super_json_set`

| Kiểu | Path |
|---|---|
| string, non-empty | `overview.name`, `overview.logline.*` (5), `genre.primary`, `theme.question`, `overview.description`, `overview.author`, `overview.target-audience`, `overview.age-rating`, `theme.resolution`, `theme.message` |
| số nguyên dương | `overview.plan.arc.count`, `overview.plan.chapter.count`, `overview.plan.word.count` |
| số nguyên ≥ 0 | `overview.plan.arc.tolerance`, `overview.plan.chapter.tolerance`, `overview.plan.word.tolerance` |
| enum `VI\|EN\|JA\|ZH_HANT\|ZH_HANS` | `overview.language` (từ enum `Language`, dbml dòng 356) |
| enum `RESOLVED\|OPEN\|AMBIGUOUS` | `theme.resolution-type` (từ `ThemeResolutionType`, dbml 802) |
| enum `REALM\|WORLD\|REGION\|COUNTRY\|CITY\|TOWN\|FACILITY\|STREET\|ROOM` | `world.setting-scale` (từ `LocationType`, dbml 666) |
| boolean | `world.has-power-system` |
| array<string>, non-empty phần tử | `genre.secondary` |

Path ngoài bảng trên → `Err`, không tạo field mới. Tổng: **25 leaf path** (11 require + 14 optional).

`super_json_missing()` trả về 11 path require còn `null`. Trên file mới tạo → đúng 11.

---

## 6. Ranh giới CHƯA chốt (cần mày quyết ở sprint sau)

| # | Vấn đề | Vì sao chưa đưa vào |
|---|---|---|
| 1 | `project_motifs` — project chọn motif nào | Là `DATA.CONTENT` (không phải system prompt), nhưng mày đã xếp "Motif" vào nhóm system prompt. Nếu chốt là project-level thì thêm `optional.motifs` (array code) |
| 2 | Style preset mặc định của project | `style_presets` bị loại, nhưng project có thể cần 1 preset mặc định. Hiện gán ở tầng `scenes.style_preset_id` |
| 3 | Narrator mặc định của project | `narrator_configs` bị loại; hiện gán ở tầng `scenes.narrator_config_id` |
| 4 | Ngôn ngữ truyện vs ngôn ngữ UI | `optional.overview.language` hiện là ngôn ngữ truyện (theo enum `Language`) |
| 5 | `arcs` / `chapters` có cần name ngay không | Hiện chỉ lưu `count`; tên arc/chapter sinh ở bước dàn ý |

---

## 7. Giá trị "init-only" — không có trong schema

Mày xác nhận: schema không có số chương/số từ ước tính **vì chúng chỉ tồn tại tại thời điểm khởi tạo**.
Bốn nhóm giá trị dưới đây thuộc dạng đó và **cố ý không map về bảng nào**:

| Path | Ghi chú |
|---|---|
| `overview.plan.arc.count` | `arcs` chỉ chứa arc *đã sinh*, không chứa dự kiến |
| `overview.plan.chapter.count` | tương tự với `chapters` |
| `overview.plan.word.count` | không có cột nào trong dbml; `scenes.word_count` là số thực tế |
| `overview.plan.*.tolerance` | biên độ sai lệch chấp nhận được — thuần quyết định sản phẩm |
| `overview.target-audience`, `overview.age-rating` | không có bảng/cột tương ứng trong dbml |
| `overview.logline.*` | logline không có bảng riêng; `projects.summarize` là tóm tắt, không phải logline |

Sau này nếu muốn logline thành dữ liệu lâu dài thì phải thêm bảng — hiện coi là metadata khởi tạo.
