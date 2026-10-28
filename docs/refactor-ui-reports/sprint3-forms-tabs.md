# Sprint 3 — Forms & Tabs

## Phạm vi / files
- `src/core/components/project/ProjectDetailSidebar.tsx`
- `src/core/components/settings/ProviderForm.tsx`

## Thay đổi
- ProjectDetailSidebar: tích hợp `Tabs` primitive, `variant="pill"`, `role=tablist/tab`, `aria-selected`, roving tabindex và keyboard handling đã có sẵn. Dùng các nhãn i18n hiện có làm fallback do bản file đầu vào không có 4 nút/panel Mục lục / Ghi chú / Nhân vật / Cài đặt hoặc state chuyển panel; nội dung đang có giữ nguyên. Tab click không đổi panel (`onChange` no-op), cần tích hợp state/panels khi luồng UI đó được đưa vào file.
- ProviderForm: chuyển select/input text & password sang `Select`/`Input`, action buttons sang `Button`; thêm aria label cho model controls, switch vẫn giữ `role="switch"` và `aria-checked`.
- Checkbox text/image được giữ là `<input type="checkbox">`, không phải input text/select thô. Không có textarea trong form; primitive `Textarea` import đã loại bỏ, nên không có lý do biến đổi textarea.
- SettingsSidebar giữ nguyên vì navigation hiện tại là semantic nav với `aria-current="page"`.
- Không sửa `dict.ts`, không thêm key; không chạy build/dev.

## Kiểm tra
- `npx tsc --noEmit`: còn lỗi ở file ngoài phạm vi đang được chỉnh song song:
  - `src/core/components/draft/overview/SceneConfigBlock.tsx`: 6 lỗi `Cannot find name 't'` (139, 152, 160, 168, 176, 184).
  - `src/core/components/draft/steps/PlaceholderStep.tsx`: không tìm thấy module `../../../shared/i18n/useT`.
- Không còn lỗi ProviderForm sau chuyển primitives.

## Ghi chú
- Nội dung snapshot của ProjectDetailSidebar tại thời điểm triển khai chỉ gồm overview/details, không chứa nhóm 4 tab được mô tả trong task; không tự tạo panels/behavior mới để tránh thay đổi logic/layout.