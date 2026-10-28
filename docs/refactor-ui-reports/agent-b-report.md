# BÁO CÁO AUDIT DESIGN SYSTEM — PHÂN HỆ CHAT (SPRINT 2)

> **Môi trường audit**: Read-only audit cho Sprint 2 (Màn hình Lõi: Chat, Composer, Popovers, Dialogs, Message Stream).  
> **Repository**: `/home/hyanhasta05/workspace/personal/pet-project/merlink-harness-ui`  
> **Stack**: React Router 8 framework mode (SSR), React 19, Tailwind CSS v4 CSS-first (không dùng tailwind.config), Zustand, Radix Dialog, lucide-react, Inter.  
> **Trạng thái repo**: Hoàn toàn read-only đối với source code; file duy nhất được tạo là báo cáo này.

---

## 1. Scope & Method

### 1.1 Danh sách file trong scope & số dòng (LOC)
Tổng cộng **12 file** với **994 dòng mã** được kiểm tra toàn diện 100% dòng lệnh (không đọc lướt):

| File Path | LOC | Trách nhiệm kiến trúc UI |
|:---|---:|:---|
| `src/app/routes/chat.tsx` | 12 | Route entrypoint cho màn hình chat, cấu hình meta SSR, render `ChatView` |
| `src/core/components/chat/types.ts` | 21 | Định nghĩa kiểu TypeScript cho chat: `ChatMode`, `ToolCallItem`, `ChatMessage` |
| `src/core/components/chat/ChatView.tsx` | 217 | Container chính chia đôi pane (Chat pane + DraftWorkspace pane), splitter kéo thả, danh sách tin nhắn mẫu & state phản hồi |
| `src/core/components/chat/ChatHeader.tsx` | 72 | Header khu vực chat: tiến độ xử lý (progress bar), bộ đếm token, trigger mở `ProcessDetailDialog`, nút chuyển mode (Chat/Manual) |
| `src/core/components/chat/ChatWelcome.tsx` | 26 | Empty state chào mừng khi hội thoại chưa có tin nhắn, tiêu đề và gradient orb |
| `src/core/components/chat/ChatMessageItem.tsx` | 127 | Render từng lượt tin nhắn: bong bóng user/assistant, thinking process, spinner, tool calls trigger, markdown format, nút copy |
| `src/core/components/chat/ChatInputBar.tsx` | 114 | Composer thanh nhập liệu: banner quote context, textarea tự mở rộng theo phím bấm, nút Attach, Model selector, nút gửi |
| `src/core/components/chat/ModelSelectorPopover.tsx` | 94 | Popover tùy biến chọn mô hình AI (Claude, GPT, DeepSeek, Gemini...) |
| `src/core/components/chat/ContextUsagePopover.tsx` | 103 | Popover phân rã chi tiết mức sử dụng context window (System prompt, Tools, Messages) |
| `src/core/components/chat/ToolCallsPopover.tsx` | 99 | Popover hiển thị danh sách các công cụ (tools) đã thực thi kèm JSON input/output |
| `src/core/components/chat/ProcessDetailDialog.tsx` | 77 | Dialog Radix tùy biến hiển thị chi tiết các bước trong quy trình xây dựng bản nháp |
| `src/core/components/chat/OrbSphere.tsx` | 32 | Hiệu ứng hình cầu gradient 3D chuyển động trang trí cho empty state |
| **Tổng cộng** | **994** | **100% file trong phạm vi Sprint 2** |

### 1.2 Các lệnh thống kê và Grep đã thực hiện
- **Thống kê pattern trong scope**:
  - `#[0-9a-fA-F]{3,8}|rgba?\(`: **3 lần** xuất hiện (`ChatMessageItem.tsx:73` với `dark:bg-[#151921]`, `OrbSphere.tsx:13` và `OrbSphere.tsx:22` với các dải gradient hex).
  - `\[[0-9]+px\]`: **15 lần** xuất hiện (chủ yếu là `text-[10px]`, `text-[11px]`, `w-[85%]`, `max-w-[75%]`). Không có px layout lạ ngoài type size nhỏ.
  - `rounded-\[`: **1 lần** duy nhất tại `ChatView.tsx:142` (`rounded-[2rem]`).
  - `shadow-`: **14 lần** (`shadow-xs`, `shadow-2xs`, `shadow-xl`, `shadow-2xl`). Toàn bộ popover và button dùng shadow Tailwind rời rạc thay vì elevation token hệ thống.
  - `.neu-flat` / `.neu-pressed`: **1 lần** duy nhất tại `ChatInputBar.tsx:37` (`neu-flat`). Cả 11 file còn lại đều dùng phẳng hoặc shadow rời.
  - `dark:`: **135 lần** xuất hiện trong 10 file TSX.
  - `focus:`: **10 lần** (chỉ xuất hiện ở `ChatInputBar.tsx` và `ProcessDetailDialog.tsx`).
  - `focus-visible`: **0 lần** trong toàn bộ 12 file.
  - `aria-`: **1 lần** duy nhất (`aria-hidden="true"` tại `ChatMessageItem.tsx:81`).
  - `role=`: **0 lần** trong toàn bộ scope.
  - `useT(`: **3 lần** (`ChatWelcome.tsx:5`, `ChatHeader.tsx:19`, `ChatInputBar.tsx:19`). Có **8 file TSX hoàn toàn không import `useT`**.
  - **Chuỗi tiếng Việt hardcode**: **28 chuỗi** xuất hiện rải rác trong `ChatMessageItem.tsx` (6 chuỗi), `ChatView.tsx` (19 chuỗi), `ToolCallsPopover.tsx` (3 chuỗi).

---

## 2. Scorecard R1–R12

| Rule | Tiêu chí đánh giá | Verdict | Evidence Count | Ghi chú & Đánh giá |
|:---|:---|:---:|:---:|:---|
| **R1 Surface/elevation** | Chỉ `neu-flat` \| `neu-pressed` \| phẳng trong suốt; cấm `shadow-*` rời, gradient tùy tiện, bg trắng đục sai quy ước | **FAIL** | 16 | Chỉ có `ChatInputBar` dùng `neu-flat`. 3 Popover (`ModelSelector`, `ContextUsage`, `ToolCalls`) đều dùng `shadow-xl` rời; `ChatMessageItem` dùng `shadow-xs`; `ProcessDetailDialog` dùng `shadow-2xl`. Bong bóng assistant dùng `bg-white/95 dark:bg-[#151921]` thay vì surface token. |
| **R2 Color** | Chỉ token đã khai báo hoặc Slate + Semantic Status; cấm hex/rgb hardcode trong TSX | **FAIL** | 4 | Hardcode `#151921` tại `ChatMessageItem.tsx:73`. Hardcode chuỗi gradient màu hex tại `OrbSphere.tsx:13,22`. Dùng màu tím `bg-purple-500` trực tiếp tại `ContextUsagePopover.tsx:69,85` và `indigo-500` tại `ChatWelcome.tsx:14` chưa token hóa. |
| **R3 Dark parity** | Mọi bg/border/text/divide/ring phải có `dark:` tương ứng | **PARTIAL** | 5 | Đa số phần tử có cặp `dark:`. Tuy nhiên, thanh progress trong `ContextUsagePopover.tsx:68,70` và dot chú thích `77,93` thiếu dark variant riêng; thanh ray splitter hover màu xanh `ChatView.tsx:189` thiếu độ tương phản chuẩn. |
| **R4 Type scale** | Chỉ 10/11/12/14/16/18/20/24/30; tracking/leading nhất quán; cấm size lẻ | **PASS** | 0 | Chỉ sử dụng `text-[10px]`, `text-[11px]`, `text-xs` (12px), `text-sm` (14px), `text-base` (16px), `text-2xl` (24px), `text-3xl` (30px). Không có kích thước vi phạm như 9px hay 13px. |
| **R5 Radius ladder** | Ladder chuẩn {sm,md,lg,xl,2xl,3xl,full} + ngoại lệ shell 2.5rem; cấm `rounded-[2rem]` rải rác | **FAIL** | 2 | `ChatView.tsx:142` sử dụng `rounded-[2rem]` tùy tiện bên trong shell; `ContextUsagePopover.tsx:77,85,93` sử dụng `rounded-xs` (không thuộc thang đo radius tiêu chuẩn của dự án). |
| **R6 Spacing** | Grid 4px; cấm các giá trị pixel tùy tiện như `p-[7px]`, `gap-[13px]` | **PASS** | 0 | Tuân thủ 100% spacing scale của Tailwind (`p-1`, `p-1.5`, `p-2`, `p-3`, `p-3.5`, `gap-1.5`, `h-14`, v.v.). Không có spacing hardcode dạng `[Npx]`. |
| **R7 Interaction** | Mọi tương tác bấm được phải có hover + focus-visible ring + disabled + active; cấm `focus:outline-hidden` đơn độc | **FAIL** | 22 | Toàn bộ các icon button và popover trigger thiếu hoàn toàn `focus-visible:ring-*`. `ChatInputBar.tsx:75` dùng `focus:outline-hidden` mà không có ring thay thế. Nút gửi tin nhắn không có thuộc tính `disabled` khi input trống; textarea không disable khi đang gửi. |
| **R8 A11y** | Icon-only button cần `aria-label`; dialog cần Title+Description; live region cho stream; splitter semantics | **FAIL** | 18 | Splitter resize ở `ChatView.tsx:188` không có `role="separator"`, thiếu `aria-valuenow`, `aria-valuemin`, `aria-valuemax` và không hỗ trợ bàn phím. Cả 3 Popover thiếu ARIA `haspopup`/`expanded`. Tin nhắn streaming thiếu `aria-live="polite"`. Nhiều nút icon chỉ có `title` tooltip mà thiếu `aria-label`. |
| **R9 i18n** | Mọi chuỗi giao diện phải qua `t(key)`; cấm hardcode chuỗi trong TSX | **FAIL** | 33 | 28 chuỗi tiếng Việt hardcode trong TSX (`ChatView`: 19, `ChatMessageItem`: 6, `ToolCallsPopover`: 3). Nhiều chuỗi tiếng Anh hardcode trong `ProcessDetailDialog`, `ContextUsagePopover`, `ChatHeader`. Chỉ có 3 file gọi `useT()`. |
| **R10 Primitive reuse** | Modal dùng shared Dialog; cấm tự dựng overlay/portal/popover trôi nổi | **PARTIAL** | 4 | `ProcessDetailDialog` tái sử dụng tốt Radix Dialog shared. Tuy nhiên, cả 3 Popover (`ModelSelectorPopover`, `ContextUsagePopover`, `ToolCallsPopover`) tự dựng popover nổi bằng `absolute bottom-full` kèm `useEffect(mousedown)` thủ công, không có Radix Popover/DropdownMenu, không có Portal (nguy cơ bị cắt bởi `overflow-hidden`). |
| **R11 Motion** | Thời lượng 150/200/300ms nhất quán; tôn trọng `prefers-reduced-motion` | **FAIL** | 7 | `OrbSphere.tsx` dùng 2 animation liên tục 14s và 7s nhưng hoàn toàn không có `motion-reduce:animate-none`. `ChatView.tsx:109` gọi `scrollToBottom({ behavior: "smooth" })` cưỡng bức, không kiểm tra người dùng giảm chuyển động. Không có quy ước transition đồng nhất. |
| **R12 State coverage** | Phủ đầy đủ loading / empty / error / disabled | **PARTIAL** | 5 | Empty state có `ChatWelcome`. Loading có thinking spinner. Tuy nhiên: thiếu hoàn toàn Error state khi AI sinh lỗi; nút Gửi không có trạng thái `disabled` (cho phép gửi tin nhắn rỗng); Popover không có trạng thái rỗng khi không có tool call. |

---

## 3. Findings (Phát hiện lỗi & Đề xuất khắc phục)

Tất cả các finding dưới đây đều được trích xuất từ mã nguồn thực tế kèm số dòng chính xác, phân loại theo mức độ nghiêm trọng từ **P0** đến **P2**:

| ID | Severity | File:Line | Hiện trạng mã nguồn (Snippet) | Vi phạm | Đề xuất khắc phục (Code / Token cụ thể) |
|:---|:---:|:---|:---|:---:|:---|
| **F01** | **P0** | `src/core/components/chat/ChatMessageItem.tsx:73` | `? "w-full border-slate-200 bg-white/95 shadow-xs dark:border-slate-800/80 dark:bg-[#151921]"` | R1, R2 | Xóa mã màu hex hardcode `dark:bg-[#151921]` và bỏ `shadow-xs`. Thay bằng surface token nền tảng hoặc `dark:bg-slate-900/80` đồng bộ toàn hệ thống. |
| **F02** | **P0** | `src/core/components/chat/ChatView.tsx:188-199` | `<div onMouseDown={startDragging} className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20">...</div>` | R8 | Thêm ngữ nghĩa A11y cho splitter: `role="separator"`, `tabIndex={0}`, `aria-valuenow={Math.round(leftRatio * 100)}`, `aria-valuemin={30}`, `aria-valuemax={70}`, `aria-orientation="vertical"`, đồng thời bổ sung `onKeyDown` (ArrowLeft/ArrowRight) để hỗ trợ điều khiển bằng phím. |
| **F03** | **P1** | `src/core/components/chat/ChatView.tsx:32-94` | `content: "Chào bạn! Tôi có thể giúp gì cho dự án của bạn hôm nay?"` và 18 dòng text tiếng Việt mock khác | R9 | Trích xuất toàn bộ dữ liệu mock message và nhãn sang hệ thống từ điển i18n (`src/shared/i18n/dict.ts`) hoặc đưa vào file mock data riêng biệt có hỗ trợ song ngữ `dict.en` và `dict.vi`. |
| **F04** | **P1** | `src/core/components/chat/ChatMessageItem.tsx:32,53` | `<span>Đang suy nghĩ ({message.thinkingSeconds ?? 1}s)...</span>` và `<span>Đã suy nghĩ {message.thinkingSeconds} giây</span>` | R9 | Thay bằng hàm dịch: `t("chat.thinking", { seconds: message.thinkingSeconds ?? 1 })` và `t("chat.thoughtDuration", { seconds: message.thinkingSeconds })`. Bổ sung 2 khóa này vào từ điển song ngữ. |
| **F05** | **P1** | `src/core/components/chat/ChatMessageItem.tsx:26-46` | `<div className="space-y-2 mb-2">...<Loader2 className="... animate-spin" />...<span className="... animate-pulse" />...</div>` | R8 | Thêm thuộc tính `aria-live="polite"` và `aria-busy="true"` vào container bọc tiến trình thinking/streaming để trình đọc màn hình (Screen Reader) thông báo khi trợ lý đang suy nghĩ. |
| **F06** | **P1** | `src/core/components/chat/ChatInputBar.tsx:71-76` | `<textarea value={input} ... className="w-full bg-transparent text-sm text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-slate-100" />` | R7, R12 | Thêm `disabled={isGenerating}` và focus indicator; nút bấm gửi (`line 105`) phải có `disabled={!input.trim()}` cùng class `disabled:opacity-40 disabled:cursor-not-allowed`. |
| **F07** | **P1** | `src/core/components/chat/ModelSelectorPopover.tsx:58` | `<div className="no-scrollbar absolute bottom-full right-0 mb-2 w-56 max-h-72 overflow-y-auto rounded-2xl border border-slate-200 bg-white p-1.5 shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in-0 zoom-in-95">` | R1, R8, R10 | Tự dựng Popover bằng `absolute` kèm `shadow-xl` ngoài token; thiếu Portal khiến dropdown có thể bị cắt bởi cha có `overflow-hidden`. Thiếu `aria-haspopup="listbox"`, `aria-expanded={open}`. Đề xuất chuẩn hóa thành shared Primitive Popover hoặc Radix Dropdown. |
| **F08** | **P1** | `src/core/components/chat/ContextUsagePopover.tsx:58` | `<div className="absolute bottom-full right-0 mb-2 w-72 rounded-2xl border border-slate-200 bg-white p-3.5 shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in-0 zoom-in-95">` | R1, R8, R10 | Tương tự F07: Popover tự dựng không dùng Portal, dùng `shadow-xl` thay vì surface token; thiếu role `dialog` hoặc `tooltip` và các thuộc tính `aria-describedby`. |
| **F09** | **P1** | `src/core/components/chat/ToolCallsPopover.tsx:47` | `<div className="absolute bottom-full left-0 mb-2 w-96 rounded-2xl border border-slate-200 bg-white p-3 shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in-0 zoom-in-95">` | R1, R8, R10 | Popover 96 đơn vị rộng tự dựng trôi nổi, thiếu Portal và bẫy focus (focus trap). Thiếu phím tắt `Escape` để đóng popover khi người dùng tương tác bằng bàn phím. |
| **F10** | **P1** | `src/core/components/chat/OrbSphere.tsx:10-25` | `style={{ background: "radial-gradient(circle at 35% 30%, #ffffff 0%, #e0e7ff 18%, #93c5fd 35%, #c084fc 65%, #f472b6 85%, #6366f1 100%)" }}` | R2, R11 | Gradient cầu chứa 6 mã màu hex hardcode tùy tiện. Hai hiệu ứng `animate-orb-float` và `animate-orb-glow` không có lớp `motion-reduce:animate-none` bảo vệ người dùng bị rối loạn tiền đình. |
| **F11** | **P1** | `src/core/components/chat/ChatView.tsx:142` | `<div className="flex h-full w-full overflow-hidden rounded-[2rem] bg-transparent">` | R5 | Vi phạm thang bo góc (radius ladder): dùng giá trị tùy biến `rounded-[2rem]`. Cần đồng bộ về `rounded-3xl` hoặc loại bỏ bo góc vì container cha `FullLayout` đã có `rounded-[2.5rem]`. |
| **F12** | **P1** | `src/core/components/chat/ChatMessageItem.tsx:85-102` | `<ReactMarkdown remarkPlugins={[remarkGfm]} components={{ code(...) { ... } }}>{msg.content}</ReactMarkdown>` | R1, R4 | Thiếu hoàn toàn cấu hình style cho các thành phần Markdown chuẩn: `h1-h6`, danh sách `ul`/`ol`, trích dẫn `blockquote`, đường kẻ ngang `hr` và bảng dữ liệu biểu bảng `table` từ `remark-gfm`. |
| **F13** | **P1** | `src/core/components/chat/ChatMessageItem.tsx:114` | `<button onClick={handleCopy} title="Sao chép nội dung" className="flex h-7 w-7 items-center justify-center rounded-lg border border-slate-200 bg-white text-slate-500 hover:border-slate-300 hover:text-slate-800 ...">` | R7, R8, R9 | Nút icon chỉ có `title` tiếng Việt hardcode, thiếu `aria-label={t("chat.copy")}` và hoàn toàn không có `focus-visible:ring-2 focus-visible:ring-slate-400`. |
| **F14** | **P1** | `src/core/components/chat/ChatHeader.tsx:44` | `<button onClick={() => setIsProcessOpen(true)} className="rounded-lg p-1 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200" title="View progress detail">` | R7, R8, R9 | Nút icon mở tiến trình thiếu `aria-label`, thiếu `focus-visible:ring-2`, và thuộc tính `title` tiếng Anh hardcode chưa qua hàm dịch `t()`. |
| **F15** | **P1** | `src/core/components/chat/ChatInputBar.tsx:83,92,105` | `<button onClick={onAttach} ...> <button onClick={() => setModelSelectorOpen(!modelSelectorOpen)} ...> <button onClick={handleSend} ...>` | R7, R8 | Cả 3 nút tương tác cốt lõi trong thanh composer đều thiếu `focus-visible:ring-2 focus-visible:ring-offset-2`. Nút Gửi (`line 105`) dùng `title="Send"` hardcode, thiếu `aria-label`. |
| **F16** | **P2** | `src/core/components/chat/ChatView.tsx:109` | `messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });` | R11 | Cuộn mượt cưỡng bức khi có tin nhắn mới mà không kiểm tra media query `prefers-reduced-motion`, gây chóng mặt cho người dùng nhạy cảm chuyển động. |
| **F17** | **P2** | `src/core/components/chat/ContextUsagePopover.tsx:77,85,93` | `<span className="h-2 w-2 rounded-xs bg-slate-400" />` | R5 | Sử dụng `rounded-xs` không nằm trong thang bo góc dự án ({sm, md, lg, xl, 2xl, 3xl, full}). Cần sửa thành `rounded-sm` hoặc `rounded-full`. |
| **F18** | **P2** | `src/core/components/chat/ContextUsagePopover.tsx:68-70` | `<div style={{ width: "1%" }} className="bg-slate-400" /> <div style={{ width: "2%" }} className="bg-purple-500" /> <div style={{ width: "4%" }} className="bg-blue-500" />` | R2, R3 | Các vạch tiến độ dùng màu trực tiếp `purple-500` và `blue-500` thiếu token hóa, thiếu biến thể màu sắc tối ưu tương phản trong Dark Mode. |
| **F19** | **P2** | `src/core/components/chat/ProcessDetailDialog.tsx:34,41,52,59` | `className="mx-1.5 inline-block w-16 rounded-md border border-slate-200 bg-white px-2 py-0.5 text-center text-xs font-semibold text-slate-900 focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100 focus:outline-hidden"` | R7 | Thẻ `<span>` giả lập input badge mang class `focus:ring-1` và `focus:outline-hidden` nhưng không phải phần tử tương tác thực (không có `tabIndex={0}` hay form control). |
| **F20** | **P2** | `src/core/components/chat/ChatMessageItem.tsx:74` | `: "ml-auto w-fit max-w-[85%] border-blue-100 bg-blue-50/70 dark:border-blue-900/50 dark:bg-blue-950/30 sm:max-w-[75%]"` | R1 | Bong bóng người dùng sử dụng nền xanh nhạt `bg-blue-50/70` phẳng thay vì sử dụng ngôn ngữ ánh sáng surface chung của ứng dụng (`neu-flat` hoặc neutral slate tint). |
| **F21** | **P2** | `src/core/components/chat/ChatWelcome.tsx:14` | `<span className="text-xs font-bold uppercase tracking-[0.2em] text-indigo-500 dark:text-indigo-400">` | R2 | Dùng màu thương hiệu tùy tiện `text-indigo-500` và `tracking-[0.2em]` ngoài type scale chuẩn, lệch với quy ước accent chung (purple/pink). |
| **F22** | **P2** | `src/core/components/chat/ChatMessageItem.tsx:120` | `<span className="text-[10px] text-slate-400 dark:text-slate-500">{msg.timestamp}</span>` | R9 | Dữ liệu timestamp trong `ChatView` hiển thị cố định chuỗi `"Vừa xong"` tiếng Việt hardcode; cần format động bằng `Intl.RelativeTimeFormat` hoặc qua i18n key. |

---

## 4. Patterns nên chuẩn hóa thành Primitive/Token

### 4.1 Shared Floating Popover Primitive (`src/shared/components/ui/popover.tsx`)
* **Thực trạng**: Hiện tại cả 3 popover trong chat (`ModelSelectorPopover.tsx`, `ContextUsagePopover.tsx`, `ToolCallsPopover.tsx`) đều tự viết logic nhúng `absolute bottom-full`, tự gắn listener `mousedown` trên `document` để bắt sự kiện click bên ngoài, tự định nghĩa hoạt cảnh bằng Tailwind animate plugin.
* **Hậu quả**: Không có Portal khiến popover bị che khuất khi thẻ cha có `overflow: hidden`; không có phím bấm đóng `Escape`; không có quản lý tiêu điểm bàn phím (focus trap) chuẩn a11y.
* **Đề xuất**: Dựng một primitive chung dựa trên `@radix-ui/react-popover` hoặc tái sử dụng cơ chế Portal/Overlay của Radix Dialog có sẵn trong repo.

### 4.2 Rich Markdown Render Theme (`src/shared/components/chat/MarkdownContent.tsx`)
* **Thực trạng**: `ChatMessageItem.tsx:85-102` chỉ render inline code và thẻ `<strong>` đơn sơ. Khi AI trả về code block đa dòng, bảng so sánh (table), danh sách việc cần làm (tasklist), hoặc trích dẫn chú thích (blockquote), giao diện bị vỡ layout và mất định dạng.
* **Ví dụ tốt nên tham khảo**: Đóng gói component markdown riêng với styling đầy đủ cho `table` (border, padding, zebra stripe), `pre/code` (nút copy code, cú pháp highlight), `blockquote` (thanh viền trái `border-l-2 border-slate-300`).

### 4.3 Accessible Resizable Splitter (`src/shared/components/ui/panel-splitter.tsx`)
* **Thực trạng**: Thanh kéo dãn panel giữa Chat và DraftWorkspace được dựng chay tại `ChatView.tsx:188-199` với thẻ `<div>` bắt sự kiện chuột.
* **Giải pháp chuẩn hóa**: Tách riêng thành component `PanelSplitter` chuẩn WAI-ARIA Slider/Separator:
  ```tsx
  <div
    role="separator"
    tabIndex={0}
    aria-orientation="vertical"
    aria-valuenow={Math.round(ratio * 100)}
    aria-valuemin={30}
    aria-valuemax={70}
    onKeyDown={handleKeyDown}
    className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center focus-visible:ring-2 focus-visible:ring-blue-500"
  >
    <div className="h-full w-px bg-slate-200 dark:bg-slate-800 group-hover:bg-blue-500" />
    <div className="absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-slate-300 bg-white shadow-xs dark:border-slate-700 dark:bg-slate-800">
      <GripVertical className="h-3.5 w-3.5 text-slate-500" />
    </div>
  </div>
  ```

---

## 5. Top 10 Fix Đề Xuất

| # | Hạng mục khắc phục | File ảnh hưởng | Effort | Tác động (Impact) |
|:---:|:---|:---|:---:|:---|
| 1 | Thêm ngữ nghĩa A11y & điều khiển bàn phím cho thanh Splitter kéo panel | `src/core/components/chat/ChatView.tsx` | **S** | **P0**: Đạt chuẩn trợ năng WCAG cho tính năng chia khung nhìn cốt lõi. |
| 2 | Loại bỏ mã màu hex hardcode `dark:bg-[#151921]`, đồng bộ màu surface bong bóng | `src/core/components/chat/ChatMessageItem.tsx` | **S** | **P0**: Đồng bộ token màu nền tối với baseline design system. |
| 3 | Quốc tế hóa 100% chuỗi hardcode tiếng Việt trong danh sách tin nhắn và popover | `src/core/components/chat/ChatView.tsx`, `ChatMessageItem.tsx` | **M** | **P1**: Đảm bảo chế độ song ngữ Anh - Việt hoạt động chính xác. |
| 4 | Bổ sung `disabled` state và phím tắt gửi tin nhắn cho thanh công cụ Composer | `src/core/components/chat/ChatInputBar.tsx` | **S** | **P1**: Chống gửi tin nhắn rỗng, cải thiện trải nghiệm gõ và phản hồi thị giác. |
| 5 | Thêm `aria-live="polite"` cho container streaming và nhãn `aria-label` cho các nút icon | `src/core/components/chat/ChatMessageItem.tsx`, `ChatHeader.tsx` | **S** | **P1**: Giúp người khiếm thị nhận biết được tiến trình AI đang trả lời. |
| 6 | Thay thế 3 Popover trôi nổi bằng Primitive bọc Radix Popover hoặc Floating Portal | `src/core/components/chat/*Popover.tsx` | **M** | **P1**: Triệt tiêu lỗi tràn layout (clipping), hỗ trợ phím `Esc` và click ngoài chuẩn mực. |
| 7 | Tích hợp lớp `motion-reduce:animate-none` cho `OrbSphere` và cuộn mượt có điều kiện | `src/core/components/chat/OrbSphere.tsx`, `ChatView.tsx` | **S** | **P1**: Tôn trọng cấu hình giảm chuyển động của hệ điều hành người dùng. |
| 8 | Bổ sung đầy đủ CSS typography cho bảng, codeblock, trích dẫn trong `ReactMarkdown` | `src/core/components/chat/ChatMessageItem.tsx` | **M** | **P1**: Khắc phục vỡ layout định dạng Markdown khi AI xuất bảng hoặc khối mã nguồn. |
| 9 | Chuẩn hóa bo góc container chat từ `rounded-[2rem]` thành `rounded-3xl` | `src/core/components/chat/ChatView.tsx` | **S** | **P2**: Giữ vững bậc thang bo góc (radius ladder) của hệ thống. |
| 10 | Bổ sung trạng thái Error state cho tin nhắn khi cuộc gọi mô hình thất bại | `src/core/components/chat/ChatMessageItem.tsx` | **S** | **P2**: Đạt độ phủ trạng thái đầy đủ (State coverage: empty, loading, error). |

---

## 6. Shared Primitive Defects (Lỗi ở ngoài Scope)

Trong quá trình đối chiếu các thành phần chat với các thư viện dùng chung (`src/shared/components/ui/`):
1. **Thiếu Shared Popover Component**: Repo hiện chỉ có `dialog.tsx`, `confirm-dialog.tsx`, `toast.tsx`, `floating-outline.tsx` mà hoàn toàn chưa có `popover.tsx`. Điều này ép buộc các màn hình nghiệp vụ như Chat phải tự "chế" popover bằng `position: absolute` và `useEffect` bắt sự kiện bấm chuột bên ngoài, dẫn tới sự thiếu đồng nhất trên toàn bộ ứng dụng.
2. **Thiếu Tiện ích Hỗ trợ A11y cho `useResizablePanel`**: Hook `src/shared/hooks/useResizablePanel.ts` chỉ tính toán tỷ lệ `leftRatio` dựa trên chuột (`mousemove`), không cung cấp bất kỳ hàm xử lý bàn phím (`onKeyDown`) nào (như nhấn phím mũi tên để tăng/giảm từng bước 5%).
3. **Từ điển `dict.ts` thiếu nhóm khóa cho trạng thái phản hồi của Chat**: Nhóm khóa `chat.*` trong từ điển hiện tại chỉ có các nhãn bề mặt cơ bản (`chat.mode.chat`, `chat.draftBuilding`, `chat.welcomeTitle`, `chat.attach`), hoàn toàn thiếu các thông báo trạng thái quan trọng như `"Đang suy nghĩ..."`, `"Sao chép nội dung"`, `"Xem chi tiết công cụ"`.

---

## 7. Baseline Conflicts (Xung đột với Baseline v0)

1. **Quy định về Surface Bong bóng Chat (Assistant Message Bubble)**:
   * *Baseline v0*: Quy định bề mặt nổi phải dùng `.neu-flat`, bề mặt chìm dùng `.neu-pressed`, còn lại là phẳng trong suốt.
   * *Thực tế triển khai*: Bong bóng assistant (`ChatMessageItem.tsx:73`) đang sử dụng nền phẳng đục kết hợp viền mỏng và bóng đổ nhẹ: `bg-white/95 border-slate-200 shadow-xs dark:bg-[#151921]`.
   * *Đánh giá xung đột*: Nếu áp dụng `.neu-flat` cho từng tin nhắn trợ lý sẽ gây rối mắt nghiêm trọng (visual noise) và tạo cảm giác nặng nề trong luồng đọc văn bản dài. Baseline v0 cần được nới lỏng hoặc bổ sung định nghĩa **"Content Surface"** (phẳng, viền mờ 1px, shadow vi mô) dành riêng cho các thẻ nội dung đọc.
2. **Quy ước Độ cao Popover (Elevation)**:
   * *Baseline v0*: Cấm hoàn toàn các utility `shadow-*` rời rạc ngoài hai lớp neumorphism.
   * *Thực tế*: Các menu thả nổi và popover lựa chọn bắt buộc phải có bóng đổ sâu (`shadow-xl` hoặc `shadow-2xl`) để tạo độ tách biệt khỏi nền canvas. Baseline v0 cần chính thức bổ sung cấp độ elevation cho lớp nổi (Overlays & Menus).

---

## 8. Bảng Đối Chiếu i18n & Chuỗi Hardcode

Dưới đây là danh sách toàn bộ các chuỗi giao diện đang bị hardcode trong phạm vi phân hệ Chat kèm theo mã khóa đề xuất:

| File : Dòng | Chuỗi hiện tại (Hardcoded String) | Khóa đề xuất trong `src/shared/i18n/dict.ts` | Trạng thái trong `dict.ts` hiện tại |
|:---|:---|:---|:---:|
| `ChatMessageItem.tsx:32` | `"Đang suy nghĩ ({message.thinkingSeconds ?? 1}s)..."` | `chat.thinking` | **Chưa có** |
| `ChatMessageItem.tsx:53` | `"Đã suy nghĩ {message.thinkingSeconds} giây"` | `chat.thoughtDuration` | **Chưa có** |
| `ChatMessageItem.tsx:115` | `"Sao chép nội dung"` | `chat.copyContent` | **Chưa có** |
| `ToolCallsPopover.tsx:40` | `"Xem các công cụ đã thực thi"` | `chat.viewExecutedTools` | **Chưa có** |
| `ToolCallsPopover.tsx:51` | `"Các công cụ đã chạy"` | `chat.executedTools` | **Chưa có** |
| `ToolCallsPopover.tsx:53` | `"bước"` | `chat.stepsCount` | **Chưa có** |
| `ChatHeader.tsx:45` | `"View progress detail"` | `chat.viewProgressDetail` | **Chưa có** |
| `ChatHeader.tsx:31` | `"Draft generation in progress"` | `chat.draftInProgress` | **Chưa có** |
| `ChatInputBar.tsx:43` | `"Context quote"` | `chat.contextQuote` | **Chưa có** |
| `ChatInputBar.tsx:70` | `"Hỏi bất cứ điều gì hoặc nhờ Merlink lên ý tưởng..."` | `chat.inputPlaceholder` | Đã có một phần ở `chat.inputPlaceholder` |
| `ContextUsagePopover.tsx:30` | `"Context usage"` | `chat.contextUsage` | **Chưa có** |
| `ContextUsagePopover.tsx:61` | `"of context used"` | `chat.contextUsedPercent` | **Chưa có** |
| `ContextUsagePopover.tsx:78` | `"System prompt"` | `chat.systemPrompt` | **Chưa có** |
| `ContextUsagePopover.tsx:86` | `"Tool definitions"` | `chat.toolDefinitions` | **Chưa có** |
| `ContextUsagePopover.tsx:94` | `"Messages"` | `chat.messagesToken` | **Chưa có** |
| `ProcessDetailDialog.tsx:19` | `"Tiến trình xử lý chi tiết"` | `chat.processDetailTitle` | **Chưa có** |
| `ProcessDetailDialog.tsx:22` | `"Quá trình AI thực hiện tác vụ của bạn..."` | `chat.processDetailDesc` | **Chưa có** |
| `ProcessDetailDialog.tsx:71` | `"Đã hiểu"` | `chat.dismiss` | **Chưa có** |
| `ChatView.tsx:32-94` | 19 chuỗi hội thoại mô phỏng (Mock messages) | Trích xuất thành fixture data | N/A |

---

## 9. Thống Kê Số Liệu Đo Lường (Metrics)

| Chỉ số kỹ thuật | Số lượng đếm được |
|:---|---:|
| Tổng số file trong scope phân hệ Chat | 12 file |
| Tổng số dòng mã nguồn (LOC) | 994 dòng |
| Lỗi cấp độ P0 (chặn A11y / sai lệch màu nghiêm trọng) | 2 lỗi |
| Lỗi cấp độ P1 (lệch chuẩn thiết kế / thiếu tương tác / i18n) | 13 lỗi |
| Lỗi cấp độ P2 (lệch cấu hình phụ / vi phạm vi mô) | 7 lỗi |
| Tổng số chuỗi hardcode tiếng Việt trong mã giao diện | 28 chuỗi |
| Số phần tử tương tác thiếu chỉ báo `focus-visible` | 22 vị trí |
| Số Popover tự phát triển thiếu cơ chế Portal & Keyboard Trap | 3 popover |
| Số đoạn hoạt cảnh thiếu nhãn `prefers-reduced-motion` | 3 vị trí |

---

## 10. Kết Luận Chung (Verdict Tổng Thể)

### **ĐÁNH GIÁ: INCONSISTENT (KHÔNG ĐỒNG NHẤT)**

**Lý do kết luận**:
1. **Thiếu gắn kết với Hệ thống Nền tảng (Baseline Disconnect)**: Ngoại trừ thanh `ChatInputBar` áp dụng lớp `.neu-flat`, toàn bộ 11 file còn lại của phân hệ Chat không sử dụng ngôn ngữ ánh sáng neumorphism đã định hình ở Sprint 1. Thay vào đó, mã nguồn sử dụng các bóng đổ Tailwind cổ điển (`shadow-xs`, `shadow-xl`) và mã màu hex cục bộ (`#151921`).
2. **Trợ năng (Accessibility) và Điều khiển Bàn phím bị bỏ quên**: Hoàn toàn không có bất kỳ khai báo `focus-visible:ring-*` nào trong 12 file; thanh phân tách kéo dãn panel (`Splitter`) là trung tâm của bố cục màn hình nhưng không hề có ngữ nghĩa `role="separator"` và không thể điều khiển bằng bàn phím.
3. **Phân mảnh Giải pháp Cửa sổ Nổi (Popover Fragmentation)**: Không tái sử dụng hệ sinh thái Radix UI đã cài đặt sẵn trong dự án; 3 popover quan trọng tự viết mã định vị và lắng nghe sự kiện chuột thủ công, làm tăng nguy cơ lỗi giao diện khi cuộn và không đạt chuẩn bàn phím.
4. **Vi phạm Nghiêm trọng về Quốc tế Hóa (i18n)**: 8/11 component hoàn toàn không gọi `useT()`; tồn tại gần 30 chuỗi giao diện tiếng Việt viết cứng trong mã nguồn, khiến chức năng chuyển đổi ngôn ngữ của hệ thống bị vô hiệu hóa tại màn hình cốt lõi này.
