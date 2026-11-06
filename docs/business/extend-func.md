# Business Requirements Document (BRD) - AI Factory

> **Dự án:** AI Factory (LLM Provider & Test Request Hub)  
> **Phiên bản:** v1.0.0  
> **Trạng thái:** Hoàn thiện giai đoạn 1 (Core Provider & Chat Testing)  
> **Đối tượng:** Product Owner, AI Engineers, Frontend/Backend Developers

---

## 1. Giới thiệu tổng quan (Executive Summary)

**AI Factory** là một ứng dụng Desktop & Web gọn nhẹ, chuyên dụng giúp các nhà phát triển AI, kỹ sư Prompt và chuyên gia phần mềm quản lý tập trung toàn bộ các LLM Provider (bao gồm cả Cloud API và Local AI Agents/Proxies), tự động đồng bộ danh sách model và thử nghiệm prompt trực tiếp thông qua một khung chat test request được giới hạn chặt chẽ (mặc định tối đa 500 tokens).

Dự án giải quyết trực tiếp bài toán phân mảnh khi làm việc với nhiều nhà cung cấp AI khác nhau, loại bỏ sự cồng kềnh khi phải cấu hình thủ công hoặc mở nhiều nền tảng/công cụ kiểm thử riêng biệt.

---

## 2. Bài toán thực tế & Nỗi đau của người dùng (Problem Statement)

1. **Phân mảnh cấu hình & API Keys:**
   - Kỹ sư AI thường sở hữu nhiều tài khoản và khoá API: OpenAI, Anthropic Claude, Google Gemini, DeepSeek, Kimi, Qwen... Mỗi bên có một Base URL, cấu trúc Header xác thực (`Authorization: Bearer`, `x-api-key`, `x-goog-api-key`) và định dạng payload khác nhau.
2. **Khó kiểm soát Local AI Agents:**
   - Khi chạy các dịch vụ AI local hoặc proxy như Antigravity (port 8045), Ollama (port 11434), OpenCode (port 4096), vLLM, LM Studio..., người dùng không biết rõ dịch vụ nào đang Online/Offline và các model nào đang sẵn sàng sử dụng.
3. **Mất công cài đặt môi trường chỉ để test nhanh một Prompt:**
   - Để kiểm tra khả năng trả lời hoặc đo độ trễ (latency) của một model với một prompt ngắn, kỹ sư thường phải viết script Python/curl hoặc mở các Web UI cồng kềnh.
4. **Nguy cơ lãng phí chi phí / Quá hạn mức Token (Token Burn):**
   - Khi test request trên các model lớn (Claude Opus, GPT-4o, Gemini 1.5 Pro), nếu không kiểm soát số token sinh ra (`max_tokens`), câu trả lời dài dòng có thể gây lãng phí chi phí và quota API.

---

## 3. Mục tiêu sản phẩm (Product Objectives)

- **Tập trung hoá (Centralization):** Đưa toàn bộ Cloud Provider và Local Agent vào một giao diện trực quan duy nhất.
- **Tự động hoá khám phá (Auto-discovery):** Tự động phát hiện các local AI agent đang chạy nền trên máy người dùng, cho phép kết nối 1-click.
- **Truy xuất Model động (Dynamic Model Sync):** Fetch trực tiếp danh sách model khả dụng từ provider thông qua API thay vì hardcode danh sách model tĩnh.
- **Khung Chat Test Request chuyên biệt:** Cung cấp sân chơi thử nghiệm prompt nhanh, phân nhóm model theo từng Provider (`<optgroup>`), áp dụng hạn mức 500 tokens để kiểm thử an toàn và tức thì.
- **Lưu trữ cục bộ & Bảo mật (Privacy First):** Toàn bộ API keys và cấu hình được lưu trữ an toàn trong máy người dùng (`localStorage` / local IPC), không chuyển qua bất kỳ máy chủ trung gian nào.

---

## 4. Chân dung người dùng (User Personas)

| Persona | Vai trò | Nhu cầu chính |
| :--- | :--- | :--- |
| **Alex - AI/Prompt Engineer** | Phát triển prompt & benchmark model | Cần test nhanh cùng một prompt qua nhiều model (Gemini 3.8 Flash, Claude Opus, Antigravity) để so sánh phong cách trả lời và độ trễ. |
| **Minh - Full-stack Developer** | Tích hợp LLM vào hệ thống web/mobile | Cần một hub kiểm tra API key, Base URL và test kết nối trước khi đưa config vào `.env` backend. |
| **Tuấn - On-premise / Local AI Enthusiast** | Chạy LLM local (Ollama, Antigravity) | Cần biết ngay agent local nào đang chạy, tải được bao nhiêu model và test prompt offline. |

---

## 5. Tính năng chính (Key Features & Functional Requirements)

### 5.1. Quản lý Provider (Provider Management)
- **Local Agents Discovery:**
  - Tự động thăm dò trạng thái các dịch vụ nội bộ (Antigravity, Ollama, OpenCode...).
  - Hiển thị badge trạng thái `ONLINE` / `OFFLINE` theo thời gian thực.
  - Hỗ trợ **1-click Connect** nạp thẳng model vào hệ sinh thái.
- **Hỗ trợ Cloud Providers phổ biến:**
  - Google Gemini (hỗ trợ OpenAI compatibility & Google Key).
  - OpenAI (GPT-4o, GPT-4o-mini...).
  - Anthropic Claude (Claude 3.5 Sonnet, Claude Opus 4.6/5.5...).
  - DeepSeek (deepseek-chat, deepseek-reasoner).
  - Kimi (Moonshot), Qwen (DashScope), Z.ai.
- **Custom Model API / Relay:**
  - Hỗ trợ nhập Base URL tuỳ biến (vLLM, LM Studio, LiteLLM, Gateway tự dựng) kèm API key tuỳ chọn.
- **Dynamic Model Fetching:**
  - Nút **"Fetch Available Models"** gọi trực tiếp API của provider để lấy danh sách model thực tế mà tài khoản/agent sở hữu.
- **Quản lý & Bền vững (Persistence):**
  - Lưu trữ danh sách provider đã kết nối vào bộ nhớ máy, tự động tải lại khi khởi động lại app.
  - Hỗ trợ làm mới (`Refresh`) danh sách model hoặc xoá (`Delete`) provider bất kỳ.

### 5.2. Khung Chat Thử nghiệm (Interactive Test Playground)
- **Select Box nhóm theo Provider:**
  - Dropdown phân chia danh mục rõ ràng bằng `optgroup` (ví dụ: `Antigravity (Local) (27 models)`, `Google Gemini (5 models)`...).
  - Tự động nạp model mặc định hoặc model ưu tiên.
- **Giới hạn Token (Max Tokens Cap):**
  - Mặc định áp dụng giới hạn **500 tokens** để phục vụ việc test request an toàn, không tốn quota.
  - Cho phép người dùng linh hoạt tuỳ biến giữa các mốc: `100`, `250`, `500 (Test)`, `1000` tokens.
- **Trải nghiệm Chat tương tác & Chế độ xem kép (Dual View Modes):**
  - Chuyển đổi linh hoạt giữa chế độ **💬 Chat** (bong bóng hội thoại trực quan) và **📜 JSON Stream** (hiển thị trực tiếp dữ liệu thô NDJSON).
  - Bóc tách chi tiết luồng JSON Events từ OpenCode: hiển thị số token vào/ra (`input` / `output` / `reasoning`), Session ID, và nút copy toàn bộ mảng JSON.
  - Đo lường và hiển thị thời gian phản hồi theo thời gian thực (`⚡ latency ms`).
  - Gắn nhãn Provider và Model xử lý trên từng tin nhắn.
  - Nút **Copy nhanh** nội dung phản hồi.
  - Các nút **Quick Prompt gợi ý** (*"Giới thiệu 1 câu"*, *"Count 1 to 5"*, *"Python reverse string"*).
  - Nút **Clear Chat** để làm sạch khung chat bất kỳ lúc nào.

---

## 6. Tiêu chí thành công (Success Metrics - KPIs)

1. **Thời gian Onboarding Provider < 10 giây:** Người dùng có thể kết nối Antigravity hoặc nhập OpenAI key và fetch danh sách model thành công trong vòng 10 giây.
2. **Tỉ lệ Fetch Model thành công:** 100% với các provider chuẩn OpenAI-compatible, Google Gemini, Anthropic Claude và Ollama.
3. **Độ trễ phản hồi (Response Latency):** Phản hồi từ local agent (Antigravity) được hiển thị đầy đủ trên giao diện dưới 500ms đối với các câu test ngắn.
4. **An toàn chi phí:** Không phát sinh bất kỳ request nào vượt quá giới hạn token đã định cấu hình (mặc định 500 tokens).

---

## 7. Kế hoạch phát triển tương lai (Future Roadmap)

- **Giai đoạn 2 (Streaming & Side-by-side Comparison):**
  - Bổ sung Server-Sent Events (SSE) để chữ hiển thị dạng gõ máy (stream tokens).
  - Chế độ so sánh song song: Gửi 1 prompt đến 2 hoặc 3 model cùng lúc để đối chiếu chất lượng và tốc độ.
- **Giai đoạn 3 (Bảo mật & Quản lý nâng cao):**
  - Mã hoá API Key thông qua khoá hệ thống (OS Keychain / Secret Service).
  - Lưu và xuất lịch sử hội thoại ra Markdown / JSON.
  - Hỗ trợ System Prompt tuỳ biến và điều chỉnh Temperature.
