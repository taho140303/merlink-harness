# AUDIT DESIGN SYSTEM — SPRINT 5: PROJECT / CHAPTER / HOME

**Audit date:** 2026-10-29  
**Auditor:** Subagent E (Audit Design System — Chế độ READ-ONLY)  
**Scope:** 8 component files (`src/core/components/project/**`) + 4 route files (`src/app/routes/{home,project,project-detail,chapter-detail}.tsx`)  
**Verdict tổng:** **INCONSISTENT** (Xem chi tiết tại Mục 9)

---

## 1. Scope & Method

### Danh sách file trong Scope & LOC thực tế
| # | File Path | LOC | Trách nhiệm chính |
|---|-----------|-----|-------------------|
| 1 | `src/core/components/project/ProjectCard.tsx` | 62 | Card hiển thị dự án trong grid (ảnh bìa, tiêu đề, tiến độ, action xóa) |
| 2 | `src/core/components/project/ProjectView.tsx` | 128 | Trang danh sách dự án: search input, sort dropdown, project grid, empty state, dialog tạo dự án |
| 3 | `src/core/components/project/ProjectDetailView.tsx` | 248 | Trang chi tiết dự án: overview card, danh sách arc/chapter, toggle collapse, dialog assets |
| 4 | `src/core/components/project/ProjectDetailSidebar.tsx` | 186 | Sidebar mục lục dự án: tabs (Mục lục, Ghi chú, Nhân vật, Cài đặt), stats, actions |
| 5 | `src/core/components/project/ProjectChatPane.tsx` | 99 | Chat drawer/split pane độc lập liên kết ngữ cảnh dự án/chapter |
| 6 | `src/core/components/project/ChapterDetailView.tsx` | 341 | Trang đọc/soạn thảo chương (prose), resizable chat split, lock block, floating outline |
| 7 | `src/core/components/project/mockProjects.ts` | 311 | Mock data dự án, chapters, blocks |
| 8 | `src/core/components/project/types.ts` | 40 | TypeScript interfaces cho Project, Arc, Chapter, Block |
| 9 | `src/app/routes/home.tsx` | 12 | Route `/` (Render `ChatView`, không phải project dashboard) |
| 10 | `src/app/routes/project.tsx` | 12 | Route `/project` (Render `ProjectView`) |
| 11 | `src/app/routes/project-detail.tsx` | 27 | Route `/project/:id` (Loader lookup mock, 404 fallback, layout wrapper) |
| 12 | `src/app/routes/chapter-detail.tsx` | 49 | Route `/project/:projectId/chapter/:chapterId` (Loader lookup, 404 fallback) |
| **Tổng** | **12 files** | **1,515** | |

### Thống kê Grep trong Scope
* **Hex / RGB / HSL hardcode:** 0 lượt trong TSX scope (tuy nhiên component dùng phụ thuộc `FloatingOutline` ở shared có 2 lần `#e61380`).
* **Kích thước tùy tiện `[0-9]+px`:** 0 lượt trong scope (chỉ xuất hiện 13px ở ngoài scope `src/app/routes/worldbuilding.tsx`).
* **Text scale tùy tiện `text-[...px]`:** 13 lượt (toàn bộ là `text-[10px]` và `text-[11px]`, thuộc nhóm label nhỏ được baseline chấp nhận).
* **Radius tùy tiện `rounded-[...px]`:** 0 lượt (nhưng có 2 lần dùng `rounded-none` bất thường phá vỡ ladder R5).
* **Shadow rời (`shadow-*`):** 16 lượt xuất hiện (`shadow-xs`, `shadow-2xs`, `shadow-sm`, `shadow-md`) trên khắp các card và button, vi phạm trực tiếp R1.
* **Neu utility (`neu-flat`, `neu-pressed`):** Chỉ xuất hiện 4 lần tại duy nhất `ProjectCard.tsx`. Toàn bộ 7 component còn lại dùng box model phẳng với `border border-slate-200 bg-white shadow-*`.
* **Dark mode parity (`dark:`):** 96 class declarations có tiền tố `dark:`, tuy nhiên phát hiện 8 vị trí thiếu dark mode (vi phạm R3).
* **Focus rings (`focus:`):** 4 lượt, trong đó 2 vị trí dùng `focus:outline-hidden` mà không có ring thay thế (vi phạm R7).
* **Accessibility attributes (`aria-*`, `role=`):** 0 lượt trong toàn bộ 12 file (0% coverage trên hơn 15 button/link có tính tương tác cao).
* **Chuỗi tiếng Việt hardcode trong TSX:** 42 chuỗi UI tĩnh chưa qua `t(key)` (vi phạm nặng R9).
* **i18n hook `useT()`:** Chỉ xuất hiện ở 3/12 file (`ProjectCard`, `ProjectView`, `ChapterDetailView`).

---

## 2. Scorecard R1–R12

| Rule | Tiêu chuẩn đánh giá | Verdict | Evidence Count | Ghi chú tóm tắt |
|---|---|:---:|:---:|---|
| **R1 Surface / Elevation** | Chỉ `neu-flat`, `neu-pressed` hoặc phẳng trong suốt. Cấm `shadow-*` rời, gradient tùy tiện, bg trắng đục sai chuẩn. | **FAIL** | 16 | Toàn bộ các view chính (`ProjectDetailView`, `ProjectDetailSidebar`, `ChapterDetailView`, `ProjectChatPane`) từ bỏ neumorphism để dùng `bg-white`, `border-slate-200` và `shadow-xs/shadow-sm/shadow-2xs`. Chỉ `ProjectCard` giữ `neu-flat`. |
| **R2 Color System** | Chỉ token hoặc slate + status semantic. Cấm hex hardcode; accent theo đúng nghĩa. | **PARTIAL** | 12 | Không có hex trực tiếp trong scope files, nhưng dùng tùy tiện `text-indigo-*`, `bg-blue-*`, `text-blue-*` không qua semantic token. |
| **R3 Dark Parity** | Mọi bg/border/text/divide/ring phải có `dark:` tương ứng. | **PARTIAL** | 8 | Đa phần có dark parity, nhưng sót: `ProjectView:69` (icon Search không dark), `ProjectDetailSidebar:140` (`border-slate-100`), `ChapterDetailView:257` (text-slate-400), `routes/project-detail.tsx:18,20`, `routes/chapter-detail.tsx:34,36` (thông báo lỗi & link không có dark). |
| **R4 Type Scale** | Chỉ 10/11/12/14/16/18/20/24/30. Tracking/leading nhất quán. | **PASS** | 0 | Không có `text-[13px]` hay `text-[9px]`. Các label nhỏ dùng `text-[10px]` và `text-[11px]` nhất quán với typography baseline. |
| **R5 Radius Ladder** | Thang `{sm, md, lg, xl, 2xl, 3xl, full}` + vỏ ngoài `2.5rem`. Cấm radius rải rác. | **PARTIAL** | 4 | `ProjectCard` dùng `rounded-3xl`, nhưng các card nội dung bên trong dùng `rounded-2xl`, nút dùng `rounded-xl`, tab dùng `rounded-lg`. Đáng chú ý có `rounded-none` xuất hiện bất thường ở badge status tại `ProjectDetailSidebar:81` và `ProjectDetailView:113`. |
| **R6 Spacing** | Lưới 4px (Tailwind standard). | **PASS** | 0 | 100% tuân thủ grid 4px (p-4, p-5, gap-2.5, px-2.5, w-28, w-32). Không có `p-[7px]` hay `gap-[13px]`. |
| **R7 Interaction** | Hover, active, focus-visible ring, disabled. Cấm `focus:outline-hidden` trọc. | **FAIL** | 14 | `ProjectChatPane:72` và `ChapterDetailView:46` dùng `focus:outline-hidden` nhưng không hề có ring thay thế. Hầu hết các button chỉ có hover, thiếu `:active` và `:focus-visible`. |
| **R8 Accessibility** | Icon-only button có `aria-label`; Dialog có Title + Description; decorative có `aria-hidden`. | **FAIL** | 18 | Toàn bộ icon-only buttons (Close Chat, X, Trash, Lock/Unlock, Collapse Sidebar) chỉ dùng thuộc tính HTML `title` hoặc hoàn toàn không có nhãn trợ năng. `aria-current` thiếu ở mục lục, cover image thiếu `alt` mô tả chuẩn. Dialog Assets thiếu Radix `DialogTitle`. |
| **R9 i18n Coverage** | Mọi copy qua `t(key)`, đủ key en/vi. Cấm hardcode string trong TSX. | **FAIL** | 42 | Vi phạm nghiêm trọng nhất. `ProjectDetailView` có 12 chuỗi, `ProjectChatPane` có 11 chuỗi, `ProjectDetailSidebar` có 10 chuỗi, `ChapterDetailView` có 5 chuỗi, routes có 4 chuỗi tiếng Việt hardcode 100%. |
| **R10 Primitive Reuse** | Modal dùng shared Dialog; confirm dùng ConfirmDialog; toast qua `toast()`. Cấm tự dựng riêng. | **PARTIAL** | 2 | `ChapterDetailView` tái sử dụng rất tốt `ConfirmDialog` và `toast()`. Tuy nhiên `ProjectDetailView:231` tự dựng `Dialog` trần cho Assets mà bỏ quên `DialogTitle`/`DialogDescription`. |
| **R11 Motion** | Duration 150/200/300ms + easing; hỗ trợ `motion-reduce`. | **FAIL** | 8 | Lạm dụng `transition-all duration-300` và `hover:scale-[1.01]` mà không hề có bất kỳ class `motion-reduce:*` nào cho người dùng nhạy cảm chuyển động. |
| **R12 State Coverage** | Đầy đủ loading / empty / error / disabled. | **PARTIAL** | 5 | Route `project-detail.tsx` và `chapter-detail.tsx` có fallback 404 nhưng không có loading spinner khi fetch data; form tạo project thiếu validation error state và submitting disable state. |

---

## 3. Findings (Tối đa 25 findings, dedup, sort P0 → P2)

| ID | Severity | File:Line | Đoạn mã hiện tại | Vi phạm Rule | Đề xuất khắc phục |
|---|:---:|---|---|:---:|---|
| **F01** | **P0** | `src/core/components/project/ProjectDetailView.tsx:231-244` | `<Dialog open={assetDialogOpen} onOpenChange={setAssetDialogOpen}><DialogContent className="max-w-xs p-6 text-center"><div className="py-4"><p className="text-sm font-medium text-slate-600 dark:text-slate-300">nothing</p></div>...` | **R8, R10** | Thiếu `DialogTitle` và `DialogDescription` bắt buộc của Radix Dialog. Màn hình đọc trợ năng (screen reader) sẽ cảnh báo lỗi a11y nghiêm trọng hoặc crash modal context. Bổ sung `<DialogTitle className="sr-only">Tài nguyên chương</DialogTitle>`. |
| **F02** | **P1** | `src/core/components/project/ProjectChatPane.tsx:72` | `className="w-full bg-transparent text-xs text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-slate-100"` | **R7** | Dùng `focus:outline-hidden` loại bỏ hoàn toàn viền focus mặc định mà không cung cấp `focus:ring-2` hoặc visual indicator. Người dùng dùng phím Tab hoàn toàn mất dấu con trỏ. Thêm `focus-visible:ring-1 focus-visible:ring-blue-500 rounded-md`. |
| **F03** | **P1** | `src/core/components/project/ChapterDetailView.tsx:46` | `"w-full resize-none overflow-hidden bg-transparent text-sm leading-relaxed focus:outline-hidden"` | **R7** | Khối soạn thảo text của từng block loại bỏ focus outline trọc lốc mà không có ring thay thế. Thêm `focus-visible:ring-1 focus-visible:ring-slate-400 rounded-sm`. |
| **F04** | **P1** | `src/core/components/project/ProjectChatPane.tsx:29-36` | `<button type="button" onClick={onClose} className="rounded-lg p-1 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors" title="Đóng Chat"><X className="h-4 w-4" /></button>` | **R8** | Nút icon-only chỉ dùng thuộc tính HTML `title`, thiếu `aria-label="Đóng bảng chat"`. Cần thay thế bằng `aria-label={t("common.close")}`. |
| **F05** | **P1** | `src/core/components/project/ChapterDetailView.tsx:280-296` | `<button type="button" onClick={() => toggleLock(block.id)} className={cn("flex h-7 w-7 items-center justify-center rounded-lg border shadow-xs transition-colors", ...)}>{block.locked ? <Lock className="h-3.5 w-3.5" /> : <Unlock className="h-3.5 w-3.5" />}</button>` | **R8** | Nút lock/unlock block nội dung là icon-only hoàn toàn không có `aria-label` lẫn `title`. Thêm `aria-label={block.locked ? t("project.unlockBlock") : t("project.lockBlock")}`. |
| **F06** | **P1** | `src/core/components/project/ProjectDetailSidebar.tsx:71-77` | `<button onClick={onToggleCollapse} className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-600 dark:hover:bg-slate-800 dark:hover:text-slate-300" title="Thu gọn sidebar"><PanelRightClose className="h-4 w-4" /></button>` | **R8** | Nút thu gọn sidebar là icon-only thiếu `aria-label`. Thêm `aria-label={t("project.collapseSidebar")}`. |
| **F07** | **P1** | `src/core/components/project/ProjectDetailView.tsx:113` & `ProjectDetailSidebar.tsx:81` | `className="rounded-none bg-slate-900 px-2 py-0.5 text-[10px] font-bold uppercase text-white dark:bg-slate-100 dark:text-slate-900"` | **R5** | Dùng `rounded-none` bất thường, phá vỡ thang bo góc mềm mại của toàn bộ Design System. Đổi thành `rounded-md` hoặc `rounded-full` đồng bộ với Badge. |
| **F08** | **P1** | `src/core/components/project/ProjectDetailView.tsx:102, 157, 169` | `rounded-2xl border border-slate-200 bg-white p-5 shadow-xs dark:border-slate-800 dark:bg-slate-900` | **R1** | Toàn bộ màn hình Project Detail dùng card phẳng với `bg-white dark:bg-slate-900` kèm `shadow-xs`, hoàn toàn mất tính chất Neumorphism / Glass (`neu-flat`). Đề xuất chuyển thành `neu-flat border border-slate-200/50 dark:border-slate-800/50 bg-white/60 dark:bg-slate-900/40 backdrop-blur-md`. |
| **F09** | **P1** | `src/core/components/project/ProjectChatPane.tsx:40-56` | Toàn bộ copy giới thiệu ngữ cảnh Chat: `"Đã liên kết ngữ cảnh dự án:"`, `"Toàn bộ thông tin thiết lập thế giới..."`, `"Đang làm việc tại:"` | **R9** | Hardcode tiếng Việt 100% trong TSX, không thể dịch đa ngôn ngữ. Cần tách key `project.chat.contextLinked`, `project.chat.contextDesc`, `project.chat.workingAt`. |
| **F10** | **P1** | `src/core/components/project/ProjectDetailSidebar.tsx:86-90` | `<button ...>{tab === "toc" ? "Mục lục" : tab === "notes" ? "Ghi chú" : tab === "characters" ? "Nhân vật" : "Cài đặt"}</button>` | **R9** | Tên 4 tab chính của Sidebar hardcode tiếng Việt trong ternary operator. Cần chuyển thành `t(`project.tab.${tab}`)`. |
| **F11** | **P1** | `src/core/components/project/ProjectDetailView.tsx:197, 200, 218` | `"Mới"`, `"Chương {idx + 1}:"`, `"Thu gọn danh sách"`, `"Xem tiếp ({count} chương)"` | **R9** | Chuỗi nhãn và nút phân trang Arc hardcode tiếng Việt. Cần chuyển vào file dictionary `dict.ts`. |
| **F12** | **P1** | `src/core/components/project/ChapterDetailView.tsx:108, 257, 307` | `"Chương {chapterIndex}:"`, `"Đăng ngày: "`, `"Khối nội dung đã bị khóa"` | **R9** | Hardcode tiếng Việt trong giao diện đọc chapter. Cần chuyển thành `t("project.chapterPrefix")`, `t("project.publishedDate")`, `t("project.blockLockedNotice")`. |
| **F13** | **P1** | `src/app/routes/project-detail.tsx:18, 20` | `<p className="text-sm text-slate-500">Không tìm thấy dự án.</p>`, `<Link ...>Quay lại danh sách Project</Link>` | **R3, R9** | Chuỗi thông báo lỗi 404 hardcode tiếng Việt; đồng thời link và chữ thiếu lớp `dark:text-slate-400` và `dark:hover:text-slate-200`. |
| **F14** | **P1** | `src/app/routes/chapter-detail.tsx:34, 36` | `<p className="text-sm text-slate-500">Không tìm thấy chương này.</p>`, `<Link ...>Quay lại danh sách Project</Link>` | **R3, R9** | Lỗi 404 chapter hardcode tiếng Việt và thiếu dark mode styling cho text/link. |
| **F15** | **P1** | `src/core/components/project/ProjectCard.tsx:14` | `className="group relative flex rounded-3xl p-5 neu-flat transition-all duration-300 hover:scale-[1.01]"` | **R11** | Dùng `hover:scale-[1.01]` và `transition-all duration-300` cố định mà không kèm `motion-reduce:transform-none` hoặc `motion-reduce:transition-none`. |
| **F16** | **P1** | `src/core/components/project/ProjectDetailSidebar.tsx:112-127` | `<button key={chap.id} onClick={() => navigate(...)} className={cn("flex w-full items-center justify-between rounded-xl px-3 py-2 text-xs transition-colors", chap.id === currentChapterId ? "bg-slate-200/80 font-bold text-slate-900 dark:bg-slate-800 dark:text-slate-100" : "... text-slate-600 ...")}>` | **R8** | Mục đang active trong mục lục Sidebar chỉ phân biệt bằng màu sắc, hoàn toàn thiếu thuộc tính trợ năng `aria-current={chap.id === currentChapterId ? "page" : undefined}`. |
| **F17** | **P2** | `src/core/components/project/ProjectView.tsx:69` | `<Search className="absolute left-3 top-2.5 h-4 w-4 text-slate-400" />` | **R3** | Icon Search cố định `text-slate-400` không có dark mode tương phản `dark:text-slate-500`. |
| **F18** | **P2** | `src/core/components/project/ProjectDetailSidebar.tsx:140` | `<div className="border-t border-slate-100 p-4 dark:border-slate-800">` | **R3** | Header phần info metadata dùng `border-slate-100`, quá sáng và mờ so với đường viền chuẩn `border-slate-200 dark:border-slate-800`. |
| **F19** | **P2** | `src/core/components/project/ChapterDetailView.tsx:257` | `<p className="mt-1 text-xs text-slate-400">Đăng ngày: {chapter.date}</p>` | **R3** | Phụ đề ngày đăng thiếu `dark:text-slate-500`, trên nền tối `slate-400` có thể bị chói nhẹ hoặc không đồng bộ với hệ thống text thứ cấp. |
| **F20** | **P2** | `src/core/components/project/ProjectCard.tsx:40` | `className="rounded-lg px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider text-indigo-600 neu-pressed dark:text-indigo-400"` | **R2** | Dùng mã màu `indigo-600 / indigo-400` không thuộc quy ước màu accent chuẩn của Baseline (`purple/pink=brand/decor`, `rose`, `emerald`, `amber`, `slate`). |
| **F21** | **P2** | `src/core/components/project/ProjectChatPane.tsx:24, 47, 51, 62` | Sử dụng màu `blue-500`, `text-blue-600`, `bg-blue-50/70`, `text-blue-800 dark:text-blue-300` rải rác | **R2** | Màu xanh dương của AI/Chat đang dùng trực tiếp palette `blue-*` thay vì chuẩn hoá thành token hoặc semantic accent. |
| **F22** | **P2** | `src/core/components/project/ChapterDetailView.tsx:252` | `className="mx-auto mt-6 w-full max-w-3xl space-y-5 pr-14"` | **R6** | Padding cứng `pr-14` nhằm né tránh thanh `FloatingOutline` nhưng làm lệch trục đối xứng của văn bản đọc (`max-w-3xl`) trên màn hình tablet/desktop nhỏ. |
| **F23** | **P2** | `src/core/components/project/ProjectDetailView.tsx:192-206` | `<div onClick={() => navigate(...)} className="flex cursor-pointer items-center justify-between py-2.5 px-3 ...">` | **R8** | Mục chương trong danh sách Arc dùng `div` gắn sự kiện `onClick` thay vì thẻ ngữ nghĩa `<Link>` hoặc `<button>`, khiến người dùng bàn phím không thể Tab tới và không thể mở tab mới. |
| **F24** | **P2** | `src/app/routes/home.tsx:1-12` | Render trực tiếp `<ChatView />` với title `"Merlink Harness - Chat"` | **R12** | Route `/` là Home route nhưng không có Dashboard/Hero tổng quan dự án như spec kỳ vọng, mà trỏ thẳng về ChatView. Không có link dẫn tới `/project`. |
| **F25** | **P2** | `src/core/components/project/ProjectCard.tsx:17` | `<img src={project.cover} alt={project.title} className="h-full w-full object-cover" />` | **R8** | Thuộc tính `alt` chỉ là tên dự án, nên đổi thành `alt={`Ảnh bìa dự án ${project.title}`}` để rõ nghĩa hơn với screen reader. |

---

## 4. Chi tiết Audit Khối Prose của ChapterDetailView

Khu vực đọc văn bản chương là nơi duy nhất chứa văn bản dài (long-form prose). Dưới đây là bảng phân tích chi tiết quy chuẩn typography hiện tại so với tiêu chuẩn đọc (readability standard):

| Cấp độ phần tử | Font Size | Leading (Line-height) | Measure (Độ rộng dòng) | Tracking | Trạng thái Dark Parity | Đánh giá & Khuyến nghị |
|---|---|---|---|---|---|---|
| **Breadcrumb** | `text-xs` (12px) | `leading-normal` | 100% header | `font-semibold` | `text-slate-500 dark:text-slate-400` | Tốt, rõ ràng, phân cấp chuẩn. |
| **Chapter Title (H1)** | `text-xl sm:text-2xl` (20-24px) | `leading-tight` | `max-w-3xl` | `tracking-tight font-bold` | `text-slate-900 dark:text-slate-100` | Rất tốt, tương phản cao, đúng ladder typography. |
| **Metadata Subtitle** | `text-xs` (12px) | `leading-normal` | `max-w-3xl` | Mặc định | `text-slate-400` (Thiếu dark:) | **Cần thêm:** `dark:text-slate-500` để đảm bảo độ tương phản nhẹ. |
| **Đoạn văn đọc (View Mode)** | `text-base` (16px) | `leading-relaxed` (1.625) | `max-w-3xl pr-14` (~75-80ch) | Mặc định | `text-slate-700 dark:text-slate-300` | Kích thước đọc dễ chịu. Tuy nhiên `max-w-3xl` (768px) hơi rộng so với chuẩn tối ưu 65–75ch (nên dùng `max-w-prose` hoặc `max-w-2xl`). |
| **Đoạn văn soạn thảo (Edit Mode)** | `text-sm` (14px) | `leading-relaxed` (1.625) | Full container lọt lòng card | Mặc định | `text-slate-800 dark:text-slate-200` | Bị giảm từ `text-base` xuống `text-sm` khi bấm Edit gây nhảy layout mắt người đọc. Nên giữ nguyên `text-base`. |
| **Khối bị khóa (Locked Block)** | `text-sm` (14px) | `leading-relaxed` | Không cho bôi chọn (`select-none`) | Mặc định | `text-slate-400 dark:text-slate-500` | Rất rõ trạng thái disabled. |
| **Drop Cap / Blockquote** | *Chưa hỗ trợ* | — | — | — | — | Chưa có styling riêng cho trích dẫn hoặc ký tự đầu chương. |

---

## 5. Patterns Nên Chuẩn Hóa Thành Primitive / Token

1. **Section Header + Action Bar Pattern:**
   * *Hiện trạng:* Xuất hiện lặp lại ở `ProjectView.tsx:49-83` và `ChapterDetailView.tsx:160-250` với cấu trúc: Tiêu đề bên trái, bộ lọc/breadcrumb ở giữa, cụm nút bấm hành động (Create, Edit, Save, Cancel, Chat) bên phải.
   * *Ví dụ tốt:* `ChapterDetailView.tsx:182-249` gom cụm action buttons rất mạch lạc, có phân cách separator `border-r border-slate-200`. Nên trừu tượng hóa thành `<ViewHeader>` primitive.

2. **Arc / Chapter Accordion Card:**
   * *Hiện trạng:* `ProjectDetailView.tsx:149-226` tự code cơ chế `expandedArcs` với danh sách chương có preview 5 mục đầu và nút "Xem tiếp".
   * *Đề xuất:* Chuẩn hóa thành generic `AccordionList` hoặc `CollapsibleSection` component để dùng lại cho các module tương tự.

3. **Status Badge Indicator:**
   * *Hiện trạng:* Mỗi nơi style badge một kiểu: `ProjectCard:40` dùng `neu-pressed text-indigo-600 rounded-lg`, `ProjectDetailView:113` dùng `rounded-none bg-slate-900 text-white`, `ProjectDetailSidebar:81` dùng `rounded-none shadow-2xs`.
   * *Đề xuất:* Tạo component `<Badge variant="neutral|brand|success|warning|danger">` kế thừa chuẩn R5 (`rounded-md` hoặc `rounded-full`).

4. **Split-Panel Resizable Container:**
   * *Hiện trạng:* `ChapterDetailView.tsx:121-158` kết hợp tuyệt vời giữa `useResizablePanel`, thanh drag divider `GripVertical` và `ProjectChatPane`. Đây là pattern mẫu mực nên đóng gói thành layout primitive `<SplitView leftPane={...} rightPane={...} />`.

---

## 6. Top 10 Fix Đề Xuất

| # | Hạng mục Fix | File mục tiêu | Effort | Tác động (Impact) |
|---|---|---|:---:|---|
| 1 | Thêm `DialogTitle` và `DialogDescription` cho dialog Assets | `src/core/components/project/ProjectDetailView.tsx:231` | **S** | Triệt tiêu lỗi vi phạm A11y cấp P0 của Radix UI. |
| 2 | Bổ sung focus-visible ring cho input Chat và textarea Block | `ProjectChatPane.tsx:72`, `ChapterDetailView.tsx:46` | **S** | Khôi phục khả năng điều hướng bằng bàn phím (A11y/Interaction P1). |
| 3 | Bổ sung `aria-label` cho toàn bộ các icon-only button (Close, X, Lock, Toggle) | `ProjectChatPane.tsx`, `ChapterDetailView.tsx`, `ProjectDetailSidebar.tsx` | **M** | Đạt chuẩn trợ năng WCAG 2.1 AA cho người khiếm thị. |
| 4 | Trích xuất toàn bộ 42 chuỗi hardcode tiếng Việt sang `dict.ts` | `ProjectDetailView.tsx`, `ProjectChatPane.tsx`, `ProjectDetailSidebar.tsx`, `ChapterDetailView.tsx`, 2 route files | **L** | Đảm bảo tính toàn vẹn 100% của hệ thống đa ngôn ngữ en/vi. |
| 5 | Thay thế `rounded-none` dị biệt ở Status Badge bằng `rounded-md` | `ProjectDetailView.tsx:113`, `ProjectDetailSidebar.tsx:81` | **S** | Đồng bộ thang bo góc Radius Ladder R5. |
| 6 | Thay thế thẻ `div` bọc sự kiện click bằng thẻ `<Link>` cho danh sách chương | `src/core/components/project/ProjectDetailView.tsx:190` | **S** | Hỗ trợ mở tab mới và điều hướng bàn phím tự nhiên. |
| 7 | Đồng bộ kích thước font chữ khi Edit Chapter (`text-sm` → `text-base`) | `src/core/components/project/ChapterDetailView.tsx:46` | **S** | Tránh giật layout (layout shift) khi chuyển chế độ xem/sửa prose. |
| 8 | Bổ sung dark parity cho các thông báo 404 và icon Search | `routes/project-detail.tsx`, `routes/chapter-detail.tsx`, `ProjectView.tsx:69` | **S** | Tránh lỗi tương phản kém ở chế độ Dark Theme. |
| 9 | Thêm thuộc tính `aria-current="page"` cho item mục lục Sidebar đang active | `src/core/components/project/ProjectDetailSidebar.tsx:114` | **S** | Hỗ trợ screen reader định vị vị trí hiện tại trong sách. |
| 10 | Bổ sung `motion-reduce:transform-none` cho `ProjectCard` | `src/core/components/project/ProjectCard.tsx:14` | **S** | Tôn trọng tùy chọn giảm chuyển động của người dùng hệ điều hành. |

---

## 7. Shared Primitive Defects (Lỗi ở primitive ngoài Scope)

1. **`FloatingOutline` (`src/shared/components/ui/floating-outline.tsx:101, 126`):**
   * Sử dụng mã màu hex cố định `#e61380` (màu hồng magenta cá nhân) và `#e61380/10` cho active indicator và active button.
   * Vi phạm trực tiếp **R2 (Cấm hex hardcode)**. Cần chuyển thành `bg-purple-600 dark:bg-purple-400` hoặc token semantic tương đương.

---

## 8. Baseline Conflicts (Xung đột với Baseline v0)

1. **Xung đột Elevation giữa Spec và Thực tế Code:**
   * *Baseline v0 quy định:* Chỉ sử dụng 2 utility `.neu-flat` và `.neu-pressed` trong `src/app/app.css`, cấm `shadow-*` rời và cấm `bg-white` phẳng đục.
   * *Thực tế triển khai:* Ngoại trừ `ProjectCard.tsx`, toàn bộ các màn hình quản trị chi tiết (`ProjectDetailView`, `ProjectDetailSidebar`, `ProjectChatPane`, `ChapterDetailView`) đều được xây dựng theo ngôn ngữ **Clean Slate / Flat Card** (`bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 shadow-xs/2xs`).
   * *Nhận định:* Phong cách Neumorphism nguyên bản (`neu-flat` với box-shadow dập nổi 2 chiều) rất khó áp dụng cho các giao diện quản trị dữ liệu phức tạp nhiều tầng như Split Panes hay Text Editor vì gây nhiễu thị giác. Nhóm phát triển đã tự động chuyển dịch sang phong cách Tailwind UI hiện đại nhưng chưa cập nhật lại tài liệu Baseline v0.

2. **Thiếu vắng Accent Token cho AI / Chatbot:**
   * Baseline chỉ quy ước: rose=destructive, emerald=success, amber=warning, purple/pink=brand/decor.
   * Tuy nhiên các tính năng AI Assistant (`ProjectChatPane`, biểu tượng Sparkles) đều dùng `blue-500 / text-blue-600 / bg-blue-50`. Cần chính thức bổ sung `sky` hoặc `blue` vào quy ước accent màu sắc cho tác vụ Trợ lý thông minh.

---

## 9. Metrics (Số liệu kiểm đếm thực tế)

* **Tổng số file audit:** 12 files
* **Tổng số dòng code (LOC):** 1,515 dòng
* **Số lỗi vi phạm (Findings):** 25 lỗi được phân loại chi tiết (1 P0, 15 P1, 9 P2)
* **Tỷ lệ vi phạm Dark Parity:** 8 / 96 class declarations (8.3%)
* **Tỷ lệ bao phủ trợ năng (A11y coverage):** 0% thuộc tính `aria-*` trên tổng số 18 nút bấm và liên kết tương tác
* **Số chuỗi copy tiếng Việt hardcode chưa qua i18n:** 42 chuỗi
* **Tỷ lệ tuân thủ Neumorphism Baseline:** 12.5% (chỉ 1/8 components tuân thủ)

---

## 10. Verdict Tổng

### **INCONSISTENT**

**Lý do:**
1. **Lệch pha ngôn ngữ thiết kế (Design Language Drift):** Tồn tại sự đứt gãy lớn giữa `ProjectCard` (dùng `neu-flat` dập nổi) và toàn bộ các view chi tiết còn lại (dùng flat cards với `border-slate-200 bg-white shadow-xs`).
2. **Khuyết thiếu hoàn toàn tầng Trợ năng (A11y):** Không có bất kỳ thẻ `aria-label`, `aria-current`, `role` nào trong toàn bộ 12 file; phát hiện 1 lỗi P0 vi phạm chuẩn Dialog accessibility của Radix UI và 2 vị trí phá vỡ outline bàn phím (`focus:outline-hidden`).
3. **Chưa nội địa hóa (i18n):** Hệ thống i18n mới chỉ được tích hợp hình thức ở 3 file; có tới 42 chuỗi giao diện người dùng tiếng Việt bị hardcode trực tiếp trong mã nguồn TSX.
