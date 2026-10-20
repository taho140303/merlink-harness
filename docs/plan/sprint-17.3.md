# SPRINT 17.3 — STAGE 2.1 (arcs + chapters) + OUTLINE WORKSPACE

> **Trạng thái: DONE.** Branch `sprint-17.3-stage-21`, base `main` tại `275266f`.
> Kết quả: `cargo test` **207 passed / 0 failed / 3 ignored**; `cargo clippy --all-targets -- -D warnings`
> sạch; `pnpm build` pass; `node scripts/gen-catalog.mjs --check` sạch.
> Route dùng: prompts + wiring `openai-codex / gpt-6-luna (orchestrator hoàn thiện)`;
> UI `openai-codex / gpt-6-luna / high` (PARTIAL, orchestrator hoàn thiện + viết lại markup).

## KẾT QUẢ

| Hạng mục | Nội dung |
|---|---|
| Prompt thật | `prompts/outline_arcs.md`, `prompts/outline_chapters.md`: luật ngân sách, `position` liên tục, `Σ chapters/scene_count`, cấm emoji và gạch ngang dài, cấm ghi state, contract JSON |
| Prompt builder | `outline/prompts.rs`: `system(lane)`, `arcs_user`, `chapters_user`; test khẳng định contract + cấm + con số ngân sách |
| Board contract | `OutlineItem` trả thêm `name/goal/outcome/summarize/chapters/beats/arc/scene_count` để card render được; test `test_outline_items_expose_proposal_content_for_the_board` |
| Rail state | `OutlineStatus.stages[]` (`empty/review/committed`) cho từng stage, tính từ item; test gián tiếp qua `status` |
| UI | `pages/OutlineWorkspace.tsx` (rail 3 tab, khoá 2.2/2.3 kèm lý do, tiến độ shard real-time, Chốt/Dừng/Về chat), `components/outline/ArcsBoard.tsx`, `components/outline/ItemCard.tsx`, `styles/outline.css` |
| Chat | Nút "Viết dàn ý" gọi `outline_start` + `outline_run("arcs")`, workspace thay transcript, "Về chat" quay lại, "Mở dàn ý" mở lại; khôi phục mode sau reload qua `outline_status` |
| IPC | `src/lib/outline.ts` (typed helper, không component nào gọi `safeInvoke`), `createAgentEventChannel<T>` generic cho `OutlineEvent` |
| Mock | `browser-mock` có fixture 3 arc + 12 chương, phát `OutlineEvent`, `outlineMockStatus` khớp shape backend (kể cả `stages`) |
| Contract test | Bỏ 8 command outline khỏi `RUST_ONLY_COMMANDS`; scan camelCase mở rộng sang `src/lib/outline.ts` (đúng call site thật) |

## QUYẾT ĐỊNH TRONG SPRINT

1. **`outline_items` trả field phẳng của proposal**, không bắt UI parse payload: board cần `goal`,
   `summarize`, `beats`, `scene_count` mà trước đó API chỉ trả metadata.
2. **`OutlineStatus.stages[]`** thay cho việc UI suy luận từ `stage` hiện tại: rail phải biết stage nào
   đã chốt dù run đã chuyển sang stage khác.
3. **Viết lại markup của agent** (nó nộp JSX dồn 1 dòng, khó bảo trì); đồng thời sửa lỗi board không
   reload sau khi chạy (thêm `reloadToken`).
4. **`src/types.ts` bị agent xoá doc-comment và dồn dòng toàn bộ** → khôi phục từ HEAD rồi chỉ thêm
   block outline mới; giữ nguyên comment gốc.
5. **Chat cần lối thoát**: mode dàn ý thay transcript nên thêm "Về chat" + nút "Mở dàn ý" khi đã có run.
6. **Contract test scan thêm `src/lib/outline.ts`** thay vì giữ 8 command trong `RUST_ONLY_COMMANDS`:
   helper module chính là nơi frontend thật sự gửi key.

## NỢ KỸ THUẬT ĐÃ BIẾT

- 2.2/2.3 chỉ là tab khoá; board thật ở 17.4/17.5.
- `outline_run` từ Chat không truyền event sink nên không thấy tiến độ shard trong lượt đầu; board tự
  chạy lại bằng command có channel thì thấy. 17.4 sẽ gộp về một đường.
- Chưa có test frontend (repo không có runner); UI kiểm bằng `pnpm build` + preview thủ công.

## NGOÀI PHẠM VI

Không có cast/world/scene lane, không có văn bản truyện, không sửa ProjectDetail.
