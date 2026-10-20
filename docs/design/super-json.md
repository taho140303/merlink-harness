# Super JSON v2.1 — đặc tả project-rooted

> **Trạng thái:** Chốt (v2.1, thay thế đặc tả 25-leaf cũ).
> **Artifact máy đọc:** [`docs/schema/runtime/super.json`](../schema/runtime/super.json)
> **Kế hoạch thi công:** [`docs/plan/super-json-v2-plan.md`](../plan/super-json-v2-plan.md)
> **Nguồn schema:** `docs/schema/system.dbml` (35 bảng / 237 attribute).

---

## 1. Hình dạng

Một document cho một project. Gốc là **một** `project`; 34 bảng còn lại nằm trong `schema`.

```json
{
  "require": {
    "project": {
      "name": null,
      "language": null,
      "plan": {
        "arc":     { "count": null },
        "chapter": { "count": null },
        "scene":   { "count": null },
        "word":    { "count": null }
      }
    },
    "schema": [
      {
        "table": "characters",
        "count": 3,
        "fields": { "name": null, "is_primary": null },
        "rows": [
          { "name": "Hoàng", "is_primary": true },
          { "name": "Phong", "is_primary": false },
          { "name": "Linh", "is_primary": false }
        ]
      }
    ]
  },
  "optional": { "project": { }, "schema": [ ] }
}
```

- `project`: 13 leaf cố định (6 require + 7 optional).
- `schema`: 34 bảng, mỗi bảng có `table`, `fields`, `count`, `rows`.
- `fields` = contract read-only cho một row; **không ghi**, **không tính progress**.
- `count` = số row dự kiến (int ≥ 0).
- `rows` = toàn bộ instance; ghi đè cả array; object chỉ chứa key trong `fields`.
- `count == rows.length` khi cả hai đã có. Bảng require cần `count > 0`; bảng optional
  nhận `count = 0, rows = []` là "không có".
- `characters.is_primary` là field synthetic; đúng một row `true` khi count > 0.

## 2. Path grammar

| Loại | Ví dụ | Ghi |
|---|---|---|
| Project leaf | `require.project.name` | có |
| Table count | `require.schema.characters.count` | có |
| Table rows | `require.schema.characters.rows` | có |
| Table selector | `require.schema.characters` | chỉ `super_json_get` |

Tổng **81 path** = 13 project + 34×2. `fields` và `table` không phải path.
Resolver tìm entry theo tên bảng, không dùng index mảng.

### 2.1 Draft references (Sprint 16.1)

- `_json_index` = **vị trí 0-based** của row trong `rows` của bảng đó. Tool gán và ghi đè mọi giá
  trị model gửi; mọi lần ghi `rows`/`add_row` đều chuẩn hoá lại `_json_index = vị trí`.
- Mỗi field khai báo kết thúc bằng `_id` có thêm cột synthetic `<field>_json_index` (int ≥ 0) để
  trỏ tới row draft của bảng đích, ví dụ `from_character_id_json_index: 0`.
- Ref **thay thế** required `_id`: row `relationships` chỉ cần 2 ref là hợp lệ; field `_id` tương ứng
  để null hoặc bỏ hẳn.
- Range check: nếu `(table, field)` nằm trong `tables::REF_TARGETS`, `<field>_json_index` phải
  `< target.rows.len()`. Bảng đích ngoài draft (`motifs`, `genres`, `style_presets`,
  `narrator_configs`) chỉ check int ≥ 0.
- `_json_index` và `<field>_json_index` là synthetic trong `table_fields()`; **không** thêm vào
  `fields` của `super.json` (giữ nguyên 81 path / 34 bảng). Table selector của `super_json_get`
  trả thêm key `refs`.
- `validate_rows` có hai mode: **Draft** (`super_json_add_row`: cho phép thiếu required, bỏ
  count-equality, characters-primary và document rule) và **Complete** (`super_json_set` trên
  `.rows`: đòi đủ mọi rule).
- Hai tool ghi tăng dần: `super_json_add_row` (append, tool gán `_json_index`, tự quản `count`) và
  `super_json_add_field` (đúng 1 field của 1 row; reject `_json_index`). Cả ba tool ghi trả
  `warnings` mềm (POV/motif), không chặn ghi và không đổi progress.

## 3. Require / optional

- require = phải biết để viết chương 1.
- optional = điền lúc nào cũng được, kể cả sau khi đã viết.
- NOT NULL / default của DBML **không** phải tiêu chí.

**12 unit require** = 6 project leaf + 6 bảng:
`project_genres`, `arcs`, `characters`, `project_motifs`, `themes`, `theme_arguments`.

**35 unit optional** = 7 project leaf + 28 bảng còn lại:
`relationship_motifs`, `chapters`, `scenes`, `scene_characters`, `scene_versions`,
`appearance_option_groups`, `appearance_options`, `character_appearances`,
`character_distinguishing_marks`, `relationships`, `relationship_categories`,
`relationship_roles`, `relationship_states`, `world_settings`, `lore_entries`,
`power_systems`, `power_tiers`, `power_abilities`, `memories`, `memory_references`,
`memory_emotions`, `memory_connections`, `memory_revisions`, `character_knowledge_sources`,
`competence_domains`, `character_competences`, `competence_capabilities`,
`competence_limitations`.

Mỗi bảng chỉ có **một** entry `{table, count, fields, rows}`; `fields` là hợp của contract core +
extra, nên đổi nhóm không làm mất field.

## 4. Progress

`percent = round(70 * filled_require / 12 + 30 * filled_optional / 35)`.

- Project leaf filled = non-null.
- Table unit filled = `count` có, `rows.length == count`, rows validate pass, và qua rule
  cross-table của bảng (§5).
- Xong 12 unit require = đúng **70%**; xong tất cả = **100%**.

## 5. Validate

- `rows` phải là array of object.
- Key lạ → reject, liệt kê key hợp lệ.
- Type theo `schema::column_kind` (text/int/bool/enum/JSON/number).
- Row phải có đủ field trong `REQUIRED_ROW_FIELDS`, map hằng **độc lập group**; bảng chuyển từ
  require sang optional vẫn giữ contract (vd `scenes` vẫn cần `name`, `chapter_id`, `position`).
- `count` và `rows` phải khớp; `characters` đúng một `is_primary`.
- Rule cross-table (soft, trả `Ok(false)` chứ không reject write):
  - `project_motifs`: chỉ filled khi có ít nhất một row `scope == "project"`.
  - `scene_characters`: nếu `scenes.rows` không rỗng thì phải có ít nhất một row `is_pov == true`;
    `scenes` rỗng/không có thì không áp rule.
- `fields`, `table` không ghi được; `id`/timestamps không nằm trong document.

## 6. Migration

Có hai migration idempotent, chạy khi load và ghi lại file nếu có thay đổi:

### 6.1 v1 (25 leaf) → v2

File 25-leaf cũ (`require.overview.*`) được migrate một lần khi load:

- `overview.name/language/author/description/plan.*` → `project.*`.
- `genre.primary/secondary` → `project_genres.rows`.
- `theme.*` → `themes.rows`.
- `world.setting-scale` → `world_settings.rows[0].type`.
- `world.has-power-system` → `power_systems.count` 1/0.
- `logline.*`, `age-rating` không có cột mới → chỉ còn trong `super.v1.json` backup.

### 6.2 v2 → v2.1

Doc v2 (bảng nằm sai group, hoặc có entry thứ hai fields-only) được chuẩn hoá:

- Gom mọi entry của một bảng trong `require.schema` + `optional.schema`.
- Nếu đúng một entry và đúng `countable_group(table)` → bỏ qua.
- Ngược lại: giữ `count`/`rows` của entry đang sở hữu `count`, lấy `fields` từ skeleton v2.1, xoá
  entry cũ và chèn một entry mới vào đúng group.
- Sort `schema` cả hai group theo thứ tự `schema::tables()` để ổn định.
- Idempotent: load lần hai không đổi gì.

### 6.3 v2.1 → v3

Migration v2.1 → v3 bổ sung các field cho phase 2 mà không thay đổi cấu trúc bảng hay số lượng path:
- Refresh `fields` của mỗi bảng bằng skeleton v3, giữ nguyên `count` và `rows`.
- Sort `schema` cả hai group theo thứ tự `schema::tables()`.
- Backup `super.v2.json` được ghi cạnh `super.json` trước lần ghi đè đầu tiên (quyền 0600 trên Unix).
- Idempotent: load lần hai không đổi byte nào và không tạo backup lần hai.

## 7. v3 — Field bổ sung cho phase 2

Bảng field bổ sung cho phase 2 outline engine:

| Nhóm bảng | Field bổ sung | Kiểu | Lý do 1 dòng |
| --- | --- | --- | --- |
| `project_genres` | `genre_code`, `note` | Text, Text | Lưu mã thể loại chuẩn hoá từ catalog và ghi chú đặc thù |
| `arcs` | `goal`, `outcome` | Text, Text | Xác định mục tiêu và kết quả chính của từng arc truyện |
| `chapters` | `beats`, `scene_count`, `goal`, `outcome` | JSON array, NonNegativeInt, Text, Text | Chứa danh sách nhịp kể (beat sheet), số scene dự kiến và đích đến của chương |
| `characters` | `role`, `archetype`, `motivation`, `want`, `need`, `flaw`, `summary`, `description` | Text (tất cả) | Định hình hồ sơ tâm lý và chức năng kể chuyện đầy đủ cho nhân vật |
| `relationships` | `summary`, `origin`, `is_primary` | Text, Text, Bool | Tóm tắt động lực quan hệ, điểm xuất phát và đánh dấu mối quan hệ nòng cốt |
| `project_motifs` | `name`, `description`, `significance` | Text, Text, Text | Mô tả rõ nội dung biểu tượng motif và ý nghĩa gắn với cốt truyện |
| `themes` | `name`, `statement`, `weight` | Text, Text, UnitNumber (0..1) | Đặt tên chủ đề, phát biểu luận đề và định trọng số phân bổ |
| `theme_arguments` | `claim`, `counter_claim`, `polarity` | Text, Text, Enum (SUPPORT/CHALLENGE/AMBIVALENT) | Khắc hoạ xung đột lập luận tư tưởng thông qua góc nhìn nhân vật |
| `scenes` | `style_preset_code`, `narrator_code`, `directives`, `rules`, `relationships`, `target_words`, `content_path`, `word_count`, `status` | Text, Text, JSON array, JSON array, JSON array, NonNegativeInt, Text, NonNegativeInt, Enum (PLANNED/WRITTEN/REVISED/LOCKED) | Cung cấp đầy đủ chỉ thị kỹ thuật, ràng buộc và trạng thái sản xuất scene |

### Quy ước đặc biệt trong phase 2:
- **`scenes.content` luôn `null` trong phase 2**: nội dung văn bản hoàn chỉnh của scene được lưu tại file riêng trỏ bởi `content_path` (ví dụ `projects/<id>/scenes/<scene_code>.md`), không nhồi trực tiếp vào `super.json`.
- **Quy ước `*_code` thay cho uuid**: các trường `genre_code`, `style_preset_code`, `narrator_code` lưu mã định danh ổn định (human-readable string từ catalog) thay vì chuỗi uuid động, giúp subagent dễ đọc và tạo prompt nhất quán.

## 8. Bất biến

- Không tool nào nhận path cũ; không có nhánh tương thích trong `get/set`.
- Mỗi bảng có đúng một entry; không có entry fields-only thứ hai.
- `super_json_missing` trả unit thiếu (project leaf hoặc `require.schema.<table>`).
- `super_json_get` table selector trả `{table, count, fields, rows}` đã merge hai group, cộng thêm
  `refs` (các cột `<field>_json_index` synthetic).
- `_json_index` do tool gán bằng vị trí row; `<field>_json_index` thay được required `_id` và được
  range check theo `tables::REF_TARGETS`. Cả hai là synthetic, không phải path.
- `super_json_set`, `super_json_add_row`, `super_json_add_field` trả `warnings` mềm cho POV/motif;
  warning không chặn ghi, không đổi `SuperJsonProgress` và không đổi 2 rule hoàn tất.
- `project_motifs` là bảng require nhưng chỉ filled khi có row `scope == "project"`; row
  `scope = arc|chapter` là tuỳ chọn. `scope` nhận `project | arc | chapter`.
- Summary 6 đoạn đọc từ document, deterministic, placeholder `[chưa có thông tin]`.
