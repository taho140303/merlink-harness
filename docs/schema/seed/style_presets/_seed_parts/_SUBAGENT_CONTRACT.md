Subagent contract:
- Read RULES.md (cùng folder) TRƯỚC KHI làm bất kỳ gì.
- Đọc toàn bộ *.json trong docs/library/style_presets/<FOLDER>/
  (KHÔNG đọc /action/ trừ khi được assign — xem task).
- Sinh ra 3 file trong docs/library/style_presets/_seed_parts/:
  <FOLDER>.sql, <FOLDER>.devices.json, <FOLDER>.report.md
- KHÔNG đụng file/folder khác.
- Báo về main agent: "DONE <FOLDER> | presets=<N> | devices=<N> | warnings=<N>"
