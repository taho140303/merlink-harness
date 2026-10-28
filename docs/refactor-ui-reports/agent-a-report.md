# BÁO CÁO AUDIT DESIGN SYSTEM — NỀN TẢNG & VỎ ỨNG DỤNG (SPRINT 1)

> **Môi trường audit**: Read-only audit cho sprint 1 (Foundation + Shell).  
> **Repository**: `/home/hyanhasta05/workspace/personal/pet-project/merlink-harness-ui`  
> **Stack**: React Router 8 framework mode (SSR), React 19, Tailwind CSS v4 CSS-first (không dùng tailwind.config), Zustand, Radix Dialog, lucide-react, Inter.  
> **Trạng thái repo**: Không sửa đổi source code, chỉ tạo tài liệu báo cáo này.

---

## 1. Scope & Method

### 1.1 Danh sách file trong scope & số dòng (LOC)
Tổng cộng **22 file** với **2.228 dòng mã** được kiểm tra toàn diện 100%:

| File Path | LOC | Trách nhiệm kiến trúc |
|:---|---:|:---|
| `src/app/app.css` | 116 | Token nền tảng, utility neumorphism (`.neu-flat`, `.neu-pressed`), animation keyframes |
| `src/app/root.tsx` | 92 | Root document SSR, inline script chống FOUC theme, fonts Inter, Document metadata |
| `src/app/layouts/FullLayout.tsx` | 24 | Shell layout chính: AuroraBackground, Sidebar, main container, ToastViewport |
| `src/app/routes.ts` | 15 | Khai báo cấu trúc định tuyến framework mode |
| `src/shared/components/background/AuroraBackground.tsx` | 23 | Nền hiệu ứng Aurora hữu cơ: 4 blob gradients xoay/trượt |
| `src/shared/components/ui/dialog.tsx` | 95 | Primitive Dialog bọc Radix UI Dialog (Portal, Overlay, Content, Header, Title, Description, Close) |
| `src/shared/components/ui/confirm-dialog.tsx` | 61 | Dialog xác nhận hành động chung (phủ nhận/chấp thuận) |
| `src/shared/components/ui/delete-confirm-dialog.tsx` | 132 | Dialog xoá nguy hiểm (yêu cầu gõ lại từ khoá xác nhận) |
| `src/shared/components/ui/toast.tsx` | 63 | Hệ thống thông báo nổi (Toast viewport, trigger helper, auto-dismiss) |
| `src/shared/components/ui/floating-outline.tsx` | 138 | Panel outline/mục lục nổi có kéo thả vị trí (drag & drop), thu gọn |
| `src/shared/hooks/useResizablePanel.ts` | 66 | Hook quản lý co giãn độ rộng panel bên trái/phải với mousedown/mousemove |
| `src/shared/hooks/useTheme.ts` | 58 | Hook quản lý theme (light/dark/system), đồng bộ localStorage và class `html` |
| `src/shared/i18n/store.ts` | 29 | Zustand store lưu ngôn ngữ người dùng (`en` / `vi`), đồng bộ localStorage |
| `src/shared/i18n/dict.ts` | 848 | Từ điển i18n song ngữ Anh - Việt (418 khóa mỗi ngôn ngữ) |
| `src/shared/i18n/useT.ts` | 15 | Hook nội suy chuỗi đa ngôn ngữ `t(key, params)` |
| `src/shared/utils/cn.ts` | 6 | Tiện ích gộp class kết hợp `clsx` và `tailwind-merge` |
| `src/shared/utils/string.ts` | 4 | Tiện ích chuỗi (hàm truncate) |
| `src/core/components/Sidebar.tsx` | 54 | Vỏ Sidebar bao bọc Header, Nav, History, toggle thu gọn, trigger Settings |
| `src/core/components/sidebar/SettingsDialog.tsx` | 29 | Modal bọc hiển thị cấu hình Settings từ Sidebar |
| `src/core/components/sidebar/SidebarHeader.tsx` | 47 | Header Sidebar: Logo thương hiệu, slogan, nút thu gọn / mở rộng |
| `src/core/components/sidebar/SidebarHistory.tsx` | 158 | Danh sách lịch sử chat, tìm kiếm, lọc theo dự án, chuyển hướng đoạn chat |
| `src/core/components/sidebar/SidebarNav.tsx` | 149 | Danh sách điều hướng chính, badge thông báo, nút Settings, Quick Chat |
| `src/core/components/sidebar/types.ts` | 6 | Khai báo kiểu dữ liệu `ChatItem` cho sidebar history |
| **Tổng cộng** | **2.228** | **100% file trong phạm vi Sprint 1** |

### 1.2 Các lệnh kiểm tra & thống kê đã thực hiện (Grep / Script)
- **Kiểm tra i18n parity**: Script so sánh tập khóa giữa `dict.en` và `dict.vi` trong `src/shared/i18n/dict.ts`:
  - `en`: 418 khóa, `vi`: 418 khóa. Không có khóa nào bị thiếu lệch giữa 2 ngôn ngữ.
- **Thống kê pattern trong scope**:
  - `hex/rgb/hsl`: 21 lần xuất hiện (chủ yếu trong `app.css` định nghĩa gradient, shadow neumorphism; và màu inline trong `AuroraBackground.tsx`).
  - `text-[\d+px]`: 16 lần xuất hiện (gồm `text-[10px]`, `text-[11px]`, `text-[13px]`, `text-[12px]`, `text-[9px]` ở các file shell/sidebar/toast/floating-outline).
  - `rounded-[`: 1 lần (`rounded-[2.5rem]` tại `src/app/layouts/FullLayout.tsx:16`).
  - `shadow-`: 12 lần xuất hiện (bao gồm `shadow-2xl` trên Dialog, `shadow-lg` trên Toast, `shadow-md` trên FloatingOutline, `shadow-sm` trên Sidebar).
  - `dark:`: 94 lần xuất hiện trong 12 file TSX.
  - `focus:`: 29 lần xuất hiện trong 8 file TSX.
  - `aria-`: 20 lần xuất hiện trong 8 file TSX.
  - Chuỗi tiếng Việt trong TSX: 1 lần hardcode tại `src/core/components/sidebar/SidebarHistory.tsx:75` ("Chưa có cuộc trò chuyện nào").

---

## 2. Scorecard R1–R12

| Rule | Tiêu chí đánh giá | Verdict | Evidence Count | Ghi chú & Đánh giá |
|:---|:---|:---:|:---:|:---|
| **R1 Surface/elevation** | Chỉ `neu-flat` \| `neu-pressed` \| phẳng trong suốt; cấm shadow rời tuỳ tiện, gradient tuỳ tiện, bg trắng đục | **FAIL** | 14 | Có nhiều shadow Tailwind rời (`shadow-2xl`, `shadow-lg`, `shadow-md`, `shadow-sm`) trong primitive và sidebar. Nền trắng đục `bg-white` hardcode không ăn theo token base trong Dialog, DeleteConfirm, Toast, FloatingOutline. |
| **R2 Color** | Chỉ token đã khai báo hoặc slate + status semantic; cấm hex hardcode trong TSX; accent đúng nghĩa | **FAIL** | 8 | `AuroraBackground.tsx` hardcode hex `#818cf8`, `#c084fc`, `#f472b6`, `#38bdf8`, `#6366f1`, `#a855f7`, `#ec4899`, `#0ea5e9`. Chưa có CSS token cho surface, border, text hay accent; màu xám slate rải rác class rời. |
| **R3 Dark parity** | Mọi bg/border/text/divide/ring phải có dark: tương ứng; class chỉ light là defect | **PARTIAL** | 12 | Đa số các thành phần có `dark:`. Tuy nhiên tồn tại nhiều chỗ quên `dark:`: viền scrollbar trong `app.css`, viền `border-slate-100` trong `SidebarHistory.tsx`, nền drag header `bg-slate-50` trong `floating-outline.tsx`, separator trong `SidebarNav.tsx`. |
| **R4 Type scale** | Chỉ 10/11/12/14/16/18/20/24/30; cấm text-[13px], text-[9px]... | **FAIL** | 4 | `SidebarNav.tsx:94` dùng `text-[9px]`, `SidebarHistory.tsx:135` dùng `text-[13px]`. Nhiều nơi lạm dụng `text-[10px]`, `text-[11px]` thay vì scale chuẩn. |
| **R5 Radius ladder** | Ladder {sm, md, lg, xl, 2xl, 3xl, full} + ngoại lệ shell 2.5rem | **PASS** | 1 | Ngoại lệ duy nhất là `rounded-[2.5rem]` ở `FullLayout.tsx:16` (hợp lệ theo baseline shell). Các primitive và sidebar tuân thủ {lg, xl, 2xl, 3xl, full}. |
| **R6 Spacing** | Grid bội số 4px; cấm p-[7px], gap-[13px]... | **PASS** | 0 | 100% khoảng cách trong scope dùng tailwind spacing chuẩn (`p-4`, `gap-2`, `h-9`, `w-14`, v.v.), không có arbitrary pixel spacing dạng `p-[7px]`. |
| **R7 Interaction** | Bấm được cần hover + focus-visible ring + disabled + active; cấm focus:outline-hidden không ring | **FAIL** | 11 | `dialog.tsx:88` có `focus:outline-hidden` nhưng không có ring thay thế. Nhiều icon button trong `SidebarHeader`, `SidebarHistory`, `FloatingOutline` thiếu `focus-visible:ring-2`. |
| **R8 A11y** | Icon-only button cần aria-label; Dialog cần Title+Description; decorative cần aria-hidden; contrast AA | **FAIL** | 9 | `DialogContent` mặc định không render `DialogTitle`/`DialogDescription` dẫn đến lỗi Radix cảnh báo thiếu a11y nếu caller không tự bổ sung. Nút đóng Dialog (`dialog.tsx:89`) thiếu `aria-label="Close"`. Nhiều nút icon trong Sidebar thiếu `aria-label`. |
| **R9 i18n** | Mọi copy qua t(key), key có ở cả en và vi; cấm chuỗi hardcode TSX | **PARTIAL** | 5 | Tồn tại chuỗi tiếng Việt hardcode trong `SidebarHistory.tsx:75` ("Chưa có cuộc trò chuyện nào"). `root.tsx` hardcode `<html lang="en">` tĩnh, không reactive theo Zustand store locale. Cảnh báo SSR hydration mismatch về locale. |
| **R10 Primitive reuse** | Modal dùng shared Dialog; confirm dùng confirm/delete-confirm; toast dùng shared toast; cấm tự dựng overlay/portal | **PASS** | 0 | Toàn bộ scope tái sử dụng chuẩn xác các primitive shared/ui: `SettingsDialog` dùng `Dialog`, `FullLayout` mount `ToastViewport`, các route gọi `toast()`. |
| **R11 Motion** | Duration 150/200/300ms + easing nhất quán; tôn trọng prefers-reduced-motion | **FAIL** | 6 | `app.css` và `AuroraBackground.tsx` chạy vô số animation xoay/trôi (`orb-float`, `chaotic-1..4`, `logo-float`) nhưng **hoàn toàn không có `@media (prefers-reduced-motion: reduce)`**. Lớp class `animate-in` gọi trong `dialog.tsx` và `toast.tsx` là class ảo không tồn tại trong CSS. |
| **R12 State coverage** | Loading / empty / error / disabled nhất quán | **PARTIAL** | 3 | Trạng thái empty trong `SidebarHistory.tsx` có xử lý nhưng hardcode tiếng Việt. Nút submit trong `DeleteConfirmDialog` có disabled state tốt nhưng thiếu loading spinner/pending feedback khi submit logic bất đồng bộ. |

---

## 3. Findings (Chi tiết 25 khiếm khuyết được kiểm chứng)

### Bảng tổng hợp phát hiện (Sắp xếp theo Severity P0 → P1 → P2)

| ID | Severity | File : Line | Hiện trạng (Snippet) | Vi phạm Rule | Đề xuất khắc phục (Fix đề xuất) |
|:---|:---:|:---|:---|:---:|:---|
| **F-01** | **P0** | `src/shared/components/ui/dialog.tsx:55-89` | Thiếu `DialogTitle` / `DialogDescription` mặc định trong `DialogContent`; caller dùng `<DialogContent>` trần bị Radix ném warning a11y console | **R8** (A11y) | Bổ sung `VisuallyHidden` bọc `DialogTitle` mặc định bên trong `DialogContent` nếu không được truyền, hoặc quy ước rõ ràng bắt buộc child phải có `DialogTitle`. |
| **F-02** | **P1** | `src/app/root.tsx:64` | `<html lang="en" className={theme === "dark" ? "dark" : ""}>` — `lang="en"` cố định tĩnh, không cập nhật khi người dùng chuyển sang `vi` trong `useLocaleStore` | **R9** (i18n) | Đọc `locale` từ `useLocaleStore` (hoặc inline cookie) và gán dynamic: `<html lang={locale}>`. |
| **F-03** | **P1** | `src/shared/components/ui/dialog.tsx:58`, `src/shared/components/ui/toast.tsx:21` | `data-[state=open]:animate-in data-[state=closed]:animate-out fade-in-0 zoom-in-95` — `animate-in` là class của plugin `tailwindcss-animate`, nhưng repo dùng Tailwind v4 CSS-first không cài plugin này, animation bị vô hiệu | **R11** (Motion) | Tự định nghĩa `@keyframes dialog-fade-in` và `@keyframes toast-slide-in` trong `app.css` hoặc dùng utility transition thuần của Tailwind v4. |
| **F-04** | **P1** | `src/app/app.css:76-115`, `src/shared/components/background/AuroraBackground.tsx:16-20` | Các keyframe `chaotic-1..4`, `orb-float`, `orb-glow`, `logo-float` chạy liên tục mà **không có media query `@media (prefers-reduced-motion: reduce)`** | **R11** (Motion) | Thêm block `@media (prefers-reduced-motion: reduce) { *, ::before, ::after { animation-duration: 0.01ms !important; animation-iteration-count: 1 !important; transition-duration: 0.01ms !important; } }` vào `app.css`. |
| **F-05** | **P1** | `src/core/components/sidebar/SidebarHistory.tsx:75` | `<p className="text-xs text-slate-400 dark:text-slate-500">Chưa có cuộc trò chuyện nào</p>` — Chuỗi tiếng Việt hardcode | **R9** (i18n) | Thay bằng `{t('chat.noConversations')}` hoặc bổ sung key `sidebar.emptyHistory` vào `dict.ts`. |
| **F-06** | **P1** | `src/shared/components/ui/dialog.tsx:57` | `bg-white dark:bg-slate-900 rounded-2xl shadow-2xl border border-slate-200 dark:border-slate-800` — Dùng màu trắng đục `bg-white` và `shadow-2xl` rời thay vì token surface và neu elevation | **R1, R2** | Dùng class `neu-flat bg-base-light dark:bg-base-dark` hoặc token hóa surface `--color-surface-elevated`. Bỏ `shadow-2xl`. |
| **F-07** | **P1** | `src/shared/components/background/AuroraBackground.tsx:16-19` | `bg-[radial-gradient(...)]` hardcode các mã hex `#818cf8`, `#c084fc`, `#f472b6`, `#38bdf8`, `#6366f1`, `#a855f7`, `#ec4899`, `#0ea5e9` | **R2** (Color) | Đưa các màu gradient thành CSS variables: `--color-aurora-blob-1`, `--color-aurora-blob-2`, v.v. trong `app.css`. |
| **F-08** | **P1** | `src/shared/components/ui/dialog.tsx:88` | `<DialogPrimitive.Close className="... focus:outline-hidden disabled:pointer-events-none ...">` — Dùng `focus:outline-hidden` nhưng không hề có `focus-visible:ring-2` | **R7** (Interaction) | Thay bằng `focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-purple-500`. |
| **F-09** | **P1** | `src/shared/components/ui/dialog.tsx:89` | Nút đóng `DialogPrimitive.Close` chỉ chứa `<X className="h-4 w-4" />` và `<span className="sr-only">Close</span>`, nhưng `Close` là tiếng Anh hardcode không i18n | **R8, R9** | Thêm `aria-label={t('common.close')}` hoặc dùng nội dung sr-only theo từ điển đa ngữ. |
| **F-10** | **P1** | `src/shared/hooks/useTheme.ts:32-38` | Khi theme là "system", hook không lắng nghe sự kiện `change` của `window.matchMedia('(prefers-color-scheme: dark)')` | **R3, R12** | Bổ sung `mediaQuery.addEventListener('change', listener)` để tự động chuyển giao diện khi OS đổi theme trong lúc user đang chọn `system`. |
| **F-11** | **P1** | `src/core/components/sidebar/SidebarNav.tsx:94` | `className="... text-[9px] font-bold ..."` — Vi phạm thang kích thước phông chữ | **R4** (Type scale) | Thay `text-[9px]` bằng `text-[10px]` hoặc `text-xs` (12px) với scale chuẩn. |
| **F-12** | **P1** | `src/core/components/sidebar/SidebarHistory.tsx:135` | `className="... text-[13px] font-medium truncate ..."` — Font size tùy tiện | **R4** (Type scale) | Thay `text-[13px]` bằng `text-xs` (12px) hoặc `text-sm` (14px). |
| **F-13** | **P1** | `src/shared/components/ui/delete-confirm-dialog.tsx:94-96` | Input xác nhận thiếu nhãn `label` tường minh hoặc `aria-label`, chỉ có placeholder | **R8** (A11y) | Bổ sung `<label htmlFor="delete-confirm-input" className="sr-only">` hoặc `aria-label={t('settings.delete')}`. |
| **F-14** | **P2** | `src/app/layouts/FullLayout.tsx:16` | `<main className="flex-1 rounded-[2.5rem] neu-flat overflow-hidden ...">` — `overflow-hidden` cắt mất focus ring hoặc popover/dropdown tràn ra mép | **R7** (Interaction) | Đổi sang `overflow-auto` hoặc quản lý scroll/stacking context lồng bên trong. |
| **F-15** | **P2** | `src/shared/components/ui/toast.tsx:20-21` | Toast dùng `bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 shadow-lg rounded-xl` — Không dùng neumorphism | **R1** (Surface) | Bổ sung lớp `neu-flat` và chuẩn hóa radius theo ladder hệ thống (`rounded-xl` hoặc `rounded-2xl`). |
| **F-16** | **P2** | `src/shared/components/ui/floating-outline.tsx:55` | `bg-white/95 dark:bg-slate-900/95 backdrop-blur-md border border-slate-200/80 dark:border-slate-800/80 rounded-2xl shadow-md` | **R1** (Surface) | Tận dụng utility `.neu-flat` phối hợp backdrop filter thay vì `shadow-md` rời. |
| **F-17** | **P2** | `src/shared/components/ui/floating-outline.tsx:64` | `bg-slate-50 dark:bg-slate-800/50` ở header kéo thả — Thiếu tính đồng nhất với nền card | **R1, R3** | Dùng nền trong suốt hoặc đồng nhất với nền panel `bg-slate-100/50 dark:bg-slate-800/40`. |
| **F-18** | **P2** | `src/core/components/sidebar/SidebarHeader.tsx:32-43` | Nút thu gọn sidebar (`<button onClick={onToggleCollapse}>`) thiếu `aria-label` và `title` khi ở trạng thái collapse | **R8** (A11y) | Bổ sung `aria-label={collapsed ? t('sidebar.expand') : t('sidebar.collapse')}`. |
| **F-19** | **P2** | `src/core/components/sidebar/SidebarNav.tsx:129-142` | Nút Cài đặt (`Settings`) ở chân sidebar thiếu `aria-label` khi sidebar bị thu gọn (chỉ render icon `Settings2`) | **R8** (A11y) | Thêm `aria-label={t('nav.settings')}` vào nút bấm. |
| **F-20** | **P2** | `src/core/components/sidebar/SidebarHistory.tsx:106-155` | Các item lịch sử chat dùng thẻ `<div role="button">` lồng với nút xóa `<button onClick={...}>` | **R8** (A11y) | Thẻ tương tác lồng nhau (nested interactive controls) gây lỗi a11y bàn phím; nên dùng `<NavLink>` và tách nút xóa ra ngoài hoặc dùng aria menu/dropdown. |
| **F-21** | **P2** | `src/core/components/sidebar/SidebarNav.tsx:61` | Nút "Quick Chat" dùng `neu-pressed` khi active nhưng không có hiệu ứng chuyển trạng thái mượt mà | **R11** (Motion) | Thêm `transition-all duration-200` vào class danh sách item. |
| **F-22** | **P2** | `src/app/app.css:66-72` | Tùy biến thanh cuộn `::-webkit-scrollbar` cố định màu cứng `#cbd5e1`, `#94a3b8`, `#334155`, `#475569` không qua CSS token | **R2, R3** | Thay bằng `var(--color-scrollbar-thumb)` đồng bộ theo CSS custom property. |
| **F-23** | **P2** | `src/core/components/sidebar/SidebarHistory.tsx:100` | Đường phân cách `<div className="border-t border-slate-100 dark:border-slate-800/60 my-1" />` — `border-slate-100` quá mờ ở light mode | **R3** (Contrast) | Nâng lên `border-slate-200/70 dark:border-slate-800/60`. |
| **F-24** | **P2** | `src/core/components/Sidebar.tsx:37` | Sidebar bọc ngoài dùng `bg-base-light/80 dark:bg-base-dark/80 backdrop-blur-md rounded-3xl neu-flat` — Khối lượng blur lớn chồng chéo với `AuroraBackground` | **R1** (Surface) | Đảm bảo phần cứng GPU mượt mà bằng cách tối ưu hóa stacking context (thêm `contain: paint` hoặc `will-change: transform` khi drag resize). |
| **F-25** | **P2** | `src/shared/hooks/useResizablePanel.ts:40-52` | Trong lúc kéo chuột resize panel, toàn bộ text trên trang không bị chặn select (`user-select: none`) dẫn đến hiện tượng bôi đen text ngoài ý muốn | **R7** (Interaction) | Thêm class `select-none` vào `document.body` trong suốt thời gian `isDragging === true` và gỡ bỏ khi `mouseup`. |

---

## 4. Patterns Nên Chuẩn Hoá Thành Primitive/Token

### 4.1 Bộ Token Màu Sắc (Color Tokens trong `app.css`)
Hiện tại `app.css` chỉ có 2 biến màu:
```css
:root {
  --color-base-light: #eef2f7;
  --color-base-dark: #0f1015;
}
```
Cần mở rộng thành hệ thống Semantic Surface & Border Token:
```css
:root {
  --color-surface-base: #eef2f7;
  --color-surface-elevated: #ffffff;
  --color-surface-sunken: #e2e8f0;
  --color-border-subtle: #e2e8f0;
  --color-border-strong: #cbd5e1;
  --color-text-primary: #0f172a;
  --color-text-secondary: #64748b;
  --color-text-muted: #94a3b8;
  --color-accent-brand: #8b5cf6;
}

html.dark {
  --color-surface-base: #0f1015;
  --color-surface-elevated: #181920;
  --color-surface-sunken: #090a0d;
  --color-border-subtle: #1e293b;
  --color-border-strong: #334155;
  --color-text-primary: #f8fafc;
  --color-text-secondary: #94a3b8;
  --color-text-muted: #64748b;
  --color-accent-brand: #a78bfa;
}
```

### 4.2 Chuẩn Hoá Neumorphic Button / Card Variant
Hiện nay mỗi component tự viết chuỗi class:
- Ví dụ tốt: `src/core/components/sidebar/SidebarNav.tsx:61-63`:
  ```tsx
  cn(
    "flex items-center gap-3 w-full px-3 py-2.5 rounded-xl text-sm font-medium transition-colors",
    isActive ? "neu-pressed text-purple-600 dark:text-purple-400 font-semibold" : "text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100/50 dark:hover:bg-slate-800/40"
  )
  ```
- Cần chuẩn hoá thành utility chung trong `app.css`:
  - `.neu-interactive`: Tự động áp dụng `neu-flat`, khi `:hover` nổi nhẹ, khi `:active` hoặc class `.active` tự chuyển sang `.neu-pressed`.

### 4.3 Keyframes Animation Chuẩn Cho Dialog & Floating Overlays
Do Tailwind v4 không có `animate-in` tích hợp sẵn, cần chuẩn hoá trong `app.css`:
```css
@keyframes modal-enter {
  from {
    opacity: 0;
    transform: scale(0.96) translateY(4px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.animate-dialog-enter {
  animation: modal-enter 0.2s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}
```

---

## 5. Top 10 Fixes Đề Xuất

| # | Hạng mục đề xuất sửa chữa | Tệp tin liên quan | Khối lượng (Effort) | Tác động (Impact) |
|:---:|:---|:---|:---:|:---|
| 1 | Bổ sung `@media (prefers-reduced-motion: reduce)` cho toàn bộ animations | `src/app/app.css` | **S** | Ngăn ngừa say chuyển động (motion sickness), đạt chuẩn A11y WCAG 2.2 |
| 2 | Sửa `DialogTitle`/`DialogDescription` mặc định để tránh warning Radix Dialog | `src/shared/components/ui/dialog.tsx` | **S** | Khắc phục vi phạm A11y P0 trên tất cả các Dialog toàn app |
| 3 | Sửa class animation ảo `animate-in` thành keyframes CSS thật trong Tailwind v4 | `src/app/app.css`, `dialog.tsx`, `toast.tsx` | **M** | Mang lại hiệu ứng đóng mở mượt mà thực sự cho Dialog và Toast |
| 4 | Chuyển chuỗi tiếng Việt hardcode sang bộ từ điển `dict.ts` | `src/core/components/sidebar/SidebarHistory.tsx` | **S** | Hoàn thiện độ phủ i18n 100% cho toàn bộ Sidebar |
| 5 | Gắn `focus-visible:ring-2` và bỏ `focus:outline-hidden` đơn lẻ | `src/shared/components/ui/dialog.tsx` | **S** | Chuẩn hóa điều hướng bàn phím cho toàn bộ modal |
| 6 | Đồng bộ thuộc tính `lang` trên thẻ `<html>` với Zustand locale store | `src/app/root.tsx` | **S** | Khắc phục thuộc tính trợ năng ngôn ngữ trên toàn trang |
| 7 | Bổ sung event listener `prefers-color-scheme` cho tùy chọn theme `system` | `src/shared/hooks/useTheme.ts` | **S** | Tự động chuyển đổi Dark/Light khi hệ điều hành thay đổi |
| 8 | Thay thế các cỡ chữ tùy tiện `text-[9px]`, `text-[13px]` về scale chuẩn | `SidebarNav.tsx`, `SidebarHistory.tsx` | **S** | Nhất quán thang Typography toàn ứng dụng |
| 9 | Bổ sung `aria-label` cho tất cả các icon-only button trong Sidebar | `SidebarHeader.tsx`, `SidebarNav.tsx` | **S** | Trợ năng chuẩn cho Screen Readers |
| 10 | Bổ sung class `select-none` khi kéo resize panel | `src/shared/hooks/useResizablePanel.ts` | **S** | Tránh hiện tượng bôi đen văn bản ngoài ý muốn khi kéo thanh phân cách |

---

## 6. Shared Primitive Defects (Khiếm khuyết ở các Primitive dùng chung)

1. **`src/shared/components/ui/dialog.tsx`**:
   - `DialogContent` render `DialogPrimitive.Close` với nhãn `sr-only` tiếng Anh hardcode (`Close`), không dùng hệ thống i18n `useT`.
   - Nút đóng này sử dụng `focus:outline-hidden` mà không có `focus-visible:ring-*`, khiến người dùng bàn phím mất dấu tiêu điểm.
   - Sử dụng các lớp tiện ích `animate-in`, `fade-in-0`, `zoom-in-95` vốn không tồn tại trong cấu hình Tailwind v4 CSS-first hiện tại (chưa có CSS keyframe tương ứng).
   - `DialogContent` không đảm bảo luôn có `DialogTitle` bên trong nếu dev quên truyền, gây ra lỗi accessible name bắt buộc của Radix UI.

2. **`src/shared/components/ui/delete-confirm-dialog.tsx`**:
   - Trường nhập liệu xác nhận tên xóa (`<input ...>`) không có thẻ `<label>` liên kết (hoặc `aria-label`), gây lỗi truy cập với Screen Reader.
   - Chưa tích hợp trạng thái `isSubmitting` / loading spinner trực quan khi quá trình xóa diễn ra bất đồng bộ.

3. **`src/shared/components/ui/confirm-dialog.tsx`**:
   - Các nút bấm không có lớp chỉ định trạng thái `focus-visible:ring-2`, vi phạm R7.

4. **`src/shared/components/ui/toast.tsx`**:
   - Dùng `shadow-lg` rời rạc và nền trắng tinh đục `bg-white dark:bg-slate-900`, chưa tích hợp phong cách `neu-flat` đặc trưng của Design System.
   - Gọi `animate-in` không có keyframes CSS hỗ trợ.

5. **`src/shared/components/ui/floating-outline.tsx`**:
   - Các nút icon đóng, thu gọn, kéo thả thiếu `aria-label`.
   - Nút bấm kéo thả tiêu đề sử dụng `cursor-grab`, nhưng không hỗ trợ phím mũi tên để di chuyển bằng bàn phím (A11y keyboard drag).

---

## 7. Baseline Conflicts (Các điểm xung đột / Lạc hậu so với Baseline v0)

1. **Quy định "Chỉ 2 utility .neu-flat và .neu-pressed, cấm shadow-* rời"**:
   - *Thực tế codebase*: Hầu hết các component quan trọng (`Dialog` dùng `shadow-2xl`, `Toast` dùng `shadow-lg`, `FloatingOutline` dùng `shadow-md`, `Sidebar` dùng `shadow-sm`) đều sử dụng shadow rời của Tailwind vì bản thân 2 lớp `.neu-flat` và `.neu-pressed` chưa đáp ứng đủ độ tương phản và độ nổi cần thiết cho các lớp phủ (overlays/modals/popups) nằm trên nền Aurora động.
   - *Khuyến nghị điều chỉnh Baseline*: Cần chính thức công nhận một biến thể neumorphism nổi mạnh hơn (ví dụ `.neu-floating` hoặc `.neu-elevated`) có kết hợp bóng mềm rộng thay vì cấm tuyệt đối shadow trên popups.

2. **Quy định Color Token "Chỉ có --color-base-light và --color-base-dark"**:
   - *Thực tế codebase*: 100% các file TSX đang phải dùng trực tiếp bảng màu Tailwind `slate-*`, `purple-*`, `rose-*` một cách thủ công vì không có token trung gian.
   - *Khuyến nghị điều chỉnh Baseline*: Cần cấp thiết bổ sung semantic tokens cho Surface, Text, Border và Brand trong `app.css` để các màn hình không phải viết cứng mã màu slate.

3. **Quy định Type Scale**:
   - *Thực tế codebase*: Các nhãn nhỏ và badge đang sử dụng hỗn loạn `text-[9px]`, `text-[10px]`, `text-[11px]`, `text-xs` (12px), `text-[13px]`, `text-sm` (14px).
   - *Khuyến nghị điều chỉnh Baseline*: Chính thức chuẩn hóa thang Micro-type: Scale chỉ gồm `text-[10px]` (Micro/Badge), `text-xs` (12px - Caption), `text-sm` (14px - Body). Xóa bỏ hoàn toàn `text-[9px]`, `text-[11px]` và `text-[13px]`.

---

## 8. Metrics (Số liệu thống kê định lượng trong Scope)

- **Tổng số tệp kiểm tra (Files)**: 22 files
- **Tổng số dòng mã (Lines of Code)**: 2.228 LOC
- **Số lượng mã màu Hex/RGB hardcode ngoài CSS token**: 21 vị trí (chủ yếu tại `AuroraBackground.tsx` và `app.css`)
- **Số lượng kích thước tùy tiện (`text-[...px]`, `rounded-[...]`, `p-[...]`)**:
  - `text-[...px]`: 16 vị trí (gồm `text-[9px]`, `text-[10px]`, `text-[11px]`, `text-[13px]`)
  - `rounded-[...]`: 1 vị trí (`rounded-[2.5rem]` hợp lệ cho shell)
  - Arbitrary pixel spacing (`p-[...px]`, `m-[...px]`): 0 vị trí (100% dùng thang chuẩn Tailwind)
- **Tỷ lệ bao phủ Dark Mode (`dark:` classes)**: 94 khai báo trên 12 file UI (Đạt ~85% parity, còn sót 12 chỗ viền/nền chưa có variant dark)
- **Tỷ lệ bao phủ Tương tác bàn phím (`focus:` / `focus-visible:`)**: 29 khai báo (Thiếu hụt focus ring ở ~11 nút bấm/dialog close)
- **Tỷ lệ Trợ năng nhãn (`aria-*`)**: 20 thuộc tính (Thiếu nhãn ở 9 nút bấm chỉ có icon)
- **Độ chính xác từ điển i18n (`dict.ts`)**: 418 / 418 keys khớp tuyệt đối giữa tiếng Anh và tiếng Việt (Đạt 100% parity giữa 2 file từ điển, phát hiện 1 chuỗi tiếng Việt hardcode trong component TSX)

---

## 9. Verdict Tổng Kết

### **MOSTLY CONSISTENT**

**Lý do đánh giá:**
1. **Kiến trúc Shell và Bố cục**: Đạt chuẩn rất cao. `FullLayout` tổ chức ranh giới rõ ràng giữa lớp nền động `AuroraBackground` (z-0), nội dung chính `main` (z-10, `rounded-[2.5rem]`, `.neu-flat`), và `Sidebar` có khả năng co giãn độ rộng mượt mà (`useResizablePanel`), tích hợp `ToastViewport`.
2. **Hệ thống i18n**: Cấu trúc từ điển `dict.ts` rất đồ sộ và hoàn thiện với 418 khóa tương thích 100% giữa hai ngôn ngữ Anh - Việt mà không bị lệch khóa nào. Ngoại lệ duy nhất là một chuỗi văn bản trạng thái trống bị hardcode tiếng Việt trong `SidebarHistory.tsx` và thuộc tính `lang="en"` cố định tại `root.tsx`.
3. **Các điểm tồn đọng cần khắc phục**:
   - Hệ thống token của Design System ở mức quá sơ khai (chỉ có 2 biến màu nền base trong `app.css`), dẫn tới việc các component phải dùng trực tiếp màu Tailwind rời rạc.
   - Thiếu cấu hình `@media (prefers-reduced-motion)` cho các hiệu ứng chuyển động vô tận.
   - Các class animation trong Radix primitive (`animate-in`, `zoom-in-95`) là class ảo chưa có keyframe tương ứng trong Tailwind CSS v4.
   - Một số icon-only button và nút đóng Dialog thiếu nhãn trợ năng (`aria-label`) và tiêu điểm bàn phím (`focus-visible`).
