# SPRINT 17.1 — STATE v3 + CATALOG + ANCHOR

> **Trạng thái: DONE.** Branch `sprint-17.1-state-v3`, base `main` tại `dfa5caa`.
> Kết quả: `cargo test` **186 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `cargo fmt --check` sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.
> Route dùng: 17.1B `antigravity / gemini-3.8-flash-high / high`; 17.1A (catalog) dừng và orchestrator
> tự làm; 17.2A `openai-codex / gpt-6-luna / high` (đã xong, tách khỏi sprint này).

## CONTEXT

Phase 1 xong: `super.json` v2.1, 81 writable path, 12 require / 35 optional unit, 7 tool. Phase 2 cần
chỗ chứa dàn ý (arc/chapter/scene chi tiết, chỉ số viết văn), catalog đọc được từ code, và khối
"anchor" 6 đoạn làm context cho subagent.

## KẾT QUẢ

| Hạng mục | Thay đổi |
|---|---|
| `super.json` | v3: thêm 30 field vào 9 bảng (xem `docs/design/super-json.md` §v3). **Vẫn 34 bảng / 81 path / 12-35 unit.** |
| Migration | `io::migrate_v3_if_needed`: refresh `fields`, giữ `count`/`rows`, sort theo `schema::tables()`, backup `super.v2.json` một lần, idempotent |
| Type rules | `FieldKind::UnitNumber` (0..=1 cho `themes.weight`), enum `polarity` + `scenes.status`, JSON cho `beats/directives/rules/relationships`, `NonNegativeInt` cho `scene_count/target_words/word_count` |
| Catalog | `docs/schema/catalog/*.json` (6 file: 20 genres, 125 motifs, 14 narrator, 21 directive, 43 relationship category, 54 style preset có vocab/focus/devices) sinh bởi `scripts/gen-catalog.mjs`; module `src-tauri/src/catalog` tra theo `code` |
| Anchor | `outline::anchor()` 6 đoạn + `anchor_compact()` ≤600 ký tự + `anchor_json()`; command `outline_anchor`, tool `outline` action `anchor` |
| Tool thứ 8 | `outline` (`status` \| `anchor`), đăng ký đủ 6 điểm (TOOL_NAMES, specs, execute, labels, session binding, harness_system) |
| IPC | `outline_anchor(projectId)`, `outline_status(sessionId)` + `CommandMap` + `types.ts` + `browser-mock` + contract test |

## QUYẾT ĐỊNH TRONG SPRINT

1. **17.1A dừng, orchestrator tự viết catalog.** Agent dữ liệu không ghi được file sau ~50 phút; script
   do orchestrator viết và chạy trong 15 phút, kết quả join đủ vocab/focus/devices (tốt hơn phương án
   fallback đã cho phép).
2. **`outline/seed.rs` dời sang 17.2.** Resolve `genre_code` cần engine + catalog cùng lúc; 17.1 chỉ
   giao "extract" (anchor) + data layer.
3. **`anchor_compact` được đưa vào payload** của command và tool (key `compact`) thay vì để dead code.
4. **`catalog` có `#![allow(dead_code)]`** vì consumer thật (`seed`, prompt builder, commit validator)
   nằm ở 17.2. **Xoá allow này ở đầu 17.2** — đây là nợ kỹ thuật đã biết, không phải bỏ quên.
5. `mod outline` để private (B khai báo `pub mod`); không có caller ngoài crate.
6. File `docs/schema/system.dbml` / `.dbdiagram` **không** nằm trong commit này (là chỉnh sửa sẵn có
   của người dùng trước sprint).

## NGOÀI PHẠM VI (đúng kế hoạch)

Không có fan-out/subagent engine, không có bảng SQLite mới, không có UI outline board, không port
`system.dbml` sang SQLite.
