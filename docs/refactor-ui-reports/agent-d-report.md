# BÁO CÁO AUDIT DESIGN SYSTEM — SPRINT 4: CHARACTER (LIST + DETAIL)

> **Căn cứ đánh giá**: Baseline v0 (Neumorphism + Glass, Tailwind CSS v4 CSS-first, không tailwind.config, React Router 8 SSR, Dark Mode `.dark` trên `<html>`, neutral slate, token `--color-base-light: #eef2f7`, `--color-base-dark: #0f1015`, 12 design rules R1–R12).  
> **Chế độ**: Read-Only Audit (không sửa/xoá mã nguồn, không chạy server/build).  
> **Target file report**: `docs/refactor-ui-reports/agent-d-report.md`.

---

## 1. Scope & Method

### Danh sách file audit và số dòng (LOC)
Tổng cộng **23 files** (2.553 LOC code TSX/TS) thuộc scope được đọc và rà soát 100%:

| STT | Đường dẫn file | LOC | Mục đích & Trách nhiệm |
|---|---|---|---|
| 1 | `src/app/routes/character.tsx` | 13 | Route `/character` nạp `CharacterListView` |
| 2 | `src/app/routes/character-detail.tsx` | 7 | Route `/character/:characterId` nạp `CharacterDetailLayout` |
| 3 | `src/core/components/character/CharacterCard.tsx` | 98 | Card nhân vật danh sách (ảnh bìa, avatar, status badge, tags) |
| 4 | `src/core/components/character/CharacterChatPane.tsx` | 166 | Panel test chat nhanh nhân vật (message bubbles, input box) |
| 5 | `src/core/components/character/CharacterDetailView.tsx` | 184 | Legacy view chi tiết 4 tab (`overview`, `attributes`, `background`, `stats`) |
| 6 | `src/core/components/character/CharacterListView.tsx` | 139 | Trang danh sách nhân vật (search, filter category/status, grid card) |
| 7 | `src/core/components/character/CreateCharacterDialog.tsx` | 196 | Modal tạo nhân vật mới (form input, category, traits tag) |
| 8 | `src/core/components/character/mockCharacters.ts` | 69 | Dữ liệu mock nhân vật cho view danh sách |
| 9 | `src/core/components/character/types.ts` | 61 | Type definitions cho module danh sách nhân vật |
| 10 | `src/core/components/character-detail/CharacterDetailLayout.tsx` | 211 | Layout 2 cột (sidebar tóm tắt + 3 tab, sticky header, action bar) |
| 11 | `src/core/components/character-detail/MemoryCard.tsx` | 148 | Card & Dialog quản lý ký ức nhân vật (List, Create, Edit modal) |
| 12 | `src/core/components/character-detail/RelationshipCard.tsx` | 114 | Card quan hệ nhân vật (relationship type, affinity slider) |
| 13 | `src/core/components/character-detail/mockData.ts` | 68 | Dữ liệu mock sâu cho character detail |
| 14 | `src/core/components/character-detail/store.ts` | 112 | Zustand store quản lý state detail, history undo/redo, saving |
| 15 | `src/core/components/character-detail/types.ts` | 107 | Type definitions cho character detail |
| 16 | `src/core/components/character-detail/shared/EditableField.tsx` | 56 | Control view/edit toggle cho chuỗi đơn giản |
| 17 | `src/core/components/character-detail/shared/EnumSelect.tsx` | 42 | Select dropdown dạng native select có bọc neu-pressed |
| 18 | `src/core/components/character-detail/shared/FieldDisplay.tsx` | 33 | Hiển thị read-only kèm icon và nhãn |
| 19 | `src/core/components/character-detail/shared/IntensitySlider.tsx` | 46 | Slider input range cường độ (0 - 100) |
| 20 | `src/core/components/character-detail/shared/ProgressBar.tsx` | 43 | Thanh tiến trình visual (0 - 100%) |
| 21 | `src/core/components/character-detail/tabs/CoreIdentityTab.tsx` | 239 | Tab 1: Danh tính cốt lõi (tên, avatar, tính cách, đặc điểm) |
| 22 | `src/core/components/character-detail/tabs/LifeExperienceTab.tsx` | 243 | Tab 2: Trải nghiệm & Ký ức & Quan hệ |
| 23 | `src/core/components/character-detail/tabs/PsychologicalTab.tsx` | 229 | Tab 3: Tâm lý, động lực, nỗi sợ, giá trị, Big Five |
| **Tổng** | **23 files** | **2.553** | |

### Lệnh Grep & Thống kê đã dùng
- **Hardcoded hex/rgb/hsl**: Không phát hiện chuỗi `#hex` hay `rgb(...)` tự do trong mã TSX (tuân thủ tốt Tailwind palette).
- **Arbitrary pixel brackets `\[\d+px\]`**: 12 lượt (`text-[10px]`, `text-[11px]`, `text-[13px]`, `text-[12px]`).
- **Font-size violations `text-[13px]`**: 2 lượt (trong `CharacterChatPane.tsx:88`, `CharacterChatPane.tsx:107`).
- **Arbitrary radius `rounded-\[`**: 0 lượt (100% dùng chuẩn Tailwind scale `rounded-lg`, `rounded-xl`, `rounded-2xl`, `rounded-full`).
- **Ad-hoc shadows `shadow-*`**: 11 lượt vi phạm nghiêm trọng (R1) do dùng `shadow-sm`, `shadow-md`, `shadow-lg`, `shadow-rose-500/20`, `shadow-purple-500/25` thay vì elevation token `neu-flat`/`neu-pressed`.
- **Dark parity check `dark:`**: 285 lượt khai báo `dark:`. Tuy nhiên có nhiều điểm hở (missing dark text/border/bg).
- **Focus ring & a11y `focus:` vs `ring`**: 44 lượt xuất hiện focus. 3 form control thiếu `focus-visible:ring-2` hoặc dùng `focus:outline-hidden` đơn độc không ring.
- **Icon-only button thiếu `aria-label`**: 8 nút icon-only (Edit memory, Delete memory, Edit relationship, Delete relationship, Send message, Close dialog, Back icon).
- **i18n check**: `useT(` chỉ được gọi tại duy nhất 1 file trong toàn bộ module (`CharacterListView.tsx:14`). **22/23 file còn lại không dùng `useT()`**, 100% text giao diện hardcode tiếng Anh (hơn 130 chuỗi).
- **Shared Dialog reuse**: `CreateCharacterDialog.tsx` và `MemoryCard.tsx` tự dựng HTML modal `fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-xs` thủ công, **không dùng shared `Dialog` component** từ `@/shared/components/ui/dialog`.
- **Toast reuse**: `CharacterDetailLayout.tsx` và `CharacterDetailView.tsx` tự dựng floating badge `"Changes saved"` hoặc không có thông báo, **không dùng `toast()`** từ `@/shared/components/ui/toast`.
- **Confirm Dialog reuse**: Thao tác xoá ký ức (Memory) và xoá quan hệ (Relationship) trong `LifeExperienceTab.tsx` gọi trực tiếp `removeMemory` và `removeRelationship` **không hề có dialog xác nhận**, bỏ qua `ConfirmDialog` / `DeleteConfirmDialog`.

---

## 2. Scorecard R1–R12

| Rule | Quy định chuẩn Baseline v0 | Verdict | Evidence Count | Ghi chú đánh giá |
|---|---|:---:|:---:|---|
| **R1 Surface/Elevation** | Chỉ `neu-flat`, `neu-pressed` hoặc phẳng trong suốt; cấm `shadow-*` rời, gradient tuỳ tiện, bg trắng đục | **FAIL** | 11 | Xuất hiện `shadow-sm`, `shadow-md`, `shadow-lg`, `shadow-rose-500/20`, `shadow-purple-500/25` tại `CharacterCard`, `CreateCharacterDialog`, `LifeExperienceTab`, `CharacterDetailLayout`. |
| **R2 Color** | Chỉ token `--color-base-*`, slate + semantic status; cấm hardcode hex/rgb; accent đúng ngữ nghĩa | **PARTIAL** | 8 | Không bị lỗi hardcode hex, nhưng lạm dụng `bg-white` đục tại modal và input; nhầm lẫn semantic giữa `purple` (decor) và `indigo`/`blue` (action). |
| **R3 Dark Parity** | Mọi `bg-`, `border-`, `text-`, `divide-`, `ring-` phải có `dark:` tương ứng; cấm lệch theme | **PARTIAL** | 18 | `CreateCharacterDialog` và `MemoryCard` modal dùng `bg-white` không có `dark:bg-slate-900` hoặc thiếu `dark:text-slate-*` khiến text bị chìm hoặc vỡ tương phản ở dark mode. |
| **R4 Type Scale** | Chỉ 10/11/12/14/16/18/20/24/30; cấm `text-[13px]`, `text-[9px]`; tracking/leading nhất quán | **PARTIAL** | 2 | `CharacterChatPane.tsx` dùng `text-[13px]` tại line 88 và 107. Nhãn badge dùng `text-[10px]` và `text-[11px]` khá lộn xộn. |
| **R5 Radius Ladder** | Thang chuẩn `{sm, md, lg, xl, 2xl, 3xl, full}`; cấm `rounded-[...]` rải rác | **PASS** | 0 | Module tuân thủ tốt hệ radius tiêu chuẩn (`rounded-lg`, `rounded-xl`, `rounded-2xl`, `rounded-full`). Không có arbitrary radius bracket. |
| **R6 Spacing Grid** | Bội số 4px (Tailwind standard); cấm arbitrary bracket spacing (`p-[7px]`, `gap-[13px]`) | **PASS** | 0 | 100% padding/margin/gap dùng spacing scale chuẩn (`p-4`, `gap-3`, `gap-4`, `space-y-4`). |
| **R7 Interaction** | Bấm được phải có hover, focus-visible ring, disabled, active; cấm `focus:outline-hidden` đơn lẻ | **FAIL** | 14 | `EnumSelect`, `EditableField` và `IntensitySlider` thiếu focus-visible ring chuẩn; nhiều button không có trạng thái `disabled:` styling; thẻ tab thiếu visual active indicator chuẩn. |
| **R8 A11y** | Icon-only button phải có `aria-label`; Dialog cần Title/Desc; Slider cần `aria-valuetext`; contrast AA | **FAIL** | 22 | Slider thiếu `aria-valuetext`/`aria-label`; ProgressBar thiếu `aria-valuenow`; 8 nút icon-only thiếu `aria-label`; Tablist thiếu `role="tablist"` / `role="tab"` / `aria-selected`. |
| **R9 i18n** | Mọi chuỗi qua `t(key)` có en/vi; cấm hardcode tiếng Anh/Việt trong TSX | **FAIL** | 134+ | 22/23 file hoàn toàn không dùng i18n; hơn 130 chuỗi giao diện tiếng Anh bị hardcode trong khi `dict.ts` đã có sẵn hơn 80 key `character.*`. |
| **R10 Primitive Reuse** | Modal dùng shared `Dialog`; confirm dùng `ConfirmDialog`; thông báo dùng `toast()`; cấm tự chế | **FAIL** | 5 | 2 dialog tự chế overlay `fixed inset-0`; 2 action xoá dữ liệu không dùng ConfirmDialog; thông báo lưu dùng custom badge thay vì `toast()`. |
| **R11 Motion** | Duration 150/200/300ms, easing nhất quán; hỗ trợ `motion-reduce` | **PARTIAL** | 12 | Có dùng `transition-colors`, `transition-all duration-200`, nhưng hoàn toàn thiếu `motion-reduce:transition-none` cho các hiệu ứng chuyển động. |
| **R12 State Coverage** | Đủ 4 trạng thái loading / empty / error / disabled nhất quán | **PARTIAL** | 9 | Đã có empty state tại `CharacterListView` và `MemoryCard`; tuy nhiên hoàn toàn thiếu loading skeleton/spinner khi fetch nhân vật và thiếu error boundary/message khi save thất bại. |

---

## 3. Bảng so sánh 5 Form Control tự chế

Module `character-detail/shared/` định nghĩa 5 form control tự chế. Dưới đây là bảng đối chiếu độ nhất quán nội bộ:

| Thuộc tính | `EditableField` | `EnumSelect` | `IntensitySlider` | `ProgressBar` | `FieldDisplay` | Nhận xét nhất quán |
|---|---|---|---|---|---|---|
| **Height / Min-h** | View: `min-h-[38px]`; Edit: `h-[38px]` | Phụ thuộc font/padding (`py-1.5 px-3`, ~`h-9`) | Không cố định (`h-2` track, thumb native) | `h-2` hoặc `h-2.5` | Không cố định (`py-1` text line) | ⚠️ **Không đều**: `EditableField` dùng 38px lẻ; `EnumSelect` dùng ~36px; thiếu chuẩn `h-9` (36px) hoặc `h-10` (40px). |
| **Surface & Elevation** | View: `neu-pressed` khi hover; Edit: `neu-pressed` | View/Select: `neu-pressed` | Track: `neu-pressed` | Track: `neu-pressed` | Phẳng trong suốt | ✅ Nhất quán dùng `.neu-pressed` cho surface input. |
| **Border & Radius** | `rounded-lg border border-slate-200 dark:border-slate-700/60` | `rounded-lg border border-slate-200 dark:border-slate-700/60` | `rounded-full border-0` | `rounded-full border-0` | Không viền | ✅ Tốt: Text/Select dùng `rounded-lg` + border slate; Bar/Slider dùng `rounded-full`. |
| **Focus Ring** | `focus:outline-hidden focus:ring-2 focus:ring-purple-500/50` | `focus:outline-hidden focus:ring-2 focus:ring-purple-500/50` | `focus:outline-hidden focus:ring-2 focus:ring-purple-500/50 focus:ring-offset-2` | N/A (read-only) | N/A (read-only) | ⚠️ Dùng `purple-500/50` ring nhưng thiếu dark parity và thiếu chuẩn focus ring toàn app. |
| **Disabled State** | Không có prop `disabled` | Có prop `disabled`, chỉ đổi opacity `disabled:opacity-50` | Có prop `disabled`, chỉ đổi cursor | N/A | N/A | ❌ **Yếu**: `EditableField` không hỗ trợ disabled; `IntensitySlider` không làm mờ khi disabled. |
| **Error State** | ❌ Không có hiển thị lỗi / invalid ring | ❌ Không có prop `error` | ❌ Không có prop `error` | N/A | N/A | ❌ **Thiếu hoàn toàn**: Không control nào hỗ trợ invalid border (`border-rose-500`), error helper text, hoặc `aria-invalid`. |
| **Dark Parity** | Có `dark:bg-slate-900/50`, `dark:text-white` | Có `dark:bg-slate-900/50`, `dark:text-white` | Track: `dark:bg-slate-800` | Track: `dark:bg-slate-800` | Có `dark:text-slate-200` | ⚠️ Cơ bản có dark, nhưng `EnumSelect` popup native option trên Windows/macOS có thể bị nền trắng chữ trắng nếu không style `dark:bg-slate-900`. |
| **A11y Attributes** | `aria-label={label}` trên input | `aria-label={label}` trên select | ❌ Thiếu `aria-valuetext`, `aria-label` | ❌ Thiếu `role="progressbar"`, `aria-valuenow` | N/A | ❌ Vi phạm A11y nghiêm trọng trên Slider & ProgressBar. |

---

## 4. Danh sách Findings (Tối đa 25 findings, P0 → P2, có file:line xác minh)

### Mức P0: Lỗi nghiêm trọng (A11y chặn, hỏng theme/tương phản, thiếu confirm dữ liệu)

#### `F01` | P0 | `src/core/components/character-detail/shared/ProgressBar.tsx:28-39` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <div className="w-full bg-slate-200 dark:bg-slate-800 rounded-full h-2 overflow-hidden neu-pressed">
    <div
      className={cn("h-full transition-all duration-300 rounded-full", colorClass)}
      style={{ width: `${clampedValue}%` }}
    />
  </div>
  ```
- **Vấn đề**: `ProgressBar` hoàn toàn không có `role="progressbar"`, không có `aria-valuenow`, `aria-valuemin="0"`, `aria-valuemax="100"`. Screen reader coi đây là một `div` trống vô nghĩa, người khiếm thị không tiếp cận được chỉ số tâm lý/tính cách.
- **Đề xuất**:
  ```tsx
  <div
    role="progressbar"
    aria-valuenow={clampedValue}
    aria-valuemin={0}
    aria-valuemax={100}
    aria-label={showLabel ? label : (label || "Progress")}
    className="w-full bg-slate-200 dark:bg-slate-800 rounded-full h-2 overflow-hidden neu-pressed"
  >
  ```

#### `F02` | P0 | `src/core/components/character-detail/shared/IntensitySlider.tsx:27-37` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <input
    type="range"
    min={min}
    max={max}
    step={step}
    value={value}
    disabled={disabled}
    onChange={(e) => onChange(Number(e.target.value))}
    className="..."
  />
  ```
- **Vấn đề**: `<input type="range">` không có thuộc tính `aria-label` hoặc `aria-labelledby`, và thiếu `aria-valuetext` để diễn đạt mức độ (ví dụ: "75% - Cao").
- **Đề xuất**: Bổ sung `aria-label={label || "Intensity"}` và `aria-valuetext={`${value}%`}` cùng liên kết id với `<label>`.

#### `F03` | P0 | `src/core/components/character-detail/tabs/LifeExperienceTab.tsx:165` & `235` | Vi phạm R10
- **Hiện tại**:
  ```tsx
  // Line 165 (Memory delete)
  onClick={() => removeMemory(mem.id)}
  // Line 235 (Relationship delete)
  onClick={() => removeRelationship(rel.id)}
  ```
- **Vấn đề**: Thao tác xoá vĩnh viễn ký ức và mối quan hệ nhân vật được kích hoạt trực tiếp ngay khi click icon thùng rác, **hoàn toàn không có hộp thoại xác nhận** (`ConfirmDialog` hoặc `DeleteConfirmDialog`). Rủi ro người dùng vô tình bấm mất dữ liệu.
- **Đề xuất**: Tích hợp shared `DeleteConfirmDialog` hoặc `ConfirmDialog` trước khi thực thi `removeMemory` / `removeRelationship`.

#### `F04` | P0 | `src/core/components/character/CreateCharacterDialog.tsx:75-78` | Vi phạm R3, R1
- **Hiện tại**:
  ```tsx
  <div className="relative w-full max-w-lg rounded-2xl bg-white p-6 shadow-xl dark:bg-slate-900 border border-slate-200/80 dark:border-slate-800">
  ```
- **Vấn đề**: Tự tạo overlay và dialog container dùng `shadow-xl`, `bg-white` không đồng bộ với Neumorphism glass baseline (`neu-flat backdrop-blur-md`). Ngoài ra bên trong modal có nhiều input bị lệch màu chữ khi hiển thị ở chế độ tối.
- **Đề xuất**: Chuyển sang sử dụng shared component `Dialog`, `DialogContent`, `DialogHeader`, `DialogTitle` (`@/shared/components/ui/dialog`).

---

### Mức P1: Lệch hệ thống rõ rệt, vi phạm R1–R3, R7, R9, R10

#### `F05` | P1 | `src/core/components/character-detail/CharacterDetailLayout.tsx:18-20` | Vi phạm R9
- **Hiện tại**:
  ```tsx
  const TABS: { id: CharacterTab; label: string; icon: typeof User }[] = [
    { id: 'core', label: 'Core Identity', icon: User },
    { id: 'life', label: 'Life Experience', icon: BookOpen },
    { id: 'psychological', label: 'Psychological', icon: Brain },
  ];
  ```
- **Vấn đề**: Nhãn 3 tab chính bị hardcode chuỗi tiếng Anh tĩnh bên ngoài component, không thể thay đổi ngôn ngữ khi người dùng chọn Tiếng Việt (`dict.ts` đã có `character.tabs.core`, `character.tabs.life`, `character.tabs.psychological`).
- **Đề xuất**: Dùng hook `const t = useT()` bên trong component và map `label: t('character.tabs.core')`, v.v.

#### `F06` | P1 | `src/core/components/character-detail/CharacterDetailLayout.tsx:109-122` | Vi phạm R7, R8
- **Hiện tại**:
  ```tsx
  <div className="flex gap-1 border-b border-slate-200 dark:border-slate-800">
    {TABS.map((tab) => {
      const Icon = tab.icon;
      const isActive = activeTab === tab.id;
      return (
        <button
          key={tab.id}
          onClick={() => setActiveTab(tab.id)}
          className={cn(
            "flex items-center gap-2 px-4 py-3 text-sm font-medium border-b-2 transition-colors -mb-px",
            isActive ? "border-purple-600 text-purple-600 dark:text-purple-400" : "..."
          )}
        >
  ```
- **Vấn đề**: Cụm tab không có `role="tablist"`, các button thiếu `role="tab"`, `aria-selected={isActive}`, `aria-controls={`tabpanel-${tab.id}`}`, và không hỗ trợ điều hướng bằng phím mũi tên (ArrowLeft/Right).
- **Đề xuất**: Thêm role tabs semantic đầy đủ và `tabIndex={isActive ? 0 : -1}`.

#### `F07` | P1 | `src/core/components/character/CharacterCard.tsx:43` & `51` | Vi phạm R1
- **Hiện tại**:
  ```tsx
  // Line 43:
  className="group relative flex flex-col overflow-hidden rounded-2xl bg-white/80 p-5 neu-flat transition-all duration-300 hover:-translate-y-1 hover:shadow-lg ..."
  // Line 51:
  <div className="relative h-44 w-full overflow-hidden rounded-xl bg-slate-100 shadow-inner dark:bg-slate-800">
  ```
- **Vấn đề**: Dùng `hover:shadow-lg` và `shadow-inner`. Theo Baseline v0 R1, shadow rời bị cấm hoàn toàn; elevation chỉ dùng `neu-flat` và `neu-pressed`.
- **Đề xuất**: Thay `hover:shadow-lg` bằng `hover:brightness-105 active:neu-pressed` hoặc hiệu ứng border glow; thay `shadow-inner` bằng `neu-pressed`.

#### `F08` | P1 | `src/core/components/character-detail/MemoryCard.tsx:42-53` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <button
    onClick={onEdit}
    className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-600 dark:hover:bg-slate-800 dark:hover:text-slate-300"
  >
    <Edit2 className="h-4 w-4" />
  </button>
  <button
    onClick={onDelete}
    className="rounded-lg p-1.5 text-slate-400 hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-950/30 dark:hover:text-rose-400"
  >
    <Trash2 className="h-4 w-4" />
  </button>
  ```
- **Vấn đề**: Nút Edit và Delete chỉ có icon, không có `aria-label`. Trình đọc màn hình chỉ thông báo "button", người dùng khiếm thị không biết chức năng nút.
- **Đề xuất**: Thêm `aria-label="Edit memory"` và `aria-label="Delete memory"` (sử dụng chuỗi từ i18n).

#### `F09` | P1 | `src/core/components/character-detail/RelationshipCard.tsx:47-58` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <button
    onClick={onEdit}
    className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 ..."
  >
    <Edit2 className="h-4 w-4" />
  </button>
  <button
    onClick={onDelete}
    className="rounded-lg p-1.5 text-slate-400 hover:bg-rose-50 ..."
  >
    <Trash2 className="h-4 w-4" />
  </button>
  ```
- **Vấn đề**: Nút Edit và Delete quan hệ thiếu `aria-label`.
- **Đề xuất**: Thêm `aria-label="Edit relationship"` và `aria-label="Delete relationship"`.

#### `F10` | P1 | `src/core/components/character-detail/CharacterDetailLayout.tsx:75-81` | Vi phạm R10
- **Hiện tại**:
  ```tsx
  {isSaving ? (
    <span className="flex items-center gap-1.5 text-xs text-slate-500">
      <Loader2 className="h-3.5 w-3.5 animate-spin" /> Saving...
    </span>
  ) : isDirty ? (
    <span className="text-xs text-amber-500">Unsaved changes</span>
  ) : (
    <span className="text-xs text-emerald-500">Saved</span>
  )}
  ```
- **Vấn đề**: Thông báo trạng thái lưu chỉ hiển thị dòng text tĩnh nhỏ trên header, không tích hợp cơ chế thông báo toast nổi (`toast.success(t('character.saved'))`) khi lưu hoàn tất hoặc `toast.error(...)` khi lỗi.
- **Đề xuất**: Gọi `toast.success()` trong handler `handleSave` hoặc hook của Zustand store khi hoàn tất thao tác lưu.

#### `F11` | P1 | `src/core/components/character-detail/MemoryCard.tsx:109-114` | Vi phạm R10, R1
- **Hiện tại**:
  ```tsx
  <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-xs p-4">
    <div className="w-full max-w-lg rounded-2xl bg-white p-6 shadow-xl dark:bg-slate-900 border border-slate-200 dark:border-slate-800 ...">
  ```
- **Vấn đề**: Tự chế Dialog overlay bằng `div` thủ công, vi phạm R10 (cấm tự chế modal) và R1 (`shadow-xl`). Không bẫy focus (focus trap), không đóng bằng phím ESC, thiếu `DialogTitle` & `DialogDescription`.
- **Đề xuất**: Tái cấu trúc sang dùng `@/shared/components/ui/dialog`.

#### `F12` | P1 | `src/core/components/character-detail/tabs/LifeExperienceTab.tsx:86-90` | Vi phạm R1
- **Hiện tại**:
  ```tsx
  <button
    onClick={() => setIsAddingMemory(true)}
    className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-white bg-purple-600 rounded-lg hover:bg-purple-700 shadow-sm transition-colors"
  >
  ```
- **Vấn đề**: Dùng `shadow-sm` vi phạm R1.
- **Đề xuất**: Bỏ `shadow-sm`, áp dụng `neu-flat hover:brightness-105 active:neu-pressed`.

#### `F13` | P1 | `src/core/components/character-detail/CharacterDetailLayout.tsx:93-97` | Vi phạm R1
- **Hiện tại**:
  ```tsx
  <button
    onClick={saveCharacter}
    disabled={!isDirty || isSaving}
    className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium text-white bg-purple-600 rounded-lg hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed shadow-sm transition-colors"
  >
  ```
- **Vấn đề**: Dùng `shadow-sm` trên nút hành động chính của layout detail.
- **Đề xuất**: Thay `shadow-sm` bằng styling chuẩn elevation không shadow.

#### `F14` | P1 | `src/core/components/character/CharacterChatPane.tsx:88` & `107` | Vi phạm R4
- **Hiện tại**:
  ```tsx
  // Line 88:
  className="text-[13px] leading-relaxed whitespace-pre-wrap break-words"
  // Line 107:
  className="text-[13px] rounded-2xl px-4 py-2.5 neu-pressed ..."
  ```
- **Vấn đề**: Dùng `text-[13px]` tuỳ tiện. Type scale của hệ thống chỉ cho phép `{10, 11, 12, 14, 16, ...}`.
- **Đề xuất**: Chuyển thành `text-xs` (12px) hoặc `text-sm` (14px).

#### `F15` | P1 | `src/core/components/character/CharacterChatPane.tsx:154-159` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <button
    type="submit"
    disabled={!input.trim()}
    className="rounded-xl p-2.5 text-white transition-opacity disabled:opacity-40 neu-flat"
    style={{ backgroundColor: "var(--color-primary, #9333ea)" }}
  >
    <Send className="h-4 w-4" />
  </button>
  ```
- **Vấn đề**: Nút Send tin nhắn không có `aria-label="Send message"`, đồng thời hardcode inline style `style={{ backgroundColor: "var(--color-primary, #9333ea)" }}` vi phạm R2 (cấm hardcode hex trong component).
- **Đề xuất**: Bổ sung `aria-label="Send message"` và thay style inline bằng utility class Tailwind `bg-purple-600 hover:bg-purple-700`.

#### `F16` | P1 | `src/core/components/character/CharacterDetailView.tsx` (Toàn bộ 184 LOC) | Vi phạm R10, Architecture Dead Code
- **Hiện tại**: Component `CharacterDetailView` tồn tại song song với `CharacterDetailLayout`. Trong khi route `src/app/routes/character-detail.tsx` đã trỏ đến `CharacterDetailLayout`, component này không được bất kỳ route nào import sử dụng nhưng vẫn duy trì phiên bản tab cũ (`overview`, `attributes`, `background`, `stats`).
- **Vấn đề**: Duplicate UI logic, gây nhầm lẫn bảo trì và không đồng bộ hệ thống.
- **Đề xuất**: Đánh dấu deprecated và lên kế hoạch dọn dẹp hoặc hợp nhất vào `CharacterDetailLayout`.

---

### Mức P2: Lệch nhỏ, thiếu sót trạng thái thứ cấp, chi tiết tương tác

#### `F17` | P2 | `src/core/components/character/CharacterListView.tsx:98` | Vi phạm R1
- **Hiện tại**:
  ```tsx
  <div className="flex flex-wrap items-center gap-2 rounded-2xl bg-white/60 p-3 neu-flat dark:bg-slate-900/40">
  ```
- **Vấn đề**: Kết hợp đồng thời `bg-white/60 neu-flat` và `dark:bg-slate-900/40`. Baseline v0 quy định surface flat đã mang sẵn background và shadow neumorphism, việc áp đè `bg-white/60` làm mất độ trong suốt tự nhiên của glass token.
- **Đề xuất**: Sử dụng `.neu-flat` đơn thuần hoặc `bg-slate-100/50 dark:bg-slate-900/50 neu-flat`.

#### `F18` | P2 | `src/core/components/character/CharacterCard.tsx:64-67` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <span className={cn("inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wider", ...)}>
    <span className={cn("h-1.5 w-1.5 rounded-full", ...)} />
    {character.status}
  </span>
  ```
- **Vấn đề**: Dấu chấm tròn màu báo status (`h-1.5 w-1.5 rounded-full`) là chi tiết trang trí trực quan nhưng không có `aria-hidden="true"`.
- **Đề xuất**: Thêm `aria-hidden="true"` vào dot indicator.

#### `F19` | P2 | `src/core/components/character-detail/shared/IntensitySlider.tsx:39-44` | Vi phạm R2, R11
- **Hiện tại**:
  ```tsx
  <div
    className="h-full bg-purple-600 rounded-full transition-all"
    style={{ width: `${percentage}%` }}
  />
  ```
- **Vấn đề**: Lớp fill của slider thiếu duration cụ thể (`transition-all` mặc định 150ms) và không có `motion-reduce:transition-none`.
- **Đề xuất**: Đổi thành `transition-[width] duration-200 motion-reduce:transition-none`.

#### `F20` | P2 | `src/core/components/character-detail/tabs/CoreIdentityTab.tsx:125` | Vi phạm R3
- **Hiện tại**:
  ```tsx
  <label className="text-xs font-semibold uppercase tracking-wider text-slate-500">
  ```
- **Vấn đề**: `text-slate-500` không có dark counterpart (`dark:text-slate-400`), dẫn đến độ tương phản trên nền tối hơi thấp (chưa đạt chuẩn WCAG AA trên nền dark).
- **Đề xuất**: Bổ sung `dark:text-slate-400`.

#### `F21` | P2 | `src/core/components/character-detail/CharacterDetailLayout.tsx:62-68` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <button
    onClick={() => navigate('/character')}
    className="rounded-lg p-2 text-slate-500 hover:bg-slate-200/50 dark:hover:bg-slate-800 transition-colors"
  >
    <ArrowLeft className="h-5 w-5" />
  </button>
  ```
- **Vấn đề**: Nút quay lại danh sách (`ArrowLeft`) thiếu `aria-label="Back to characters"`.
- **Đề xuất**: Thêm `aria-label={t('common.back') || "Back to characters"}`.

#### `F22` | P2 | `src/core/components/character-detail/shared/EnumSelect.tsx:28-39` | Vi phạm R7
- **Hiện tại**:
  ```tsx
  <select
    value={value}
    disabled={disabled}
    onChange={(e) => onChange(e.target.value)}
    className="w-full appearance-none rounded-lg border border-slate-200 dark:border-slate-700/60 bg-white/80 dark:bg-slate-900/50 py-1.5 pl-3 pr-8 text-sm ..."
  >
  ```
- **Vấn đề**: Native `<select>` với `appearance-none` có icon mũi tên `ChevronDown` chồng lên nhưng không bọc `pointer-events-none` cho icon, khiến người dùng click trúng góc icon trên một số trình duyệt không mở được dropdown.
- **Đề xuất**: Thêm `pointer-events-none` vào phần tử bọc icon `ChevronDown`.

#### `F23` | P2 | `src/core/components/character-detail/CharacterDetailLayout.tsx:70-73` | Vi phạm R4
- **Hiện tại**:
  ```tsx
  <h1 className="text-lg font-bold text-slate-900 dark:text-white truncate">
    {character.name}
  </h1>
  <p className="text-xs text-slate-500 truncate">
    {character.archetype} • {character.world}
  </p>
  ```
- **Vấn đề**: `h1` dùng `text-lg` (18px) hơi nhỏ so với tiêu đề trang chính của một Layout chi tiết (Design system thường dùng `text-xl` hoặc `text-2xl` cho Page Title).
- **Đề xuất**: Cân nhắc chuẩn hoá page title cấp Layout thành `text-xl font-bold tracking-tight`.

#### `F24` | P2 | `src/core/components/character/CreateCharacterDialog.tsx:180-188` | Vi phạm R7
- **Hiện tại**:
  ```tsx
  <button
    type="submit"
    disabled={!name.trim()}
    className="flex items-center gap-2 rounded-xl bg-purple-600 px-5 py-2.5 text-sm font-medium text-white shadow-md hover:bg-purple-700 disabled:opacity-40 disabled:cursor-not-allowed neu-flat"
  >
  ```
- **Vấn đề**: Nút submit kết hợp cả `shadow-md` và `neu-flat`, đồng thời thiếu trạng thái loading spinner khi bấm gửi tạo nhân vật.
- **Đề xuất**: Bỏ `shadow-md`, bổ sung trạng thái `isSubmitting` kèm `<Loader2 className="animate-spin" />`.

#### `F25` | P2 | `src/core/components/character/CharacterListView.tsx:32-38` | Vi phạm R8
- **Hiện tại**:
  ```tsx
  <input
    type="text"
    value={search}
    onChange={(e) => setSearch(e.target.value)}
    placeholder={t("character.searchPlaceholder")}
    className="w-full bg-transparent pl-9 pr-4 py-2 text-sm text-slate-800 placeholder-slate-400 focus:outline-hidden dark:text-slate-100 dark:placeholder-slate-500"
  />
  ```
- **Vấn đề**: Ô tìm kiếm nhân vật chỉ có `placeholder` mà thiếu `aria-label="Search characters"`. Placeholder sẽ biến mất khi nhập liệu khiến trợ năng không đọc được tên trường.
- **Đề xuất**: Thêm `aria-label={t("character.searchPlaceholder")}` vào `<input>`.

---

## 5. Patterns nên chuẩn hoá thành Primitive / Token

Qua rà soát 23 files, có 4 pattern lặp lại nhiều lần với các biến thể tự phát cần được trừu tượng hoá thành shared primitive:

### 1. `FormSectionCard` (Card phân nhóm thiết lập trong Tabs)
- **Hiện trạng**: Cả 3 tab `CoreIdentityTab`, `LifeExperienceTab`, `PsychologicalTab` đều lặp lại cấu trúc container:
  ```tsx
  // Ví dụ tốt tại CoreIdentityTab.tsx:43:
  <div className="rounded-2xl border border-slate-200/80 bg-white/60 p-6 neu-flat dark:border-slate-800 dark:bg-slate-900/40">
    <div className="mb-4 flex items-center justify-between">
      <h3 className="text-sm font-semibold text-slate-800 dark:text-slate-200">{title}</h3>
      {action}
    </div>
    {children}
  </div>
  ```
- **Đề xuất**: Tạo primitive `@/shared/components/ui/form-section-card.tsx` để thống nhất padding (`p-5` vs `p-6`), border opacity, và title style.

### 2. `StatusBadge` (Huy hiệu trạng thái có dot indicator)
- **Hiện trạng**: Xuất hiện ở `CharacterCard.tsx:64-77`, `CharacterDetailLayout.tsx:143`, `CharacterDetailView.tsx:84`. Mỗi nơi tự viết một object map màu (`emerald-500`, `amber-500`, `purple-500`) và kích thước dot indicator khác nhau.
- **Đề xuất**: Tạo component `StatusBadge` chuẩn hoá:
  ```tsx
  <StatusBadge status="active" variant="dot" size="sm" />
  ```

### 3. `FormInput` / `InputWithLabel` chuẩn hoá
- **Hiện trạng**: `EditableField`, `EnumSelect`, `CreateCharacterDialog` và `MemoryCard` đều tự tính toán nhãn `<label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">` nhưng lệch nhau về khoảng cách `mb-1` vs `mb-1.5` và font weight (`font-medium` vs `font-semibold`).
- **Đề xuất**: Thống nhất Label typography và error text rendering trong bộ form control.

### 4. `TabListPrimitive` (Accessible Tabs)
- **Hiện trạng**: `CharacterDetailLayout.tsx:109` tự render flex div với state `activeTab`. Không có bàn phím điều hướng WAI-ARIA.
- **Đề xuất**: Xây dựng Radix Tabs primitive hoặc bọc accessibility chuẩn vào một shared tab component.

---

## 6. Top 10 Fix Đề Xuất

| # | Hạng mục đề xuất Fix | File ảnh hưởng | Effort | Tác động (Impact) |
|---|---|---|:---:|---|
| 1 | Bổ sung ARIA đầy đủ (`role="progressbar"`, `valuenow`, `valuetext`) cho `ProgressBar` và `IntensitySlider` | `shared/ProgressBar.tsx`, `shared/IntensitySlider.tsx` | **S** | **P0**: Đạt chuẩn trợ năng WCAG AA cho toàn bộ thanh chỉ số. |
| 2 | Bổ sung hộp thoại xác nhận xoá (`DeleteConfirmDialog`) cho Ký ức và Mối quan hệ | `character-detail/tabs/LifeExperienceTab.tsx` | **S** | **P0**: Ngăn chặn mất dữ liệu do bấm nhầm thao tác xoá. |
| 3 | Thay thế Dialog tự chế bằng shared component `Dialog` trong `CreateCharacterDialog` và `MemoryCard` | `CreateCharacterDialog.tsx`, `MemoryCard.tsx` | **M** | **P0/P1**: Đảm bảo focus trap, phím ESC, overlay backdrop và Neumorphism nhất quán. |
| 4 | Bổ sung `aria-label` cho 8 nút icon-only (Edit, Delete, Back, Send, Close) | `MemoryCard.tsx`, `RelationshipCard.tsx`, `CharacterChatPane.tsx`, `CharacterDetailLayout.tsx` | **S** | **P1**: Khắc phục lỗi đọc màn hình cho người khiếm thị. |
| 5 | Gắn `role="tablist"` / `role="tab"` / `aria-selected` và hỗ trợ phím cho cụm Tab điều hướng | `CharacterDetailLayout.tsx:109-130` | **S** | **P1**: Chuẩn hoá điều hướng bàn phím theo WAI-ARIA Tabs pattern. |
| 6 | Thay thế toàn bộ 11 vị trí dùng `shadow-*` rời sang hệ elevation `neu-flat` / `neu-pressed` | `CharacterCard.tsx`, `CharacterDetailLayout.tsx`, `LifeExperienceTab.tsx`, `CreateCharacterDialog.tsx` | **S** | **P1**: Tuân thủ triệt để Rule R1 (Elevation/Neumorphism). |
| 7 | Tích hợp i18n hook `useT()` và khai báo key dịch cho 22 file trong module | `character-detail/**`, `CreateCharacterDialog.tsx`, `CharacterChatPane.tsx` | **L** | **P1**: Hỗ trợ chuyển đổi song ngữ Anh - Việt hoàn chỉnh. |
| 8 | Thay thế thông báo lưu tĩnh bằng shared `toast()` | `CharacterDetailLayout.tsx:75-81`, `store.ts` | **S** | **P1**: Thống nhất phản hồi người dùng toàn ứng dụng. |
| 9 | Sửa lỗi type scale `text-[13px]` thành `text-xs` hoặc `text-sm` | `CharacterChatPane.tsx:88`, `107` | **S** | **P1**: Triệt tiêu arbitrary font-size không nằm trong scale chuẩn. |
| 10 | Dọn dẹp / Deprecate file dead code `CharacterDetailView.tsx` | `src/core/components/character/CharacterDetailView.tsx` | **M** | **P1**: Giảm tải mã nguồn dư thừa, tập trung vào `CharacterDetailLayout`. |

---

## 7. Shared Primitive Defects (Phát hiện ngoài scope)

Trong quá trình đối chiếu với các component dùng chung tại `src/shared/components/ui/`, phát hiện một số điểm cần lưu ý:
1. **Thiếu Form Controls Primitive**: Thư mục `src/shared/components/ui/` chỉ có `dialog`, `confirm-dialog`, `delete-confirm-dialog`, `toast`, `floating-outline`. **Chưa hề có** các primitive cơ bản như `Input`, `Select`, `Slider`, `Textarea`, `Tabs`. Điều này dẫn đến việc module Character buộc phải tự code các control nội bộ (`EditableField`, `EnumSelect`, `IntensitySlider`, `ProgressBar`) gây phân mảnh styling so với module Settings.
2. **`Dialog` (`src/shared/components/ui/dialog.tsx`)**: Đã được bọc Radix Dialog rất tốt và có hỗ trợ Neumorphism glass (`bg-base-light/90 dark:bg-base-dark/90 neu-flat backdrop-blur-md`), nhưng `CreateCharacterDialog` và `MemoryCard` lại không import sử dụng mà tự viết thẻ `div` cố định `bg-black/50`. Cần tài liệu hoá hoặc phổ biến việc tái sử dụng shared Dialog.

---

## 8. Baseline Conflicts (Nếu baseline v0 có điểm lạc hậu/mâu thuẫn)

1. **Quy ước Shadow trên Interactive Element**: Baseline v0 nghiêm cấm `shadow-*` rời và chỉ cho phép `neu-flat` / `neu-pressed`. Tuy nhiên, trong thực tế các nút nhấn nổi bật (ví dụ: `bg-purple-600 text-white`) khi áp dụng `.neu-flat` sẽ bị hiện tượng bóng xám của thẻ cha đè lên màu tím, dẫn đến lập trình viên hay thêm `shadow-sm` hoặc `shadow-md` để tạo độ sâu. Cần bổ sung token shadow màu cho primary button (ví dụ: `shadow-purple-500/25`) vào baseline v0 hoặc chuẩn hoá class `.btn-primary-neu`.
2. **Type Scale**: Baseline v0 liệt kê `text-[10px]` và `text-[11px]` là hợp lệ nhưng không khuyến khích arbitrary class. Việc rải rác các class `text-[10px]` và `text-[11px]` trong badge gây khó kiểm soát typographic scale. Nên chính thức token hoá `text-2xs` (10px) và `text-xs` (12px).

---

## 9. Metrics (Số liệu đếm được)

| Metric | Giá trị đo được | Đánh giá |
|---|:---:|---|
| **Tổng số files trong scope** | **23** files | Đã đọc và phân tích 100% |
| **Tổng số dòng mã (LOC)** | **2.553** LOC | Module quy mô lớn |
| **Số lỗi Hardcoded Hex/RGB màu** | **1** (`CharacterChatPane.tsx:158`) | Rất tốt, hầu như dùng Tailwind palette |
| **Số lỗi Arbitrary Font Size (`text-[...]`)** | **12** (trong đó **2** lỗi `text-[13px]`) | Cần chuẩn hoá về scale |
| **Số lỗi Arbitrary Radius (`rounded-[...]`)** | **0** | Tuyệt đối tuân thủ thang radius chuẩn |
| **Số lỗi Ad-hoc Shadow (`shadow-*`)** | **11** vị trí | Vi phạm Rule R1 |
| **Số lượng `dark:` variants** | **285** khai báo | Tỷ lệ phủ dark mode cao |
| **Số nút icon-only thiếu `aria-label`** | **8** nút | Vi phạm trợ năng cần khắc phục |
| **Số chuỗi copy bị hardcode (thiếu i18n)** | **> 130** chuỗi | 22/23 files chưa gắn `useT()` |
| **Tỷ lệ tái sử dụng Shared Dialog** | **0%** (0/2 dialog) | Cả 2 modal đều tự dựng div thủ công |
| **Tỷ lệ tái sử dụng Shared Toast** | **0%** (0/2 view) | Dùng text thông báo tĩnh thay vì toast |
| **Tỷ lệ xác nhận trước khi xoá (Confirm Dialog)** | **0%** (0/2 action) | Cả xoá Memory và Relationship đều thiếu confirm |

---

## 10. Verdict Tổng Thể

### **INCONSISTENT** (Không nhất quán)

**Lý do kết luận**:
1. **Thiếu vắng gần như hoàn toàn hệ thống đa ngôn ngữ (i18n)**: Mặc dù `dict.ts` đã chuẩn bị sẵn bộ từ điển phong phú với hơn 80 key cho `character.*`, có tới **22/23 file** trong module hoàn toàn không kết nối với `useT()`, để lộ hơn 130 chuỗi tiếng Anh hardcode trong TSX (vi phạm nghiêm trọng Rule R9).
2. **Không tái sử dụng Shared Primitive cốt lõi**: Module tự tạo lại các modal bằng `div fixed inset-0` thay vì dùng shared `Dialog` component; tự dựng trạng thái lưu thay vì dùng `toast()`; và thực hiện các thao tác phá huỷ dữ liệu (xoá ký ức, quan hệ) mà không có `ConfirmDialog` / `DeleteConfirmDialog` (vi phạm Rule R10).
3. **Phân mảnh Form Controls và lỗi Trợ năng (A11y)**: Cả 5 form control tự chế (`EditableField`, `EnumSelect`, `IntensitySlider`, `ProgressBar`, `FieldDisplay`) thiếu sự đồng nhất về chiều cao, thiếu hoàn toàn khả năng xử lý trạng thái lỗi (`error` / `aria-invalid`), và vi phạm nghiêm trọng chuẩn WCAG trên thanh đo tiến độ và nút bấm icon-only (vi phạm R7, R8).
4. **Vi phạm Elevation (Neumorphism)**: Xuất hiện 11 vị trí dùng `shadow-*` tuỳ tiện (`shadow-sm`, `shadow-lg`, `shadow-inner`) xung đột trực tiếp với chuẩn elevation Neumorphism phẳng/inset của hệ thống (vi phạm R1).
