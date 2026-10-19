# Sprint 11.9 — ĐÃ TÁCH

> **Trạng thái: SPLIT (2026-10-19).** Sprint này không còn được chạy như một khối. Nội dung đã chia thành
> hai sprint độc lập, chạy **tuần tự** (không song song — cả hai đều sửa `src/index.css`, và repo không có
> git để cứu khi hai agent ghi đè nhau).

| Sprint | Nội dung | File |
|---|---|---|
| **11.91** | Render markdown · nút sao chép · nhãn `Tổng: X.Xs` · history mũi tên | [`sprint-11.91.md`](sprint-11.91.md) |
| **11.92** | Bỏ khối "Local Mode" · truy nguyên & giảm CPU của atmosphere (blob / backdrop-filter) | [`sprint-11.92.md`](sprint-11.92.md) |

## Vì sao tách

1. **Khác loại rủi ro.** 11.91 là viết feature thuần frontend (parser, renderer, footer, bàn phím).
   11.92 là **đo lường + tối ưu hiệu năng**, cần chạy app thật và lấy số `%CPU` — kiểu việc dễ nuốt trọn
   thời gian một sprint.
2. **Nếu gộp, phần lag dễ bị bỏ rơi.** Sprint 11.9 gộp có 5 việc feature xếp trước việc tối ưu. Một agent
   hết hơi giữa đường sẽ bỏ đúng cái người dùng khó chịu nhất. Tách ra thì 11.92 không thể bị lãng quên.
3. **11.92 cần một bước truy nguyên riêng.** Có hai nghi phạm (4 blob `blur(64px)` animate `scale`, và
   `backdrop-filter: blur(12px)` trên các `.surface`), chưa biết cái nào chiếm phần lớn. Tối ưu trước khi
   truy nguyên là tối ưu sai chỗ.

## Trạng thái cây khi tách

Một lần chạy 11.9 gộp đã bị dừng giữa chừng, **không để lại thay đổi thực chất nào**:
`src/pages/Chat.tsx` vẫn còn `Tổng thời gian lượt` và meta line cũ, không có `Tổng:`, không có copy, không có
markdown; `src/lib/markdown.ts` và `src/components/Markdown.tsx` chưa tồn tại; `pnpm build` xanh.
