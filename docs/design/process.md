Cây thư mục `meta/character/character001/` của m đã hình thành xong khung xương nền tảng cho một nhân vật (nhận dạng, tâm lý, năng lực, tri thức, ký ức và quan hệ xã hội).

Nhìn vào cây thư mục tổng thể hiện tại, m đang đứng trước ngã rẽ giữa việc **hoàn thiện dữ liệu tĩnh (Meta)** hay **chuyển sang thiết lập thực thi (Content)**. Dưới đây là thứ tự ưu tiên logic tiếp theo:

---

### Ưu tiên 1: Định hình `meta/overview/` và `meta/theme/` (Mỏ neo toàn bộ dự án)

Trước khi viết bất kỳ scene nào trong `content/` hay điền chi tiết hàng chục thành phố trong `settings/`, m phải có **file định hướng cấp cao (High-level Anchor)**. Nếu thiếu cái này, AI khi sinh dàn ý hay viết văn sẽ mất định hướng về thông điệp và tông giọng chung.

* `meta/overview/project-overview.json`:
* Tên tác phẩm, logline (1 câu tóm tắt cốt lõi), tóm tắt toàn truyện (synopsis).
* Đối tượng độc giả mục tiêu, rating độ tuổi.
* Dung lượng dự kiến: số Arc/Volume, số Chapter, target word count.
* `meta/theme/theme.json`:
* Chủ đề tư tưởng trung tâm (Central Theme - ví dụ: *Sự tha hóa của quyền lực*, *Cái giá của sự bất tử*).
* Thông điệp cốt lõi (Premise / Controlling Idea).

---

### Ưu tiên 2: Tạo Schema kết nối không gian tại `meta/world-building/settings/`

Cây thư mục `realm/world/region/country/city/facility` đang lồng nhau 6 tầng sâu. Giờ là lúc tạo file JSON đại diện cho các địa điểm diễn ra những cảnh đầu tiên (ví dụ: một địa điểm cụ thể trong `city/` hoặc `facility/`).

Mỗi file địa điểm cần chứa các trường mà Scene sẽ gọi vào:

```json
{
  "location_id": "LOC_FACILITY_001",
  "name": "Chiến hạm kỳ hạm Đế Quốc",
  "parent_ref": "city_001",
  "spatial_features": "Hành lang kim loại hẹp, trần cao phủ kết giới ma pháp",
  "sensory_defaults": {
    "visual": "Ánh đèn ma pháp le lói, tông màu xám lạnh của kim loại",
    "auditory": "Tiếng động cơ ma đạo gầm rú trầm đục",
    "olfactory": "Mùi ozon khét lẹt của kết giới bị quá tải"
  }
}
```

---

### Ưu tiên 3: Dựng khung `content/` (Bắt đầu với Arc 1 và Chapter 1)

Khi đã có nhân vật (`character001`), tông giọng tổng thể (`overview`), và địa điểm tối thiểu (`settings`), m chuyển thẳng sang nhánh **`content/`** để triển khai cấu trúc trần thuật:

```text
content/
└── volume001/
    └── chapter001/
        ├── chapter-overview.json    <-- Dàn ý chapter, danh sách beats
        └── scenes/
            ├── scene001.json         <-- Target scene đầu tiên
            └── scene002.json

```

File `scene001.json` sẽ là nơi tổng hợp và tham chiếu tất cả những gì m đã làm:

* Trỏ tới nhân vật: `meta/character/character001`
* Trỏ tới phong cách viết: `entity/style/action/combat.json`
* Trỏ tới địa điểm: `meta/world-building/settings/.../facility_001.json`
* Gán chỉ số giác quan (Sensory intensity) và kỹ thuật tự sự (POV, Subtext, Dramatic irony).

---

### Bước hành động khuyến nghị ngay lúc này

Tập trung xử lý ngay **`meta/overview/project-overview.json`** để chốt thông tin dự án (như bước tạo form bắt buộc mà m đã vạch ra trong UX flow), sau đó nhảy thẳng vào dựng cấu trúc **`content/volume001/chapter001/`** để kiểm thử quy trình generate scene đầu tiên của AI, Sugoi na!
