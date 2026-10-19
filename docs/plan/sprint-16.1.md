# SPRINT 16.1 — TOOL CONTRACT: `_json_index`, `add_row`/`add_field`, PROMPT CÓ JSON CONTEXT

> **Trạng thái: DONE — merged vào `feat/brand-mark` tại `c15351a`** (commit nội dung `a6ab1dd`).
> Branch: `sprint-16.1-tool-contract` · Base: `feat/brand-mark` (sau merge Sprint 16, `d9c502b`)
> Tiền đề: Sprint 16 đã đưa super json lên v2.1 (12 require / 35 optional, 1 entry/bảng,
> `project_motifs` require + scope, migration v2→v2.1). Sprint này **không** đổi nhóm bảng.
> Baseline trên branch này: `cargo test` **131 passed / 3 ignored**, `pnpm build` xanh.

## CONTEXT

- Tool hiện tại: `project_create`, `super_json_get`, `super_json_set`, `super_json_missing`, `echo`.
  `super_json_set` thay **cả** mảng `rows`, nên thêm 1 row phải đọc cả bảng rồi ghi lại — model không
  thấy row/id hiện có nên hay điền tên vào chỗ id ("id bị ghi đè thành tên").
- Prompt chỉ có `Project: <id>`; model phải gọi `super_json_get` mới biết hiện trạng.
- Yêu cầu người dùng: tool contract chặt, prompt luôn cấp JSON context, có `add_field`/`add_row`,
  convention `_json_index` cho FK draft, và 2 câu bắt buộc (check field + arc default).

## DECISIONS (chốt)

### D1 — Draft ref: `_json_index` + `<field>_json_index`

- `_json_index` = **vị trí 0-based của row trong `rows[]`**. Do **tool gán**, không phải id, không do
  model viết; mọi lần ghi `rows`/`add_row` đều chuẩn hoá lại `_json_index = vị trí`.
- Với mọi field khai báo kết thúc bằng `_id`, cho phép thêm `<field>_json_index` (int ≥ 0) để trỏ
  tới row draft của bảng đích. Ví dụ: `from_character_id_json_index: 0`.
- Ref **thay thế** được cho required field `_id`: row `relationships` chỉ cần 2 ref là hợp lệ;
  field `_id` tương ứng để null hoặc bỏ hẳn.
- Range check: nếu `(table, field)` có trong `REF_TARGETS`, `_json_index` phải `< target.rows.len()`.
  Bảng đích ngoài draft (`motifs`, `genres`, `style_presets`, `narrator_configs`) chỉ check int.
- `_json_index` và `<field>_json_index` là **synthetic** trong `table_fields()`; không thêm vào
  `fields` của `super.json` (giữ nguyên 81 path / 34 bảng).
- `null` trên cột **optional** = "chưa đặt" (hợp lệ, kể cả `world_setting_id`/`*_json_index`); `null`
  trên cột **required** bị coi là thiếu ở Complete và bị `add_field` từ chối. Bổ sung sau khi merge
  (hotfix root) vì `fields` contract hiển thị `null` nên model có thể copy nguyên vào row.

### D2 — Hai tool mới (tổng 7)

`TOOL_NAMES = [project_create, super_json_get, super_json_set, super_json_add_row, super_json_add_field, super_json_missing, echo]`.

- `super_json_add_row {table, row}`:
  - append `row` vào `rows`, gán `_json_index = rows.len()-1`.
  - `count`: nếu `count == null` → `rows.len()`; nếu `count == len_cũ` → `rows.len()`;
    nếu `count > len_cũ` (đang draft theo kế hoạch) → **giữ nguyên**.
  - validate **Draft** (không đòi đủ required, không đòi `count == len`).
- `super_json_add_field {table, json_index, field, value}`:
  - set đúng 1 field của row tại `json_index`; reject `_json_index`/`fields`/`count`/`rows`/`table`.
  - field phải có trong `table_fields(table)`; type/ref validate như `set`.
- `super_json_set` giữ nguyên: project leaf, `.count`, replace toàn bộ `.rows` (Complete mode).
- **Mọi tool ghi** (`set`, `add_row`, `add_field`) trả thêm `warnings: []` (xem D4).

### D3 — Prompt luôn có JSON context

- `assemble_prompt` nhúng **toàn bộ document** (pretty JSON) qua placeholder `{super_json}`;
  đọc trực tiếp `paths::super_json_path(project_id)` **chỉ khi file tồn tại** (không tạo ghost),
  parse lỗi thì nhúng raw. `project_id` rỗng → `(chưa có project)`.
- Thay `{super_json}` **sau cùng** (sau các placeholder khác) để không replace dây chuyền vào JSON.
- Prompt thêm 4 khối: Current super json, Draft references, Mandatory field check, No-arc default;
  cập nhật scenes/motifs/POV và tool rhythm. Giữ đủ 34 tên bảng + 13 project leaf.

### D4 — Warning mềm

`tables::warnings(document) -> Vec<String>` trả đúng 2 câu (tiếng Anh, khớp style error hiện có):
- `scene_characters needs at least one is_pov row while scenes has {N} row(s)`
- `project_motifs needs at least one row with scope = "project"`

Trả trong kết quả `super_json_set`/`add_row`/`add_field` dưới key `warnings`. Không chặn ghi,
không đổi `SuperJsonProgress`, không đổi 2 rule hoàn tất của Sprint 16.

## TASKS

### T1 — `src-tauri/src/super_json/schema.rs`

1. Tách `fn declared_fields(table) -> Vec<String>` (phần union `fields` hiện có trong `table_fields`).
2. `pub fn ref_fields(table: &str) -> Vec<String>`: mỗi field trong `declared_fields` kết thúc `_id`
   → `<field>_json_index`.
3. `table_fields(table)` = `declared_fields` + `_json_index` + `ref_fields` (dedupe, giữ thứ tự).
4. `column_kind`: đặt **trước** các nhánh khác:
   `if column == "_json_index" || column.ends_with("_json_index") { return FieldKind::NonNegativeInt; }`
5. `merged_table_entry` thêm key `"refs": ref_fields(table)` (additive; giữ 4 key cũ).
6. Test mới: `test_table_fields_expose_json_index_and_refs`,
   `test_column_kind_marks_refs_as_non_negative_ints`.

### T2 — `src-tauri/src/super_json/tables.rs`

1. `pub enum RowMode { Draft, Complete }`.
2. `pub fn normalize_json_index(rows: &mut Value)`: mọi object trong array nhận `_json_index = index`.
3. Đổi `validate_rows` thành `(table, rows, count, document: Option<&Value>, mode: RowMode)`:
   - luôn: array of object; key lạ reject (liệt kê allowed); type qua `column_kind`.
   - required: `object.contains_key(field)` **hoặc** (`field.ends_with("_id")` và có
     `<field>_json_index`). Message nêu cả hai lựa chọn.
   - ref: mỗi `<x>_id_json_index` phải là int ≥ 0; nếu `(table, x_id)` trong `REF_TARGETS` thì
     `< target.rows.len()`; `document == None` thì bỏ range check.
   - `Complete`: + `count == rows.len()` (khi count có), + characters đúng 1 `is_primary`
     (khi rows không rỗng), + `document_rule` (POV/motif của Sprint 16).
   - `Draft`: bỏ count-equality, required, characters-primary, document_rule.
4. `const REF_TARGETS: &[(&str, &str, &str)]` (table, column, target) — ít nhất:
   `chapters.arc_id→arcs`, `scenes.chapter_id→chapters`, `scenes.world_setting_id→world_settings`,
   `scene_characters.scene_id→scenes`, `scene_characters.character_id→characters`,
   `relationships.from_character_id→characters`, `relationships.to_character_id→characters`,
   `relationship_roles.relationship_id→relationships`,
   `relationship_roles.category_id→relationship_categories`,
   `relationship_motifs.relationship_id→relationships`,
   `relationship_states.parent_id→relationships`,
   `scene_versions.parent_id→scenes`,
   `themes.arc_id→arcs`, `themes.parent_theme_id→themes`,
   `theme_arguments.theme_id→themes`, `theme_arguments.character_id→characters`,
   `project_motifs.arc_id→arcs`, `project_motifs.chapter_id→chapters`,
   `lore_entries.world_setting_id→world_settings`,
   `power_tiers.power_system_id→power_systems`,
   `power_abilities.power_system_id→power_systems`, `power_abilities.tier_id→power_tiers`,
   `memories.source_scene_id→scenes`, `memory_revisions.source_scene_id→scenes`,
   `memory_references.memory_id→memories`, `memory_emotions.memory_id→memories`,
   `memory_connections.source_memory_id→memories`, `memory_connections.target_memory_id→memories`,
   `memory_revisions.memory_id→memories`,
   `character_appearances.character_id→characters`,
   `character_distinguishing_marks.character_id→characters`,
   `appearance_options.group_id→appearance_option_groups`,
   `character_competences.character_id→characters`,
   `character_competences.domain_id→competence_domains`,
   `character_knowledge_sources.character_id→characters`,
   `competence_capabilities.competence_id→character_competences`,
   `competence_limitations.competence_id→character_competences`,
   `world_settings.parent_id→world_settings`.
5. `pub fn warnings(document: &Value) -> Vec<String>` (D4).
6. Cập nhật 5 test cũ của `validate_rows` sang signature mới; thêm
   `test_ref_satisfies_required_id`, `test_ref_out_of_range_is_rejected`,
   `test_json_index_is_normalised`, `test_draft_allows_partial_rows`,
   `test_warnings_report_missing_pov_and_project_motif`.

### T3 — `src-tauri/src/super_json/io.rs` + `mod.rs`

1. `set(rows)`: `normalize_json_index` trước khi `validate_rows(..., Some(&document), Complete)`.
2. `pub fn add_row(project_id, table, row) -> Result<(i64, i64), String>` → `(json_index, count)`.
   - unknown table → `Unknown super json table: {table}`; `row` không phải object → nói rõ.
   - append, gán `_json_index`, tính `count` theo D2, validate `Draft` với `Some(&document)`, save.
3. `pub fn add_field(project_id, table, json_index: i64, field: &str, value: Value) -> Result<(), String>`.
   - index âm/ngoài range → `Row {json_index} not found in {table}.rows`.
   - field `_json_index` → `_json_index is assigned by the tool and cannot be written`.
   - field ngoài `table_fields` → `Unknown field {field:?} for {table}; allowed fields are: ...`.
   - type/ref validate (document hiện tại), save.
4. `pub fn warnings(project_id) -> Result<Vec<String>, String>` (load + `tables::warnings`).
5. Re-export `add_row`, `add_field`, `warnings` ở `super_json/mod.rs`.
6. Test: `test_add_row_appends_and_tracks_count`, `test_add_row_keeps_planned_count`,
   `test_add_field_updates_one_row`, `test_set_rows_renumbers_json_index`.

### T4 — `src-tauri/src/tools/{mod,specs,execute,labels}.rs`

1. `TOOL_NAMES: [&str; 7]` đúng D2.
2. `specs.rs`: 2 spec mới; `table` enum = `super_json::schema_tables()` (34); mô tả nêu draft/complete,
   `_json_index`, “không đặt tên vào field id”; cập nhật mô tả `super_json_set` (bulk only).
   Test `test_tool_specs_cover_seven_tools`.
3. `execute.rs`: dispatch 2 tool, trả đúng shape:
   - add_row: `{ok, table, json_index, count, row, missing_required, warnings}`
   - add_field: `{ok, table, json_index, field, value, missing_required, warnings}`
   - set: thêm `warnings`.
   Test `test_execute_add_row_and_add_field_round_trip`.
4. `labels.rs`: status `Đang thêm 1 {bảng}` / `Đang cập nhật {bảng}`; `tool_path` trả
   `format!("{group}.schema.{table}")` nếu tìm được group, ngược lại `None`.

### T5 — Prompt

1. `src-tauri/src/runtime/prompt.rs`:
   ```rust
   fn embedded_document(project_id: &str) -> String { /* D3 */ }
   ```
   Thay `{super_json}` **sau cùng**. Thêm `#[cfg(test)] mod tests`:
   - `test_prompt_embeds_the_live_super_json`: TempHome + `super_json::init_project("p", None)` +
     `super_json::set("p","require.project.name",json!("Nửa Đêm"))` → prompt chứa `"Nửa Đêm"` và
     `"require"`; không còn `{super_json}`.
   - `test_prompt_without_project_does_not_create_a_file`: `assemble_prompt(..., "")` → chứa
     `(chưa có project)`; `paths::super_json_path("p")` không tồn tại sau khi gọi.
2. `src-tauri/prompts/harness_system.md` (giữ nguyên các câu test cũ đang assert:
   `# Asking the author`, `Ask **at most three questions per reply**`, `One decision per question.`,
   `Never re-ask a field that is already set`, `# Only write what the author actually said`,
   `writes **exactly one** field with **exactly the value the author stated**`,
   `Never invent a value to fill a field`, `the only thing you may write is`,
   `` `require.project.plan.chapter.count` = 120 ``, `` `project_create` **without** a name ``,
   `**Never make one up.**`, `When it is unclear which field the author meant, ask instead of writing.`):
   - chèn sau “What you are doing”:
     ```text
     # Current super json

     The whole document is embedded below and re-read after every tool call. Trust it over your
     memory of earlier rounds.

     ```json
     {super_json}
     ```
     ```
   - sửa câu “You never write that file yourself. You change it only through the `super_json_set` tool.”
     thành liệt kê `super_json_set`, `super_json_add_row`, `super_json_add_field`.
   - thêm sau “Counts and rows”:
     ```text
     # Draft references (`_json_index`)

     - `_json_index` is the 0-based position of a row in that table's `rows` array. The tool assigns
       it; never write it yourself.
     - When the target row has no database id yet, point at it with `<field>_json_index`
       (for example `from_character_id_json_index`) and leave the id field empty. The target is the
       row whose `_json_index` matches.
     - Never put a name, a slug or a code where an id is expected. A wrong id means a wrong row later.
     ```
   - thêm ngay trước “Only write what the author actually said”:
     ```text
     # Mandatory field check

     Answer this question for every author message before calling a write tool:
     mô tả trên có thông tin nào chứa field mà JSON chưa tồn tại để bổ sung không?

     - If the answer is yes, write it now: `super_json_add_field` for one field of an existing row,
       `super_json_add_row` for a new row, `super_json_set` for a project leaf.
     - If the information already exists and contradicts what is stored, restate what the author
       asked before, then ask whether to overwrite. Never silently overwrite.
     ```
   - thêm sau “Asking the author”:
     ```text
     # No-arc default

     If the author says the story has no arc, set `require.schema.arcs.count` to 1 (and add one row
     `{"name": "Arc chính", "position": 1}` when `arcs.rows` is still empty), then answer with
     exactly this sentence and nothing else:
     Vậy coi như project chỉ có 1 arc duy nhất
     ```
   - sửa “Ask relationships only after both endpoints exist.” → được phép khai báo quan hệ ngay khi
     cả hai nhân vật đã có row, dùng `<field>_json_index`.
   - sửa “Ask the opening arc → chapter → scene as one hierarchy. A scene row must carry
     `chapter_id`.” → scenes optional, được suy luận có kiểm soát:
     ```text
     Scenes are optional: infer them from what the author described instead of interrogating the
     author field by field. A scene row may only use keys from its `fields` contract — never invent
     one. If you add a scene, also add at least one `scene_characters` row with `is_pov: true`.
     ```
     và thêm: `project_motifs` require, ≥1 row `scope: "project"`; row `scope: "arc"|"chapter"` optional.
   - cập nhật “Tool-calling rhythm”: ưu tiên `add_row`/`add_field`; `set(.rows)` chỉ bulk; add_row tự
     quản `count` (không ghi `count` sau đó); đọc `warnings` trong kết quả và sửa trước khi đề nghị
     viết dàn ý.
   - cập nhật “Rules”: table selector trả thêm `refs`; prompt luôn có document.
   - “Tools”: 7 dòng (thêm `super_json_add_row`, `super_json_add_field`).
3. Test trong `src-tauri/src/runtime/agent/tests.rs` (file này đã được thêm vào allow-list từ Sprint 16):
   thêm `test_prompt_contains_the_mandatory_field_check`, `test_prompt_contains_the_no_arc_sentence`,
   `test_prompt_explains_json_index_refs`, `test_prompt_lists_seven_tools`.

### T6 — Frontend mock + tool log

1. `src/lib/browser-mock.ts`: handler `super_json_add_row` / `super_json_add_field` (cùng lỗi cơ bản:
   unknown table/index/field), cập nhật `count` + `_json_index`; `mockStatusMessage` cho 2 tool.
2. `src/components/ToolLogDialog.tsx`: 2 nhánh mô tả:
   `Đã thêm một dòng vào {table}` / `Đã cập nhật một trường của {table}` (kèm error line khi fail).

### T7 — Docs + IPC test

1. `src-tauri/src/ipc_contract_tests.rs`: `test_ipc_execute_tool_add_row_end_to_end` — tạo project,
   `add_row` characters (`{"name":"Hoàng","is_primary":true}`), `add_field` (nếu cần), assert
   `json_index`, `count`, `missing_required` giảm đúng 1.
2. `docs/design/super-json.md`: thêm mục “Draft references” (`_json_index`, `<field>_json_index`,
   range, synthetic fields, `refs` trong table selector) + cập nhật §7 bất biến.
3. `AGENTS.md`: §1 thêm 1 dòng về `_json_index`/`<field>_json_index` và 7 tool.

### T8 — Verify + commit + push

```bash
cargo test --manifest-path src-tauri/Cargo.toml   # 0 failed / 3 ignored
pnpm build                                        # exit 0
git diff --stat
git add <allow-list>
git -c user.name=taho140303 -c user.email="202986530+taho140303@users.noreply.github.com" \
    commit -m "feat(tools): draft refs and add_row/add_field contract"
git push -u origin sprint-16.1-tool-contract
```

## ACCEPTANCE

- [ ] `TOOL_NAMES.len() == 7`; specs khớp; `super_json_get` table selector trả thêm `refs`.
- [ ] `table_fields("relationships")` chứa `_json_index`, `from_character_id_json_index`,
      `to_character_id_json_index`.
- [ ] Row `relationships` chỉ có 2 ref (không có id) vẫn qua `Complete`; ref ngoài range bị reject.
- [ ] `set(rows)` chuẩn hoá `_json_index` = vị trí; `add_row` gán index mới.
- [ ] `add_row` count: null→len; đã bằng len→+1; đang draft (count>len)→giữ nguyên.
- [ ] `add_row` cho phép row thiếu field (Draft); `set(rows)` vẫn reject row thiếu field (Complete).
- [ ] `add_field` sửa đúng row, reject `_json_index` và field lạ.
- [ ] 3 tool ghi trả `warnings`; warning POV/motif xuất hiện đúng điều kiện.
- [ ] Prompt chứa document thật (giá trị vừa ghi), 2 câu nguyên văn, 7 tên tool; không tạo file khi
      project chưa tồn tại.
- [ ] IPC test `add_row`/`add_field` end-to-end pass.
- [ ] `cargo test` 0 failed / 3 ignored; `pnpm build` exit 0; `git diff --stat` chỉ allow-list;
      branch push; commit author `taho140303`.

## ALLOW-LIST

```
docs/plan/sprint-16.1.md
docs/design/super-json.md
AGENTS.md
src-tauri/prompts/harness_system.md
src-tauri/src/runtime/prompt.rs
src-tauri/src/runtime/agent/tests.rs
src-tauri/src/super_json/schema.rs
src-tauri/src/super_json/tables.rs
src-tauri/src/super_json/io.rs
src-tauri/src/super_json/mod.rs
src-tauri/src/tools/mod.rs
src-tauri/src/tools/specs.rs
src-tauri/src/tools/execute.rs
src-tauri/src/tools/labels.rs
src-tauri/src/ipc_contract_tests.rs
src/lib/browser-mock.ts
src/components/ToolLogDialog.tsx
```

Ngoài danh sách: **KHÔNG sửa**. Không đổi `models.rs`, `types.ts`, `SuperJsonProgress`, `summary.rs`,
`system.dbml`, `super.json` (skeleton giữ nguyên 81 path / 34 bảng), `docs/schema/system.dbdiagram`
(đang có thay đổi cục bộ của root, KHÔNG add/stash/sửa).

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- ACCEPTANCE pass hết → DỪNG, báo `SPRINT DONE`.
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate kèm output.
- Cần quyết định thiết kế ngoài spec (đổi shape progress, thêm path/bảng, đổi tên tool có sẵn) → DỪNG, hỏi.
- Cần thêm dependency → DỪNG, hỏi.
- Task vượt 30 phút ước tính → DỪNG, chia nhỏ.

───────────────────────────────────────────────────────
🔁 RETRY BUDGET
───────────────────────────────────────────────────────
- Mỗi task tối đa 2 lần; lần 2 phải đổi approach, không lặp y hệt.
- Sau 2 lần fail: dừng task đó, chuyển task khác; cuối sprint liệt kê.

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS
───────────────────────────────────────────────────────
1. Lỗi không rõ nguyên nhân sau 2 lần debug.
2. Phải chọn giữa 2 approach mà spec không nói.
3. Phải đổi kiến trúc (folder, store, IPC shape).
4. Phát hiện bug sprint trước ảnh hưởng sprint này.
5. Cần user test thủ công (visual/UX/perf).

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- Không đổi nhóm require/optional, không đổi số 12/35, không thêm/bớt bảng hay field.
- Không đổi UI/UX ngoài 2 nhánh tool log; không đổi `SuperJsonProgress`.
- Không sửa `docs/schema/{runtime/super.json,system.dbml,system.dbdiagram}`, `.agents/`,
  `dist/`, `target/`, `node_modules/`.
- Không merge, không push `main`, không force-push, không `git add -A`.
- Không thêm dependency.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 16.1
ROUTE: deepseek-account / deepseek-flash

✅ COMPLETED:
- [task N]: [file changed] + [output thật]

❌ FAILED:
- [task]: [lỗi] | [đã thử] | [cần gì]

🤔 NEED_INPUT:
- [câu hỏi] | Option A | Option B

📋 NEXT ACTION:
- DONE → "Ready for root merge"
- BLOCKED → "Waiting for user response on X"

═══════════════════════════════════════════════════════
