À, đúng. Cái m đang lo thực ra **không phải chỉ là version control**, mà là bài toán:

> **Long-lived dynamic state → context bloat + storage bloat + historical reconstruction.**

Và tao nghĩ cách tốt nhất không phải chọn giữa **“một dynamic state khổng lồ”** và **“snapshot mỗi scene”**, mà là tách **source of truth** khỏi **context snapshot/cache**.

---

# 1. Có 3 thứ khác nhau

Tao sẽ định nghĩa:

```text
Knowledge / Competence
        │
        ├── Canonical State
        │
        ├── State History
        │
        └── Context Snapshot
```

### Canonical State

Trạng thái hiện tại thật sự:

```json
{
  "knowledge": [...],
  "competence": {...}
}
```

### State History

Những gì làm state thay đổi:

```text
Scene 001 → +A
Scene 004 → +B
Scene 017 → competence +0.1
Scene 023 → -C
```

### Context Snapshot

Một **bản rút gọn được materialize cho một scene**, chỉ chứa thứ scene đó cần.

Đây mới là thứ giải quyết context overflow.

---

# 2. Không nên snapshot toàn bộ knowledge mỗi scene

Ví dụ nhân vật đã sống qua 200 chapter:

```text
Knowledge:
A
B
C
D
...
X
Y
Z
```

Scene 201 chỉ cần:

```text
A
D
K
```

Thì generator **không cần nhận A→Z**.

Nó nhận:

```json
{
  "knowledge_context": [
    "A",
    "D",
    "K"
  ]
}
```

Nhưng canonical state vẫn biết nhân vật đang sở hữu toàn bộ knowledge.

---

# 3. Vậy snapshot nên là "view", không phải "copy"

Đây là điểm quan trọng.

Đừng:

```text
scene001_snapshot.json = copy(character.knowledge)
scene002_snapshot.json = copy(character.knowledge)
scene003_snapshot.json = copy(character.knowledge)
```

Mà:

```text
Character State
       │
       ▼
Context Resolver
       │
       ├── relevance
       ├── POV
       ├── scene summary
       ├── relationships
       └── current events
       │
       ▼
Scene Context
```

Tức là snapshot được **derive khi cần**.

---

# 4. Nhưng m nói đúng: derive lại mỗi lần cũng tốn

Đây là lúc cache xuất hiện.

Có thể có:

```text
canonical/
history/
cache/
```

Ví dụ:

```text
character/
└── yang-guang/
    ├── canonical.json
    ├── history/
    │   ├── scene-001.delta.json
    │   ├── scene-004.delta.json
    │   └── scene-017.delta.json
    │
    └── cache/
        ├── chapter-001.json
        ├── chapter-010.json
        └── chapter-020.json
```

Nhưng `cache` **không phải source of truth**.

Xóa cache cũng không mất dữ liệu.

---

# 5. Thậm chí không cần lưu snapshot mỗi scene

Có thể materialize theo **checkpoint**.

Ví dụ:

```text
Scene 001 ──Δ── Scene 002 ──Δ── ... ──Δ── Scene 020
     │                                      │
     └──────── checkpoint ──────────────────┘
```

Mỗi 20 scene:

```text
checkpoint-020
```

Sau đó scene 37 cần state:

```text
checkpoint-020
+
delta 21
+
delta 22
...
+
delta 37
```

Thay vì:

```text
delta 1 → delta 37
```

Tức là:

### Checkpoint + Delta

Đây là kiểu kiến trúc rất hợp với bài toán của m.

---

# 6. Knowledge còn có một vấn đề khác: nó không nên là một array khổng lồ

Đừng để:

```json
"knowledge": [
    "biết A",
    "biết B",
    "biết C",
    "biết D",
    ...
]
```

vì về lâu dài sẽ rất khó quản lý.

Tao sẽ chia thành **knowledge units**:

```json
{
  "id": "knowledge.sui.canal_system",
  "domain": "history",
  "content": "Đại Vận Hà tồn tại...",
  "acquired_at": "scene-017",
  "confidence": 0.9,
  "source": "character-observation"
}
```

Competence cũng vậy:

```json
{
  "domain": "politics",
  "level": 0.45,
  "evidence": [
    "scene-012",
    "scene-031"
  ]
}
```

---

# 7. Quan trọng hơn: Knowledge cần `relevance`

Không phải knowledge nào cũng đáng bơm vào context.

Ví dụ Dương Quảng có 10.000 knowledge units:

```text
Nông nghiệp
Quân sự
Tùy triều
Quan hệ Dương Dũng
Tài chính
Địa lý
...
```

Scene:

> Dương Quảng nói chuyện với tướng quân về quân lương.

Resolver chỉ lấy:

```text
military
logistics
taxation
relationship with general
recent relevant events
```

Không lấy:

```text
kiến thức về nông cụ Giang Nam
lịch sử Nam Bắc triều
sở thích cá nhân của Dương Dũng
```

---

# 8. Competence cũng không cần nhét toàn bộ lịch sử

Ví dụ:

```json
"competence": {
    "politics": 0.72,
    "military": 0.31,
    "agriculture": 0.18,
    "administration": 0.61
}
```

Scene chỉ cần:

```text
politics: 0.72
administration: 0.61
```

Còn **evidence** nằm ngoài context.

Đừng đưa:

> “politics = 0.72 vì scene 12, 19, 24, 31...”

cho model mỗi lần.

---

# 9. Và đây là lý do tao nghĩ `Dynamic State` nên có 2 tầng

```text
DYNAMIC STATE
│
├── CURRENT
│   ├── knowledge
│   ├── competence
│   ├── beliefs
│   └── relationships
│
└── HISTORY
    ├── deltas
    ├── events
    └── checkpoints
```

Khi generate:

```text
CURRENT
   ↓
Context Resolver
   ↓
Relevant Context
   ↓
LLM
```

Khi sửa:

```text
OLD SCENE
   ↓
State History
   ↓
Rebuild from affected point
   ↓
CURRENT'
```

---

# 10. Vậy storage thực tế sẽ không khủng như m nghĩ

Ví dụ 1 knowledge unit:

```json
{
    "id": "knowledge.xxx",
    "content": "...",
    "source": "scene-032"
}
```

Nó chỉ tồn tại **một lần**.

Scene 50 cần nó?

```text
reference → knowledge.xxx
```

Scene 100 cần nó?

```text
reference → knowledge.xxx
```

Không copy.

Tương tự competence:

```text
competence.politics
    current = 0.72
```

History:

```text
scene-010 +0.05
scene-023 +0.08
scene-041 +0.04
```

---

# 11. Cái hay nhất: Scene Snapshot có thể cực nhỏ

Ví dụ scene 300:

```json
{
    "scene_id": "scene-300",

    "character_context": {
        "yang-guang": {
            "knowledge": [
                "knowledge.tax_reform",
                "knowledge.yang_su_military_power"
            ],
            "competence": {
                "politics": 0.72,
                "administration": 0.61
            }
        }
    }
}
```

Chỉ vậy thôi.

Không phải:

```text
10.000 knowledge units
+ 500 historical events
+ 300 scene states
```

---

# 12. Và cái này còn giải quyết vấn đề "quay lại sửa Chapter 1"

Ví dụ:

```text
OFFICE v1
│
├── Chapter 1
├── Chapter 2
├── Chapter 3
└── ...
```

User fork:

```text
OFFICE v1
     ↓
RAW v2
     ↓
sửa Chapter 1
```

LinearFlow biết:

```text
Chapter 1 changed
       ↓
invalidate state after Chapter 1
       ↓
replay deltas
       ↓
rebuild canonical state
       ↓
invalidate affected caches
```

Nhưng **không cần regenerate toàn bộ prose**.

Chỉ cần regenerate những thứ bị ảnh hưởng.

---

# 13. Nên kiến trúc version của m có thể thành

```text
                    PROJECT
                       │
          ┌────────────┴────────────┐
          │                         │
        RAW                       OFFICE
          │                         │
    mutable/versioned          immutable
          │                         │
          ▼                         ▼
      State Deltas              Snapshot
          │
          ▼
   Current Dynamic State
          │
          ▼
   Context Resolver
          │
          ▼
    Scene Context Cache
```

### RAW

Có thể:

* sửa
* rollback
* branch
* regenerate
* replay state

### OFFICE

Đã chốt:

* immutable
* versioned
* encrypted
* có integrity hash

### Cache

Có thể xóa bất cứ lúc nào.

**Cache không phải dữ liệu canonical.**

---

## Và tao sẽ thêm một nguyên tắc rất quan trọng

> **Never duplicate canonical dynamic knowledge merely to optimize context retrieval. Use references, derived views, checkpoints, or caches instead.**

Và:

> **Scene context is ephemeral; character state is persistent; state history is reconstructable.**

Ba tầng này giải quyết đúng ba bài toán khác nhau:

| Tầng                      | Giải quyết                             |
| -------------------------- | ---------------------------------------- |
| **Persistent State** | Nhân vật hiện tại là ai             |
| **State History**    | Vì sao nhân vật trở thành như vậy |
| **Scene Context**    | Scene hiện tại cần biết gì          |

Nên **m không cần chọn giữa “snapshot mỗi scene” và “một đống dynamic state khổng lồ”**.

Cách hợp lý là:

**Canonical state + delta history + checkpoint + on-demand context snapshot/cache.**

Đặc biệt với LinearFlow, tao sẽ coi **context snapshot là một materialized view**, chứ không phải một bản sao state. Đây là điểm giúp hệ thống vừa không tràn context vừa không nổ storage.
