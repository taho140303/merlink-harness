Khi người dùng mới mở app sẽ vào mục chat.

Trong mục chat người dùng có thể chọn model, chọn project để bắt đầu chat, với lần đầu người dùng dùng app và chưa có project nào được tạo thì sau khi người dùng gõ tin nhắn đầu tiên, ai sẽ tự động gọi tool tạo project và dựa vào prompt mà người dùng yêu cầu để đặt tên cho project. mỗi cuộc trò chuyện sẽ có temp.json, là nơi lưu một json siêu đầy đủ về nhiều thông tin khác nhau trong quá trình ai trò chuyện với người dùng, ai không sửa file này, mà mỗi lần trò chuyện với người dùng ai sẽ gọi tool được app build sẵn để thay đổi từng trường trong này. Sau khi các trường bắt buộc đã được fill, người dùng mới có thể kêu ai bắt đầu viết. Dù người dùng nói không có ý tưởng gì thì ai cũng sẽ hỏi để tạo gợi ý, nếu ai hỏi người dùng về các trường bắt buộc mà người dùng không thể trả lời hoặc nói rằng không có ý tưởng gì, ai sẽ tự động đề xuất và gọi hàm nhưng chỉ truyền vào json, hàm nhận json đó được khởi tạo và tạo thành một dialog cho người dùng các option để chọn hoặc thêm một option nữa để người dùng tự nhập liệu, dialog có sẵn nút skip và submit, submit chỉ sáng lên khi người dùng đã chọn một option mà ai đề xuất hoặc chọn option tự nhập mà trong đó có kí tự. Ngoài ra trong file json siêu đầy đủ còn có các trường không bắt buộc, khi các trường bắt buộc được fill đầy, nút viết dàn ý được sáng lên và người dùng có thể ấn vào, trong quá trình này khi gọi siêu json vẫn thấy null thì có thể prompt cho ai tự bổ sung các phần sau.

Trong quá trình siêu json được fill sẽ có một thanh tiến độ ở header, chia thành 2 chặn là 70% với mô tả, Độ mạch lạc tạm chấp nhận và chặn 100% độ mạch lạc: tốt.

Khung chat mà ai phản hồi là một component, gồm 2 block xếp chồng. ResponseComponent(md,TemplateComponent), md là phản hồi chữ của ai.

Siêu JSON có dạng project-rooted v2 (1 `project` + 34 bảng, mỗi bảng `count`/`rows`):

> Đặc tả đầy đủ: [`docs/design/super-json.md`](super-json.md). Bản 25-leaf phẳng cũ đã bị thay thế.

## Các tool gọi:

### Bộ tool random:

**1. Nhóm tên:**

**1.1. Dựa trên các bảng:**

```sql
    Table FirstNameCharacter(id INT, char_name TEXT, gender TEXT, length INT, language VARCHAR[4]);
    Table Meaning(id INT, char_name_id INT, meaning TEXT, category TEXT);
    Table LastNameCharacter(id INT, name TEXT, info TEXT, language VARCHAR[4]);
```

Biết rằng ``Gender``, ``Lang``, ``Category`` là các ``ENUM`` được định nghĩa sẵn. Có 2 class ``Name`` và ``FullName`` được định nghĩa như sau:

```rust
pub struct Name {
    pub name: String,
    pub meaning: String,
}

pub struct FullName {
    pub first: String,
    pub last: String,
    pub priority: Lang,
    pub first_meaning: String,
    pub last_meaning: String,
}

impl FullName {
    pub fn new(first: impl Name, last: impl , priority: impl Lang) -> Self {
        Self {
            first: first::name.into(),
            last: last::name.into(),
            priority: priority,
            first_meaning: first::meaning.into()
            last_meaning: last::meaning.into()
        }
    }
}
```

**1.2. Tool:**

- Lấy tên:

```rust
    fn geneRandFirstName(length: u32, lang: Lang, gender: Option<Gender>, category Option<Category>) -> Option<Name> {}
```

- Lấy họ:

```rust
    fn geneRandLastName(lang: Lang) -> Option<Name> {}
```

> Hàm chỉ nhận ngôn ngữ vì họ không phân biệt giới tính và độ dài của từ thường mặc định là 1. Đối với tên tiếng việt dù có 2 hoặc nhiều họ, thì không cần xử lí, vì hàm này chỉ gợi ý, không phải sinh best result.

- Lấy tên:

```rust
    fn geneRandCharacterNamesOnly(count: u32, length: u32, lang: Lang, gender: Option<Gender>, category Option<Category>) -> Vec<Name> {
        let mut result: Vec<Name> = Vec::new();
        for _ in 0..quantity {
            let first: Name = geneRandFirstName(length, lang, gender, category);
            result.push(Name::new(first, lang));
        }
        result
    }
```

- Lấy tên đầy đủ:

```rust
    fn geneRandCharacterFullName(count: u32, length: u32, lang: Lang, gender: Option<Gender>, category Option<Category>) -> Vec<Name> {
        let mut result: Vec<FullName> = Vec::new();
        for _ in 0..quantity {
            let first: Name = geneRandFirstName(length-1, lang, gender, category);
            let first: Name = geneRandFirstName(lang);
            result.push(FullName::new(first, last, lang));
        }
        result
    }
```

Bộ tool tạo nhân vật:

Bộ tool tạo project:
Hàm tạo project đọc super json sao đó lấy dữ liệu của nó gọi tiếp các hàm con như tạo nhân vật, tạo quyển, tạo chương,......
