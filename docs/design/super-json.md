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

## 7. Bất biến

- Không tool nào nhận path cũ; không có nhánh tương thích trong `get/set`.
- Mỗi bảng có đúng một entry; không có entry fields-only thứ hai.
- `super_json_missing` trả unit thiếu (project leaf hoặc `require.schema.<table>`).
- `super_json_get` table selector trả `{table, count, fields, rows}` đã merge hai group.
- `project_motifs` là bảng require nhưng chỉ filled khi có row `scope == "project"`; row
  `scope = arc|chapter` là tuỳ chọn. `scope` nhận `project | arc | chapter`.
- Summary 6 đoạn đọc từ document, deterministic, placeholder `[chưa có thông tin]`.
