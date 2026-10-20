# SPRINT 17 — PHASE 2: OUTLINE ENGINE (State fabrication + review gates)

> **Trạng thái: PLAN.** Chưa có dòng code nào của phase 2. Tài liệu này là roadmap 7 sprint
> `17.1 → 17.7`, mỗi sprint có gate review riêng và phải merge được độc lập.
> **Base:** `main` sau Sprint 16.6 (`cargo test` 170 passed / 3 ignored, `pnpm build` pass).
> **Branch:** `sprint-17.1-state-v3`, `sprint-17.2-outline-engine`, `sprint-17.3-stage-21`,
> `sprint-17.4-stage-22`, `sprint-17.5-stage-23`, `sprint-17.6-writing-engine`,
> `sprint-17.7-writing-ui`.
> **Route dev-subagent:** `openai-codex / gpt-6-luna / high` (task nặng, code Rust/TS) và
> `commandcode / deepseek/deepseek-v4.1-flash / high` (task dữ liệu/script/boilerplate).
> **Ghi chú:** từ sau Sprint 17.1 KHÔNG dùng `antigravity` nữa; 2 agent 17.1A/17.1B đã khởi chạy
> trước khi đổi route nên được để hoàn tất.

---

## 0. ĐỌC NHANH (30 giây)

Phase 1 đã xong: người dùng chat để điền **12 unit `require`** của `projects/<id>/super.json`.
Phase 2 bắt đầu từ nút **Viết dàn ý**: AI tự bịa **toàn bộ phần còn lại** của state (optional +
các bảng entity chưa có row), ghi vào state theo đúng thứ tự phụ thuộc, người dùng review từng
chặng, chốt xong thì phase 3 mới viết văn.

Ba tầng cần phân biệt rõ, **không lẫn** (đây là điểm user nhấn mạnh):

| Tầng | Là gì | Ai gọi |
|---|---|---|
| **Chat agent** | Agent loop hiện tại: model → tool → model, tối đa **5 tool round** (`runtime/agent.rs`) | Người dùng gõ chat |
| **Merlink subagent** | Các **lượt gọi provider song song** do **backend Merlink tự orchestrate** cho phase 2/3. Mỗi subagent là 1 system prompt vai trò + 1 input JSON + 1 output JSON, **không** có tool loop | Chính Merlink (nút trên UI hoặc tool `outline`) |
| **Dev subagent** | Agent lập trình của tôi khi thi công sprint | Tôi, ngoài runtime |

Toàn bộ mục 5 nói về **Merlink subagent**. Chat agent chỉ giữ một cửa duy nhất vào phase 2: tool
`outline` (mục 8).

---

## 1. HIỆN TRẠNG (đã đọc code, không suy đoán)

| Hạng mục | Sự thật | Nguồn |
|---|---|---|
| State store | `projects/<id>/super.json`, v2.1, 34 bảng, **81 path** ghi được (13 project leaf + 34×2) | `src-tauri/src/super_json/schema.rs:127-154`, `docs/design/super-json.md:44-62` |
| Unit tiến độ | 12 require / 35 optional, 70/30 | `src-tauri/src/super_json/progress.rs:102-143` |
| Tool surface | **7 tool**: `project_create`, `super_json_get/set/add_row/add_field/missing`, `echo` | `src-tauri/src/tools/mod.rs:18-25`, `tools/specs.rs:11-204` |
| Validate row | `fields` read-only, `count==rows.len()`, required row fields, ref range check | `src-tauri/src/super_json/tables.rs:143-180`, `schema.rs:338-363` |
| Agent loop | tối đa 5 tool round, prompt embed **toàn bộ** document | `src-tauri/prompts/harness_system.md:12-27`, `runtime/prompt.rs:10-59` |
| Nút Viết dàn ý | chỉ gửi user message `"Bắt đầu viết dàn ý."`, **không có pipeline** | `src/pages/Chat.tsx:35-39, 779-792` |
| SQLite | **chỉ 3 bảng** `projects/sessions/messages`, `user_version=1` | `src-tauri/src/db/mod.rs:24-51, 80-112` |
| DBML | 61 bảng là **thiết kế đích**, chưa port; nhiều sprint cấm port | `docs/schema/system.dbml`, `docs/plan/sprint-11.md:203`, `sprint-15.md:168` |
| Catalog | `style_presets`, `narrator_configs`, `narrative_directives`, `motifs`, `genres` chỉ có **seed SQL PostgreSQL** (`gen_random_uuid()`) | `docs/schema/seed/*.sql` |
| super.json scenes | có `style_preset_id`, `narrator_config_id`, `world_setting_id`; **không** có POV, directive, rule, content path | `docs/schema/runtime/super.json:141-156` |
| super.json characters | rất mỏng: `name`, `is_primary`, `birth`, `species`, 3 field giới; **không** có role/archetype/motivation/summary | `docs/schema/runtime/super.json:43-56` |
| IPC | `CommandMap` typed, camelCase arg, `Channel<T>` cho event | `src/lib/ipc.ts:17-45`, `src-tauri/src/runtime/events.rs:11-32` |
| Browser preview | `browser-mock.ts` mirror **mọi** command + tool | `src/lib/browser-mock.ts:601-703, 941-1012` |
| UI hiện có | `StoryConfig` là `ComingSoon`; `Characters`/`Home` dùng mock | `src/pages/StoryConfig.tsx:4-19`, `src/pages/Characters.tsx:12-26` |

**Kết luận hiện trạng:** phase 2 không thể bắt đầu bằng "gọi model viết thêm row" vì (a) schema
thiếu field để chứa dàn ý, (b) prompt đang embed cả document nên sẽ nổ context khi có 60 chương +
180 scene, (c) không có chỗ chứa bản nháp để user review, (d) catalog style/narrator/directive
chưa đọc được từ code.

---

## 2. MỤC TIÊU PHASE 2 (Definition of Done của 17.7)

Với một project đã đủ 12 unit `require`, người dùng phải làm được **toàn bộ** chuỗi sau, không
rời app, không sửa file tay:

1. Bấm **Viết dàn ý** → màn chat đổi sang **Outline Workspace**, chạy subagent 2.1.
2. **2.1** đọc được summarize của **từng arc** và summarize của **n chương người dùng yêu cầu**
   (`n` chọn theo range, không bắt buộc cả truyện). Sửa / viết lại 1 item / chốt.
3. **2.2** chạy **2 lane song song**: `cast_smith` (nhân vật, ngoại hình, năng lực, ký ức, hiểu
   biết, động lực, motif, quan hệ) và `world_smith` (địa điểm, văn hoá, lịch sử, item, hệ thống
   sức mạnh). Review dạng card, chốt.
4. **2.3** mỗi chương chạy `scene_binder`: ra danh sách scene, mỗi scene có **style preset, setting,
   cast + POV, quan hệ trong scene, narrative directive + weight, rule/kỹ thuật viết, target words**.
   Review dạng bảng chương × scene, chốt.
5. Toàn bộ state được **commit theo thứ tự phụ thuộc** (mục 7), pass `tables::validate_rows`, không
   đụng `require`, không tạo row mồ côi.
6. **Prompt kéo ngược**: mỗi subagent và mỗi lượt viết văn nhận **anchor** + **lát cắt đúng phạm vi**
   (không embed cả document).
7. **Phase 3**: tab `Write` trong ProjectDetail, chương = các **block scene**, viết / viết lại
   **1 hoặc nhiều scene được chọn**, mọi prompt cấm **emoji** và **gạch ngang `–`**, vi phạm bị
   chặn và sửa.
8. `scenes.content` **luôn null** trong phase 2; văn bản nằm ở file, `scenes.rows[].content_path`
   trỏ tới file đó (mục D10).

---

## 3. QUYẾT ĐỊNH (chốt, không hỏi lại)

### D1 — State vẫn là `super.json`, không port DBML sang SQLite
- `super.json` là **committed state** của project và được nâng lên **v3** (mục 4).
- SQLite chỉ nhận **runtime data**: `outline_runs`, `outline_items` (nháp/review), `messages`.
- Lý do: giữ nguyên hợp đồng 81 path + validate + progress + prompt embed; các sprint 11/15/16 đã
  cấm port `system.dbml` sang SQLite; port 61 bảng là 3 sprint không sinh giá trị phase 2.

### D2 — Catalog là JSON theo `code`, sinh từ seed SQL
- Nguồn: `docs/schema/seed/*.sql` (PostgreSQL). Artefact: `docs/schema/catalog/*.json`
  (`genres.json`, `motifs.json`, `style_presets.json`, `narrator_configs.json`,
  `narrative_directives.json`, `relationship_categories.json`).
- Sinh bằng `scripts/gen-catalog.mjs` (Node có sẵn qua pnpm, **không** thêm dependency), kết quả
  **commit vào repo**, có test đếm số entry để chống drift.
- Runtime: `src-tauri/src/catalog/mod.rs` nạp bằng `include_str!`, `OnceLock`, tra theo `code`.
- Model **không bao giờ** ghi catalog; chỉ đọc và tham chiếu `code`.

### D3 — Tham chiếu bằng `code`, không bằng uuid
- v3 thêm các field `*_code` (`genre_code`, `style_preset_code`, `narrator_code`, `motif_code`,
  `category_code`, `directive.code` trong JSON).
- Các field `*_id` cũ giữ nguyên và **null** cho tới khi có sprint materialize DB thật. Không xoá,
  không đổi nghĩa field cũ (tránh phá test 81 path).

### D4 — Merlink subagent là fan-out completion, không phải tool loop
- Mỗi subagent = 1 call `provider.complete(system, user)` **không tool**, output là **một JSON
  object** theo contract của lane.
- Backend orchestrate: bounded concurrency **5** (cấu hình 1..8), cancel, retry ≤ 2, resume.
- Chat agent **không** tham gia fan-out; nó chỉ gọi tool `outline` (1 round) và đọc kết quả tóm tắt.
  Nhờ vậy giới hạn 5 round vẫn đúng.

### D5 — "Một tool lớn, nhiều tool nhỏ, hướng dẫn chuẩn chỉ"
- Thêm **1 tool thứ 8** tên `outline`, tham số `action` (enum) + `stage`/`keys`/`range`/`note`.
  Các "tool nhỏ" là **action** bên trong, cùng một validate/ghi.
- Playbook chuẩn: `src-tauri/prompts/outline_playbook.md`, chèn vào system prompt khi project đã
  qua 70%; mô tả từng action, thứ tự bắt buộc, điều cấm, format lỗi.

### D6 — Pipeline có gate, tách "đề xuất" và "đã ghi"
- Stage: `seed → arcs → chapters → cast → world → scenes → ready → writing`.
- Subagent sinh **proposal** (bảng `outline_items`, `status=pending`). **Không** ghi state.
- Chỉ action `commit` (hoặc nút Chốt) mới ghi state, và chỉ ghi item `accepted`.
- `commit` từ chối nếu stage trước chưa `committed` (gate cứng, trả message rõ).

### D7 — Thứ tự ghi là hằng số, không phải lời khuyên
- `outline/stage.rs` giữ `STATE_ORDER: &[Entity]` (mục 7). `apply` duyệt đúng thứ tự đó, không theo
  thứ tự model trả về.
- Thiếu phụ thuộc → lỗi dạng `"scenes[3].chapter_id: chương ch-004 chưa commit"`, không ghi một
  phần rồi bỏ dở: **mỗi stage commit là một transaction logic** (validate hết → ghi hết → fsync).

### D8 — Review ở mức item, không chỉ mức stage
- Mỗi item (arc / chapter / character / setting / scene) có `accept | reject | edit | regenerate`.
- `regenerate` nhận `note` của người dùng và chỉ chạy lại item đó (1 call), không chạy lại cả stage.
- `accept` theo range (`1-10`, `all`, danh sách key) để user chốt từng phần.

### D9 — Nháp nằm ở SQLite, bump schema version
- `db::SCHEMA_VERSION` 1 → 2, thêm 2 bảng (`outline_runs`, `outline_items`) + index.
- Test `test_open_migrates_v0_and_reopening_is_idempotent` đang assert `version == 1`
  (`src-tauri/src/db/mod.rs:128,140`) **phải** cập nhật thành 2, và thêm test migrate 1 → 2.

### D10 — Văn bản truyện nằm ở file, không nằm trong JSON
- Đường dẫn: `projects/<id>/content/arc-<a>/chapter-<c>/<position>-<slug>.md`.
- `scenes.rows[].content_path` = đường dẫn tương đối; `scenes.rows[].content` **luôn null**.
- Bản cũ khi viết lại: `.../versions/<position>-v<n>.md`, metadata trong `scene_versions` row
  (nếu bật) hoặc chỉ file + `word_count`.
- Lý do: 120 chương ≈ 1 MB; ghi song song 5 scene vào một file JSON là lost-update. File-per-scene
  cho phép 5 subagent viết cùng lúc mà không khoá nhau.

### D11 — Prompt theo lát cắt, không embed cả document
- `anchor` (≤ 1.500 token) + bundle theo phạm vi:
  - 2.1: anchor + plan counts + 34 tên bảng (không rows).
  - 2.1 chapters: anchor + arc đã chốt + range chương + tổng budget.
  - 2.2 cast: anchor + chapters đã chốt + danh sách motif/competence domain.
  - 2.2 world: anchor + chapters + genre/motif.
  - 2.3 scenes: anchor + arc/chapter + cast sheet rút gọn + settings rút gọn + catalog
    style/narrator/directive (rút gọn) ≤ 6.000 token.
  - Phase 3: 1 scene bundle ≤ 6.000 token (chapter, 2 scene trước, cast liên quan + ký ức/năng lực,
    setting + sensory, preset + narrator + directive + rule, quan hệ, recap chương trước).
- `harness_system.md` phase 1 giữ embed toàn bộ; nhưng thêm **guard**: nếu document > 24 KB thì
  embed bản compact (counts + project leaves + tên bảng) và ghi rõ "dùng `super_json_get` để đọc".
- Guard nằm **trong** `embedded_document` (`runtime/prompt.rs:18-32`), không đổi chữ ký
  `assemble_prompt` (3 call site: `agent.rs:126`, `agent.rs:187`, `session.rs:129`). Prompt được
  render lại **mỗi round**, không phải mỗi tool call (`agent.rs:184-195`) — nghĩa là playbook phase 2
  chỉ xuất hiện từ round kế tiếp sau khi `require` đủ, chấp nhận được.

### D12 — Cấm emoji và gạch ngang
- Mọi prompt sinh văn (kể cả summarize) có dòng luật cố định: **"Cấm emoji. Cấm ký tự `–` (en
  dash) và `—` (em dash). Dùng dấu phẩy, dấu chấm hoặc xuống dòng."**
- Hậu kiểm deterministic `prose::violations(text) -> Vec<Violation>` quét emoji + U+2013/U+2014.
- Có vi phạm → retry 1 lần kèm danh sách vi phạm; vẫn vi phạm → sanitize (thay bằng `, `) và đánh
  dấu `prose_clean=false` để UI cảnh báo. Không bao giờ im lặng.

### D13 — Bất biến số lượng
- `arcs.rows.len() == plan.arc.count`; `chapters.rows.len() == plan.chapter.count`;
  `Σ chapters.rows[].scene_count == plan.scene.count` (cho phép lệch 0; nếu user đặt tolerance thì
  dùng tolerance).
- Subagent nhận budget trong prompt; `commit` chặn nếu lệch, in ra lệch bao nhiêu.

### D14 — Không bịa dữ kiện người thật, và `require.project` là bất khả xâm phạm
- `optional.project.author` **không bao giờ** được bịa; để null nếu user chưa nói.
- **Read-only là `require.project.*`** (6 leaf tác giả tự quyết). Commit từ chối mọi path này.
- `require.schema.<table>.count|rows` **được ghi** khi `<table>` là bảng require (6 bảng), vì phase 2
  chính là bước mở rộng các bảng required: phase 1 chỉ đặt `name` + `position` cho `arcs`, phase 2 mới
  điền `goal/outcome/summarize` và tăng số row cho khớp `plan.arc.count`. Guard suy ra danh sách bảng
  từ `schema::countable_group` chứ không hardcode.
- Nếu commit thay một row đã có `name` mà đề xuất không chứa, ghi `warnings` ("Thay arc người dùng đã
  đặt: <name>") để không âm thầm ghi đè; 17.3 sẽ cho UI seed generator từ các row đó.
- `summarize`/`target-audience` chỉ sinh **sau** khi arcs commit, và chỉ khi user bấm "Sinh tóm tắt".

### D15 — UI: chat đổi mode, phase 3 ở ProjectDetail
- `Chat.tsx` thêm `outlineMode`; khi run đang mở, vùng message được thay bằng `<OutlineWorkspace>`
  (chat composer thu gọn vẫn dùng được để trao đổi).
- Reload khôi phục mode từ `outline_status` (server là nguồn sự thật).
- Phase 3: `ProjectDetail` thêm tab `Write` (mục 9).

### D16 — Không thêm dependency
- Fan-out dùng `tokio::task::JoinSet`. Nếu feature `rt` chưa bật (tokio hiện khai báo
  `features = ["time"]`, `src-tauri/Cargo.toml:26`), bật thêm feature trên **crate đã có**, không
  thêm crate mới. Nếu vẫn không được: fallback `std::thread::scope` + `reqwest::blocking` builder
  riêng (đã có `reqwest`).
- JSON contract validate bằng tay (không `jsonschema`).

### D17 — Kiểm soát chi phí
- Mỗi stage có `max_calls` và `max_tokens` ước tính; vượt → dừng và báo user chọn range nhỏ hơn.
- Shard mặc định: 1 call / 10 chương (2.1), 1 call / 6 nhân vật (2.2 cast), 1 call / 1 chương (2.3).
- Mọi run ghi `provider_id`, `model_id`, `calls`, `tokens_in/out` (nếu adapter trả usage).

### D18 — Ngôn ngữ
- Mọi nội dung sinh ra là **tiếng Việt**, trừ `code`/enum (UPPER_SNAKE hoặc slug ASCII).
- UI copy tiếng Việt, không hardcode tiếng Anh mới (giữ các chuỗi tiếng Anh đã có).

---

## 4. DATA MODEL v3 (super.json)

Nguyên tắc: **chỉ thêm field**, không thêm/ bớt bảng, không đổi path. Vì vậy **81 path, 34 bảng,
12/35 unit giữ nguyên**; chỉ `fields` (contract row) dài ra, và test nào assert danh sách field
phải cập nhật.

### 4.1 Field thêm

| Bảng | Field mới | Kiểu | Ghi chú |
|---|---|---|---|
| `project_genres` | `genre_code` | text | khớp `genres.json.code` |
| `project_genres` | `note` | text | lý do chọn thể loại phụ |
| `arcs` | `goal` | text | mục tiêu của arc |
| `arcs` | `outcome` | text | kết quả/điểm xoay |
| `chapters` | `beats` | JSON array<string> | 3-6 beat |
| `chapters` | `scene_count` | int ≥ 0 | budget scene của chương |
| `chapters` | `goal` | text | |
| `chapters` | `outcome` | text | |
| `characters` | `role` | text | vai trò trong truyện |
| `characters` | `archetype` | text | |
| `characters` | `motivation` | text | động lực gốc |
| `characters` | `want` / `need` | text | |
| `characters` | `flaw` | text | |
| `characters` | `summary` | text | 2-3 câu |
| `characters` | `description` | text | chân dung |
| `relationships` | `summary` | text | |
| `relationships` | `origin` | text | nguồn gốc quan hệ |
| `relationships` | `is_primary` | bool | tuyến quan hệ chính |
| `project_motifs` | `name` / `description` / `significance` | text | để anchor đọc được |
| `themes` | `name` / `statement` / `weight` | text/text/number 0..1 | |
| `theme_arguments` | `claim` / `counter_claim` | text | |
| `theme_arguments` | `polarity` | enum `SUPPORT\|CHALLENGE\|AMBIVALENT` | |
| `scenes` | `style_preset_code` | text | `style_presets.json.code` |
| `scenes` | `narrator_code` | text | `narrator_configs.json.code` |
| `scenes` | `directives` | JSON array<{code,weight,instruction}> | `NONE..ABSOLUTE` |
| `scenes` | `rules` | JSON array<string> | kỹ thuật viết riêng scene |
| `scenes` | `relationships` | JSON array<{relationship_index,beat,tension}> | quan hệ trong scene |
| `scenes` | `target_words` | int ≥ 0 | |
| `scenes` | `content_path` | text | đường dẫn file prose, phase 3 |
| `scenes` | `word_count` | int ≥ 0 | |
| `scenes` | `status` | enum `PLANNED\|WRITTEN\|REVISED\|LOCKED` | |

### 4.1b Quy ước tham chiếu chéo (đã có trong code, phase 2 dùng lại)

- Row không có id thật: mọi tham chiếu chéo dùng **vị trí** trong `rows`.
  - `<field>_json_index` **thay thế** required `<field>_id` (`src-tauri/src/super_json/tables.rs:195-208`,
    test `test_ref_satisfies_required_id`). Range check theo `tables::REF_TARGETS`
    (`tables.rs:35-90`).
  - `_json_index` do tool gán = vị trí row, model không được ghi (`normalize_json_index`,
    `tables.rs:20-30`).
- **POV không thêm field**: `scene_characters.rows[].is_pov = true` đã là nguồn sự thật
  (`docs/schema/system.dbml:500-510`). Binder phải bảo đảm đúng một `is_pov` mỗi scene.
- **Quan hệ trong scene** dùng `scenes.relationships[].relationship_index` = vị trí row trong
  `relationships.rows`.
- **Memory → scene** dùng `source_scene_id_json_index` ở link pass (bước 24), không phải id.
- Hệ quả cho subagent: output dùng `ref` slug nội bộ; `apply` đổi `ref → index` sau khi biết vị trí
  chèn. Model **không bao giờ** thấy hay ghi index thật.

### 4.2 Migration v2.1 → v3

- Giống v2 → v2.1: đọc document, **thay `fields` mỗi bảng bằng skeleton mới**, giữ nguyên
  `count`/`rows`, sort `schema` theo `schema::tables()`.
- Ghi backup `super.v2.json` **một lần** trước khi rewrite (giống `super.v1.json` hiện có).
- Idempotent: load lần hai không đổi byte nào (`io::load_or_init` chỉ ghi khi `changed`).

### 4.3 Anchor (khối văn user yêu cầu)

`outline/anchor.rs::anchor(document) -> String` sinh **đúng 6 đoạn**:

1. Tên, ngôn ngữ, thể loại (+ `genre_code`), `plan.word/arc/chapter/scene`.
2. Diễn tiến arc (mở đầu + các arc còn lại).
3. Nhân vật chính (role, archetype, summary, description) + tuyến đối trọng.
4. Motif chủ đạo (name, scope, description, significance).
5. Theme (name, statement, weight).
6. Xung đột tư tưởng (character, claim, polarity, counter_claim).

Mỗi mục null → `[chưa có: <path>]`. Ngoài ra `anchor_compact()` (1 đoạn, để nhồi prompt subagent)
và `anchor_json()` (object có key ổn định cho contract).

---

## 5. MERLINK SUBAGENT RUNTIME (fan-out)

### 5.1 Lane (vai) và số call

| Lane | Dùng ở | Số call | Input | Output |
|---|---|---|---|---|
| `arc_smith` | 2.1a | 1 | anchor + plan counts | `{arcs:[{position,name,code,goal,outcome,summarize,chapters}]}` |
| `chapter_smith` | 2.1b | ⌈N/10⌉ (song song ≤5) | anchor + arc + range chương | `{chapters:[{arc,position,name,code,summarize,beats,scene_count,goal,outcome}]}` |
| `cast_smith` | 2.2a | ⌈M/6⌉ (≤5) | anchor + chapters | `{characters:[...]}` (identity, appearance, competences, memories, knowledge, motifs, motivation) |
| `bond_smith` | 2.2b | 1-2 | cast đã commit | `{relationships:[{from,to,summary,origin,is_primary,states,roles,motifs}]}` |
| `world_smith` | 2.2c | 1-2 | anchor + chapters + genre | `{settings,lore,power_systems}` |
| `scene_binder` | 2.3 | N chương (≤5) | anchor + chapter + cast sheet + settings + catalog | `{chapter, scenes:[...]}` |
| `prose_writer` | phase 3 | 1 scene | scene bundle | text thuần |

### 5.1b Hai giai đoạn của 2.1 (đúng yêu cầu "arc trước, chapter sau")

- **Giai đoạn A — arc (1 subagent, 1 call):** `arc_smith` đọc anchor + `plan.arc.count` và chia
  `plan.chapter.count` cho từng arc. Output là **nguồn sự thật** cho mọi bước sau; chưa commit.
- **Giai đoạn B — chapter (≥ 5 subagent song song):** sau khi user chốt arc, `chapter_smith` chạy
  theo shard `range` (mặc định 10 chương/shard), mỗi shard nhận "arc đã chốt + vị trí chương +
  budget scene + tóm tắt chương trước đó" để mạch liền.
- Vì B chỉ chạy sau khi A được chốt, hệ quả là **tổng số call tối thiểu** cho 2.1 là
  `1 + ceil(n/10)`, và với `n ≥ 50` luôn có ≥ 5 shard chạy song song.

≥ 5 subagent song song xảy ra tự nhiên ở 2.1b, 2.2a và 2.3 (mục 5.1), không cần "giả lập" song song.

**Hoàn tất optional:** nếu user chốt cả 3 stage 2.1/2.2/2.3, toàn bộ 35 unit optional phải filled
(progress 100%) **trừ** `optional.project.author` (D14). Test `test_optional_reaches_full_after_all_stages`
kiểm đúng điều này.

### 5.2 Hợp đồng output (rút gọn, bản đầy đủ trong `outline/contract.rs`)

```json
// arc_smith
{"arcs":[{"position":1,"code":"arc-01","name":"…","goal":"…","outcome":"…","summarize":"4-6 câu","chapters":12}]}

// chapter_smith
{"chapters":[{"arc":1,"position":1,"code":"ch-001","name":"…","summarize":"2-4 câu",
  "beats":["…"],"scene_count":3,"goal":"…","outcome":"…"}]}

// cast_smith
{"characters":[{"ref":"char-01","name":"…","role":"…","archetype":"…","motivation":"…",
  "want":"…","need":"…","flaw":"…","summary":"…","description":"…","is_primary":true,
  "motifs":["motif-code"],
  "appearance":{"height":170,"height_unit":"CM","hair":"…","eyes":"…","outfit_style":"…","marks":["…"]},
  "competences":[{"domain":"…","name":"…","level":"…","capabilities":["…"],"limitations":["…"]}],
  "memories":[{"type":"EPISODIC","summary":"…","certainty":"CERTAIN","source":"FIRSTHAND","significance":0.7}],
  "knowledge":[{"type":"LORE","path":"lore:…","certainty":"HIGH"}]}]}

// scene_binder
{"chapter":"ch-001","scenes":[{"position":1,"name":"…","summarize":"3-5 câu",
  "world_setting":"ws-03",
  "style_preset_code":"action.combat","narrator_code":"third_limited","target_words":900,
  "directives":[{"code":"show_dont_tell","weight":"HIGH","instruction":"…"}],
  "rules":["…","…"],
  "cast":[{"character":"char-01","is_pov":true,"role_in_scene":"…"}],
  "relationships":[{"a":"char-01","b":"char-04","beat":"…","tension":72}],
  "beats":["…"]}]}
```

Luật chung của contract:
- Chỉ JSON, không markdown fence (parser vẫn chấp nhận fence, nhưng prompt cấm).
- `code`/`ref` là slug ASCII, unique trong output.
- Không có key lạ; key thiếu optional thì bỏ, không gửi `null` trừ khi contract cho phép.
- Text tiếng Việt, **cấm emoji và `–`/`—`**.
- Không bịa id của entity không có trong input.

### 5.3 Fan-out engine (`outline/fanout.rs`)

- `Plan` = danh sách `Shard { lane, key, system, user, budget }`.
- Chạy `JoinSet`, semaphore 5; mỗi shard:
  1. `complete()` (mục 5.4) → text
  2. `contract::parse(lane, text)` → JSON; lỗi → retry 1 lần với **error feedback** trong user
     message ("Lần trước lỗi ở …; trả lại đúng JSON").
  3. `contract::validate(lane, json, committed_state)` → lỗi referential (vd cast ref không tồn
     tại) → retry 1 lần; vẫn lỗi → item `status=failed` + `error` lưu DB, **không** chặn shard khác.
- Hủy: `Arc<AtomicBool>`; shard xong muộn bị bỏ, không ghi.
- Resume: shard đã `done` trong lần chạy trước thì skip (key-based).
- Progress: emit event sau mỗi shard (`shard_started/finished`), UI vẽ thanh tiến độ theo shard.

### 5.4 Gọi provider không tool (`runtime/complete.rs`)

- `async fn complete(provider, model, system, user, max_tokens) -> Result<Completion, String>`.
- Tái dùng `adapters::{openai, anthropic}` phần build request, bỏ `tools`, `tool_choice`.
- `Completion { text, usage: Option<Usage> }`; `Usage` map `prompt_tokens/completion_tokens` nếu
  provider trả.
- Timeout riêng 180 s **mỗi shard** (chat client đang 120 s, `adapters/http.rs:41-50`); nếu cần,
  thêm client thứ ba `OUTLINE_CLIENT` trong `http.rs`.
- Lỗi HTTP 429 → backoff 2 s rồi retry (tối đa 2).

---

## 6. STAGE MACHINE

```
(require đủ 12) ──start──▶ seed ──▶ arcs ──review/chốt──▶ chapters ──review/chốt──▶
cast + world (song song) ──review/chốt──▶ scenes ──review/chốt──▶ ready ──▶ writing
```

| Stage | Ai chạy | Item user thấy | Commit ghi gì |
|---|---|---|---|
| `seed` | backend, không LLM | (không có UI riêng) | `project_genres.genre_code`, `optional.project.target-audience` (nếu user đã nói) |
| `arcs` | `arc_smith` | card từng arc | `arcs.rows` + `arcs.count` |
| `chapters` | `chapter_smith` | list chương theo arc | `chapters.rows` + `count` + `scene_count` |
| `cast` | `cast_smith` | card nhân vật (tab Ngoại hình/Năng lực/Ký ức/Motif) | `characters`, `character_appearances`, `character_distinguishing_marks`, `character_motifs`, `competence_domains`, `character_competences`, `capability/limitation`, `memories` (+emotions), `character_knowledge_sources` |
| `world` | `world_smith` + `bond_smith` | list setting/lore/power; list quan hệ | `world_settings`, `lore_entries`, `power_systems/tiers/abilities`, `relationships`, `relationship_states`, `relationship_roles`, `relationship_motifs` |
| `scenes` | `scene_binder` | bảng chương × scene | `scenes`, `scene_characters`, `scenes.relationships`, link `memories.source_scene_id` |
| `ready` | backend | nút "Bắt đầu viết" | không ghi, chỉ set status |

Gate: `commit(stage)` kiểm tra `stage - 1` đã `committed` (trừ `seed`). `scenes` cần `cast` **và**
`world` committed. Bỏ qua gate = lỗi, không có cờ override.

---

## 7. THỨ TỰ GHI STATE (câu trả lời trực tiếp cho "cơ sở dữ liệu có ràng buộc")

`outline/stage.rs::STATE_ORDER` — `apply` luôn đi theo danh sách này, bất kể thứ tự model trả.
Bảng dưới đây có **25 dòng** vì hai dòng gộp nhiều entity (dòng 9 = power_systems → tiers → abilities,
dòng 19 = themes → theme_arguments); `Entity` trong code có **26 variant** và test assert
`STATE_ORDER.len() == 26`.

| # | Entity | Phụ thuộc | Bắt buộc trước vì |
|---|---|---|---|
| 1 | catalogs | — | tra `code` → name/description khi build prompt + validate |
| 2 | `project` leaves | — | phase 1 đã có, phase 2 **không** ghi |
| 3 | `project_genres` | catalog genres | `genre_code` phải tồn tại |
| 4 | `arcs` | — | `chapters.arc_id` trỏ tới arc index |
| 5 | `chapters` | arcs | `scenes.chapter_id`, `chapters.arc_id` |
| 6 | `characters` | — | mọi bảng `*_character` trỏ tới đây |
| 7 | `world_settings` | self (parent) | `lore/scenes.world_setting_id`, cha phải có trước con (REALM→…→ROOM) |
| 8 | `lore_entries` | world_settings | `world_setting_id` |
| 9 | `power_systems` → `power_tiers` → `power_abilities` | lẫn nhau | tier cần system, ability cần tier |
| 10 | `character_appearances`, `character_distinguishing_marks` | characters | FK character |
| 11 | `character_motifs` | characters, catalog motifs | motif_code |
| 12 | `competence_domains` → `character_competences` → `capabilities`/`limitations` | characters, domains | FK + domain |
| 13 | `memories` | characters | `owner_id`; `source_scene_id` **null ở bước này** |
| 14 | `memory_emotions`, `memory_references`, `memory_connections` | memories | FK memory |
| 15 | `relationships` | characters | from/to |
| 16 | `relationship_states` | relationships | `parent_id` |
| 17 | `relationship_roles` | relationships, catalog categories | `category_code` |
| 18 | `relationship_motifs` | relationships, motifs | |
| 19 | `themes` → `theme_arguments` | characters | argument trỏ character |
| 20 | `project_motifs` | catalog motifs | phải có ≥1 row `scope=project` |
| 21 | `scenes` | chapters, world_settings, catalog style/narrator | ref hierarchy |
| 22 | `scene_characters` | scenes, characters | FK; cần ≥1 `is_pov=true` khi scenes không rỗng |
| 23 | `scenes.relationships` (patch) | relationships, scene_characters | quan hệ trong scene phải có relationship row |
| 24 | `memories.source_scene_id` (patch) | scenes | vòng phụ thuộc, giải bằng **patch sau** |
| 25 | phase 3: `scenes.content_path`/`word_count`/`status` | scenes | văn bản ra đời sau cùng |

Ghi chú vòng phụ thuộc:
- `memories.source_scene_id` ↔ `scenes`: memory sinh ở 2.2 (chưa biết scene), scene sinh ở 2.3.
  Giải: 2.2 ghi memory với `source_scene_id = null` (Draft mode cho phép), 2.3 chạy **link pass**
  `add_field` từng memory khi binder xác định được scene nguồn.
- `themes` có `arc_id` (arc phải có trước) nhưng `theme_arguments.character_id` cần cast → themes
  ghi ở bước 19 (sau cast) dù nội dung đến từ `require`.
- `project_motifs` (require) có `arc_id`/`chapter_id` optional → ghi sau chapters nếu có ref.

Kiểm tra bắt buộc khi commit (mỗi bảng):
1. `fields` key hợp lệ (`schema::table_fields`).
2. Required row fields (`schema::required_row_fields`).
3. `count == rows.len()`.
4. `characters` đúng một `is_primary = true` khi count > 0.
5. Ref `*_json_index` trong range (`tables::validate_draft_reference`).
6. Rule mềm: `project_motifs` có row `scope=project`; `scene_characters` có `is_pov` khi có scenes.
7. Bất biến số lượng D13.

---

## 8. TOOL + IPC SURFACE

### 8.0 Sáu điểm đăng ký bắt buộc khi thêm tool thứ 8 (đã xác minh trong code)

Thêm `outline` **không** chỉ là thêm một arm `match`. Phải sửa đủ 6 chỗ, nếu thiếu sẽ fail test:

1. `src-tauri/src/tools/mod.rs:18-25` — `TOOL_NAMES: [&str; 7]` → `[&str; 8]`; giữ `project_create`
   ở vị trí đầu.
2. `src-tauri/src/tools/specs.rs:50-203` — thêm spec; test đếm spec ở `specs.rs:231-249` và
   `:262-267` (so với `TOOL_NAMES.len()`) phải cập nhật.
3. `src-tauri/src/tools/execute.rs:21-220` — thêm arm `"outline"`; arm dùng
   `project_or_error(project_id)?` (`tools/project.rs:11-20`). Trả `Result<Value, String>`: thành
   công là object `{ok:true,...}`, lỗi là `Err(String)` (agent đổi thành `{"error": …}` ở
   `runtime/agent.rs:288-293`). Không dùng `ok:false` trong payload.
4. `src-tauri/src/tools/labels.rs:118-164` + `:168-180` — `status_message` (mặc định hiện là
   `"Đang chạy {other}"`) và `tool_path` (mặc định `None`); thiếu thì UI hiện chuỗi thô.
5. `src-tauri/src/runtime/session.rs:65-87` — `execute_tool_internal` chỉ kiểm session-binding cho
   danh sách hardcode ở `:71-74`; thêm `outline` nếu muốn chặn gọi chéo session.
6. `src-tauri/prompts/harness_system.md:176-188` — danh sách "# Tools"; test
   `runtime/agent/tests.rs:601-614` đang assert **đúng 7 tool**, phải đổi thành 8.

Ngoài ra, mọi command mới cần: `#[tauri::command] pub(crate) fn` trong `commands/outline.rs`,
re-export ở `commands/mod.rs`, thêm vào `merlink_handlers!` (`lib.rs:18-40`), thêm `ContractEntry` +
`body_*` ở `ipc_contract_tests.rs:319-388`, và — quan trọng — **tên command phải xuất hiện trong một
file frontend** (`Chat.tsx`/`Projects.tsx`/`ProjectDetail.tsx`/`SettingsDialog.tsx`) hoặc phải khai
báo trong `RUST_ONLY_COMMANDS` (`ipc_contract_tests.rs:1231-1300`), nếu không test contract fail.

### 8.1 Tool `outline` (tool thứ 8)

```json
{
  "type": "function",
  "function": {
    "name": "outline",
    "description": "Phase 2 engine. ONE entry point for the whole outline pipeline: seed the anchor, generate a stage with parallel subagents, review proposals, and commit approved proposals into the project state in dependency order. Never write state directly with this tool: commit only accepts items the author approved.",
    "parameters": {
      "type": "object",
      "properties": {
        "action": {"type": "string", "enum": ["status","start","run","items","regenerate","commit","cancel"]},
        "stage":  {"type": "string", "enum": ["seed","arcs","chapters","cast","world","scenes"]},
        "range":  {"type": "string", "description": "vd '1-10' cho chapters; bỏ trống = tất cả"},
        "keys":   {"type": "array", "items": {"type": "string"}},
        "note":   {"type": "string", "description": "hướng dẫn khi regenerate"}
      },
      "required": ["action"]
    }
  }
}
```

Hành vi từng action (ghi trong `outline_playbook.md`, tiếng Việt):

| action | Làm gì | Trả về |
|---|---|---|
| `status` | đọc run hiện tại | `{stage, status, counts, next_stage, blockers}` |
| `start` | seed anchor + tạo run (idempotent) | như `status` |
| `run` | fan-out stage, **không** ghi state | `{stage, shards, items, failed, usage}` |
| `items` | liệt kê proposal (rút gọn) | `{items:[{key,title,status}]}` |
| `regenerate` | chạy lại 1 item với `note` | item mới |
| `commit` | validate + ghi state theo STATE_ORDER | `{committed, skipped, missing, warnings}` |
| `cancel` | dừng run | `{cancelled}` |

Playbook bắt buộc chứa:
- Thứ tự stage; không nhảy cóc; `scenes` cần cả `cast` và `world`.
- Chỉ commit khi user đã nói "chốt"/bấm nút; nếu user còn sửa thì `regenerate`.
- Không ghi `require.*`; không bịa `author`.
- Khi lỗi validate: đọc đúng path trong message, sửa đúng field, không gọi lại y nguyên.
- Không tự tăng `max_calls`; đề xuất range nhỏ hơn.

### 8.2 Command (IPC) mới

`src-tauri/src/commands/outline.rs`:

```rust
outline_status(sessionId) -> OutlineStatus
outline_start(projectId, sessionId) -> OutlineStatus
outline_run(sessionId, stage, range, channel: Channel<OutlineEvent>) -> OutlineRunResult
outline_items(sessionId, stage) -> Vec<OutlineItem>
outline_regenerate(sessionId, stage, key, note) -> OutlineItem
outline_commit(sessionId, stage, keys: Option<Vec<String>>) -> CommitResult
outline_cancel(sessionId) -> bool
outline_anchor(projectId) -> String
```

- CamelCase arg ở JS (`sessionId`, `projectId`), đăng ký qua `merlink_handlers!`.
- Thêm vào `CommandMap` (`src/lib/ipc.ts:18-36`), `browser-mock.ts` (`941-1012`), và bảng
  `ipc_contract_tests.rs` (`337-350`).

### 8.3 Event

`OutlineEvent` là **enum riêng**, không thêm variant vào `AgentEvent` (`models.rs:178-218`): mọi
variant mới ở đó buộc phải sửa match exhaustive trong `runtime/agent/tests.rs:14-23`, sửa union
`AgentEvent` ở `src/types.ts:143-163`, `applyLiveEvent` (`Chat.tsx:67-98`) và mock
(`browser-mock.ts:806-833`) — churn không cần thiết cho một luồng chạy nền khác.

`OutlineEvent` (models.rs) qua `Channel<OutlineEvent>`:

```
run_started{stage,calls}
shard_started{lane,key,index,total}
shard_finished{lane,key,ok,failed}
items_ready{stage,count}
run_finished{stage,ok,failed,usage}
run_failed{stage,message}
commit_finished{stage,committed,warnings}
```

---

## 9. UI

### 9.0 Ràng buộc frontend đã kiểm (không suy đoán)

- Router là **HashRouter**, mọi route là con phẳng của `Shell` (`src/App.tsx:388-403`); Chat có
  `/chat` và `/chat/:sessionId` (`App.tsx:393-394`), Chat được nới rộng bằng `is-wide` khi path bắt
  đầu `/chat` (`App.tsx:369`).
- **Không có Tauri `listen()`** trong repo. Tín hiệu liên trang đi bằng DOM CustomEvent
  (`merlink:new-chat`, `merlink:sessions-changed` — `App.tsx:113,167`) hoặc qua `Channel<T>` truyền
  vào từng command (`src/lib/tauri.ts:21-30`).
- `AgentEvent` **không mang payload dữ liệu** (`src/types.ts:143-163`): `tool_started` chỉ có
  `id/name/path/status`. Vì vậy tiến độ outline phải đi qua `Channel<OutlineEvent>` **riêng** của
  command outline (mục 8.3), và board đọc dữ liệu bằng command (`outline_items`), không đọc từ event.
- `Markdown` **không hỗ trợ bảng** (`src/lib/markdown.ts:36-71`): board scene là JSX tự dựng, không
  dùng markdown table.
- **Không có test runner frontend** (không có `*.test.tsx`): acceptance UI chỉ có `pnpm build` +
  kiểm tay; đừng đặt acceptance dạng "component test".
- `super_json_read` trả `unknown` (`src/lib/ipc.ts:29`): kiểu row cho phase 2 phải khai báo ở
  `src/types.ts`.
- Mọi command mới phải có `case` trong `browserInvoke` (`src/lib/browser-mock.ts:939-1018`), nếu
  không preview ném lỗi ở default case (`1016-1017`).
- Chat đã có máy epoch/ref chống race (`Chat.tsx:246-275`); outline async phải dùng cùng pattern
  (mỗi lần chạy stage tăng epoch, callback cũ bị bỏ).

### 9.1 Chat → Outline mode (`src/pages/Chat.tsx`)

- `const [outline, setOutline] = useState<OutlineStatus | null>(null)`.
- Nút `Viết dàn ý` (`Chat.tsx:779-792`): thay `sendMessage(OUTLINE_PROMPT)` (`Chat.tsx:788`, xoá
  `OUTLINE_PROMPT` ở `:39`) bằng `await startOutline()` → `outline_start` rồi
  `outline_run("arcs", channel)`.
- Slot chèn: vùng message là `Chat.tsx:796-1037`, composer ở `1037-1041`. Khi `outline.active`,
  render `<OutlineWorkspace/>` **trong slot message** (giữ topbar/progress/provider/model); composer
  thu gọn còn 1 dòng "Trao đổi thêm với Merlink" nhưng vẫn gửi chat bình thường.
- Giữ nguyên gate `canOutline` (`Chat.tsx:664-667`) và logic project binding (`310-347`, `388-396`).
- Reload: `Chat.tsx` mount → `outline_status(sessionId)`; nếu active thì vào mode luôn (server là
  nguồn sự thật, không lưu localStorage).
- Không thêm route mới cho 2.1-2.3 (giữ đúng yêu cầu "màn hình chat thay đổi"). Nếu sau này cần
  deep-link, thêm `<Route path="chat/:sessionId/outline">` cạnh `App.tsx:394`; **không** làm ở 17.3.

### 9.2 `<OutlineWorkspace>` (`src/pages/OutlineWorkspace.tsx`)

- Header: 3 tab `2.1 Tóm tắt`, `2.2 Nhân vật & Thế giới`, `2.3 Phân cảnh` + trạng thái từng tab
  (chưa chạy / đề xuất / đã chốt) + nút Chạy/Chạy lại. Dùng pattern rail của `SettingsDialog`
  (`SettingsDialog.tsx:451-472`, class `.is-active`), không phát minh stepper mới.
- Body theo tab: board (mục 9.3-9.5).
- Footer: `Chốt stage`, `Huỷ`, `Sang bước tiếp`.
- Chặn tab sau khi tab trước chưa chốt (disabled + tooltip lý do).
- Trong lúc shard đang chạy: hiển thị `shard_started/finished` theo kiểu live-frame của Chat
  (`Chat.tsx:1001-1035`), nhưng là component riêng.

### 9.3 Board 2.1 (`components/outline/ArcsBoard.tsx`)

- Card arc: position, name, goal, outcome, summarize (markdown), số chương.
- Danh sách chương: name, summarize, beats (chip), scene_count.
- Chọn range chương để expand (`1-10`, `1-30`, `all`) — mặc định 10.
- Item action: Sửa (inline textarea), Viết lại (note), Bỏ, Duyệt.

### 9.4 Board 2.2 (`components/outline/CastWorldBoard.tsx`)

- 2 cột: `Nhân vật` (card có tab Ngoại hình / Năng lực / Ký ức / Motif / Hiểu biết) và
  `Thế giới` (setting tree theo `type`, lore theo `type`, power system + tier).
- Card quan hệ: A ↔ B, summary, 7 chỉ số `relationship_states` (thanh nhỏ), category/role.
- Duyệt theo card; "Duyệt tất cả" chỉ duyệt item `pending`.

### 9.5 Board 2.3 (`components/outline/ScenesBoard.tsx`)

- Bảng chương × scene (accordion theo chương): mỗi scene 1 dòng: position, name, POV, setting,
  style preset, narrator, directive (chip), target words, cast (avatar chip), quan hệ (chip).
- Cột cảnh báo: thiếu POV, thiếu cast, preset không có trong catalog, target_words lệch budget.

### 9.6 Phase 3: tab `Write` (`src/pages/ProjectDetail.tsx` + `components/write/*`)

- Sub-tab: `Tổng quan`, `Arc/Chương`, `Nhân vật`, `Thế giới`, `Phong cách`, `Viết`.
- `Viết`: chọn chương → danh sách **block scene** dọc; mỗi block có:
  - header: số thứ tự, tên, POV, preset, word count, trạng thái (`PLANNED/WRITTEN/REVISED/LOCKED`);
  - body: prose (markdown), hoặc nút `Viết scene này`;
  - action: `Viết lại` (1 scene), `Khoá`, `Mở khoá`, `Xem chi tiết thiết lập`.
- Multi-select block → `Viết lại các scene đã chọn` với 1 note.
- Nút `Viết chương này` chạy tuần tự các block chưa viết (không song song trong 1 chương để giữ
  mạch).
- Cảnh báo vi phạm prose (emoji/`–`) hiển thị trên block + nút "Sửa tự động".

### 9.7 CSS

- Thêm `src/styles/outline.css`, **import từ `OutlineWorkspace.tsx`** (đúng pattern `Projects.tsx:7`
  import `../styles/projects.css`, `Characters.tsx:10`). Không nhồi vào `src/index.css` (file này
  packed 1 dòng/rule-group) trừ khi cần override chat shell.
- Dùng lại token trong `src/index.css:2` (`--ink --muted --line --glass --accent --success
  --warning --error …`), class prefix `outline-*`, state `.is-*` như phần còn lại. Không tạo palette
  mới, không thêm Tailwind/CSS module (không có trong dependency).
- Dark theme tự động nếu chỉ dùng token; không hardcode màu.

---

## 10. SPRINT BREAKDOWN

### SPRINT 17.1 — STATE v3 + CATALOG + ANCHOR

**GOAL:** state chứa được dàn ý, catalog đọc được từ code, anchor sinh đúng 6 đoạn, không đổi 81
path.

**CONTEXT:** schema v2.1 thiếu field (mục 4.1); catalog chỉ có SQL Postgres; chưa có anchor.

**TASKS:**
1. `docs/schema/runtime/super.json`: thêm field mục 4.1; cập nhật `$comment` → v3.
2. `docs/design/super-json.md`: thêm mục "v3 — field bổ sung cho phase 2" + bảng field + lý do.
3. `scripts/gen-catalog.mjs`: parse `docs/schema/seed/*.sql` → `docs/schema/catalog/*.json`
   (`{code,name,description,...}`); chạy 1 lần, commit output; script có `--check` để CI so drift.
4. `src-tauri/src/catalog/mod.rs`: `Catalog::get()`, `lookup(kind, code)`, `list(kind)`,
   `validate_code(kind, code)`; test: mỗi JSON parse được, `code` unique, count khớp hằng số.
5. `src-tauri/src/super_json/io.rs`: migration `v2 → v3` (refresh `fields`, backup `super.v2.json`,
   idempotent); test round-trip.
6. `src-tauri/src/outline/anchor.rs` (+ `outline/mod.rs` khung): `anchor`, `anchor_compact`,
   `anchor_json`; test 6 đoạn + placeholder `[chưa có: …]`. Đây là **bước "trích xuất"**: đọc state
   → khối văn + JSON có key ổn định (đúng các path trong template user đưa).
7. `outline/seed.rs`: **bước "ghi nhận"**: `genre_code` resolve từ `project_genres.rows[].genre_id`
   qua `catalog::genres` (khớp `code` hoặc `name`, không khớp → item `needs_review`), ghi
   `optional.project.target-audience`/`summarize` khi user đã nói. Không đụng `require`.
8. `commands/outline.rs::outline_anchor`, `outline_seed`, đăng ký `merlink_handlers!`, thêm
   `CommandMap` + browser-mock.
9. `tools/specs.rs`: thêm spec `outline` **chỉ với** `status|start|items` (phần còn lại bật ở 17.2);
   làm đủ 6 điểm ở mục 8.0: `tools/mod.rs:18-25` (7 → 8, `project_create` vẫn đầu), `tools/execute.rs`
   arm mới, `tools/labels.rs` `status_message`/`tool_path`, `runtime/session.rs:71-74`, và
   `prompts/harness_system.md:176-188`.
10. Cập nhật test đếm tool: `tools/specs.rs:231-249`, `:262-267`, `runtime/agent/tests.rs:601-614`
    (7 → 8), `test_tool_specs_cover_all_tools_and_expose_81_paths`, và bất kỳ test assert danh sách
    field.

**ACCEPTANCE:**
- `cargo test` xanh; test mới: `test_migration_v2_to_v3_is_idempotent`,
  `test_catalog_entries_parse_and_are_unique`, `test_anchor_has_six_paragraphs_and_placeholders`,
  `test_outline_tool_is_registered`.
- `super_json::valid_paths().len() == 81`, `required_paths().len() == 12`,
  `optional_paths().len() == 35` (test khẳng định lại).
- `pnpm build` pass; `outline_anchor` gọi được từ browser preview.

**OUT OF SCOPE:** chưa có fan-out, chưa có bảng SQLite mới, chưa có UI workspace.

---

### SPRINT 17.2 — OUTLINE ENGINE CORE (runs, contracts, fan-out, commit skeleton)

**GOAL:** chạy được một stage giả lập end-to-end bằng `scripted` adapter: seed → đề xuất → duyệt →
commit, có event, có resume.

**CONTEXT:** 17.1 xong; chưa có chỗ lưu nháp, chưa có parser contract, chưa có commit.

**TASKS:**
1. `db/mod.rs`: `SCHEMA_VERSION = 2`; bảng `outline_runs(id, project_id, session_id, stage, status,
   calls, tokens_in, tokens_out, created_at, updated_at)` và
   `outline_items(id, run_id, stage, lane, key, position, payload_json, status, error, created_at)`
   + index `(run_id, stage, status)`; test migrate 1→2 và 0→2; sửa test assert `version == 1`.
2. `db/outline.rs`: CRUD (`create_run`, `run_for_session`, `upsert_items`, `list_items`,
   `set_item_status`, `update_run_progress`).
3. `runtime/complete.rs`: `complete()` không tool + `Completion`/`Usage`; test bằng `scripted`.
4. `outline/contract.rs`: `parse(lane, text)` (chấp nhận fence, sửa lỗi trailing comma), `validate`
   (key bắt buộc, kiểu, slug, ref tồn tại, counts); test cho từng lane với fixture JSON.
5. `outline/fanout.rs`: `run_plan(plan, ctx)` với semaphore 5, `JoinSet`, cancel, retry ≤2, resume;
   test với adapter giả trả lỗi 1 lần rồi đúng.
6. `outline/stage.rs`: `Stage`, `STATE_ORDER`, gate; test gate `scenes` cần `cast`+`world`.
7. `outline/apply.rs`: commit `arcs` + `chapters` (đủ để test end-to-end); validate trước khi ghi;
   test out-of-order bị chặn.
8. `outline/mod.rs`: `start/status/run/items/regenerate/commit/cancel`.
9. `commands/outline.rs`: đủ 7 command + `Channel<OutlineEvent>`; `models.rs` `OutlineStatus`,
   `OutlineItem`, `OutlineRunResult`, `CommitResult`, `OutlineEvent`.
10. `tools/execute.rs`: `outline` action đầy đủ; `prompts/outline_playbook.md`; `runtime/prompt.rs`
    chèn playbook khi `filled_required == required_total`.
11. IPC: `CommandMap`, `browser-mock`, `ipc_contract_tests` cho toàn bộ 7 command — nhớ `ContractEntry`
    + `body_*` (`ipc_contract_tests.rs:319-388`) và để tên command xuất hiện trong file frontend hoặc
    khai báo `RUST_ONLY_COMMANDS` (`:1231-1300`), nếu không contract test fail.

**ACCEPTANCE:**
- `test_outline_engine_end_to_end_scripted`: seed → arcs → chapters → accept → commit → state có 3
  arc + 12 chương, `count` khớp, `require` không đổi.
- `test_outline_commit_refuses_out_of_order`, `test_outline_fanout_is_bounded_and_resumable`,
  `test_outline_contract_rejects_lane_drift`, `test_outline_events_stream_per_shard`.
- `pnpm build` pass; browser preview chạy được `outline_status`.

**OUT OF SCOPE:** chưa có prompt thật cho arcs/chapters (17.3), chưa có UI board.

---

### SPRINT 17.3 — STAGE 2.1 (arcs + chapters) + BOARD 2.1

**GOAL:** người dùng chạy 2.1 thật, đọc summarize của mọi arc và `n` chương, sửa/viết lại/chốt.

**TASKS:**
1. `prompts/outline_arcs.md`, `prompts/outline_chapters.md` (hướng dẫn chi tiết: budget, tone,
  cấm emoji/`–`, JSON-only, không bịa tên riêng ngoài require, phân bổ chương theo tỉ lệ).
2. `outline/prompts.rs`: builder cho `arc_smith`, `chapter_smith` (nhận `range`, `arc`, budget).
3. `outline/plan.rs`: `plan_arcs`, `plan_chapters(range)`; chia shard 10 chương; tính `max_calls`.
4. `outline/apply.rs`: commit arcs/chapters + `scene_count` + kiểm D13.
5. UI: `src/pages/OutlineWorkspace.tsx` (shell + 3 tab disabled đúng gate) +
   `src/components/outline/ArcsBoard.tsx`, `src/components/outline/ItemCard.tsx`;
   `Chat.tsx` mode switch tại slot `796-1037`; `src/styles/outline.css` import từ
   `OutlineWorkspace.tsx` (pattern `Projects.tsx:7`); thêm type vào `src/types.ts`.
6. `browser-mock`: dữ liệu mẫu 3 arc/12 chương để preview không cần backend.

**ACCEPTANCE:**
- `test_stage_21_prompt_contains_budget_and_prohibitions` (không có emoji, có chuỗi cấm `–`).
- `test_stage_21_commit_respects_plan_counts` (đặt plan 3/12/36 → commit 3 arc, 12 chương,
  Σscene_count = 36; lệch → lỗi).
- `pnpm build` pass + `cargo test` xanh (không có test runner frontend, nên UI kiểm bằng tay).
- Manual: bấm Viết dàn ý → thấy 3 arc + 12 chương, sửa 1 chương, chốt, reload vẫn ở mode outline.

**OUT OF SCOPE:** chưa có 2.2/2.3; chưa ghi character/world/scene.

---

### SPRINT 17.4 — STAGE 2.2 (cast + world, 2 lane song song) + BOARD 2.2

**GOAL:** 2 lane chạy song song, user review card nhân vật/thế giới/quan hệ, chốt, state có đủ
entity phụ thuộc.

**TASKS:**
1. `prompts/outline_cast.md` (nhân vật + ngoại hình + năng lực + ký ức + hiểu biết + motif +
  động lực), `prompts/outline_world.md` (setting/lore/item/power), `prompts/outline_bonds.md`.
2. `outline/plan.rs`: `plan_cast` (shard 6 nhân vật), `plan_world` (2 shard), `plan_bonds`
  (sau cast).
3. `outline/apply.rs`: commit bước 6-20 của STATE_ORDER (characters → appearances → motifs →
  competences → memories → relationships/states/roles/motifs → themes/arguments → project_motifs).
4. `outline/contract.rs`: validator riêng cho cast/world/bonds (ref chéo: `motifs[].code` phải có
  trong catalog; `world_settings.parent` phải là ref trong cùng output).
5. UI: `CastWorldBoard.tsx` + `ItemCard` + filter/tab; card quan hệ với thanh 7 chỉ số.
6. Test fixture: 8 nhân vật, 12 setting, 6 quan hệ; commit và assert toàn bộ ref.

**ACCEPTANCE:**
- `test_stage_22_two_lanes_run_concurrently` (đo overlap ≥ 2 shard cùng lúc).
- `test_stage_22_commit_orders_characters_before_relationships`.
- `test_stage_22_rejects_unknown_motif_code` (lỗi nêu đúng code).
- `test_memories_committed_with_null_source_scene` (vòng phụ thuộc).
- Manual: chốt 2.2 → tab 2.3 mở khoá; xoá 1 nhân vật → quan hệ liên quan bị đánh dấu lỗi.

**OUT OF SCOPE:** chưa sinh scene.

---

### SPRINT 17.5 — STAGE 2.3 (scene binder) + BOARD 2.3 + LINK PASS

**GOAL:** mỗi chương có danh sách scene đầy đủ preset/setting/cast/POV/quan hệ/directive/rule.

**TASKS:**
1. `prompts/outline_scenes.md`: binder đọc chapter summarize + beats + cast sheet + settings +
   catalog (style/narrator/directive) và **chọn** preset/narrator theo nội dung; đặt rule + kỹ
   thuật; target words theo budget chương.
2. `outline/plan.rs`: `plan_scenes(range)` — 1 call/chương, 5 chương song song.
3. `outline/apply.rs`: commit bước 21-24 (scenes, scene_characters, `scenes.relationships`,
   link `memories.source_scene_id`).
4. `outline/contract.rs`: validator scene (preset/narrator code tồn tại; cast ref tồn tại;
   `cast[].character` phải tồn tại và đúng **một** `is_pov: true` mỗi scene; `Σtarget_words` quanh
   budget chương; `position` liên tục từ 1; `relationships` chỉ trỏ relationship đã commit).
5. UI: `ScenesBoard.tsx` (bảng chương × scene, cột cảnh báo) + preview panel cho 1 scene.
6. `db/outline.rs`: `regenerate` theo `key` cho scene; resume theo chương.

**ACCEPTANCE:**
- `test_stage_23_every_scene_has_pov_preset_narrator_setting`.
- `test_stage_23_scene_positions_are_contiguous`.
- `test_stage_23_memory_source_scene_linked_after_commit`.
- `test_stage_23_rejects_preset_code_not_in_catalog`.
- Manual: 12 chương × 3 scene, sửa 1 scene (đổi preset), chốt, `super_json_get` thấy đúng.

**OUT OF SCOPE:** chưa viết văn.

---

### SPRINT 17.6 — PHASE 3 WRITING ENGINE (backend)

**GOAL:** sinh prose theo scene, lưu file, version, chặn emoji/`–`, viết lại 1 hoặc nhiều scene.

**TASKS:**
1. `outline/prose.rs` (hoặc `writing/mod.rs`): `Bundle` dựng từ state (mục D11), `write_scene`,
   `rewrite_scenes(keys, note)`, `violations(text)`, `sanitize(text)`.
2. `paths.rs`: `content_dir(project_id)`, `scene_path(project_id, arc, chapter, position)`,
   `version_path(...)`; test path an toàn (không `..`).
3. `prompts/prose_scene.md` + `prompts/prose_rules.md` (cấm emoji, cấm `–`/`—`, tiếng Việt, không
   tiêu đề markdown, không meta-comment, độ dài ±15% target_words).
4. `commands/writing.rs`: `writing_status(projectId)`, `write_scene(sessionId, sceneKey, channel)`,
   `rewrite_selection(sessionId, sceneKeys, note, channel)`, `lock_scene`, `unlock_scene`.
5. `outline/apply.rs`: ghi `content_path`, `word_count`, `status` vào scene; `scene_versions` row cho
   bản cũ (nếu bảng được bật; nếu không, chỉ file `versions/`).
6. Test: retry khi vi phạm emoji (scripted), sanitize fallback, rewrite giữ version cũ, lock chặn
   regenerate.

**ACCEPTANCE:**
- `test_write_scene_creates_file_and_updates_state` (file tồn tại, `content_path` đúng,
  `content` vẫn null, `word_count` khớp ±15%).
- `test_write_scene_retries_on_emoji_and_dash` (lần 1 có emoji → retry; lần 2 sạch).
- `test_rewrite_selection_keeps_previous_version`.
- `test_locked_scene_is_not_regenerated`.
- `test_scene_bundle_respects_token_budget` (≤ 6.000 token ước tính).

**OUT OF SCOPE:** chưa có UI tab Write.

---

### SPRINT 17.7 — PHASE 3 UI + HARDENING + DOCS

**GOAL:** người dùng đọc/chốt cấu hình phase 2 trong ProjectDetail và viết chương theo block scene.

**TASKS:**
1. `ProjectDetail.tsx`: sub-tab `Tổng quan / Arc-Chương / Nhân vật / Thế giới / Phong cách / Viết`.
2. `components/write/ChapterWorkspace.tsx`, `SceneBlock.tsx` (markdown, collapse, multi-select,
   lock, word count, cảnh báo prose), `WriteToolbar.tsx`.
3. `commands/project.rs`: `project_outline(projectId)` trả state đã commit cho các tab đọc.
4. Guard prompt phase 1 khi document > 24 KB (D11).
5. `docs/design/phase-2.md`: mô tả pipeline, lane, thứ tự ghi, lỗi thường gặp.
6. Cập nhật `AGENTS.md`: 81 path giữ nguyên nhưng `fields` mở rộng; **8 tool**; thêm mục phase 2/3
   invariants (STATE_ORDER, prose rule, content_path).

**ACCEPTANCE:**
- `test_project_outline_returns_committed_state_for_tabs`.
- `test_prompt_compact_mode_when_document_is_large`.
- Manual end-to-end: project mới → 12 unit → Viết dàn ý → 2.1 → 2.2 → 2.3 → chốt → tab Viết →
  viết chương 1 (3 block) → viết lại block 2 → file + version đúng → reload giữ nguyên.
- `cargo test`, `cargo clippy`, `cargo fmt --check`, `pnpm build` xanh.

---

## 11. VERIFICATION

```bash
cargo test --manifest-path src-tauri/Cargo.toml   # phải 0 failed, ignored không tăng
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml --check
pnpm build
pnpm tsc --noEmit            # nếu script tồn tại
node scripts/gen-catalog.mjs --check
```

Test xương sống (phải tồn tại trước khi merge 17.7):

| Test | Bắt cái gì |
|---|---|
| `test_outline_engine_end_to_end_scripted` | toàn pipeline không mạng |
| `test_outline_commit_refuses_out_of_order` | STATE_ORDER |
| `test_stage_21_commit_respects_plan_counts` | D13 |
| `test_stage_22_commit_orders_characters_before_relationships` | phụ thuộc FK |
| `test_stage_23_every_scene_has_pov_preset_narrator_setting` | dàn ý đủ |
| `test_write_scene_retries_on_emoji_and_dash` | D12 |
| `test_migration_v2_to_v3_is_idempotent` | D1/4.2 |
| `test_outline_tool_is_registered` | 8 tool |

Kiểm tay (mỗi sprint): preview browser + app thật, dùng `scripted` provider cho 17.2, provider thật
cho 17.3+ với project 3 arc / 12 chương / 36 scene.

---

## 12. RỦI RO & CÁCH CHẶN

| Rủi ro | Chặn |
|---|---|
| 60-120 chương làm nổ chi phí | D17 `max_calls` + range mặc định 10 chương; resume |
| Model trả JSON hỏng/thiếu key | `contract.rs` retry 1 + error feedback; item `failed` không chặn shard khác |
| Ghi song song đè state | subagent **không** ghi state; chỉ `apply` tuần tự ở commit |
| super.json phình vì prose | D10 prose ở file; `content` luôn null |
| Prompt embed cả document | D11 + guard 24 KB ở 17.7 |
| Trùng tên nhân vật / quan hệ mồ côi | validator ref theo `ref` slug + commit theo STATE_ORDER |
| Port DBML ngầm | D1: SQLite chỉ thêm 2 bảng runtime, không bảng entity |
| `tokio` thiếu feature `rt` | D16: bật feature trên crate có sẵn; fallback thread scope |
| UI phình | mỗi sprint 1 board, không làm board của sprint sau |

---

## 13. SUBAGENT ROUTING (dev-time) + HANDOFF

Mỗi sprint giao cho **một** dev subagent (hoặc 2 nếu UI/backend tách được), theo template trong
`AGENTS.md` mục 4, với:

- `provider=openai-codex`, `model=gpt-6-luna`, `reasoning_effort=high` cho task code/nặng.
- `provider=commandcode`, `model=deepseek/deepseek-v4.1-flash`, `reasoning_effort=high` cho task
  dữ liệu/script/boilerplate (song song được, rẻ hơn).
- Không dùng `antigravity` nữa (yêu cầu user, từ sau 17.1).

Handoff format giữ nguyên `STATUS / SPRINT / COMPLETED / FAILED / NEED_INPUT / NEXT ACTION`, và
bổ sung mục `STATE_ORDER CHECK` ghi rõ stage nào đã commit được, stage nào còn `pending`.

Câu hỏi duy nhất còn mở (cần user trả lời trước 17.6): phase 3 có cần bảng `scene_versions` thật
trong SQLite hay chỉ cần file `versions/`? Mặc định của plan: **chỉ file** (đủ để xem/khôi phục,
không thêm bảng).

---

## 14. KHỐI LỆNH CHUẨN (template bắt buộc khi giao từng sprint 17.x)

Chép nguyên khối này vào prompt dev subagent, chỉ thay `[N]` và phần TASKS/ACCEPTANCE của sprint
tương ứng. Đây là template ở `AGENTS.md` mục 4, đã điền sẵn cho phase 2.

```text
═══════════════════════════════════════════════════════
SPRINT 17.[N] — [TÊN SPRINT]
═══════════════════════════════════════════════════════

CONTEXT:
Phase 1 đã xong (super.json v3, 81 path, 12 required unit). Phase 2 bắt đầu từ nút "Viết dàn ý":
backend Merlink orchestrate các subagent song song để viết full state, người dùng review theo gate.
Spec nguồn: docs/plan/sprint-17.md (mục 3 DECISIONS, mục 4 data model, mục 7 STATE_ORDER).

GOAL:
[Một câu đo được, lấy nguyên từ mục "GOAL" của sprint 17.N trong docs/plan/sprint-17.md.]

TASKS:
[Chép nguyên danh sách TASKS của sprint 17.N; mỗi task phải nêu file cụ thể.]

ACCEPTANCE:
[Chép nguyên mục ACCEPTANCE của sprint 17.N. Không thêm tiêu chí chủ quan.]

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong docs/plan/sprint-17.md → DỪNG, hỏi
- Cần thêm dependency ngoài list đã cho → DỪNG, hỏi
- Task vượt quá 30 phút ước tính → DỪNG, chia nhỏ lại
- Muốn port bảng entity từ docs/schema/system.dbml sang SQLite → DỪNG (D1 cấm)

───────────────────────────────────────────────────────
🔁 RETRY BUDGET (giới hạn số lần thử)
───────────────────────────────────────────────────────
- Mỗi task: tối đa 2 lần thử
- Mỗi lần thử: phải thay đổi approach, KHÔNG được lặp y hệt
- Sau lần 2 fail: DỪNG task đó, chuyển sang task khác
- Cuối sprint: list ra các task chưa xong + lý do

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS (khi nào hỏi user)
───────────────────────────────────────────────────────
DỪNG và hỏi user khi:
1. Lỗi không rõ nguyên nhân sau 2 lần debug
2. Cần chọn giữa 2 approach mà spec không nói
3. Cần thay đổi kiến trúc hiện tại (store shape, STATE_ORDER, schema v3)
4. Phát hiện bug ở sprint trước ảnh hưởng sprint này
5. Cần user test thủ công (visual, UX, chất lượng văn)

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- KHÔNG refactor code ngoài file list ở TASKS
- KHÔNG thêm feature ngoài GOAL của sprint
- KHÔNG đổi UI/UX của màn hình khác
- KHÔNG optimize premature
- KHÔNG port docs/schema/system.dbml sang SQLite
- KHÔNG thêm dependency mới (xem D16)
- KHÔNG bịa `optional.project.author` (D14)
- KHÔNG cho subagent/LLM ghi thẳng state: chỉ `apply` được ghi (D6)

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 17.[N]

✅ COMPLETED:
- [task 1]: [file changed]
- [task 2]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- Câu hỏi 1: [mô tả] | Option A: [...] | Option B: [...]

🧱 STATE_ORDER CHECK:
- stage đã commit được: [arcs | chapters | cast | world | scenes]
- stage còn pending: [...]
- vi phạm thứ tự (nếu có): [entity] cần [entity] trước

📋 NEXT ACTION:
- Nếu DONE → "Ready for sprint 17.[N+1], paste next prompt"
- Nếu BLOCKED → "Waiting for user response on X"
- Nếu PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════
```

---

## 15. QUYẾT ĐỊNH VẬN HÀNH (uỷ quyền cho orchestrator)

Người dùng đã uỷ quyền: **orchestrator tự chốt mọi nhánh còn mở theo phương án recommend**, không
dừng hỏi lại, miễn giữ đúng các gate ở mục 11.

Đã chốt thay cho các câu hỏi mở:

| Câu hỏi | Chốt | Hệ quả |
|---|---|---|
| Phase 3 có bảng `scene_versions` trong SQLite? | **Không.** Chỉ file `content/.../<pos>-v<n>.md` + `word_count` trong state | Không bump schema lần 3; revert = copy file cũ lại |
| `scenes.content` hay `content_path`? | `content` **luôn null**, `content_path` trỏ file | JSON state nhỏ, ghi song song an toàn |
| Thêm bảng `scene_relationships`? | **Không.** `scenes.relationships` là JSON array | Giữ nguyên 34 bảng / 81 path |
| State store: super.json hay SQLite? | **super.json v3** | SQLite chỉ `outline_runs`/`outline_items` |
| `outline` action ở 17.1 | chỉ `status` + `anchor` | `run/commit` bật ở 17.2 |
| Seed `genre_code` (`outline/seed.rs`) | chuyển sang **17.2** (cần engine + catalog cùng lúc) | 17.1 giao anchor (extract) + catalog; 17.2 giao seed (record) |
| Số shard mặc định | chapters: 10 chương/shard; cast: 6 nhân vật/shard; scenes: 1 chương/call; concurrency 5 | có thể chỉnh bằng hằng số, không đổi contract |
| Catalog vocab/focus/devices của style preset | nếu seed không parse được join `preset_id` thì để mảng rỗng + ghi `notes` trong JSON | không chặn 17.5; bổ sung sau bằng `--check` |
| Commit các file seed/init_schema đang untracked | **có**, cùng sprint 17.1 (là nguồn của catalog) | tránh drift giữa SQL nguồn và JSON sinh ra |

Quy ước khi gặp escalation trong lúc thi công:

1. Nếu là lựa chọn kỹ thuật có 2 phương án tương đương → chọn phương án **ít xâm lấn code hiện có**
   (giữ 81 path, giữ 7 model provider, không đổi chữ ký hàm public).
2. Nếu là lựa chọn sản phẩm (UX, số bước, mặc định) → chọn phương án **ít bước nhất cho người dùng**
   và ghi lại trong `docs/design/phase-2.md`.
3. Nếu là rủi ro mất dữ liệu → chọn phương án **có backup/khôi phục** (vd ghi `super.v2.json` trước
   khi migrate).
4. Mọi quyết định thay đổi so với tài liệu này phải được ghi vào mục 15 này cùng lý do 1 dòng.
