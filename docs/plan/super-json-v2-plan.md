# KẾ HOẠCH — SUPER JSON v2 (project-rooted, cardinality-aware) + CI/CD

> Trạng thái: PLAN (đã triển khai trong `0174ce9`). Nguồn design ban đầu:
> `docs/schema/super_temp/super_json_template_deepseek_flash.json` (đã xóa cùng thư mục tạm).
> Schema chốt: `docs/schema/runtime/super.json` (gọi tắt **D**) + 1 patch bắt buộc: chuyển
> `scenes.chapter_id` từ optional sang require.
> Mục tiêu: thay 25 leaf phẳng bằng một document 1 project → 34 bảng có `count`/`rows`, cập nhật
> toàn bộ dàn câu hỏi/prompt/summary/labels, và thêm CI/CD GitHub Actions dùng cú pháp hiện hành.

---

## 0. Chốt đầu vào

- Schema: 35 bảng / 237 attribute (`docs/schema/system.dbml`, extract 35 bảng đã đối chiếu).
- `projects` = 1 root (`project` block, 13 leaf). 34 bảng còn lại nằm trong `schema`.
- require = phải biết để viết chương 1; optional = điền lúc nào cũng được. **NOT NULL/default không
  phải tiêu chí** (theo yêu cầu user).
- `characters.is_primary` là field synthetic, bắt buộc có.
- Trả lời được: "3 nhân vật: Hoàng (chính), Phong, Linh" mà không thu về 1 character.

---

## 1. Mô hình dữ liệu (decision)

Mỗi entry trong `schema`:

```json
{ "table": "characters", "count": 3,
  "fields": { "name": null, "is_primary": null },
  "rows": [ { "name": "Hoàng", "is_primary": true },
            { "name": "Phong", "is_primary": false },
            { "name": "Linh", "is_primary": false } ] }
```

- `fields` = **contract read-only** cho một row (key + null). Không ghi, không tính progress.
- `count` = số row dự kiến, integer ≥ 0. Ghi được.
- `rows` = **toàn bộ instance**, array of object. Ghi được (replace whole array).
- Bất biến: `count == rows.len()` khi cả hai đã có; row chỉ chứa key có trong `fields`.
- Bảng require: `count > 0`. Bảng optional: `count = 0, rows = []` là câu trả lời hợp lệ ("không có").
- `characters`: mỗi row phải có `name` + `is_primary`; đúng **một** `is_primary = true` khi count > 0.
- Bảng bị split require/optional: `count`/`rows` nằm đúng 1 entry (entry có `count`); `fields` hai bên
  hợp lại thành contract; required row keys lấy từ entry require.

---

## 2. Path grammar (thay dot-path cũ)

| Loại | Path | Ghi? |
|---|---|---|
| Project leaf | `require.project.name`, `optional.project.plan.arc.tolerance`, … | có |
| Table count | `require.schema.characters.count` | có |
| Table rows | `require.schema.characters.rows` | có (whole array) |
| Table selector (get only) | `require.schema.characters` | không |
| `fields` | — | không bao giờ |

- Tổng path set được: **81** = 13 project + 34×2 (`count`, `rows`).
- Resolver tìm entry theo `table`, **không** dùng index mảng → thứ tự `schema` không ảnh hưởng path.
- `super_json_get` nhận thêm table selector để đọc `fields`/`count`/`rows` của một bảng.
- `super_json_missing` trả unit còn thiếu: project leaf, hoặc `require.schema.<table>`.
- `fields` không nằm trong enum `super_json_set`; model học shape từ `fields` qua `get`.

### 2.1 Toàn bộ 5 tool xoay quanh schema mới

Không tool nào được giữ path/shape cũ. `echo` là tool diagnostics duy nhất không đổi.

| Tool | Hành vi mới | Bỏ hẳn |
|---|---|---|
| `project_create` | `name` (nếu có) ghi vào `require.project.name`; tạo file từ skeleton mới | `require.overview.name` |
| `super_json_get` | không path → cả document; path → 81 path set được; thêm **table selector** `require.schema.characters` trả `{table, fields, count, rows}` để model đọc contract trước khi ghi rows | đọc path cũ |
| `super_json_set` | 81 path: project leaf, `schema.<table>.count`, `schema.<table>.rows`; `rows` validate theo `fields` + `count == rows.length` + đúng 1 `is_primary` | 25 path cũ; ghi `fields` |
| `super_json_missing` | trả **18 unit** còn thiếu: project leaf hoặc `require.schema.<table>` | danh sách 11 path cũ |
| `echo` | giữ nguyên | — |

Kéo theo: `tools/specs.rs` (enum + description từng tool), `tools/execute.rs` (dispatch), `tools/labels.rs`
(status message + tool log), `runtime/prompt.rs` + `harness_system.md` (model biết cách gọi), `lib.rs`
(`merlink_handlers!` không đổi tên command), `ipc_contract_tests.rs`, và toàn bộ test tool end-to-end.
Không có nhánh tương thích path cũ trong `set/get`; dữ liệu cũ chỉ đi qua migration ở `load_or_init`.

Test bắt buộc cho "toàn bộ tool":
- `test_tool_specs_cover_all_tools_and_expose_81_paths`
- `test_project_create_writes_require_project_name`
- `test_get_table_selector_returns_fields_count_rows`
- `test_set_rows_validates_against_fields_and_count`
- `test_missing_required_returns_18_units`
- `test_ipc_execute_tool_end_to_end_super_json_v2`
- `test_tool_log_status_for_rows_and_count`

---

## 3. Thay đổi Rust (file-by-file)

### 3.1 Skeleton + type manifest

| File | Việc |
|---|---|
| `docs/schema/runtime/super.json` | Thay bằng D + patch `scenes.chapter_id` require. Giữ `$comment`; `skeleton()` strip như cũ. |
| `docs/schema/runtime/super_fields.json` (mới) | Manifest `table -> { column -> kind }` cho 185 cột + `characters.is_primary`. Kind: text/int+/int≥0/bool/text-array/choice/json/number. |
| `scripts/gen-super-fields.py` (mới, optional) | Sinh manifest từ `docs/schema/system.dbml`; `--check` để CI phát hiện drift. |

Test: mọi `fields` key trong skeleton có kind; mọi kind không thừa; enum choices khớp DBML
(`ThemeScope`, `ThemeResolutionType`, `LocationType`, `LocationAccess`, `LoreType`, `Memory*`,
`SceneSource`, `Hierarchy`, `RelationDimensionLevel`, `CertaintyLevel`, `gender`).

### 3.2 `src-tauri/src/super_json/schema.rs`

- Bỏ `collect_leaves` coi array là leaf.
- `valid_paths()` = 13 project + 68 schema paths, sort ổn định.
- `required_units()` = 6 project require + 12 bảng require = **18**.
- `optional_units()` = 7 project optional + 22 bảng optional = **29**.
- `field_kind(path)` tra manifest; path `count` → `IntNonNegative`; `rows` → `Rows`.
- `required_paths()` / `optional_paths()` giữ tên API nhưng trả unit keys (project leaf hoặc
  `require.schema.<table>`) để `missing_required`/prompt dùng.
- `skeleton()` vẫn `include_str!` + strip `$comment`.

### 3.3 `src-tauri/src/super_json/tables.rs` (mới)

- `TableSpec { name, columns: &[(name, FieldKind)], required_columns: &[&str] }` dựng một lần từ skeleton
  + manifest (`OnceLock`).
- `validate_rows(table, value, count) -> Result<(), String>`:
  - array of object; key lạ → error liệt kê key hợp lệ;
  - type từng value theo manifest;
  - required columns non-null trong **mọi** row;
  - `count == rows.len()`;
  - `characters`: đúng 1 `is_primary = true`, `name` non-empty.
- `table_entry(doc, group, table)` trả entry có `count`; `merged_fields` trả union `fields` 2 group.

### 3.4 `src-tauri/src/super_json/io.rs`

- `get`/`set` parse 3 dạng path: `*.project.*`, `*.schema.<table>.count`, `*.schema.<table>.rows`.
- `set(rows)` validate qua `validate_rows`; nếu `count` đã set thì phải khớp.
- `set(count)` validate ≥ 0; nếu `rows` đã có thì phải khớp length.
- `init_project` ghi `require.project.name` (không còn `require.overview.name`).
- Giữ lock per-project + atomic save; không tạo field mới.

### 3.5 `src-tauri/src/super_json/progress.rs`

- `is_filled(unit)`: project leaf = non-null (array non-empty); table unit = count != null,
  `rows.len() == count`, rows validate pass.
- `percent = round(70 * filled_req/18 + 30 * filled_opt/29)`.
- Toàn bộ require xong = đúng **70%**; toàn bộ = 100%.
- `required_total = 18`, `optional_total = 29` trong `SuperJsonProgress` (wire shape giữ nguyên).
- `missing_required` trả 18 unit keys còn thiếu.

### 3.6 `src-tauri/src/super_json/summary.rs`

6 đoạn, deterministic, placeholder `[chưa có thông tin]`:

1. Identity: `project.name`, `language`, `author`, `target-audience`, primary genre (từ
   `project_genres.rows`).
2. Cast: `characters.rows` (đủ tên + `is_primary`), `relationships.count`.
3. Theme: `themes.rows[0]` (scope/question/resolution_type/resolution/message), `theme_arguments.count`.
4. Plan: `project.plan.*.count/tolerance` + số arc/chapter/scene đã khai báo.
5. World: `world_settings.count/type`, `power_systems.count/source`, `lore_entries.count`.
6. Status: `project.summarize` + `filled/required` + `filled/optional` + rows/count từng bảng.

### 3.7 `src-tauri/src/tools/specs.rs`

- `path` enum = 81 path set được (project + count/rows). Không liệt kê 185 cột.
- `project_create` description đổi `require.overview.name` → `require.project.name`.
- `super_json_set` description nói rõ: `count` integer, `rows` array of object, `count == rows.length`,
  `fields` read-only.
- `super_json_missing` mô tả unit keys.
- Test: enum length = `valid_paths().len()`, chứa `require.schema.characters.rows`, không còn path cũ.

### 3.8 `src-tauri/src/tools/labels.rs`

- Bỏ `PATH_LABELS[25]`.
- `PROJECT_LABELS[13]`, `TABLE_LABELS[34]`, `FIELD_LABELS[(table, field)]` cho các field phổ biến;
  fallback = tên field.
- `field_labels()` trả map 81 path: project leaf exact; `count` → `số lượng <bảng>`;
  `rows` → `danh sách <bảng>`.
- `status_message`: `rows` → `Đang ghi nhận {len} {bảng}`; `count` → `Đang cập nhật số lượng {bảng}`.
- Test: phủ đủ 81 path, fallback non-empty, rows không dump array.

### 3.9 Prompt + dàn câu hỏi

- `src-tauri/prompts/harness_system.md`: thay `# Asking the author` và `# Super json fields`.
- `src-tauri/src/runtime/prompt.rs`: thêm placeholder `{required_total}` / `{optional_total}` lấy từ
  `super_json::required_units().len()` / `optional_units().len()`; bỏ hardcode 11/14/25.
- Xem §5 cho nội dung.

### 3.10 Tests phải sửa / thêm

Sửa:
- `test_super_json_required_path_count_is_eleven` → `..._is_eighteen_units`.
- `test_status_message_covers_all_25_paths`, `test_labels_command_covers_all_25_paths` → 81.
- `test_tool_specs_cover_all_tools_and_expose_full_whitelist` (25 → 81).
- `test_progress_is_seventy_when_required_complete`, `test_progress_is_hundred_when_all_fields_filled`,
  `test_progress_label_thresholds`, `test_progress_counts_empty_array_as_unfilled`.
- Toàn bộ `summary.rs` tests (fixture 25 leaf → fixture D).
- `test_super_json_skeleton_and_set_round_trip`, `..._rejects_unknown_path`, `..._rejects_wrong_type`,
  `..._enum_paths_reject_invalid_value`, `..._missing_is_empty_when_all_required_set`.
- `ipc_contract_tests::test_ipc_super_json_labels_end_to_end` (25 → 81).
- `runtime/agent/tests.rs` prompt tests: bỏ chuỗi `require.overview.plan.chapter.count`,
  cập nhật rule câu hỏi.

Thêm:
- `test_set_rows_accepts_three_characters_with_one_primary`.
- `test_set_rows_rejects_unknown_column`.
- `test_set_rows_rejects_count_mismatch`.
- `test_set_count_after_rows_must_match`.
- `test_characters_require_exactly_one_primary`.
- `test_get_table_selector_returns_contract`.
- `test_progress_table_unit_needs_count_and_rows`.
- `test_migration_from_v1_document_moves_mapped_leaves`.
- `test_summary_lists_all_three_characters`.
- `test_prompt_mentions_rows_contract_and_dynamic_totals`.

---

## 4. Migration (bắt buộc trước khi merge)

`load_or_init`: nếu có `require.overview` và không có `require.project` → migrate, backup
`super.v1.json` (0600) một lần, rồi ghi đè bằng shape mới.

| Old | New |
|---|---|
| `require.overview.name` | `require.project.name` |
| `optional.overview.language` | `require.project.language` |
| `optional.overview.author` | `optional.project.author` |
| `optional.overview.description` | `optional.project.summarize` |
| `optional.overview.target-audience` | `optional.project.target-audience` |
| `require.overview.plan.*.count` | `require.project.plan.*.count` (scene count = null) |
| `optional.overview.plan.*.tolerance` | `optional.project.plan.*.tolerance` |
| `require.genre.primary` | `require.schema.project_genres`: count 1, rows `[{genre_id, is_primary:true}]` |
| `optional.genre.secondary[]` | append rows `{genre_id, is_primary:false}`, count = len |
| `require.theme.question` + optional theme.* | `require.schema.themes`: count 1, rows `[{scope:"PROJECT", question, resolution_type?, resolution?, message?}]` |
| `optional.world.setting-scale` | `optional.schema.world_settings`: count 1, rows `[{type}]` |
| `optional.world.has-power-system` | `optional.schema.power_systems`: count 1/0, rows `[]` |
| `logline.*`, `optional.overview.age-rating` | drop (chỉ còn trong backup) |

Nếu không làm migration: `load_or_init` phải báo lỗi rõ và **không** ghi đè file cũ.

---

## 5. Dàn câu hỏi mới

Nguyên tắc: tối đa **3 câu/lượt**; một câu = một quyết định (nhóm `plan` 4 count được phép hỏi gộp);
không hỏi lại field đã có; ghi ngay sau khi user trả lời; không bịa row/ID/catalog.

| Thứ tự | Hỏi gì | Ghi gì |
|---|---|---|
| 1 | Tên truyện (nếu chưa có), ngôn ngữ, thể loại chính, câu hỏi chủ đề | `project.name/language`, `project_genres`, `themes` |
| 2 | Số arc/chapter/scene/từ dự kiến | 4 `project.plan.*.count` |
| 3 | Số nhân vật + tên từng người + ai là chính | `characters.count`→3, `characters.rows`→3 object, đúng 1 `is_primary` |
| 4 | Arc/chapter/scene mở đầu (tên, thứ tự) | rows `arcs/chapters/scenes`; `scenes.chapter_id` bắt buộc |
| 5 | Ai ở scene mở đầu, POV là ai | rows `scene_characters` (`scene_id`, `character_id`, `is_pov`) |
| 6 | Quan hệ giữa các nhân vật (loại, chiều, vai chính) | rows `relationships` + `relationship_roles` |
| 7 | Bối cảnh mở đầu (tên, scale), có hệ thống sức mạnh không | `world_settings`, `power_systems` |
| 8 | Optional: tác giả, tóm tắt, độ tuổi, dung sai, theme resolution, ngoại hình, memory, competence | optional paths |

Quy tắc row (đưa nguyên văn vào prompt):

> `fields` là schema read-only, không phải row đầu tiên. `rows` là toàn bộ dữ liệu. Khi ghi rows:
> đọc bảng trước nếu đang sửa, giữ mọi row đã biết, chỉ thêm/sửa dữ liệu user đã xác nhận; object chỉ
> được chứa key có trong `fields`; đặt `count` đúng bằng `rows.length`. Không bịa row.

Ví dụ 3 nhân vật: 2 tool calls, mỗi call 1 field:

```json
{"path":"require.schema.characters.count","value":3}
{"path":"require.schema.characters.rows","value":[
  {"name":"Hoàng","is_primary":true},
  {"name":"Phong","is_primary":false},
  {"name":"Linh","is_primary":false}]}
```

Prompt field section: bảng descriptor theo 34 bảng (table, required/optional, fields list ngắn), không
liệt kê 185 dòng; nhấn mạnh `project` là root, count đi từ project xuống.

---

## 6. CI/CD (GitHub Actions)

### 6.1 Baseline trước khi bật gate

- `cargo fmt --manifest-path src-tauri/Cargo.toml --check` **đang đỏ**: 118 diff. Task riêng:
  `cargo fmt --all`, commit format-only.
- Chạy `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings`;
  sửa baseline nếu có. Không dùng `continue-on-error` vĩnh viễn.

### 6.2 `.github/workflows/ci.yml`

```yaml
name: CI
on:
  push: { branches: [main] }
  pull_request: { branches: [main] }
permissions:
  contents: read
concurrency:
  group: ci-${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true
jobs:
  check:
    runs-on: ubuntu-22.04
    timeout-minutes: 45
    steps:
      - uses: actions/checkout@v7
      - name: Tauri Linux dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y --no-install-recommends \
            libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf xdg-utils
      - uses: pnpm/action-setup@v6
        with: { version: 10.33.0 }
      - uses: actions/setup-node@v7
        with:
          node-version: '22'
          cache: pnpm
          cache-dependency-path: pnpm-lock.yaml
      - uses: dtolnay/rust-toolchain@stable
        with: { components: 'rustfmt, clippy' }
      - uses: Swatinem/rust-cache@v2
        with: { workspaces: './src-tauri -> target' }
      - run: pnpm install --frozen-lockfile
      - run: pnpm build
      - run: cargo fmt --manifest-path src-tauri/Cargo.toml --check
      - run: cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings
      - run: cargo test --manifest-path src-tauri/Cargo.toml --locked
  workflow-lint:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@v7
      - name: actionlint
        run: |
          bash <(curl -fsSL https://raw.githubusercontent.com/rhysd/actionlint/v1.7.12/scripts/download-actionlint.bash) 1.7.12
          ./actionlint -color
```

Lý do: `dist/` gitignored nhưng `tauri_build`/`generate_context!` đọc `frontendDist` lúc compile →
`pnpm build` phải trước cargo. `pnpm/action-setup` trước `setup-node` vì `cache: pnpm` gọi
`pnpm store path`. Không bật cache ở cả hai action (trùng key). `ubuntu-22.04` giữ `libappindicator3-dev`
và glibc 2.35 cho AppImage/deb.

### 6.3 `.github/workflows/release.yml`

```yaml
name: Release
on:
  push: { tags: ['v*'] }
  workflow_dispatch:
permissions:
  contents: write
concurrency:
  group: release-${{ github.ref }}
  cancel-in-progress: false
jobs:
  publish:
    strategy:
      fail-fast: false
      matrix:
        include:
          - { platform: ubuntu-22.04,  target: x86_64-unknown-linux-gnu, args: '' }
          - { platform: windows-latest, target: x86_64-pc-windows-msvc, args: '' }
          - { platform: macos-latest, target: aarch64-apple-darwin, args: '--target aarch64-apple-darwin' }
          - { platform: macos-latest, target: x86_64-apple-darwin,  args: '--target x86_64-apple-darwin' }
    runs-on: ${{ matrix.platform }}
    timeout-minutes: 60
    steps:
      - uses: actions/checkout@v7
      - name: Tauri Linux dependencies
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y --no-install-recommends \
            libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf xdg-utils
      - uses: pnpm/action-setup@v6
        with: { version: 10.33.0 }
      - uses: actions/setup-node@v7
        with:
          node-version: '22'
          cache: pnpm
          cache-dependency-path: pnpm-lock.yaml
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: '${{ matrix.target }}' }
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: './src-tauri -> target'
          key: ${{ matrix.target }}
      - run: pnpm install --frozen-lockfile
      - uses: tauri-apps/tauri-action@v1
        env: { GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}' }
        with:
          tagName: v__VERSION__
          releaseName: 'merlink-harness v__VERSION__'
          releaseDraft: true
          prerelease: false
          uploadUpdaterJson: false
          args: ${{ matrix.args }}
```

- `__VERSION__` lấy từ `tauri.conf.json` (0.1.0); bump đồng bộ `package.json` + `Cargo.toml` trước khi tag.
- Chưa signing: macOS bị Gatekeeper, Windows SmartScreen. Khi cần: secret Apple + PFX Windows.
- `uploadUpdaterJson: false` vì chưa cấu hình updater plugin.
- Có thể thêm `uploadWorkflowArtifacts: true` để giữ bundle dù release lỗi.

### 6.4 Phiên bản action (đã đối chiếu official docs, không dùng cú pháp cũ)

`actions/checkout@v7` (v7.0.1) · `actions/setup-node@v7` (v7.1.0) · `pnpm/action-setup@v6` (v6.1.0,
pnpm 10) · `dtolnay/rust-toolchain@stable` · `Swatinem/rust-cache@v2` · `tauri-apps/tauri-action@v1`
(action-v1.0.0, `uploadUpdaterJson`) · actionlint 1.7.12. Không dùng `pnpm/action-setup@v4` /
`checkout@v4` / `setup-node@v4` / `tauri-action@v0` (cũ).

---

## 7. Thứ tự thực hiện

| Phase | Việc | Acceptance |
|---|---|---|
| P0 | Baseline `cargo fmt --all`; đo/fix clippy | fmt check xanh; clippy 0 warning |
| P1 | Skeleton + `super_fields.json` + `tables.rs` + `schema.rs` | unit test schema/types; drift test |
| P2 | `io.rs` resolution + rows validation; `progress.rs` 18/29 | 3-character test; 70/100 test |
| P3 | Prompt + dàn câu hỏi + `specs.rs` + `labels.rs` + `summary.rs` | prompt tests; labels 81; summary 6 đoạn |
| P4 | Migration + backup | fixture v1 → v2; idempotent; backup |
| P5 | Frontend mock/type/ToolLog | browser preview chạy; 3 tên hiển thị |
| P6 | CI/CD workflows + actionlint | `actionlint` xanh; CI xanh trên PR |
| P7 | Docs `super-json.md`, `AGENTS.md`, `idea.md` | không còn "25 path / 11 required / 14 optional" |

---

## 8. Verification

```bash
pnpm install --frozen-lockfile
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --locked      # baseline 127 + test mới, 3 ignored
python3 scripts/gen-super-fields.py --check                   # nếu dùng generator
docker run --rm -v "$PWD":/repo -w /repo rhysd/actionlint:1.7.12 -color
```

Manual: mở browser preview, tạo project, nhập "3 nhân vật Hoàng/Phong/Linh", xác nhận
`count=3`, `rows` 3 object, progress nhảy đúng 1 unit, summary in đủ 3 tên; `super_json_get` table
selector trả `fields`; thử ghi row có key lạ → lỗi, file không đổi.

CI: push branch → `gh run watch`; xác nhận cache pnpm + Rust restore ở run thứ hai. Release: chỉ tag
`v0.1.1` sau khi CI xanh; kiểm tra draft assets trước khi publish.

---

## 9. Rủi ro / điểm cần user chốt

### 9.1 Thanh tiến độ: đếm theo "mục" hay theo "field"?

Code hiện tại đếm **từng field**: 11 field require, mỗi field ~6.36pp (70/11); 14 field optional,
mỗi field ~2.14pp. Schema mới không còn "mỗi field một giá trị" nữa: `fields` là khuôn read-only,
dữ liệu nằm trong `rows`, và một bảng có thể có N row.

Ba cách:

| Cách | Đếm gì | Require | Optional | Mỗi mục | Vấn đề |
|---|---|---|---|---|---|
| **A (plan chọn)** | 1 project leaf = 1 mục; 1 bảng = 1 mục (count + đủ rows) | 18 | 29 | 3.89 / 1.03pp | bảng `characters` chỉ xong khi count=3 và rows có đủ 3 |
| B | tách `count` và `rows` thành 2 mục cho mỗi bảng | 30 | 51 | 2.33 / 0.59pp | bảng có thể "nửa xong": count=3 nhưng rows=0 vẫn được 1 mục |
| C | đếm từng value trong từng row | thay đổi | thay đổi | — | mẫu số đổi khi user thêm nhân vật, thanh nhảy loạn |

Plan chọn **A** vì ổn định và khớp nghĩa "bảng đã đủ dữ liệu". Nếu muốn field-level như cũ thì phải
đổi `fields` thành "giá trị row đầu tiên" (khác D) — nói rõ để tôi đổi plan.

### 9.2 Các điểm khác

1. **Migration**: mapping làm mất `logline.*` và `age-rating` (không có cột mới). Chấp nhận backup?
2. **Token prompt**: system prompt 34 descriptor + enum 81 path ~3–5 KB; nếu quá lớn, chuyển sang
   grammar `{table, kind}` (2 enum) và `get` table selector.
3. **Release signing**: xem 9.3.
4. **Baseline fmt**: 118 diff hiện tại; phải commit format-only trước, nếu không CI đỏ ngay.
5. **Rows whole-array**: mỗi lần sửa 1 nhân vật phải replace cả array; model phải đọc trước để không
   mất row. Nếu thấy tốn token, cân nhắc tool `row_upsert` (ngoài scope plan này).

### 9.3 Release: có ký (signing) hay không?

- **Không ký (plan hiện tại)**: build và tải về vẫn chạy, nhưng macOS Gatekeeper báo "damaged/unknown
  developer" (phải right-click Open hoặc `xattr -dr com.apple.quarantine`), Windows SmartScreen báo
  "unknown publisher". Phù hợp dev/nội bộ.
- **Có ký**: cần Apple Developer Program (~99 USD/năm) + certificate Windows (OV/EV, tốn tiền), thêm
  secrets và bước import vào workflow. Chỉ nên làm khi phát hành cho người ngoài tải.

Plan đang để **unsigned** + `releaseDraft: true` để kiểm tra asset trước khi publish.

---

## 10. Trạng thái thực hiện (2026-10-19)

Đã thực hiện toàn bộ P0–P7 trong working tree (chưa commit):

- [x] Skeleton v2 + patch `scenes.chapter_id`; 81 path, 18 required / 29 optional unit.
- [x] `schema.rs`, `tables.rs` (mới), `io.rs`, `progress.rs`, `summary.rs`.
- [x] 5 tool: `project_create`, `super_json_get` (table selector), `super_json_set` (`count`/`rows`),
      `super_json_missing` (18 unit), `echo`; specs 81 path.
- [x] Prompt + dàn câu hỏi mới; `{required_total}`/`{optional_total}` động.
- [x] Labels 81 path; status rows không dump array.
- [x] Migration v1 → v2 + backup `super.v1.json`.
- [x] Browser mock + `types.ts` + `Chat.tsx` fallback 18.
- [x] `.github/workflows/ci.yml` + `release.yml` (unsigned, draft release).
- [x] Docs: `docs/design/super-json.md`, `AGENTS.md`, `README.md`, `idea.md`.

Verification:

```text
cargo fmt --all --check          OK
cargo clippy --all-targets -D warnings  OK
cargo test --locked              126 passed, 0 failed, 3 ignored
pnpm build                       OK
actionlint 1.7.12                OK
```

Chưa có commit/tag/push. Baseline `cargo fmt --all` đã chạy nên diff có format-only ở nhiều file;
nên tách commit format trước khi mở PR theo plan.
