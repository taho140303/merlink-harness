# Sprint 5 — Multiline Chat Input

## Thay đổi
- `ChatInputBar`: thay input bằng textarea `rows={1}`; tự tăng chiều cao dựa trên `scrollHeight`, giới hạn bằng `max-h-52` (~208px), vượt ngưỡng thì cuộn dọc. Chiều cao và overflow được reset sau khi gửi.
- Input căn icon theo dòng đầu; áp dụng `resize-none`, `leading-relaxed` và giữ `tier-input focus-ring rounded-xl text-sm py-2`.
- `ChatMessageItem`: user messages render plain text với `whitespace-pre-wrap break-words`; Markdown của assistant không đổi.

## Phím tắt
- Enter không modifier gửi; Shift+Enter và Alt+Enter xuống dòng.
- Bỏ qua Enter khi `nativeEvent.isComposing` để tránh gửi nhầm trong IME.

## Kiểm tra
- `npx tsc --noEmit`: PASS (exit 0).
- `npm run lint`: PASS (exit 0).
- `npm run build`: PASS (exit 0).
- Visual/browser lần đầu: chưa xác nhận do Chrome không khởi chạy (`No usable sandbox`).
- Browser retest Alt+Enter trên `http://localhost:5199/`: nhập `dòng 1`, nhấn Alt+Enter, nhập `dòng 2`; giá trị textarea xác nhận có newline giữa hai dòng và ô tự cao thêm một dòng. Screenshot lưu tại `/tmp/sprint5-alt-enter.png`. Lưu ý browser daemon đã chạy sẵn nên cảnh báo `--args` bị bỏ qua; lần khởi động ban đầu vẫn mở trang và kiểm tra tương tác thành công.
