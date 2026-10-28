# SPRINT 4 — KẾT QUẢ (responsive split-pane + form primitives + a11y)

**Ngày:** 2026-10-29 · **Trạng thái:** DONE (a11y pass một phần, đã ghi rõ remaining)
**Tiền đề:** `43ff284` (Sprint 3).

---

## 1. Responsive split-pane (< 1280px)

Vấn đề: 3 màn split-pane có min-width 2 cột (chat 545px + draft 530px) lớn hơn viewport 1024 → layout bị bóp méo, header wrap dọc.

Đã thêm:
- `src/shared/hooks/useMediaQuery.ts` (`useIsCompact`, ngưỡng `max-width: 1279px`).
- `ChatView`: dưới 1280px, chat chiếm full width; mở draft → overlay full-pane (`z-40`, `bg-surface-panel`), splitter ẩn.
- `CharacterDetailLayout`: chat pane → overlay full-pane; splitter ẩn.
- `ProjectDetailView`: chat pane → overlay full-pane; splitter ẩn.
- `useResizablePanel` expose `setLeftRatio`; splitter ở character/project có thêm `role="separator"`, `aria-valuenow/min/max`, `tabIndex`, phím ←/→ (±5%).

Đã kiểm chứng 1024×768: cả 3 overlay phủ kín, có nút đóng, không còn tràn/bóp.

## 2. Primitive form mới

- `Checkbox`: native input + label/hint, `aria-invalid`, focus-ring.
- `Switch`: `role="switch"` + `aria-checked`, track accent, disabled.
- `Segmented`: `role="radiogroup"` + `role="radio"`/`aria-checked`, keyboard ←/→/Home/End.

Migration:
- `ProviderForm`: 2 checkbox (text/image) → `Checkbox`; switch Reasoning → `Switch`; input/select/button đã dùng primitive từ Sprint 3.
- `GeneralTab`: 2 nhóm theme/language → `Segmented` (screen reader đọc `radio "Sáng" [checked]`).

## 3. A11y pass

| Chỉ số | Trước | Sau |
|:--|--:|--:|
| `aria-label` | 27 | **42** |
| `aria-expanded` | 0 | **12** |
| `aria-pressed` | 0 | **8** |
| `role="separator"` | 1 | **3** |
| `focus-ring` | 73 | **75** |

Bổ sung: `aria-expanded` cho accordion draft (Arc/Chapter/Segment/Scene, core identity), `aria-pressed` cho filter/toggle, `DialogDescription` cho CreateCharacterDialog/CharacterDetailDialog, slider `aria-valuetext`, icon trang trí `aria-hidden`.

## 4. Sự cố từ agent & cách xử lý

- Agent a11y 2 lần làm vỡ cú pháp JSX (`className="focus-ring "flex ..."`) ở `SegmentBlock` và `DraftToolbar` → dev server PARSE_ERROR; đã yêu cầu sửa và verify `tsc` sau mỗi lần.
- Agent forms báo `tsc` fail do lỗi ngoài scope (cùng nguyên nhân) — sau khi a11y sửa, `tsc`/`lint` xanh.
- Không có thay đổi nào bị mất; working tree chỉ còn thay đổi hợp lệ.

## 5. Kiểm chứng

- `npx tsc --noEmit` → **0 lỗi**
- `npm run lint` → **0 error**
- `npm run build` → **✓ 327ms**
- QA: chat+draft 1440 (split) & 1024 (overlay), character chat 1024, project chat 1024, settings general (Segmented), provider form (Checkbox/Switch), draft toolbar.

## 6. Còn lại

1. A11y pass chưa phủ 100% control cũ (một số nút icon-only ở project card/character detail, filter toggle) — report `sprint4-a11y.md` liệt kê.
2. `system-prompt/**` thuộc `/config` — vẫn loại trừ.
3. Chưa có automated a11y test (axe/pa11y) — cần dependency mới nếu muốn.
4. `ProviderForm` tab switcher (third-party/custom) vẫn là button tự chế, có thể chuyển `Segmented` ở sprint sau.
