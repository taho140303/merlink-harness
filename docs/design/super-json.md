# Super JSON v2 — đặc tả project-rooted

> **Trạng thái:** Chốt (thay thế đặc tả 25-leaf cũ).
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

**18 unit require** = 6 project leaf + 12 bảng:
`project_genres`, `arcs`, `chapters`, `scenes`, `scene_characters`, `characters`,
`relationships`, `relationship_roles`, `world_settings`, `power_systems`, `themes`,
`theme_arguments`.

**29 unit optional** = 7 project leaf + 22 bảng còn lại.

## 4. Progress

`percent = round(70 * filled_require / 18 + 30 * filled_optional / 29)`.

- Project leaf filled = non-null.
- Table unit filled = `count` có, `rows.length == count`, rows validate pass.
- Xong 18 unit require = đúng **70%**; xong tất cả = **100%**.

## 5. Validate

- `rows` phải là array of object.
- Key lạ → reject, liệt kê key hợp lệ.
- Type theo `schema::column_kind` (text/int/bool/enum/JSON/number).
- Row của bảng require phải có đủ field trong entry require.
- `count` và `rows` phải khớp; `characters` đúng một `is_primary`.
- `fields`, `table` không ghi được; `id`/timestamps không nằm trong document.

## 6. Migration

File 25-leaf cũ (`require.overview.*`) được migrate một lần khi load:

- `overview.name/language/author/description/plan.*` → `project.*`.
- `genre.primary/secondary` → `project_genres.rows`.
- `theme.*` → `themes.rows`.
- `world.setting-scale` → `world_settings.rows[0].type`.
- `world.has-power-system` → `power_systems.count` 1/0.
- `logline.*`, `age-rating` không có cột mới → chỉ còn trong `super.v1.json` backup.

## 7. Bất biến

- Không tool nào nhận path cũ; không có nhánh tương thích trong `get/set`.
- `super_json_missing` trả unit thiếu (project leaf hoặc `require.schema.<table>`).
- `super_json_get` table selector trả `{table, count, fields, rows}` đã merge hai group.
- Summary 6 đoạn đọc từ document, deterministic, placeholder `[chưa có thông tin]`.
