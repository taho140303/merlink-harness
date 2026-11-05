## TEMPLATE BẢNG USECASE

| Trường | Nội dung | Ghi chú |
|---|---|---|
| **Mã UC** | UC_[Module]_[STT] | VD: UC_ORDER_01 |
| **Tên Use Case** | [Động từ + Đối tượng] | VD: "Đặt hàng trực tuyến" |
| **Module/Phân hệ** | ... | VD: Quản lý đơn hàng |
| **Actor chính** | ... | Người khởi tạo usecase |
| **Actor phụ** | ... | Hệ thống/dịch vụ liên quan (VD: VNPay, SMS Gateway) |
| **Mô tả ngắn** | 1-2 câu tóm tắt mục đích usecase | |
| **Trigger** | Sự kiện kích hoạt usecase | VD: Khách bấm "Thanh toán" |
| **Precondition** | Điều kiện trước khi chạy | VD: User đã đăng nhập, giỏ hàng có SP |
| **Postcondition** | Trạng thái sau khi chạy thành công | VD: Đơn hàng được tạo ở trạng thái "Chờ xử lý" |
| **Main Flow** | 1. ...<br>2. ...<br>3. ... | Luồng chính (happy path) |
| **Alternative Flow** | 2a. Nếu ... thì ...<br>2b. Nếu ... thì ... | Luồng rẽ nhánh hợp lệ |
| **Exception Flow** | E1. Nếu ... thì ...<br>E2. Nếu ... thì ... | Luồng lỗi/ngoại lệ |
| **Business Rules** | BR1: ...<br>BR2: ... | Quy tắc nghiệp vụ ràng buộc |
| **Priority** | High / Medium / Low | |
| **Ghi chú** | ... | Link tài liệu, phụ thuộc UC khác |

---

## 🔹 VÍ DỤ MẪU (để AI bám theo format)

| Trường | Nội dung |
|---|---|
| **Mã UC** | UC_ORDER_01 |
| **Tên Use Case** | Đặt hàng trực tuyến |
| **Module** | Quản lý đơn hàng |
| **Actor chính** | Khách hàng |
| **Actor phụ** | Cổng thanh toán VNPay, Hệ thống kho |
| **Mô tả ngắn** | Khách hàng chọn sản phẩm và tạo đơn hàng trên website |
| **Trigger** | Khách hàng bấm nút "Đặt hàng" |
| **Precondition** | - Khách đã đăng nhập<br>- Giỏ hàng có ≥1 sản phẩm<br>- Sản phẩm còn tồn kho |
| **Postcondition** | - Đơn hàng được tạo với trạng thái "Chờ xác nhận"<br>- Tồn kho được giữ (reserved)<br>- Email xác nhận được gửi |
| **Main Flow** | 1. Khách mở giỏ hàng<br>2. Hệ thống hiển thị danh sách SP + tổng tiền<br>3. Khách chọn địa chỉ giao hàng<br>4. Khách chọn phương thức thanh toán<br>5. Khách bấm "Xác nhận đặt hàng"<br>6. Hệ thống tạo đơn, giữ tồn kho<br>7. Hệ thống gửi email xác nhận<br>8. Hiển thị mã đơn hàng |
| **Alternative Flow** | 3a. Nếu chưa có địa chỉ → chuyển sang UC_PROFILE_02 (Thêm địa chỉ)<br>4a. Nếu chọn VNPay → chuyển sang UC_PAY_01 (Thanh toán online) |
| **Exception Flow** | E1. Sản phẩm hết hàng → thông báo, yêu cầu cập nhật giỏ<br>E2. Thanh toán thất bại → đơn ở trạng thái "Chờ thanh toán", cho retry trong 15 phút |
| **Business Rules** | BR1: Đơn tối thiểu 50.000đ<br>BR2: Mỗi SP tối đa 10 cái/đơn |
| **Priority** | High |
| **Ghi chú** | Phụ thuộc UC_PAY_01, UC_INVENTORY_01 |