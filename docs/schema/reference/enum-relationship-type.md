
## 3. Enum cho `category` và `type` (dùng chung mọi thể loại)

### `category`: 5 nhóm lớn

| Category | Tiếng Việt | Ví dụ `type` |
|---|---|---|
| `family` | Gia đình | `father`, `mother`, `older_brother`, `younger_sister`, `son`, `daughter`, `uncle`, `aunt`, `cousin`, `grandfather`, `grandmother` |
| `romantic` | Tình cảm | `lover`, `spouse`, `fiance`, `ex_lover`, `crush`, `soulmate`, `unrequited_love` |
| `social` | Xã hội | `friend`, `best_friend`, `acquaintance`, `neighbor`, `classmate`, `roommate`, `ally`, `comrade` |
| `professional` | Công việc | `boss`, `subordinate`, `colleague`, `mentor`, `student`, `apprentice`, `servant`, `master` |
| `antagonistic` | Đối kháng | `rival`, `enemy`, `nemesis`, `traitor`, `betrayer`, `hunter`, `prey` |

### `sub_type`: Phân loại chi tiết (chủ yếu cho `family`)

| Sub_type | Tiếng Việt | Nghĩa |
|---|---|---|
| `biological` | Ruột thịt | Cùng huyết thống |
| `adopted` | Nhận nuôi | Được nhận nuôi hợp pháp |
| `step` | Kế | Cha/mẹ kế, con riêng của vợ/chồng |
| `half` | Cùng cha/mẹ | Anh chị em cùng cha khác mẹ hoặc ngược lại |
| `foster` | Nuôi tạm | Được chăm sóc tạm thời, không hợp pháp |
| `god` | Đỡ đầu | Cha/mẹ đỡ đầu |
| `chosen` | Tự chọn | Không huyết thống nhưng tự nhận là gia đình |

### `hierarchy`: Thứ bậc

| Value | Tiếng Việt | Ví dụ |
|---|---|---|
| `superior` | Trên | Cha mẹ, anh chị, sếp, sư phụ, chủ |
| `equal` | Ngang | Bạn bè, đồng nghiệp, người yêu, anh em cùng vai vế |
| `inferior` | Dưới | Con cái, em út, cấp dưới, đệ tử, người hầu |

---

## 4. Ví dụ đầy đủ cho các trường hợp phổ biến

### 4.1. Gia đình — Chị gái ruột
```json
{
  "type": "older_sister",
  "category": "family",
  "sub_type": "biological",
  "formal_term_vi": "chị gái",
  "informal_term_vi": "chị",
  "hierarchy": "superior",
  "is_primary_role": true
}
```

### 4.2. Tình cảm — Người yêu
```json
{
  "type": "lover",
  "category": "romantic",
  "sub_type": "chosen",
  "formal_term_vi": "người yêu",
  "informal_term_vi": "em / anh",
  "hierarchy": "equal",
  "is_primary_role": true
}
```

### 4.3. Công việc — Cấp trên
```json
{
  "type": "boss",
  "category": "professional",
  "sub_type": "formal",
  "formal_term_vi": "cấp trên",
  "informal_term_vi": "sếp",
  "hierarchy": "superior",
  "is_primary_role": true
}
```

### 4.4. Đối kháng — Kẻ thù truyền kiếp
```json
{
  "type": "nemesis",
  "category": "antagonistic",
  "sub_type": "chosen",
  "formal_term_vi": "kẻ thù truyền kiếp",
  "informal_term_vi": "kẻ thù",
  "hierarchy": "equal",
  "is_primary_role": true
}
```

### 4.5. Trường hợp phức tạp — Vừa là cha vừa là kẻ thù
```json
"roles": [
  {
    "type": "father",
    "category": "family",
    "sub_type": "biological",
    "formal_term_vi": "cha",
    "informal_term_vi": "ba",
    "hierarchy": "superior",
    "is_primary_role": true,
    "active_from": "chapter001-scene01",
    "active_to": "chapter015-scene03",
    "note": "Cha ruột nhưng đã phản bội gia đình"
  },
  {
    "type": "nemesis",
    "category": "antagonistic",
    "sub_type": "chosen",
    "formal_term_vi": "kẻ thù",
    "informal_term_vi": "kẻ thù",
    "hierarchy": "equal",
    "is_primary_role": false,
    "active_from": "chapter012-scene04",
    "active_to": "chapter015-scene03",
    "note": "Từ khi phát hiện cha là kẻ chủ mưu"
  }
]
```

### 4.6. Trường hợp sư phụ — vai trò chấm dứt khi chết
```json
{
  "type": "mentor",
  "category": "professional",
  "sub_type": "formal",
  "formal_term_vi": "sư phụ",
  "informal_term_vi": "thầy",
  "hierarchy": "superior",
  "is_primary_role": true,
  "active_from": "chapter005-scene01",
  "active_to": "chapter008-scene03",
  "note": "Mất ở chapter008, nhưng ký ức vẫn ảnh hưởng đến nhân vật"
}
```

> **Lưu ý quan trọng:** `active_to` khác với `narrative_status` của nhân vật. Sư phụ chết → vai trò `mentor` **vẫn còn** trong tâm trí nhân vật, nhưng có thể đánh dấu `active_to` để biết **từ chương 8 trở đi không còn tương tác trực tiếp**.

---

## 5. Bảng tra nhanh — Từ tiếng Việt sang `type`

| Tiếng Việt | `type` | `category` | `hierarchy` |
|---|---|---|---|
| Ba, cha, bố | `father` | family | superior |
| Mẹ, má | `mother` | family | superior |
| Anh trai | `older_brother` | family | superior |
| Chị gái | `older_sister` | family | superior |
| Em trai | `younger_brother` | family | inferior |
| Em gái | `younger_sister` | family | inferior |
| Con trai | `son` | family | inferior |
| Con gái | `daughter` | family | inferior |
| Ông, bà | `grandfather`, `grandmother` | family | superior |
| Cô, dì, chú, bác | `aunt`, `uncle` | family | superior |
| Anh họ, chị họ | `cousin` | family | equal |
| Người yêu | `lover` | romantic | equal |
| Vợ, chồng | `spouse` | romantic | equal |
| Hôn phu, hôn thê | `fiance` | romantic | equal |
| Người yêu cũ | `ex_lover` | romantic | equal |
| Bạn thân | `best_friend` | social | equal |
| Bạn bè | `friend` | social | equal |
| Bạn cùng lớp | `classmate` | social | equal |
| Bạn cùng phòng | `roommate` | social | equal |
| Đồng minh | `ally` | social | equal |
| Đồng đội | `comrade` | social | equal |
| Sếp, cấp trên | `boss` | professional | superior |
| Cấp dưới | `subordinate` | professional | inferior |
| Đồng nghiệp | `colleague` | professional | equal |
| Sư phụ, thầy | `mentor` | professional | superior |
| Đệ tử, học trò | `student` | professional | inferior |
| Người hầu | `servant` | professional | inferior |
| Chủ nhân | `master` | professional | superior |
| Đối thủ | `rival` | antagonistic | equal |
| Kẻ thù | `enemy` | antagonistic | equal |
| Kẻ thù truyền kiếp | `nemesis` | antagonistic | equal |
| Kẻ phản bội | `traitor` | antagonistic | equal |


```json
{
    "id": "relationship_001",
    "dynamics": {
        "power": "inferior",
        "trust": "high",
        "respect": "high",
        "affection": "moderate",
        "hostility": "none",
        "fear": "low",
        "familiarity": "high",
        "dependency": "moderate"
    },
    "roles": [
        {
            "type": "older_sister",
            "category": "family",
            "sub_type": "biological",
            "hierarchy": "superior",
            "is_primary_role": true
        },
        {
            "type": "mentor",
            "category": "professional",
            "sub_type": "formal",
            "hierarchy": "superior",
            "is_primary_role": false
        }
    ],
    "address": {
        "self_reference": { // (tự xưng - nhân vật tự gọi mình)
            "default": "Ta", // (mặc định - dùng trong hầu hết ngữ cảnh)
            "formal": "Ta", // (trang trọng - khi nghi lễ, trước mặt kẻ thù)
            "informal": "Ta", // (thân mật - khi nói với người thân)
            "to_superior": "Thần", // (với bề trên - khi nói với vua, sư phụ)
            "to_inferior": "Ta", // (với bề dưới - khi nói với đệ tử, người hầu)
            "to_stranger": "Tôi", // (với người lạ - lịch sự, giữ khoảng cách)
            "to_lover": "Ta", // (với người yêu - thân mật)
            "internal_monologue": "Ta" // (độc thoại nội tâm - khi tự nói với mình)
        },
        "other_reference": { // (gọi đối phương)
            "to_face": "Ngươi", // (mặt đối mặt - nói trực tiếp với đối phương)
            "behind_back": "Hắn", // (sau lưng - nói về đối phương khi vắng mặt)
            "in_front_of_others": "Hắn ta", // (trước mặt người khác - khi đối phương có mặt)
            "to_third_party": "Rimuru", // (với người thứ 3 - khi kể về đối phương)
            "formal": "Rimuru-sama", // (trang trọng - khi cần lịch sự)
            "derogatory": "Tên khốn đó", // (miệt thị - khi khinh bỉ, tức giận)
            "affectionate": "Rimuru", // (trìu mến - khi thân thiết)
            "internal_monologue": "Hắn" // (độc thoại nội tâm - khi nghĩ về đối phương)
        }
    }
}
```