# 🔓 SPRINT 15 — REFACTOR: TỤ JSON + TÁCH MODULE + DESIGN PATTERN

> ## STATUS: UNLOCKED — superseded by Sprint 16 / Sprint 16.1
>
> **Đã đủ điều kiện mở khoá (2026-10-20).** Phạm vi còn lại được tách lại và giao cho:
> - [`sprint-16.md`](sprint-16.md) — schema v2.1 + docs (`system.dbml`, `super.json`, design docs).
> - [`sprint-16.1.md`](sprint-16.1.md) — tool contract + prompt (`add_row`/`add_field`, `_json_index`).
>
> Phần tách module đã xong ở Phase 2 (P2-S1–P2-S6); phần design-pattern đã xong ở Phase 1 (S0–S8).
> Tài liệu dưới đây giữ nguyên như bản khảo sát lịch sử, không phải spec thi công.
> - Điều kiện 1 (Cơ chế rollback / git): Đã thoả (git repo hoạt động, commit baseline, branch + CI).
> - Điều kiện 2 (Sprint 11.6 IPC contract test): Đã thoả từ 2026-10-19.
> - Điều kiện 3 (Không còn sprint hotfix dở): Phase 1 (S0–S8) và Phase 2 (P2-S1–P2-S6) đã xong.
> - Điều kiện 4 (Xác nhận lại phạm vi với mã nguồn hiện tại): Đã đối chiếu ngày 2026-10-20.


---

## 1. Vì sao bị khoá

Người dùng đã chọn **hoãn** nhóm việc này khi được hỏi, với lý do cụ thể:

1. **Repo KHÔNG có git.** `git rev-parse --is-inside-work-tree` → `fatal: not a git repository`.
   Refactor đổi cấu trúc module mà không có VCS nghĩa là không rollback, không diff, không biết
   file nào đã đổi. Người dùng từ chối `git init` ở thời điểm đó.
2. Sprint 11.6 đang chạy để bịt lỗ hổng verification (IPC contract test). Trộn refactor vào cùng lúc
   sẽ làm nhiễu nguyên nhân khi test fail.
3. Lý do người dùng nêu ban đầu: *"tools.rs nếu phình to rất khó sửa"* — đây là **nợ kỹ thuật có thật**,
   không phải ý thích. Nhưng nó chưa chặn được gì ở thời điểm hiện tại, nên hoãn là hợp lý.

---

## 2. Điều kiện mở khoá (tất cả phải thoả)

- [x] **Có cơ chế rollback.** Git repository đã hoạt động, workflow theo nhánh sprint, commit baseline đầy đủ.
- [x] ~~Sprint 11.6 (IPC contract test) đã DONE và `cargo test` xanh.~~
      **Đã thoả 2026-10-19**: 71 passed / 0 failed / 3 ignored, 0 warning. Hiện tại hệ thống đạt 127 test passed.
- [x] Không còn sprint hotfix nào đang dở (Phase 1 sprints S0–S8 và Phase 2 P2-S1–P2-S6 đã hoàn tất).
- [x] Người dùng yêu cầu đối chiếu lại phạm vi và mã nguồn hiện tại (2026-10-20). Đã đối chiếu hoàn tất.

---

## 3. Vấn đề đã VERIFY (số liệu thật, đo ngày 2026-10-19)

### 3.1 `tools.rs` trộn 5 mối quan tâm

`src-tauri/src/tools.rs` — **718 dòng**, `#[cfg(test)]` bắt đầu ở **dòng 325** ⇒ **~393 dòng production**
chứa 5 nhóm việc khác hẳn nhau:

| # | Mối quan tâm | Item trong file |
|---|---|---|
| 1 | Danh sách tên tool | `TOOL_NAMES` (5 phần tử) |
| 2 | Nhãn tiếng Việt cho UI | `PATH_LABELS` (25 phần tử), `path_label()`, `status_message()`, `tool_path()` |
| 3 | JSON Schema gửi cho model | `tool_specs()` (~100 dòng) |
| 4 | Sinh id project | `slugify()`, `unique_project_id()` |
| 5 | Dispatch + guard | `execute_tool()`, `project_or_error()`, `NO_PROJECT` |

Điểm đáng chú ý: nhãn UI (nhóm 2) không phải mối quan tâm của "tool" — nó là **presentation**. Hiện nó
nằm trong `tools.rs` chỉ vì lần đầu viết thì tiện tay.

### 3.2 `runtime.rs` còn to hơn `tools.rs`

`src-tauri/src/runtime.rs` — **1591 dòng**, `#[cfg(test)]` bắt đầu ở **dòng 525**
⇒ **~524 dòng production + ~1067 dòng test**. Người dùng chưa nhắc tới file này, nhưng nó là file lớn
nhất repo và gánh ít nhất: agent loop, phát event, gọi tool, persist DB, ráp system prompt, và các
wrapper cho command.

### 3.3 Kích thước toàn bộ backend

| File | Tổng dòng | Ghi chú |
|---|---|---|
| `runtime.rs` | 1591 | test bắt đầu dòng 525 |
| `adapters.rs` | 895 | transport + trait `LlmBackend` + 2 impl + parsing |
| `super_json.rs` | 761 | skeleton + path walk + validate + progress + atomic IO |
| `tools.rs` | 718 | test bắt đầu dòng 325 |
| `db.rs` | 327 | |
| `models.rs` | 304 | |
| `store.rs` | 165 | |
| `lib.rs` | 164 | command surface + `generate_handler!` + test |
| `test_support.rs` | 87 | |
| `paths.rs` | 85 | |
| `main.rs` | 6 | |
| **Tổng** | **5103** | |

### 3.4 JSON nằm rải 3 nơi

| Nhóm | Vị trí | Số file | Ai đọc |
|---|---|---|---|
| Runtime schema | `docs/schema/runtime/` | 4 (`super`, `chapter`, `narrative`, `scene`) | `super.json` bị `include_str!` ở `super_json.rs:13` **và** `lib.rs:57` |
| Seed data | `docs/schema/seed/style_presets/_seed_parts/` | 9 (`*.devices.json`) | chưa xác minh |
| Prompt rules | `docs/prompts/rules/` | 2 (`absolute_rules.json`, `business_rules.json`) | **không ai đọc** |

### 3.5 Thư mục chết / mồ côi

| Đường dẫn | Trạng thái | Bằng chứng |
|---|---|---|
| `src-tauri/prompt/` (không "s") | **CHẾT** | 413 bytes, `grep` toàn repo không có tham chiếu; `runtime.rs:20` dùng `../prompts/harness_system.md` (7683 bytes) |
| `docs/prompts/` | **MỒ CÔI** | 4 file (`rules/*.json`, `chapter_rules.txt`, `scene_rules.txt`); `grep -rn "docs/prompts" src-tauri/src/ src/` → không có kết quả |

> `src-tauri/prompt/` **đã bị xoá ở Sprint 11.6** (2026-10-19). Ba thư mục còn lại vẫn nguyên:
> `src-tauri/prompts/` (đang sống, `include_str!` từ `runtime.rs:20`), `docs/prompts/` (mồ côi, 4 file),
> `docs/schema/runtime/` (4 file, `super.json` bị `include_str!` ở 2 chỗ).

---

## 4. Vấn đề CHƯA VERIFY (đừng tin, phải đo lại)

Hai subagent khảo sát đã bị dừng khi người dùng hoãn refactor, nên các mục này **chưa có số liệu**:

- [ ] **Bản đồ coupling đầy đủ** giữa `runtime.rs` ↔ `tools.rs` ↔ `adapters.rs`. Cụ thể chưa biết
      `runtime.rs` có "với tay qua" trait `LlmBackend` để dùng thẳng `HttpBackend` không.
- [ ] **Ai gọi vào nội bộ `super_json.rs`** (`VALID_PATHS`, `field_kind`, `skeleton`) thay vì API public.
- [ ] **"Độ mở để mở rộng" thực tế:** thêm 1 tool mới phải sửa bao nhiêu chỗ, ở những file nào?
      Thêm 1 loại provider mới phải sửa bao nhiêu chỗ? Đây là thước đo định lượng cho câu hỏi
      "chọn design pattern gì" — chưa đo thì chưa được chốt pattern.
- [ ] **Trùng lặp/near-duplicate** giữa `docs/schema/runtime/*.json` và `docs/schema/system.dbml`, và
      giữa `docs/prompts/rules/*.json` với `src-tauri/prompts/harness_system.md`.
- [ ] **Danh sách đầy đủ** mọi chỗ `include_str!` / `import` trỏ tới file JSON (mới biết 2 chỗ của
      `super.json`).

---

## 5. Ứng viên quyết định (CHƯA CHỐT — cần đo mục 4 trước)

Ghi lại để không phải nghĩ lại từ đầu:

| Chủ đề | Ứng viên | Ghi chú |
|---|---|---|
| Tool | Registry: mỗi tool là một struct/impl đăng ký vào một bảng, thay cho `match` + `tool_specs()` phình ra | Phải đo trước: hiện thêm 1 tool sửa mấy chỗ? |
| Provider | Trait đã có sẵn (`LlmBackend` + `HttpBackend`) — câu hỏi là phần **config/store** có cần pattern riêng không | Đừng thiết kế lại thứ đã đủ tốt |
| Nhãn UI | Tách `PATH_LABELS`/`status_message` ra khỏi `tools.rs` — đây là presentation, không phải tool | 25 nhãn, có test `test_status_message_covers_all_25_paths` đang giữ |
| JSON | Một cây thư mục gốc, phân nhóm theo **loại** (code-embedded template / reference schema / seed / prompt rules / config) | `docs/` là của người dùng; mọi `include_str!` là đường dẫn **compile-time**, đổi là vỡ build |
| Module | Tách `runtime.rs` theo mối quan tâm (loop / event / prompt assembly / persistence) | File lớn nhất repo, chưa ai nhắc nhưng là ứng viên số 1 |

**Nguyên tắc bắt buộc khi chốt:** đo trước, chốt sau. Không chọn pattern vì nó nghe hay — chọn vì số
liệu ở mục 4 chỉ ra đau ở đâu.

---

## 6. Ràng buộc cho người thực thi (khi được mở khoá)

- **KHÔNG thêm dependency nào** (Rust lẫn npm) trừ khi người dùng đồng ý rõ ràng.
- **`cargo test` phải xanh sau TỪNG bước nhỏ**, không phải chỉ ở cuối. Refactor 5000 dòng mà để test đỏ
  suốt là cách chắc chắn nhất để mất khả năng truy nguyên.
- **Không đổi hành vi.** Sprint này không thêm feature, không đổi UI/UX, không đổi format
  `~/.merlink/config.json`, không đổi shape `super.json` / 25 path.
- **`docs/` là của người dùng.** Di chuyển file trong đó phải hỏi trước, và phải cập nhật mọi
  `include_str!` cùng lúc.
- **Kiểm tra lại mọi số liệu ở mục 3** trước khi dùng — chúng đo ngày 2026-10-19 và sprint 11.6+ sẽ làm
  chúng lệch.

---

## 7. Phác thảo task (DRAFT — phải viết lại thành spec đầy đủ khi mở khoá)

1. Backup/rollback mechanism được xác nhận (điều kiện mở khoá).
2. Đo lại toàn bộ mục 4, ghi số liệu thật vào chính file này.
3. Chốt pattern dựa trên số liệu, viết ADR ngắn cho từng quyết định.
4. Tách theo từng bước nhỏ, mỗi bước một commit/backup + `cargo test` xanh.
5. Dọn thư mục chết còn lại (`docs/prompts/` nếu người dùng xác nhận).
6. Verify: `cargo test`, `pnpm build`, `pnpm tauri dev` sống > 30s, không đổi hành vi.

---

## 8. OUT OF SCOPE (kể cả khi đã mở khoá)

- Không thêm feature mới, không đổi UI/UX.
- Không port `docs/schema/system.dbml` sang SQLite.
- Không làm session list/resume/rename/delete (người dùng chưa chốt).
- Không làm dialog gợi ý option, không viết logic sinh dàn ý.
- Không đổi công thức tiến độ 70%/30%, không đổi giới hạn 5 vòng của agent loop.

---

## 9. Lịch sử

| Ngày | Việc |
|---|---|
| 2026-10-19 | Người dùng nêu vấn đề: tools.rs phình to, JSON rải rác, cần design pattern cho tool/provider |
| 2026-10-19 | Khảo sát ban đầu: xác định 5 mối quan tâm trong `tools.rs`, kích thước `runtime.rs`, 3 nơi chứa JSON, 2 thư mục chết |
| 2026-10-19 | Người dùng từ chối `git init` → **KHOÁ sprint này**, hoãn sang sprint 15 |
| 2026-10-19 | 2 subagent khảo sát (coupling map + JSON inventory) bị dừng theo quyết định hoãn → mục 4 vẫn trống |
| 2026-10-19 | Sprint 11.6 DONE (71 test xanh) → điều kiện mở khoá #2 thoả. **Sprint vẫn LOCKED** vì chưa có cơ chế rollback |
| 2026-10-20 | Đã có git rollback; Phase 1 (S0–S8) và Phase 2 (P2-S1–P2-S6) đã xong phần tách module + design pattern. Sprint 15 **UNLOCKED**; phạm vi schema/tool còn lại chuyển sang Sprint 16 và 16.1. |
