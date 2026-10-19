# RULES — Style Preset → SQL Seed

## 0. DB target (Postgres)
Schema: docs/schema/system.dbml, section SYSTEM_PROMPT.STYLE_PRESET.
Bảng liên quan:
- style_presets (id, code, category, tone_pace,
  tone_emotion, is_system, visual, auditory, tactile,
  olfactory, gustatory, language_abstract,
  syntax_sentence, syntax_variation, syntax_structure)
- rhetorical_devices (id, code, description)
- style_preset_rhetorical_devices (id, preset_id,
  rhetorical_device_id, usage_type, custom_instruction)
- style_preset_vocab_items (id, preset_id, content)
- style_preset_focus_items (id, preset_id, content)

Enum TonePace: VERY_LOW | LOW | MODERATE | FAST | VERY_FAST
Enum NarrativeWeight: NONE | LOW | MEDIUM | HIGH
Enum SentenceLength: VERY_SHORT | SHORT | MEDIUM | LONG
Enum SentenceStructure: SIMPLE | COMPOUND | COMPLEX | FRAGMENT | MIXED

## 1. Mapping JSON → SQL

### 1.1 code và category
File `style_presets/<folder>/<file>.json`:
- code = '<folder>.<file>'  (vd: 'romance_relationship.confession')
- category = '<folder>.<file>'  (giống hệt code — note DBML
  ghi 'type.variants' cho category; ta giữ full path cho
  UNIQUE-safe; Sprint sau có thể tách)
- is_system = true (mọi preset trong seed là system)

### 1.2 name
DBML KHÔNG có cột `name`. Ghi name của JSON vào dòng
comment ngay trước INSERT: `-- preset: <folder>.<file> | name: "<name>"`.

### 1.3 tone_pace (enum TonePace)
Match keyword trong `tone.pace` (lowercase, check thứ tự):
- chứa "cực nhanh" hoặc "rất nhanh" → VERY_FAST
- chứa "nhanh dần" hoặc "nhanh, dồn dập" hoặc "nhanh" → FAST
- chứa "cực chậm" hoặc "rất chậm" hoặc "cực kỳ chậm" → VERY_LOW
- chứa "chậm" hoặc "trang trọng" hoặc "nặng nề" → LOW
- chứa "vừa" hoặc "trung bình" hoặc "nhịp nhàng" hoặc
  "thong thả" hoặc "đều" → MODERATE
- fallback → MODERATE
Pace gốc (raw) ghi vào comment: `-- tone.pace.raw: "<raw>"`.

### 1.4 tone_emotion
Copy nguyên `tone.emotion` từ JSON (giữ tiếng Việt, giữ
dấu câu). Nếu JSON thiếu `tone.emotion` → empty string ''.

### 1.5 NarrativeWeight (visual, auditory, tactile,
olfactory, gustatory, language_abstract, syntax_variation)
Áp dụng cho `imagery.<key>` và `language.abstract` và
`syntax.variation`.
Normalize value:
- "rất thấp" | "rat thap" | "rất thâp" → LOW
- "thấp đến trung bình" | "thap den trung binh" → LOW
- "rất cao" / "rat cao" / "cao" → HIGH
- "trung bình" / "trung binh" / "vừa" / "medium" → MEDIUM
- "thấp" / "thap" / "thâp" / "low" → LOW
- "khong" / "không" / "none" / "" / null / missing → NONE
Giá trị lạ → NONE + ghi warning vào report.

### 1.6 SentenceLength (syntax_sentence)
Match `syntax.sentence` (raw, lowercase):
- "cực ngắn" / "rất ngắn" → VERY_SHORT
- "ngắn đến trung bình" → SHORT
- "ngắn" (không có "rất"/"cực") → SHORT
- "trung bình" (không có "dài"/"ngắn") → MEDIUM
- "dài" (không có "rất") → LONG
- "dài, trầm" / "dài, mượt" → LONG
- default → MEDIUM

### 1.7 SentenceStructure (syntax_structure)
Match `syntax.structure` (raw, lowercase, theo từ khóa):
- có >= 2 trong {"đơn", "ghép", "phức", "cụt", "hỗn"}
  → MIXED
- chỉ "câu cụt" hoặc "cụt" → FRAGMENT
- "phức hợp" hoặc "phức" → COMPLEX
- "ghép" → COMPOUND
- "đơn" → SIMPLE
- "hỗn hợp" / "mixed" → MIXED
- default → SIMPLE

### 1.8 language.vocab[]
Mỗi string trong array → 1 row
style_preset_vocab_items(content). Giữ nguyên tiếng Việt,
KHÔNG trim nội dung, KHÔNG dedupe (giữ đúng count array).

### 1.9 language.focus[]
Mỗi string → 1 row style_preset_focus_items(content).
Như 1.8.

### 1.10 rhetorical_devices
JSON có:
- rhetorical_devices.primary: "string" (1 câu mô tả)
- rhetorical_devices.secondary: ["string", ...]
- rhetorical_devices.avoid: ["string", ...]
- rhetorical_devices.intensity: "cao" | "trung bình" | "thấp"
- rhetorical_devices.examples: [] (thường rỗng — bỏ qua)

Với mỗi phrase (primary, từng secondary, từng avoid):
1. Slugify → code (xem 1.11)
2. Emit 1 link row:
   - usage_type = 'primary' | 'secondary' | 'avoid'
   - custom_instruction = 'intensity=<X>; phrase=<original>'
     (X = normalize intensity: "cao"→"high",
      "trung bình"→"medium", "thấp"→"low", missing→"medium")
3. Append {code, description=<original phrase>} vào
   devices manifest của subagent.

KHÔNG dedupe trong folder — dedupe xảy ra ở main agent.

### 1.11 slugify (dùng chính xác snippet Python này)
```python
import unicodedata, re
def slugify(s: str) -> str:
    s = unicodedata.normalize('NFD', s)
    s = ''.join(c for c in s if unicodedata.category(c) != 'Mn')
    s = s.replace('đ', 'd').replace('Đ', 'D')
    s = re.sub(r'[^a-z0-9]+', '_', s.lower())
    return s.strip('_') or 'unnamed'
```
Ví dụ: "Điệp cấu trúc vui vẻ" → "diep_cau_truc_vui_ve";
"Câu hỏi tu từ sắc bén" → "cau_hoi_tu_tu_sac_ben".

### 1.12 rules.* (không có cột — comment block)
`rules.avoid_words[]`, `rules.avoid_structure[]`,
`rules.emotion_limit` không có cột tương ứng trong DBML.
Ghi vào comment BLOCK phía trên INSERT:
```
-- preset: <folder>.<file> | name: "<name>"
-- tone.pace.raw: "<raw>"
-- rhetorical_devices.intensity: "<raw>"
-- unmapped.rules.avoid_words: ["a","b"]
-- unmapped.rules.avoid_structure: ["..."]
-- unmapped.rules.emotion_limit: "..."
```
Bỏ dòng nếu field null/missing.

## 2. Output format của MỖI subagent

### 2a. File `<folder>.sql` — idempotent DO block
```sql
-- ==== <folder> ====
-- preset: <folder>.<file> | name: "<name>"
-- tone.pace.raw: "..."
-- rhetorical_devices.intensity: "..."
-- unmapped.rules.avoid_words: [...]
DO $$
DECLARE pid uuid;
BEGIN
  IF NOT EXISTS (SELECT 1 FROM style_presets WHERE code = '<folder>.<file>') THEN
    INSERT INTO style_presets
      (id, code, category, tone_pace, tone_emotion, is_system,
       visual, auditory, tactile, olfactory, gustatory,
       language_abstract, syntax_sentence, syntax_variation,
       syntax_structure)
    VALUES
      (gen_random_uuid(),
       '<folder>.<file>',
       '<folder>.<file>',
       '<ENUM>', '<emotion string>', true,
       '<NARR>', '<NARR>', '<NARR>', '<NARR>', '<NARR>',
       '<NARR>', '<LEN>', '<NARR>', '<STRUCT>')
    RETURNING id INTO pid;

    INSERT INTO style_preset_vocab_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, v
      FROM unnest(ARRAY[
        'vocab 1', 'vocab 2'
      ]) AS v;
    -- Nếu array rỗng: bỏ hẳn INSERT vocab (không emit).

    INSERT INTO style_preset_focus_items (id, preset_id, content)
      SELECT gen_random_uuid(), pid, f
      FROM unnest(ARRAY[
        'focus 1'
      ]) AS f;

    INSERT INTO style_preset_rhetorical_devices
      (id, preset_id, rhetorical_device_id, usage_type, custom_instruction)
      SELECT gen_random_uuid(), pid, rd.id, m.usage, m.instr
      FROM rhetorical_devices rd
      JOIN (VALUES
        ('<slug>', 'primary', 'intensity=high; phrase=<orig>'),
        ('<slug>', 'secondary', 'intensity=high; phrase=<orig>'),
        ('<slug>', 'avoid', 'intensity=high; phrase=<orig>')
      ) AS m(code, usage, instr) ON m.code = rd.code;
  END IF;
END $$;
```
Mỗi preset 1 DO block, xếp theo thứ tự alphabet filename
trong folder.

### 2b. File `<folder>.devices.json`
```json
[
  {"code": "diep_cau_truc_vui_ve", "description": "Điệp cấu trúc vui vẻ"},
  ...
]
```
Union toàn bộ device code/description xuất hiện trong
folder. Nếu cùng code xuất hiện nhiều lần với description
khác nhau: giữ description của lần xuất hiện đầu tiên,
ghi các description khác vào `<folder>.report.md` mục
"device description conflicts".

### 2c. File `<folder>.report.md`
```markdown
# <folder>
- files processed: N
- presets emitted: N
- vocab items: N
- focus items: N
- device links: N (primary: P, secondary: S, avoid: A)
- unique devices: N
- warnings:
  - "confession.json: imagery.olfactory='thap' → LOW (typo 'thap')"
  - ...
- device description conflicts:
  - code `x`: kept "desc1"; also saw "desc2" (file1.json), "desc3" (file2.json)
```

## 3. Quy tắc cứng
- KHÔNG sửa file JSON gốc.
- KHÔNG chạy SQL lên DB (chỉ viết file).
- KHÔNG format lại SQL ngoài indent như ví dụ.
- Idempotency: DO block + IF NOT EXISTS là bắt buộc.
- KHÔNG dùng `INSERT ... ON CONFLICT` cho preset block
  (dùng DO/IF NOT EXISTS) — vì cần bỏ qua toàn bộ block,
  không chỉ preset row.
- Mọi chuỗi SQL escape `'` → `''` (double single-quote).
