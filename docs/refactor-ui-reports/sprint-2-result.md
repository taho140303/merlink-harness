# SPRINT 2 — KẾT QUẢ (Design System v1: token + primitive + migration)

**Ngày:** 2026-10-29 · **Trạng thái:** DONE (phần visual/a11y), i18n để Sprint 3
**Phạm vi:** toàn bộ `src/`, trừ `/config`, `/worldbuilding` (theo quyết định), mock data không đổi nội dung.

---

## 1. Đã làm

### Nền tảng
- `docs/design-system.md`: chốt mô hình **2 tầng** (neu shell / flat content), type scale **min 12px**, i18n cho UI chrome, loại trừ config/worldbuilding.
- `src/app/app.css`: thêm 12 token màu (surface/fg/line/accent) + 4 radius (`shell`, `shell-sm`, `panel`, `card`), utility `.tier-panel`, `.tier-card`, `.tier-input`, `.tier-overlay`, `.focus-ring`, `prefers-reduced-motion` toàn cục, keyframes dialog/toast thật (thay `animate-in` chết).
- Token tự đổi theo `html.dark` — code mới không cần `dark:` cho màu.

### Primitive mới (`src/shared/components/ui/`)
`button.tsx`, `input.tsx`, `textarea.tsx`, `select.tsx`, `field.tsx`, `tabs.tsx` (role/aria/keyboard), `popover.tsx` (portal + Esc + click-outside + trả focus), `tooltip.tsx`, `card.tsx`, `badge.tsx`.
Nâng cấp: `dialog.tsx` (tier-overlay, focus-ring, i18n nút đóng), `toast.tsx`, `confirm-dialog.tsx` (DialogTitle/Description), `delete-confirm-dialog.tsx` (label + aria-invalid + i18n), `floating-outline.tsx` (token + focus).

### Migration
- Shell: FullLayout, Sidebar, SidebarHeader/Nav/History, SettingsDialog.
- Toàn bộ module chat, draft, project, character/detail, settings, system-prompt: token hoá surface/text/border; bỏ `shadow-*`, `bg-white`, `rounded-[...]`, `text-[Npx]`, `dark:*` cho màu đã token.
- A11y: splitter `role="separator"` + phím mũi tên; progressbar; slider `aria-valuetext`; tab bar dùng `Tabs`; icon-only có `aria-label`; dialog đủ Title/Description.
- Xoá dead code `CharacterDetailView.tsx` (697 dòng, không route nào import).
- Sửa baseline: 19 key i18n thiếu, 2 key trùng trong `dict.ts`, 1 key trùng trong mock (giữ nguyên hành vi), ref type `FloatingOutline`.

---

## 2. Số liệu trước → sau

| Chỉ số | Trước | Sau |
|:--|--:|--:|
| `focus-visible` | 0 | **73** chỗ dùng `.focus-ring` |
| `focus:outline-hidden` trần | 39 | **0** |
| `aria-label` | 1 | **27** |
| `text-[Npx]` | 256 | **0** (còn 1 ở worldbuilding — loại trừ) |
| `rounded-[...]` | 6 | **0** |
| `shadow-*` | 134 | **0** (còn 1 ở worldbuilding) |
| `bg-white` | 154 | **2** (knob switch + highlight orb — cố ý) |
| `border-slate-*` | 196 | **0** (trừ worldbuilding) |
| `text-slate-*` | 297 | **0** (trừ worldbuilding) |
| `animate-in` (class chết) | 20 | **0** |
| `duration-600` | 3 | **0** |
| `neu-*` | 28 | **13** (shell + ProjectCard đúng spec) |
| `tsc` | 25 lỗi | **0** |
| Build | — | **✓ 617ms** |

Visual QA thủ công: chat (light/dark), chat + draft, popover model, project list/detail, chapter, character list/detail/edit, settings, provider list, system prompt — 1440×900 và 1024×800.

---

## 3. Việc còn lại (Sprint 3)

1. **i18n UI chrome**: còn ~131 dòng copy hardcode (theo quyết định: không dịch mock data). Ưu tiên chat, project, character, settings; thêm key en/vi, không trùng.
2. **Tab bar còn tự chế**: `ProjectDetailSidebar`, `SettingsSidebar` (đã có aria-current), filter tab của `DraftOverviewBlock`, view-mode của `DraftToolbar` → chuyển sang `Tabs`.
3. **Chuẩn hoá form**: `ProviderForm` vẫn dùng input/select thô (đã token + focus-ring nhưng chưa dùng `Input/Select/Textarea`).
4. **ESLint gate**: cấm `text-[`, `rounded-[`, `shadow-`, `bg-white`, `focus:outline-hidden`, hex trong TSX; bắt `aria-label` cho icon-only.
5. **Responsive**: grid project đã sửa (`xl:grid-cols-2`); cần rà thêm 768px cho các split pane.

---

## 4. Ghi chú vận hành

- 4 agent `gpt-6-luna` ban đầu dừng giữa chừng (partial edits, không report). Phần còn lại được hoàn thiện bằng sweep tất định + sửa tay + QA trình duyệt; các file dở dang đã được chuẩn hoá lại toàn bộ.
- Không có thay đổi ngoài `src/` và `docs/`; không thêm dependency.
- `git status`: thay đổi chưa commit — nên review diff theo module trước khi commit.
