# Sprint 3 — ESLint design-system gate

**Trạng thái:** DONE · không thêm dependency · không sửa `src/**`.

## Rule và phạm vi

Block `no-restricted-syntax` áp dụng cho `src/**/*.tsx`, với thông báo giải thích cách thay thế:

| Nhóm | Selector | Message / ý nghĩa |
|---|---|---|
| Arbitrary font-size | `Literal[value=/text-\\[\\d+px\\]/]` | Dùng typography token đã duyệt. |
| Arbitrary radius | `Literal[value=/rounded-\\[/]` | Dùng radius token. |
| Shadow rời | `Literal[value=/\\bshadow-(2xs|xs|sm|md|lg|xl|2xl|inner)\\b/]` | Dùng `.tier-overlay` thay vì shadow utility rời. |
| Surface trắng đục | `Literal[value=/\\bbg-white\\b/]` | Dùng surface token. |
| Bỏ focus outline | `Literal[value=/focus:outline-(hidden|none)/]` | Dùng `.focus-ring`. |
| Hex trong TSX | `Literal[value=/#[0-9a-fA-F]{3,8}/]` | Dùng color token. |
| Dark override token | `Literal[value=/dark:[a-z:]*-(fg|surface|line|accent)/]` | Token tự đổi theo theme, bỏ `dark:` override. |

### Exclusions

- `src/app/routes/config.tsx`, `src/app/routes/worldbuilding.tsx`: route mock được design system §1/§8 loại trừ.
- `src/shared/components/background/AuroraBackground.tsx`: hiệu ứng trang trí dùng arbitrary palette/geometry.
- `src/core/components/chat/OrbSphere.tsx`: hiệu ứng trang trí và highlight trắng được cho phép theo §8.
- `src/core/components/character/CreateCharacterDialog.tsx`: hex values là dữ liệu màu người dùng chọn, không phải CSS.
- `src/core/components/sidebar/SidebarHistory.tsx`: `bg-white` ở knob switch được cố ý cho phép; file không bị bỏ khỏi baseline ESLint, chỉ ignore trong block gate này.

## Kết quả

- `npm run lint` (`eslint .`): **pass, exit 0, 0 error**.
- `npx eslint src --max-warnings=0`: **pass, exit 0, 0 error**.
- Không còn vi phạm design gate ngoài các file loại trừ ở trên.
- `package.json`: chỉ thêm script `"lint": "eslint ."`; không đổi dependencies.

## Cách xác minh 7 rule

Chạy một probe không lưu file:

```sh
printf 'const Probe = () => <div className="text-[10px] rounded-[3px] shadow-md bg-white focus:outline-none #abc dark:text-fg" />;\n' | npx eslint --stdin --stdin-filename src/__lint_probe__.tsx --no-warn-ignored
```

Probe cho đúng **7 lỗi `no-restricted-syntax`**, mỗi lỗi tương ứng một nhóm rule. ESLint cũng báo unused-vars warning do probe chỉ phục vụ kiểm thử; warning không liên quan gate. Không có file source nào được tạo/sửa để thử.

## Đề xuất

Giữ danh sách exclusion có chủ đích và thu hẹp/xóa khi các route mock hoặc dữ liệu trang trí được chuẩn hóa. `aria-label` cho icon-only chưa được thêm vào 7 rule vì yêu cầu Sprint 3 này giới hạn đúng các selector nêu trên.
