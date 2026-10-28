# TỔNG HỢP & ĐÁNH GIÁ 6 BÁO CÁO AUDIT DESIGN SYSTEM

**Ngày:** 2026-10-29 · **Repo:** `merlink-harness-ui` (React Router 8 / React 19 / Tailwind v4 CSS-first)
**Nguồn:** `docs/refactor-ui-reports/agent-{a..f}-report.md` (6 subagent antigravity · gemini-3.8-flash-high, read-only)
**Cách đánh giá:** đọc toàn bộ 6 report + kiểm chứng độc lập bằng grep/đọc file thật (5–8 claim/agent), đối chiếu LOC, đối chiếu chéo giữa các report. Không có file source nào bị sửa (`git status` chỉ có `docs/`).

---

## 1. Kết luận điều hành

- **6/6 report hoàn thành**, đúng thư mục, đúng chế độ read-only.
- **5/6 kết luận INCONSISTENT**; riêng A kết luận `MOSTLY CONSISTENT` và C `MOSTLY CONSISTENT` — nhưng A chỉ soi lớp nền (nơi neumorphism còn sống) còn C bỏ qua việc toàn bộ draft dùng shadow thay vì neu. Hai ngoại lệ này **không đại diện cho toàn app**.
- **Design system hiện tại KHÔNG thống nhất.** Nó là **3 phương ngữ** cùng tồn tại:
  1. **Neumorphic shell** (FullLayout, Sidebar, dialog/toast, ProjectCard, ChatInputBar, DraftOverviewBlock) — 28 lần dùng `neu-*` trong 10/85 file.
  2. **Flat-bordered data UI** (project detail, character, settings, system-prompt) — `bg-white + border-slate-200 + shadow-xs`, **0 lần** `neu-*`.
  3. **Ad-hoc glass/gradient/hex** (chat bubble `#151921`, orb, aurora blob, canvas `#6366f1`, `#cbd5e1`, `#e61380`).
- **Chất lượng bằng chứng không đều:** B và E rất chắc; A/C/F có số liệu tự chế; D sai nhiều claim cụ thể (xem §4). Vì vậy **không nên dùng report làm backlog nguyên văn** — phải lọc theo bảng kiểm chứng dưới đây.

---

## 2. Bảng tổng hợp 6 agent

| Agent | Phạm vi | LOC thật (report khai) | Findings | Verdict report | Độ tin cậy |
|:--|:--|--:|--:|:--|:--|
| **A** | app.css, root, FullLayout, routes, shared/**, Sidebar | 2.275 (2.228) | 25 | MOSTLY CONSISTENT | ⚠️ Trung bình: 4/10 claim mẫu sai |
| **B** | chat/** + route chat | 994 (994) | 21 | INCONSISTENT | ✅ Cao: 6/6 claim mẫu đúng |
| **C** | draft/** | 2.470 (2.470) | 23 | MOSTLY CONSISTENT | ⚠️ Trung bình: code dẫn đúng, metric sai |
| **D** | character/**, character-detail/**, route | 3.693 (2.553) | 25 | INCONSISTENT | ❌ Thấp: 5 claim quan trọng sai |
| **E** | project/**, home/project/chapter routes | 1.515 (1.515) | 25 | INCONSISTENT | ✅ Cao: 6/6 claim mẫu đúng |
| **F** | system-prompt/**, settings/**, config/worldbuilding | 3.821 (2.678) | 25 | INCONSISTENT | ⚠️ Trung bình: 1 P0 sai, LOC sai ~30% |

Tổng: **144 findings**, trong đó ~8 P0 (sau khi lọc còn ~4 P0 thật).

---

## 3. Bảy vấn đề gốc — cả 6 report đều chạm, đã kiểm chứng độc lập

| # | Vấn đề | Bằng chứng định lượng (tôi tự đo) | Nơi xảy ra |
|:--|:--|:--|:--|
| **1** | **Bàn phím/a11y gần như không tồn tại** | `focus-visible`: **0**; `focus:outline-hidden` trần: **39**; `aria-label`: **1** toàn app; `role="tab"/aria-selected`: **0**; `prefers-reduced-motion`: **0** | 100% module |
| **2** | **i18n nửa vời** | 46/85 file TSX dùng `useT`; **131 dòng** chứa chuỗi tiếng Việt hardcode; `tsc` fail **19 key thiếu** + **2 key trùng** trong `dict.ts` | chat, project, draft/dialog, system-prompt, character |
| **3** | **Surface drift (3 phương ngữ)** | `neu-*`: **28** lần/10 file; `shadow-*`: **134**; `bg-white` đục: **154**; module `character`, `character-detail`, `settings`, `system-prompt`: **0 neu** | toàn app |
| **4** | **Thiếu primitive nền** | `shared/ui` chỉ có dialog, confirm, delete-confirm, toast, floating-outline. **Không có** Input/Select/Tabs/Popover/Tooltip/Card/Badge | chat popover ×3 tự chế; 6 kiểu tab bar; 2 kiểu stepper |
| **5** | **Utility Tailwind chết** | `animate-in/out`: **20** chỗ nhưng **không có** plugin `tailwindcss-animate` và không có keyframes → mọi animation dialog/toast/tab **không chạy**; `duration-600`: **3** chỗ (ngoài scale) | dialog, toast, draft workspace, 3 chat popover |
| **6** | **Dialog thiếu ngữ nghĩa** | `confirm-dialog` + `delete-confirm-dialog` dùng `<h3>/<p>` thay `DialogTitle/Description`; 5 dialog instance thiếu Title/Description; 2 backdrop `fixed inset-0` tự dựng | foundation, project, draft, settings |
| **7** | **Type & radius trôi** | 4 cỡ "nhỏ" chồng nhau: `text-[10px]` 99, `text-[11px]` 111, `text-xs` 248, `text-[13px]` 2 (+ `[9px]`, `[14px]`, `[15px]`); radius: `[2rem]` (Sidebar), `[2.5rem]` (shell), `[2rem]` (ChatView), `rounded-none` badge | toàn app |

**Điểm tốt đáng giữ:** spacing bám grid 4px (không có `p-[7px]`); i18n có hạ tầng tốt (418 key, en/vi parity 100%); `cn()` + zustand + Radix được dùng đúng chỗ; shell layout (aurora z-0 / content z-10 / neu shell) là pattern tốt nên giữ.

---

## 4. Những chỗ report báo sai — phải biết trước khi dùng

| Agent | Claim trong report | Thực tế trong code |
|:--|:--|:--|
| **A** | F-06: `dialog.tsx` dùng `bg-white dark:bg-slate-900 rounded-2xl shadow-2xl` | `dialog.tsx:35` dùng `rounded-3xl p-6 neu-flat`, **không** có bg-white/shadow-2xl |
| **A** | F-10: `useTheme` thiếu listener `prefers-color-scheme` | `useTheme.ts:53` **có** `addEventListener('change', handleChange)` |
| **A** | F-15: toast dùng `bg-white shadow-lg` | `toast.tsx:42` dùng `neu-flat`, **không** shadow/bg-white |
| **A** | F-24: Sidebar dùng `bg-base-light/80 rounded-3xl` | `Sidebar.tsx:19` là `rounded-[2rem] neu-flat`, không bg-base |
| **C** | 23 lần `neu-*`, 40 `shadow-*`, 11×`[10px]`, 18×`[11px]` | thực tế: **6** neu, **29** shadow, **8**×`[10px]`, **21**×`[11px]` |
| **C** | Level-1 card: `bg-white/60 shadow-xl` | `DraftOverviewBlock.tsx:68-71`: `rounded-3xl p-5 neu-flat` |
| **D** | `CreateCharacterDialog` tự dựng overlay, không dùng shared Dialog | file **import** `Dialog, DialogContent` từ shared và dùng (`:2`, `:75`) |
| **D** | `MemoryCard` tự dựng modal `fixed inset-0` | file **không có** modal nào (dùng `<details>` accordion) |
| **D** | `CharacterChatPane` có `text-[13px]` tại 88/107 | **không tồn tại** `text-[13px]` trong file |
| **D** | `EditableField/EnumSelect` dùng `neu-pressed` | dùng `bg-slate-50 + border + focus:ring-2`; module character-detail có **0** `neu-*` |
| **D** | Tổng 2.553 LOC | thực tế **3.693 LOC** (CharacterDetailView 697 dòng, không phải 184) |
| **F** | F-03 (P0): `RecycleBinTab` tự dựng dialog xoá | file **import và dùng** `DeleteConfirmDialog` (`:5`, `:189`) |
| **F** | Tổng 2.678 LOC, `mockData.ts` 309 dòng | thực tế **3.821 LOC**; `mockData.ts` **768 dòng**; `MotifsTab` 526 (không phải 179) |

**Kết luận về chất lượng:** các agent đọc code khá tốt ở mức *dòng cụ thể* (B/E gần như chính xác 100%), nhưng khi tự tổng hợp số liệu hoặc nhớ lại pattern shadcn thì bịa. Quy tắc dùng report: **tin `file:line` đã trích, không tin con số tổng và snippet không kèm dòng**.

**Điểm D phát hiện đúng và có giá trị:** `CharacterDetailView.tsx` (697 dòng) là **dead code** — không route nào import; đã verify bằng grep toàn repo.

---

## 5. Chẩn đoán gốc

1. **Token layer gần như rỗng** (chỉ 2 màu nền) → không có "single source of truth", nên mỗi màn tự chọn màu/radius/shadow.
2. **Không có primitive form/tab/popover** → 5 module tự chế lại cùng một thứ với 5 style khác nhau.
3. **Không có gate tự động** (eslint/CI) cho luật design → drift tích tụ âm thầm qua nhiều sprint.
4. **Copy pattern shadcn vào Tailwind v4 không kèm plugin** → `animate-in`, `zoom-in-95`, `fade-in-0` chết im lặng; không ai thấy vì UI vẫn "trông ổn".
5. **i18n có hạ tầng nhưng áp dụng 54% file** → nửa app đổi ngôn ngữ được, nửa còn lại thì không.
6. **Hai ngôn ngữ thị giác chưa được quyết định dứt khoát**: neumorphism cho shell nhưng team đã tự chuyển sang flat-bordered cho màn dữ liệu dày — và chưa cập nhật chuẩn.

---

## 6. Lộ trình refactor đề xuất

| Phase | Nội dung | Deliverable | Effort |
|:--|:--|:--|:--:|
| **0. Chốt chuẩn** | Quyết định mô hình surface 2 tầng (xem §7); sửa `tsc` (19 key thiếu, 2 key trùng); xoá `CharacterDetailView` dead code | `DESIGN-SYSTEM.md` + `tsc` xanh | 0.5 ngày |
| **1. Token + primitive** | Mở rộng `@theme`: surface/border/text/accent; `--radius-*`; `text-2xs`; utility `.focus-ring`, `.motion-safe`; primitive `Button/Input/Select/Textarea/Tabs/Popover/Tooltip/Card/Badge` trên Radix | `src/shared/ui/*` + app.css | 1.5–2 ngày |
| **2. Migrate shell + chat** | Shell/sidebar/dialog/toast giữ neu; chat thay 3 popover bằng primitive; splitter có `role="separator"` + keyboard; `aria-live` cho streaming | 12 file chat + 5 file nền | 1 ngày |
| **3. Migrate màn dữ liệu** | Project/character/settings/system-prompt về **flat-bordered tier** (không neu), dùng chung Tabs/Card/Badge/Input; thêm `aria-current`, `role=tablist` | ~40 file | 2 ngày |
| **4. Quét i18n + a11y + QA** | 131 dòng copy → dict; focus ring toàn bộ; 2 theme visual QA; `prefers-reduced-motion` | PR cuối + checklist | 1 ngày |
| **Gate** | ESLint custom rule: cấm hex/`text-[..px]` ngoài token, cấm `focus:outline-hidden` trần, bắt `aria-label` cho icon-only, cấm `shadow-*` ngoài token | `eslint.config.js` | 0.5 ngày |

Tổng ~6–7 ngày người. Thứ tự bắt buộc: **0 → 1 → 2/3 song song → 4**.

---

## 7. Câu hỏi cần bạn quyết (chặn phase 0)

1. **Mô hình surface:** (A) neumorphism thuần toàn app, hay (B) **2 tầng** — neu cho shell/overlay, flat-bordered cho màn dữ liệu dày? *Đề xuất: B*, vì 4/6 module đã tự chuyển sang B và B đọc tốt hơn ở bảng/form.
2. **Type scale chốt:** giữ micro-label 10/11px hay gộp về `text-xs` (12px) + `text-sm` (14px)? *Đề xuất: bỏ 9/13/14/15px, giữ 10/11 như token `text-2xs/text-micro`.*
3. **i18n:** có bắt buộc 100% (kể cả mock data/chat demo) hay chỉ UI chrome? *Đề xuất: UI chrome 100%, mock data tách file fixture.*
4. **`/config` và `/worldbuilding`** hiện là mock — có nằm trong scope refactor UI lần này không?

---

## 8. Phụ lục — phân bố neumorphism vs flat/shadow theo module

| Module | `neu-*` | `shadow-*` | Cách hiểu |
|:--|--:|--:|:--|
| app + layouts + shared/ui | 10 | 4 | Nền tảng: neu là chuẩn |
| sidebar (kể cả Sidebar.tsx) | 7 | 2 | Trung tính |
| chat | 1 | 13 | Gần như flat + ad-hoc |
| draft | 6 | 29 | Lai: overview neu, block flat |
| project | 4 | 20 | Chỉ ProjectCard neu |
| character + character-detail | 0 | 33 | Flat hoàn toàn |
| system-prompt | 0 | 23 | Flat hoàn toàn |
| settings | 0 | 12 | Flat hoàn toàn |

> Đọc bảng này là thấy ngay: "design system" hiện chỉ tồn tại ở vỏ app; phần nội dung đã trôi sang một hệ khác.
