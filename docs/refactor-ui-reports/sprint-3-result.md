# SPRINT 3 — KẾT QUẢ (i18n + form/tabs + lint gate)

**Ngày:** 2026-10-29 · **Trạng thái:** DONE (trừ system-prompt thuộc diện loại trừ)
**Tiền đề:** commit `c678188` (Sprint 2).

---

## 1. i18n UI chrome

| Chỉ số | Trước | Sau |
|:--|--:|--:|
| Key trong `dict.ts` (mỗi locale) | 418 | **576** |
| Parity en/vi | 418/418 | **576/576** |
| Key trùng / thiếu | 2 trùng | **0 / 0** |
| Module đã phủ | chat, project (một phần) | chat, settings, sidebar, project, character, character-detail, draft, routes |

- 100% UI chrome (nút, label, placeholder, title, aria-label, empty state, 404, toast) đi qua `t()` ở các module trong phạm vi.
- **Dữ liệu giữ nguyên** (theo quyết định): mock chat/thinking/tool output, MOCK_HISTORY, mockProjects/mockCharacters/mockOverview, nội dung chương, tên model/provider/protocol, số liệu, `Merlink-harness`.
- `ProcessDetailDialog`: 6 câu mô tả có input nội suy đã tách thành key trước/giữa/sau, giữ nguyên vị trí input.

**Còn sót có kiểm soát:**
- `ChatView.tsx`: 19 dòng mock hội thoại + thinking log (data).
- `ChapterDetailView.tsx:63`: câu fallback nội dung chương mẫu (data).
- Comment tiếng Việt trong code (không phải UI).
- `system-prompt/**`: thuộc `/config` — loại trừ theo spec (dù đã được token hoá ở Sprint 2).

## 2. Form / Tabs

- `ProviderForm`: chuyển input/select/nút sang `Input`, `Select`, `Button` (checkbox native giữ nguyên cho text/image + switch reasoning giữ `role="switch"`/`aria-checked`).
- `ProjectDetailSidebar`: agent thêm một hàng `Tabs` **no-op** (file gốc không có 4 tab/state) → đã **gỡ bỏ**, giữ nguyên hành vi quick-view; thay vào đó hoàn thiện i18n + aria-label cho nút đóng.
- `SettingsSidebar`: giữ `<nav>` + `aria-current="page"` (đúng ngữ nghĩa điều hướng, không phải tab panel).

## 3. ESLint gate

`npm run lint` = 0 error với 7 nhóm rule (`no-restricted-syntax`):
`text-[Npx]`, `rounded-[`, `shadow-*`, `bg-white`, `focus:outline-*`, hex trong TSX, `dark:` cho màu đã token.
Loại trừ có lý do: `routes/config.tsx`, `routes/worldbuilding.tsx`, `AuroraBackground`, `OrbSphere`, `CreateCharacterDialog` (màu là data), `SidebarHistory` (knob trắng cố ý).

## 4. Sửa lỗi phát sinh từ agent

- 7 lỗi `tsc` do agent bỏ dở: thiếu `useT` trong `SceneConfigBlock` (SceneCard), sai đường dẫn import `PlaceholderStep`.
- `dict.ts` bị chèn trùng 6 key (đợt 1) và 21 key trùng (đợt 1 lần đầu) → đã reconcile bằng script: 576/576, 0 trùng.
- Gỡ hàng `Tabs` no-op ở `ProjectDetailSidebar`; sửa class rác `-subtle`, `focus-ring` lặp.

## 5. Kiểm chứng cuối

- `npx tsc --noEmit` → **0 lỗi**
- `npm run lint` → **0 error**
- `npm run build` → **✓ 661ms**
- Visual QA: Provider form (Settings → AI Providers → Sửa), Process detail dialog, draft characters tab, project quick-view.

## 6. Còn lại (không chặn)

1. System-prompt i18n (thuộc `/config`, loại trừ).
2. 2 checkbox native trong ProviderForm (đúng loại control, chưa có primitive Checkbox).
3. `DraftOverviewBlock` filter và `DraftToolbar` view-mode vẫn là button — bản chất là filter/view switch, không phải tab panel.
4. Có thể thêm rule ESLint bắt `aria-label` cho icon-only (cần jsx-a11y plugin — chưa cài).
