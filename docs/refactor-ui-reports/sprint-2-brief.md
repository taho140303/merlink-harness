# SPRINT 2 — MIGRATION BRIEF (đọc kỹ trước khi sửa)

Mục tiêu: đưa từng module về **Design System v1** (2 tầng surface), sửa a11y cơ bản. **Không đổi nội dung chữ, không đổi logic.**

## 0. Đọc trước (theo thứ tự)

1. `docs/design-system.md` — chuẩn duy nhất.
2. `src/app/app.css` — token + `.tier-panel`, `.tier-card`, `.tier-input`, `.tier-overlay`, `.focus-ring`.
3. `src/shared/components/ui/` — `button.tsx`, `input.tsx`, `textarea.tsx`, `select.tsx`, `field.tsx`, `tabs.tsx`, `popover.tsx`, `tooltip.tsx`, `card.tsx`, `badge.tsx`.
4. Module của bạn (đọc 100% file trong scope).

## 1. Luật cứng

- **KHÔNG** sửa: `src/shared/**`, `src/app/app.css`, `src/shared/i18n/dict.ts`, `package.json`, `src/app/routes/config.tsx`, `src/app/routes/worldbuilding.tsx`, mọi file `mock*.ts`.
- **KHÔNG** thêm dependency, không tạo primitive mới, không đổi tên export/props public.
- **KHÔNG** chạy `npm run build` (nhiều agent chạy song song sẽ ghi đè `build/`). Chỉ chạy `npx tsc --noEmit`.
- **KHÔNG** dịch hay sửa chuỗi hiển thị. Chuỗi hardcode: **ghi vào report**, không sửa.
- Giữ nguyên hành vi: state, handler, điều kiện render, mock data.
- Chỉ sửa file trong scope của bạn.

## 2. Bảng ánh xạ bắt buộc

| Cũ | Mới |
|:--|:--|
| `text-[9px]`, `text-[10px]`, `text-[11px]`, `text-[13px]` | `text-xs` |
| `text-[14px]`, `text-[15px]` | `text-sm` |
| `text-slate-900 dark:text-slate-100` | `text-fg` |
| `text-slate-600/700 dark:text-slate-300` | `text-fg-muted` |
| `text-slate-400/500 dark:text-slate-400/500` | `text-fg-subtle` (phụ) / `text-fg-muted` (chính) |
| `border-slate-200 dark:border-slate-800` | `border-line` |
| `border-slate-300 dark:border-slate-700` | `border-line-strong` |
| `bg-white dark:bg-slate-900` (surface) | `.tier-card` hoặc `bg-surface-card` |
| `bg-slate-50 dark:bg-slate-950` (input) | `.tier-input` hoặc `bg-surface-input` |
| card: `rounded-2xl border ... shadow-xs` | `tier-card rounded-card` |
| section: `rounded-3xl border ... shadow-xl` | `tier-panel rounded-panel` |
| `rounded-[2rem]` / `rounded-[2.5rem]` | `rounded-shell-sm` / `rounded-shell` |
| `shadow-xs/sm/md/lg/xl/2xl` trên card | bỏ hẳn |
| `shadow-*` trên overlay | `tier-overlay` |
| `focus:outline-hidden` / `focus:outline-none` trần | thêm `focus-ring` (giữ class cũ cũng được) |
| `transition-all` | `transition-colors` / `transition-transform` |
| `duration-600`, `duration-75` (enter) | `duration-200` / `duration-150` |

**Ngoại lệ:** màu status (`rose/emerald/amber/blue`) giữ nguyên, được giữ `dark:` tương ứng. Gradient trang trí (aurora, orb, blob) giữ nguyên.

## 3. Ví dụ chuẩn

### Card
```tsx
// TRƯỚC
<div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
// SAU
<div className="tier-card rounded-card p-4">
// hoặc
<Card padding="md">…</Card>
```

### Section panel
```tsx
// TRƯỚC
<section className="rounded-3xl border border-slate-200/80 bg-white/60 p-5 shadow-xl dark:border-slate-800/80 dark:bg-slate-900/60">
// SAU
<section className="tier-panel rounded-panel p-5">
```

### Input
```tsx
// TRƯỚC
<input className="h-9 w-full rounded-xl border border-slate-200 bg-white px-3 text-xs text-slate-900 focus:outline-hidden" />
// SAU
<Input label={t('…')} className="h-9" />
// hoặc nếu cần giữ layout đặc biệt:
<input className="tier-input focus-ring h-9 w-full rounded-xl px-3 text-xs text-fg" />
```

### Icon button
```tsx
// TRƯỚC
<button className="rounded-lg p-1 text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800"><X className="h-4 w-4" /></button>
// SAU
<Button variant="ghost" size="icon" aria-label={t('common.close')}><X className="h-4 w-4" aria-hidden="true" /></Button>
```

### Tab bar
Dùng `<Tabs items={[{id,label,icon?}]} value={tab} onChange={setTab} ariaLabel="…" variant="underline|pill" />`
KHÔNG tự dựng `<div>` tab + state nữa.

### Popover (thay 3 popover chat)
```tsx
<Popover label={t('chat.model')} trigger={<Cpu className="h-4 w-4" />} align="end">
  …nội dung…
</Popover>
```

### Splitter (chat)
`role="separator"` + `tabIndex={0}` + `aria-valuenow={Math.round(leftRatio*100)}` + `aria-valuemin/max` + `aria-orientation="vertical"` + `onKeyDown` (ArrowLeft/Right đổi ratio 5%).

## 4. A11y tối thiểu phải đạt

- Mọi `button`/`a`/`input`/`select`/`textarea` có `focus-ring`.
- Icon-only: `aria-label` (dùng `t()` nếu key đã có; nếu chưa có key → dùng nhãn tiếng Anh tạm và ghi vào report).
- Tab bar: dùng `<Tabs>` (đã có role/aria/keyboard).
- Dialog: đảm bảo có `DialogTitle`; nếu thiếu `DialogDescription` → thêm `className="sr-only"`.
- Toggle/switch: `role="switch"` + `aria-checked`.
- Slider: `aria-label` + `aria-valuetext`.
- Progress: `role="progressbar"` + `aria-valuenow={0-100}` + `aria-valuemin/max`.
- Splitter: như §3.
- Ảnh: `alt` có nghĩa.

## 5. Quy trình

1. Inventory: liệt kê file + số dòng, grep các pattern cấm trước khi sửa (ghi lại counts).
2. Migrate từng file, ưu tiên file nhiều vi phạm.
3. `npx tsc --noEmit` → phải 0 lỗi.
4. Grep lại counts sau khi sửa.
5. Viết report `docs/refactor-ui-reports/sprint2-<agent>.md`:
   - Bảng file → thay đổi chính.
   - Counts trước/sau cho: `text-[`, `rounded-[`, `shadow-`, `bg-white`, `border-slate-`, `focus:outline-hidden`, `neu-`.
   - Danh sách chuỗi hardcode còn lại (file:line + chuỗi + key đề xuất) — **không sửa**.
   - Primitive gap / lỗi primitive phát hiện (không tự sửa).
   - Output `npx tsc --noEmit`.
6. Trả HANDOFF ngắn.

## 6. Acceptance (bắt buộc)

- [ ] `npx tsc --noEmit` = 0 lỗi.
- [ ] Trong scope: `text-[...px]` = 0; `rounded-[` = 0; `shadow-*` = 0 (trừ `tier-overlay`); `focus:outline-hidden` trần = 0.
- [ ] Trong scope: không còn `bg-white`/`border-slate-*`/`text-slate-*` cho surface (status color được phép).
- [ ] `neu-*` = 0 (trừ `ProjectCard` ở scope G2).
- [ ] 100% icon-only button có `aria-label`; 100% interactive có `focus-ring`.
- [ ] Không file ngoài scope bị sửa (`git status`).
- [ ] Report đã ghi, có counts trước/sau.
