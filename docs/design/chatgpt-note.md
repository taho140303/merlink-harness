Knowledge ≠ Competence (Kiến thức không cung cấp năng lực thực thi)
Knowledge does not automatically grant competence. A character may know that a problem exists without knowing how to solve, implement, or manage it.
=> Nhân vật có điểm mạnh/yếu rõ ràng, dễ đồng cảm, phải nỗ lực. Các vấn đề của cốt truyện được giải quyết qua thử-sai, hy sinh, và học hỏi. Thế giới có sức nặng, có quy luật riêng, tôn trọng trí tuệ người khác. Tạo ra bài học và trải nghiệm: "Biết là bước đầu tiên. Làm được mới là thách thức thật sự."

```
Meta
├── Theme                          ← node mới
│   └── [theme_id]
│       ├── scope: "project" | "arc"
│       ├── arc_ref: <arc_id> | null      # null nếu single-arc hoặc theme gốc toàn truyện
│       ├── parent_theme_ref: <theme_id> | null   # theme con trỏ về theme gốc (multi-arc)
│       ├── question: string
│       ├── respondents: [...]
│       ├── resolution_type: "resolved" | "open" | "ambiguous"
│       ├── resolution: string
│       └── message: string | null
```

| role           | ý nghĩa                                                                               | trong ví dụ            |
| -------------- | --------------------------------------------------------------------------------------- | ------------------------ |
| `primary`    | nhân vật mà kết quả/hành trình của họ = câu trả lời chính thức của Theme | Yuji                     |
| `antithesis` | đại diện giá trị đối lập, bị bác bỏ                                          | Sukuna                   |
| `cautionary` | có phần đúng nhưng thiếu, thất bại vì thiếu                                   | Gojo                     |
| `witness`    | góp mặt, không mang lập trường riêng biệt, chỉ củng cố phe primary           | Megumi, Higuruma, Hakari |
