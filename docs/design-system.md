# Merlink UI — Design System (v1)

> Trạng thái: **CHỐT** ngày 2026-10-29. Tài liệu này thay thế mọi quy ước ngầm trước đó.
> Phạm vi áp dụng: toàn bộ `src/`, trừ các màn bị loại trừ ở §8.

---

## 1. Quyết định đã chốt

| # | Quyết định | Hệ quả |
|:--|:--|:--|
| 1 | **Mô hình surface 2 tầng** | Tầng 1 (shell/overlay) = neumorphism. Tầng 2 (nội dung dữ liệu) = flat-bordered glass. Không trộn lẫn. |
| 2 | **Type scale tối thiểu 12px** | Bỏ toàn bộ `text-[9px]`, `text-[10px]`, `text-[11px]`, `text-[13px]`, `text-[14px]`, `text-[15px]`. Nhỏ nhất là `text-xs` (12px). |
| 3 | **i18n cho UI chrome 100%** | Mock data / fixture là **dữ liệu**, không dịch, không đụng tới. |
| 4 | **Loại trừ `/config` và `/worldbuilding`** | Không refactor 2 route này (vẫn là mock). Không tính vào Definition of Done. |

---

## 2. Hai tầng surface

### Tầng 1 — Shell (neumorphic)

Dùng cho: `FullLayout`, `Sidebar` và **duy nhất** các overlay (Dialog/Toast/Popover/Tooltip). `ProjectCard` là ngoại lệ được giữ `neu-flat` vì là card nổi ngoài cùng.

- Utility: `.neu-flat` (raised), `.neu-pressed` (inset).
- Radius: `rounded-shell` (2.5rem) cho main; `rounded-shell-sm` (2rem) cho sidebar/pane.
- **Không** dùng `shadow-*`, không `bg-white` đục, không gradient tự phát.
- Overlay (dialog/popover/tooltip) được phép có bóng **thông qua token duy nhất** `.tier-overlay` (shadow mềm), không dùng `shadow-xl/2xl` rải rác.

### Tầng 2 — Nội dung (flat-bordered)

Dùng cho: project detail, chapter, character list/detail, settings, system-prompt, các card/block trong draft, bảng, form.

- `.tier-panel` — panel lớn: nền `surface-panel` 82% + `border-line` + blur 16.
- `.tier-card` — card nội dung: `surface-card` + `border-line` + radius `rounded-card`.
- `.tier-input` — input/select/textarea: `surface-input` + `border-line`.
- **Không** dùng `neu-flat`/`neu-pressed` cho nội dung; **không** `shadow-*` (trừ token overlay).
- Card lồng nhau tối đa 2 cấp hiển thị: panel → card. Cấp 3 dùng divider, không thêm viền.

### Bảng phân vai

| Thành phần | Tầng | Class nền |
|:--|:--|:--|
| App shell, sidebar | 1 | `.neu-flat` / `.neu-pressed` |
| Panel nội dung, section | 2 | `.tier-panel` |
| Card, block, list row | 2 | `.tier-card` |
| Input, select, textarea | 2 | `.tier-input` |
| Dialog, popover, tooltip, toast | overlay | `.tier-overlay` |

---

## 3. Token

Khai báo trong `src/app/app.css` (`@theme` + override `html.dark`). **Token tự đổi theo theme — không viết `dark:` cho màu đã có token.**

| Token | Light | Dark | Utility |
|:--|:--|:--|:--|
| `--color-surface-panel` | `#ffffff` | `#12151c` | `bg-surface-panel` |
| `--color-surface-card` | `#ffffff` | `#151922` | `bg-surface-card` |
| `--color-surface-input` | `#f8fafc` | `#0f131a` | `bg-surface-input` |
| `--color-surface-muted` | `#f1f5f9` | `#1a1f29` | `bg-surface-muted` |
| `--color-fg` | `#0f172a` | `#f1f5f9` | `text-fg` |
| `--color-fg-muted` | `#475569` | `#cbd5e1` | `text-fg-muted` |
| `--color-fg-subtle` | `#94a3b8` | `#94a3b8` | `text-fg-subtle` |
| `--color-line` | `#e2e8f0` | `#232a36` | `border-line` |
| `--color-line-strong` | `#cbd5e1` | `#334155` | `border-line-strong` |
| `--color-accent` | `#8b5cf6` | `#a78bfa` | `bg-accent` / `text-accent` |
| `--color-accent-soft` | `#f5f3ff` | `#1e1b2e` | `bg-accent-soft` |

Radius: `rounded-shell` 2.5rem · `rounded-shell-sm` 2rem · `rounded-panel` 1.5rem · `rounded-card` 1rem · còn lại dùng thang mặc định (`rounded-lg/xl/full`).

**Cấm:** hex/rgb/hsl trong TSX; `bg-white`, `bg-slate-*` cho surface; `border-slate-*` cho viền; `shadow-*` ngoài token; `dark:` cho màu đã token hoá.

---

## 4. Typography

| Vai trò | Class | Ghi chú |
|:--|:--|:--|
| Micro label / badge | `text-xs` (12px) | thay toàn bộ 9/10/11px |
| Body / control | `text-sm` (14px) | mặc định cho UI |
| Prose đọc | `text-base` (16px) `leading-relaxed` | chapter, markdown |
| Section title | `text-lg` (18px) `font-semibold tracking-tight` | |
| Page title | `text-xl` (20px) | |
| Hero | `text-2xl`+ | |

Mapping tự động khi migrate: `[9px]→xs`, `[10px]→xs`, `[11px]→xs`, `[13px]→xs`, `[14px]→sm`, `[15px]→sm`.
Line-height: body `leading-relaxed`; label `leading-none`/`leading-tight`. Tracking: chỉ `tracking-tight` (title) và `tracking-wide` (uppercase label).

---

## 5. Interaction & a11y (bắt buộc)

- Mọi phần tử bấm được: `class="focus-ring"` (outline 2px accent, offset 2). **Cấm** `focus:outline-hidden`/`focus:outline-none` trần.
- Icon-only button: `aria-label` + `title` (tooltip) hoặc dùng `<Tooltip>`.
- Tabs/stepper: `role="tablist"`, `role="tab"`, `aria-selected`, `aria-controls`, phím ←/→/Home/End.
- Dialog: luôn có `DialogTitle`; `DialogDescription` hoặc `sr-only`.
- Ẩn trang trí: `aria-hidden="true"`; progress: `role="progressbar"` + `aria-valuenow/min/max`.
- Splitter: `role="separator"` + `tabIndex=0` + `aria-valuenow` + phím mũi tên.
- Màu chữ phụ trên nền phải đạt AA (4.5:1); dùng `text-fg-muted`, không dùng `slate-400` trên nền sáng.

## 6. Motion

- Duration: `150` (hover), `200` (enter), `300` (panel/accordion). Không dùng `duration-600`, `duration-75` cho enter.
- Chỉ animate `opacity`/`transform`/`colors`; tránh `transition-all` cho layout.
- `app.css` đã có block `prefers-reduced-motion` toàn cục — không cần thêm `motion-reduce:*` từng chỗ, nhưng không được vô hiệu hoá block đó.

## 7. Primitive chính thức (`src/shared/components/ui/`)

| Primitive | API chính |
|:--|:--|
| `Button` | `variant: primary \| secondary \| ghost \| danger`, `size: sm \| md \| icon` |
| `Input` / `Textarea` / `Select` | `label?`, `hint?`, `error?`, `invalid?`; tự sinh `id`, `aria-describedby`, `aria-invalid` |
| `Field` | wrapper label/hint/error dùng chung |
| `Tabs` | `items`, `value`, `onChange`, `variant: underline \| pill`, `ariaLabel` |
| `Popover` | `trigger`, `label`, `align`, `side`; portal + Esc + click-outside + trả focus |
| `Tooltip` | `content`, `side`; hover + focus, `aria-describedby` |
| `Card` | `tier: panel \| card`, `padding: none \| sm \| md \| lg` |
| `Badge` | `variant: neutral \| accent \| success \| warning \| danger` |
| `Checkbox` | native input + label/hint, `aria-invalid` |
| `Switch` | `role="switch"` + `aria-checked` |
| `Segmented` | `role="radiogroup"` + keyboard ←/→ |
| `Dialog` (có sẵn) | luôn kèm `DialogTitle`; `ConfirmDialog`/`DeleteConfirmDialog` đã bọc sẵn |

Cấm tạo mới overlay/popover/dropdown/confirm ngoài các primitive này.

## 7.1 Responsive

- `< 1280px`: split-pane (chat/draft, character chat, project chat) chuyển sang **1 pane + overlay**; splitter ẩn. Hook: `useIsCompact()`.
- `>= 1280px`: giữ 2 cột, min-width theo từng màn (chat 545/530, character/project 340+).

## 8. Loại trừ

- `src/app/routes/config.tsx`, `src/app/routes/worldbuilding.tsx` và component chỉ chúng dùng: **không sửa**.
- `src/core/components/character/CharacterDetailView.tsx`: **dead code**, xoá ở Phase 0.
- Mock/fixture (`mock*.ts`, `dict.ts` values là dữ liệu mẫu): giữ nguyên nội dung, không dịch.
- `AuroraBackground.tsx`, `OrbSphere.tsx`: tác phẩm trang trí (gradient/blob/highlight), được phép dùng palette/hex, không token hoá.
- `CreateCharacterDialog` các giá trị `hairHex/eyeHex/skinHex` là **dữ liệu màu do người dùng chọn**, không phải CSS.

## 9. Definition of Done (mỗi module)

1. 0 `neu-*` ở tầng nội dung; 0 `shadow-*` ngoài `.tier-overlay`.
2. 0 `text-[...px]`; 0 `[Npx]` spacing lẻ; 0 `rounded-[...]`.
3. 0 `bg-white` / `border-slate-*` / `dark:*` cho surface (đã token hoá).
4. 100% interactive có `focus-ring`; icon-only có `aria-label`.
5. 100% UI chrome qua `t()`; không thêm key trùng; `tsc` xanh.
6. `grep -c "focus:outline-hidden"` = 0 trong module.
