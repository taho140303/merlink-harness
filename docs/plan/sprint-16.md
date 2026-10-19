# SPRINT 16 — SUPER JSON v2.1: NHÓM REQUIRE/OPTIONAL MỚI + `project_motifs` SCOPE

> Branch: `sprint-16-schema-v21` · Base: `feat/brand-mark` (KHÔNG phải `main`; `main` chưa có super json v2)
> Sprint kế tiếp: `sprint-16.1-tool-contract` (tool mới + prompt + `_json_index`)

## CONTEXT

- Super json v2 hiện tại: 1 project root + 34 bảng, **81 writable path**, **18 unit require / 29 unit optional**.
- Người dùng yêu cầu đổi cấu trúc: `scenes`, `scene_characters`, `chapters`, `relationships`,
  `relationship_roles`, `power_systems`, `world_settings` chuyển sang **optional**; `project_motifs`
  chuyển sang **require** (scope=project), và `system.dbml` bổ sung `scope`/`from`/`to`.
- Kèm yêu cầu: `relationships` dùng convention `<field>_json_index` cho FK khi draft chưa có id.
  Sprint này chỉ **chuẩn bị schema + docs**; tool `add_row`/`add_field` và prompt nằm ở 16.1.

## DECISIONS (chốt, không hỏi lại)

### D1 — Nhóm mới (mỗi bảng còn đúng MỘT entry)

Mỗi bảng trong `require.schema`/`optional.schema` chỉ còn **một** entry dạng
`{ "table", "count", "fields", "rows" }`. Entry thứ hai (fields-only) của v2 được gộp vào entry chính.

| Bảng | v2 | v2.1 |
| --- | --- | --- |
| `chapters` | require | **optional** |
| `scenes` | require | **optional** |
| `scene_characters` | require | **optional** |
| `relationships` | require | **optional** |
| `relationship_roles` | require | **optional** |
| `world_settings` | require | **optional** |
| `power_systems` | require | **optional** |
| `project_motifs` | optional | **require** |

Còn lại giữ nguyên. Kết quả:

- **Require (12 unit)** = 6 project leaf (`name`, `language`, `plan.arc.count`, `plan.chapter.count`,
  `plan.scene.count`, `plan.word.count`) + 6 bảng: `project_genres`, `arcs`, `characters`,
  `project_motifs`, `themes`, `theme_arguments`.
- **Optional (35 unit)** = 7 project leaf + 28 bảng còn lại.
- Tổng path vẫn **81** (13 leaf + 34×2). `require.project.plan.chapter.count` **vẫn required** —
  chỉ bảng `chapters` chuyển optional, nên không còn mâu thuẫn "count bắt buộc nhưng rows không".

### D2 — `project_motifs` (require, scope-aware)

- `fields`: `motif_id`, `scope`, `arc_id`, `chapter_id`.
  - `scope` = Choice `["project", "arc", "chapter"]`.
  - `arc_id`/`chapter_id` là **draft anchor** (không có trong dbml): importer dùng chúng để dựng
    cột `from`; ở draft chúng là chuỗi id hoặc `<field>_json_index` (16.1).
- Rule hoàn tất (soft, trả `Ok(false)` chứ không chặn ghi): bảng chỉ tính là filled khi có
  **ít nhất một row `scope == "project"`**; row `scope = arc|chapter` là tuỳ chọn.
- `project_motifs` là bảng require ⇒ `count` không được 0 (rule sẵn có của bảng require).

### D3 — `scenes` vẫn giữ 9 field, không thêm field

`name`, `chapter_id`, `position`, `code`, `world_setting_id`, `style_preset_id`, `narrator_config_id`,
`summarize`, `content`. "Cho LLM suy luận" chỉ là chuyện prompt (16.1); validation vẫn chỉ cho
đúng 9 key này. Không thêm `word_count`.

### D4 — `system.dbml` chỉ đụng `project_motifs`

Thêm 3 cột varchar + index:

```dbml
    scope varchar [note: 'project | arc | chapter']
    from  varchar [note: 'scope=project → project.code; scope=arc → "project.code"."arc.code"; scope=chapter → "project.code"."arc.code"."chapter.code"']
    to    varchar [note: 'code của đối tượng đích, vd motifs.code']

    indexes {
        ...
        (scope, from)
    }
```

Kèm `Note:` nói rõ `scope=project` là require trong super json, `scope=arc|chapter` là optional.
Không đổi bảng nào khác. `docs/schema/system.dbdiagram` là artifact generate — **KHÔNG sửa**.

### D5 — Validation không được nới khi bảng đổi nhóm

`required_row_fields(table)` hiện lấy key của entry `require`; khi bảng sang optional nó sẽ trả rỗng
⇒ validation yếu đi âm thầm. Phải thay bằng map hằng **độc lập group** (xem T3).

### D6 — Tài liệu v2 của người dùng phải migrate

Doc đã có (scenes ở require, characters tách 2 entry) phải được chuẩn hoá thành v2.1 khi load,
idempotent, giữ nguyên `count`/`rows` (xem T5). Không có migration này thì dữ liệu vẫn nằm trên đĩa
nhưng không đọc/không tính progress được.

---

## TASKS

### T1 — `docs/schema/runtime/super.json` → v2.1

1. `$comment` mới:
   `"SUPER JSON v2.1 — 1 project root + 34 schema tables. fields = read-only row contract; count = planned rows; rows = actual instances. v2.1: scenes/chapters/relationships/world/power + relation tables are optional; project_motifs is required at project scope. Source: docs/plan/super-json-v2-plan.md"`.
2. Gộp mỗi bảng về 1 entry; **giữ đúng thứ tự tương đối** của các bảng hiện có khi có thể.
   - `require.schema` (6): `project_genres`, `arcs`, `characters`, `project_motifs`, `themes`,
     `theme_arguments`.
   - `optional.schema` (28): `relationship_motifs`, `chapters`, `scenes`, `scene_characters`,
     `scene_versions`, `appearance_option_groups`, `appearance_options`, `character_appearances`,
     `character_distinguishing_marks`, `relationships`, `relationship_categories`,
     `relationship_roles`, `relationship_states`, `world_settings`, `lore_entries`, `power_systems`,
     `power_tiers`, `power_abilities`, `memories`, `memory_references`, `memory_emotions`,
     `memory_connections`, `memory_revisions`, `character_knowledge_sources`, `competence_domains`,
     `character_competences`, `competence_capabilities`, `competence_limitations`.
3. Entry `project_motifs`:
   ```json
   {
     "table": "project_motifs",
     "count": null,
     "fields": { "motif_id": null, "scope": null, "arc_id": null, "chapter_id": null },
     "rows": []
   }
   ```
4. Mọi entry khác: `fields` = **hợp** của các field đang khai báo (require core + optional extras),
   giữ nguyên thứ tự: core trước, extra sau. `count: null`, `rows: []`.
5. 13 project leaf giữ nguyên hoàn toàn.
6. Không thêm/bớt bảng. Tổng: 6 + 28 = 34 entry.

### T2 — `docs/schema/system.dbml`

Theo D4. `Note:` của bảng:

```
Note: 'Motif áp cho project/arc/chapter. Super json: scope=project là require; scope=arc|chapter là optional. from/to do importer điền từ scope + code.'
```

### T3 — `src-tauri/src/super_json/schema.rs`

1. Xoá logic `required_row_fields()` đọc entry `require`; thay bằng:

```rust
/// Fields every row must carry, independent of the table's group. A table that
/// moves from require to optional must not silently lose its row contract.
const REQUIRED_ROW_FIELDS: [(&str, &[&str]); 13] = [
    ("project_genres", &["genre_id", "is_primary"]),
    ("arcs", &["name", "position"]),
    ("characters", &["name", "is_primary"]),
    ("project_motifs", &["motif_id", "scope"]),
    ("themes", &["scope", "question"]),
    ("theme_arguments", &["theme_id", "character_id", "role"]),
    ("chapters", &["name", "position"]),
    ("scenes", &["name", "chapter_id", "position"]),
    ("scene_characters", &["scene_id", "character_id", "is_pov"]),
    ("relationships", &["from_character_id", "to_character_id"]),
    ("relationship_roles", &["relationship_id", "category_id", "is_primary"]),
    ("world_settings", &["name", "type"]),
    ("power_systems", &["name", "source"]),
];

pub fn required_row_fields(table: &str) -> Vec<String> {
    REQUIRED_ROW_FIELDS
        .iter()
        .find(|(known, _)| *known == table)
        .map(|(_, fields)| fields.iter().map(|field| (*field).to_owned()).collect())
        .unwrap_or_default()
}
```

2. Thêm hằng + nhánh `column_kind`:
   ```rust
   const PROJECT_MOTIF_SCOPES: &[&str] = &["project", "arc", "chapter"];
   ...
   ("project_motifs", "scope") => FieldKind::Choice(PROJECT_MOTIF_SCOPES),
   ```
3. Giữ nguyên `valid_paths`, `units`, `tables`, `countable_group`, `merged_table_entry`,
   `table_fields`, `field_kind` (không đổi logic; chúng tự chạy theo skeleton mới).

### T4 — `src-tauri/src/super_json/tables.rs`

Thêm rule hoàn tất, dùng chung cho cả require và optional:

```rust
/// Cross-table rules that make a table "not yet complete" without rejecting the
/// write itself (drafting is incremental).
fn document_rule(document: &Value, table: &str, rows: &Value) -> bool
```

- `project_motifs`: `rows` có ít nhất một object `scope == "project"`.
- `scene_characters`: nếu `countable_entry(document, "scenes")["rows"]` là array **không rỗng** thì
  `rows` phải có ít nhất một object `is_pov == true`; nếu `scenes` rỗng/không có → `true`.
- Bảng khác: `true`.

Gọi sau `validate_rows(...)?` trong **cả** `is_table_complete` và `is_optional_table_complete`;
nếu `false` → `Ok(false)` (không phải `Err`).

### T5 — `src-tauri/src/super_json/io.rs` — migration v2 → v2.1

Thêm `fn migrate_groups_if_needed(document: &mut Value) -> bool` và gọi trong
`load_or_init_unlocked` **sau** `migrate_if_needed` (nếu một trong hai trả `true` thì `save_unlocked`):

1. Với từng tên bảng trong `schema::tables()`:
   - Gom mọi entry của bảng đó trong `require.schema` + `optional.schema` (giữ thứ tự gặp).
   - Nếu đúng 1 entry **và** nằm đúng `schema::countable_group(table)` → bỏ qua.
   - Ngược lại: lấy `count`/`rows` từ entry đang sở hữu `count` (nếu có; mặc định `null`/`[]`),
     lấy `fields` từ skeleton v2.1 (`schema::table_entry(skeleton, table)`), xoá mọi entry cũ,
     chèn 1 entry mới `{table, count, fields, rows}` vào `countable_group(table)`.
2. Sau khi xử lý, sort `schema` của cả hai group theo thứ tự `schema::tables()` để ổn định.
3. Trả `true` nếu có bất kỳ thay đổi nào.

Test mới: `test_migration_from_v2_groups_to_v2_1` — dựng document kiểu v2 (2 entry `characters`,
`scenes` ở require có count/rows, `project_motifs` ở optional), gọi migrate, assert:
1 entry/bảng, đúng group, `fields` = skeleton v2.1, `count`/`rows` giữ nguyên giá trị.

### T6 — Đồng bộ cơ học (giữ xanh)

| File | Sửa |
| --- | --- |
| `src-tauri/src/super_json/progress.rs` | comment 12/35; `fill_required` chỉ còn 6 bảng + 6 leaf (bỏ scenes/scene_characters/chapters/relationships/relationship_roles/world_settings/power_systems khỏi required; thêm `project_motifs` row `{"motif_id":"mo-tif","scope":"project"}`); test `18→12`, `29→35`; vòng optional trong test 100% thêm 7 bảng vừa chuyển, bỏ `project_motifs` |
| `src-tauri/src/super_json/schema.rs` (test) | `test_required_units_are_18_and_optional_29` → `..._12_and_35`; required chứa `require.schema.project_motifs`, không chứa `require.schema.scenes`; optional chứa `optional.schema.{scenes,chapters,relationships,relationship_roles,world_settings,power_systems}`; `test_scenes_chapter_id_is_required` → `test_scenes_chapter_id_is_a_required_row_field` (assert `required_row_fields("scenes")` chứa `chapter_id` + skeleton có đúng 1 entry `scenes` nằm ở optional); thêm `test_one_entry_per_table` và `test_project_motifs_declares_scope_and_anchors` |
| `src-tauri/src/tools/execute.rs` (test) | `18→12`, `17→11`, `29→35`, `4→6` (comment `70*1/12`) |
| `src-tauri/src/ipc_contract_tests.rs` | `json!(18)→12`, `json!(29)→35`, missing `17→11`, percent `4→6`; hai path `require.schema.world_settings.*` → `optional.schema.world_settings.*` |
| `src-tauri/src/models.rs`, `src/types.ts` | comment `18/29` → `12/35` |
| `src/pages/Chat.tsx` | fallback `?? 18` → `?? 12` |
| `src/lib/browser-mock.ts` | `MOCK_REQUIRED_TABLES` / `MOCK_OPTIONAL_TABLES` theo D1; unit list + percent tự theo hằng; nhãn giữ nguyên |
| `src-tauri/prompts/harness_system.md` | bảng “Required tables” còn 6 dòng; “Optional tables” liệt kê 28 tên; câu “18 required / 29 optional” → “12 required / 35 optional”; giữ đủ 34 tên bảng + 81 path statement |
| `AGENTS.md` | “81 writable paths … 12 required / 35 optional units” |

### T7 — Docs

- `docs/design/super-json.md`: §3 (12/35 + danh sách), §4 công thức (vẫn 70/30 nhưng 12/35),
  §5 validate (thêm 2 rule), §6 migration (thêm v2→v2.1), §7 invariants (thêm `project_motifs` scope).
- `docs/plan/super-json-v2-plan.md`: append `## 10. v2.1 delta (Sprint 16)` — bảng nhóm, 12/35,
  `project_motifs`, migration, ghi rõ các số 18/29 phía trên là lịch sử.
- `docs/plan/sprint-15.md`: trạng thái `UNLOCKED` (điều kiện rollback đã thoả) và ghi
  “superseded by Sprint 16 / 16.1” — **không** ghi là đã hoàn thành Sprint 15.

### T8 — Verify + commit + handoff

```bash
cargo test --manifest-path src-tauri/Cargo.toml   # 0 failed, 3 ignored
pnpm build                                        # exit 0
git diff --stat                                   # chỉ file allow-list
```

Commit (một hoặc hai commit đều được), identity bắt buộc:

```bash
git add <đúng file allow-list>
git -c user.name=taho140303 \
    -c user.email="202986530+taho140303@users.noreply.github.com" \
    commit -m "feat(schema): super json v2.1 groups and project_motifs scope"
git push -u origin sprint-16-schema-v21
```

---

## ACCEPTANCE (executable)

- [ ] `crate::super_json::valid_paths().len() == 81`; `schema_tables().len() == 34`.
- [ ] `required_paths().len() == 12`; `optional_paths().len() == 35`.
- [ ] `required_paths()` chứa `require.schema.project_motifs`; `optional_paths()` chứa
      `optional.schema.{scenes,chapters,scene_characters,relationships,relationship_roles,world_settings,power_systems}`.
- [ ] `required_row_fields("scenes")` chứa `name`,`chapter_id`,`position` (dù scenes optional).
- [ ] `project_motifs` lead row không có `scope:"project"` ⇒ `is_table_complete` = `false`; có ⇒ `true`.
- [ ] `scenes.rows` không rỗng + `scene_characters` không có `is_pov:true` ⇒ unit optional
      `scene_characters` **không** filled; thêm 1 row `is_pov:true` ⇒ filled.
- [ ] Migration test pass; file v2 cũ load xong ghi lại thành 34 entry, mỗi bảng 1 entry.
- [ ] `cargo test` 0 failed / 3 ignored; `pnpm build` exit 0.
- [ ] `git diff --stat` không có `docs/schema/system.dbdiagram`, `.agents/`, `dist/`, `target/`.
- [ ] Branch `sprint-16-schema-v21` đã push; commit author `taho140303`.

## ALLOW-LIST

```
docs/schema/runtime/super.json
docs/schema/system.dbml
docs/design/super-json.md
docs/plan/super-json-v2-plan.md
docs/plan/sprint-15.md
docs/plan/sprint-16.md
AGENTS.md
src-tauri/prompts/harness_system.md
src-tauri/src/super_json/schema.rs
src-tauri/src/super_json/tables.rs
src-tauri/src/super_json/io.rs
src-tauri/src/super_json/progress.rs
src-tauri/src/tools/execute.rs
src-tauri/src/ipc_contract_tests.rs
src-tauri/src/models.rs
src/types.ts
src/pages/Chat.tsx
src/lib/browser-mock.ts
```

Ngoài danh sách trên: **KHÔNG sửa**. `src-tauri/src/super_json/summary.rs` không cần đổi
(các bảng nó đọc vẫn là require/đọc được qua `countable_entry`); nếu buộc phải đổi thì dừng và hỏi.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo `SPRINT DONE`.
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate kèm output.
- Phát hiện cần quyết định thiết kế ngoài spec (ví dụ phải đổi shape `progress`, đổi tên bảng) → DỪNG, hỏi.
- Cần thêm dependency → DỪNG, hỏi.
- Task vượt 30 phút ước tính → DỪNG, chia nhỏ.

───────────────────────────────────────────────────────
🔁 RETRY BUDGET
───────────────────────────────────────────────────────
- Mỗi task tối đa 2 lần; lần 2 phải đổi approach, không lặp y hệt.
- Sau 2 lần fail: dừng task đó, chuyển task khác, cuối sprint liệt kê.

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS
───────────────────────────────────────────────────────
1. Lỗi không rõ nguyên nhân sau 2 lần debug.
2. Phải chọn giữa 2 approach mà spec không nói.
3. Phải đổi kiến trúc (folder, store, shape IPC).
4. Phát hiện bug sprint trước ảnh hưởng sprint này.
5. Cần user test tay (visual/UX/perf).

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- Không thêm tool mới, không sửa prompt logic (đó là 16.1) — chỉ cập nhật **danh sách bảng** trong prompt.
- Không sửa `docs/schema/system.dbdiagram`, `.agents/`, `dist/`, `target/`, `node_modules/`.
- Không đổi công thức 70/30, không đổi cap 5 vòng, không đổi UI/UX ngoài fallback số ở Chat.tsx.
- Không xoá `docs/prompts/`; không port dbml sang SQLite.
- Không merge, không push `main`, không force-push, không `git add -A`.
- Không thêm dependency.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 16
ROUTE: deepseek-account / deepseek-flash

✅ COMPLETED:
- [task N]: [file changed] + [output thật]

❌ FAILED:
- [task]: [lỗi] | [đã thử] | [cần gì]

🤔 NEED_INPUT:
- [câu hỏi] | Option A | Option B

📋 NEXT ACTION:
- DONE → "Ready for sprint 16.1"
- BLOCKED → "Waiting for user response on X"

═══════════════════════════════════════════════════════
