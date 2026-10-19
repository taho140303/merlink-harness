import type { AgentEvent, ChatTurn, ContextUsage, FetchedModel, MessageEnvelope, ProviderConfig, SuperJsonProgress, ToolCall } from "../types";
import type { SendChatArgs } from "./ipc";

const MODEL_PRESETS: Record<
  string,
  Partial<{
    context_window: string;
    max_tokens: string;
    input_types: { text: boolean; image: boolean };
    reasoning_level: boolean;
  }>
> = {
  "gpt-4o": {
    context_window: "128K",
    max_tokens: "4K",
    input_types: { text: true, image: true },
  },
  "claude-sonnet-4-5": {
    context_window: "200K",
    max_tokens: "8K",
    input_types: { text: true, image: true },
    reasoning_level: true,
  },
  "deepseek-reasoner": { reasoning_level: true },
};

/**
 * Preview-only model presets (Sprint 11.13 · D2).
 *
 * The real app ships **no** provider catalog any more; a provider's model
 * metadata is whatever the author configured. These presets exist solely so the
 * browser preview can render a couple of convincing models without a backend.
 */
function mockModel(id: string) {
  const preset = MODEL_PRESETS[id] || {};
  return {
    id,
    name: id,
    context_window: preset.context_window ?? "128K",
    max_tokens: preset.max_tokens ?? "4K",
    input_types: {
      text: preset.input_types?.text ?? true,
      image: preset.input_types?.image ?? false,
    },
    reasoning_level: preset.reasoning_level ?? false,
  };
}

function mockProvider(
  id: string,
  name: string,
  kind: string,
  base_url: string,
  modelIds: string[],
): ProviderConfig {
  return {
    id,
    name,
    kind,
    api_key: "",
    base_url,
    models: modelIds.map(mockModel),
    default_model_id: modelIds[0] ?? "",
  };
}

/**
 * Two sample providers so the browser preview has something to select, and the
 * reasoning panel stays reachable (Sprint 11.13 · D2).
 *
 * This is **not** a mirror of Rust any more: production ships no catalog at all,
 * and `load_providers` in the desktop app returns only what the author added.
 */
function createMockProviders(): ProviderConfig[] {
  return [
    mockProvider("openai", "OpenAI", "openai", "https://api.openai.com/v1", [
      "gpt-4o",
      "gpt-4o-mini",
    ]),
    mockProvider("deepseek", "DeepSeek", "openai", "https://api.deepseek.com", [
      "deepseek-chat",
      "deepseek-reasoner",
    ]),
  ];
}

/**
 * Browser-only storage for the preview build. The desktop app persists to
 * `~/.merlink/config.json` through the Rust backend instead.
 */
let browserProviders: ProviderConfig[] | null = null;

function readBrowserProviders(): ProviderConfig[] {
  if (!browserProviders) browserProviders = createMockProviders();
  return browserProviders;
}

/* ------------------------------------------------------------------ */
/* Browser-preview state and tools                                     */
/* ------------------------------------------------------------------ */

/** Preview subset only: 25 paths, not a second full Rust validator. */
interface MockSession {
  projectId: string;
  messages: MessageEnvelope[];
  values: Map<string, unknown>;
  unnamedCount: number;
}
const sessions = new Map<string, MockSession>();
function mockSession(sessionId: string): MockSession {
  let session = sessions.get(sessionId);
  if (!session) {
    session = { projectId: "", messages: [], values: new Map(), unnamedCount: 0 };
    sessions.set(sessionId, session);
  }
  return session;
}
function projectSession(projectId: string): MockSession | undefined {
  return [...sessions.values()].find((session) => session.projectId === projectId);
}
function uniqueProjectId(base: string): string {
  let candidate = base;
  let suffix = 2;
  while (projectSession(candidate)) candidate = `${base}-${suffix++}`;
  return candidate;
}

/** The 11 required paths, used for the mock `missing_required` result (D7). */
const MOCK_REQUIRED_PATHS = [
  "require.overview.name",
  "require.overview.logline.trigger-event",
  "require.overview.logline.main-character",
  "require.overview.logline.action",
  "require.overview.logline.risk",
  "require.overview.logline.major-loss",
  "require.overview.plan.arc.count",
  "require.overview.plan.chapter.count",
  "require.overview.plan.word.count",
  "require.genre.primary",
  "require.theme.question",
] as const;

function mockIsFilled(value: unknown): boolean {
  if (value === null || value === undefined) return false;
  if (typeof value === "string") return value.trim().length > 0;
  if (Array.isArray(value)) return value.length > 0;
  return true;
}

function mockMissingRequired(values: Map<string, unknown>): string[] {
  return MOCK_REQUIRED_PATHS.filter((path) => !mockIsFilled(values.get(path)));
}

/** Mirrors the D3 slug rule: lowercase, runs of non-`[a-z0-9]` collapse to `-`. */
function mockSlug(name: string): string {
  const slug = name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return slug || "project";
}

/** Inserts `value` at a dotted path, creating the intermediate objects. */
function assignPath(
  document: Record<string, unknown>,
  path: string,
  value: unknown,
): void {
  const segments = path.split(".");
  let cursor = document;
  for (const segment of segments.slice(0, -1)) {
    const next = cursor[segment];
    if (typeof next !== "object" || next === null) cursor[segment] = {};
    cursor = cursor[segment] as Record<string, unknown>;
  }
  cursor[segments[segments.length - 1]] = value;
}

function mockSuperJsonDocument(values: Map<string, unknown> = new Map()): Record<string, unknown> {
  const document: Record<string, unknown> = { require: {}, optional: {} };
  for (const [path, value] of values) assignPath(document, path, value);
  return document;
}

/** How many filled leaf paths sit under `require.` / `optional.`. */
function countMockFilled(values: Map<string, unknown>, prefix: string): number {
  let count = 0;
  for (const [path, value] of values) {
    if (!path.startsWith(prefix)) continue;
    if (!mockIsFilled(value)) continue;
    count += 1;
  }
  return count;
}

/* ------------------------------------------------------------------ */
/* Sprint 11.11 · the super json as prose (browser preview)            */
/* ------------------------------------------------------------------ */

/**
 * Mirrors `super_json::summary` (D2/D3/D4). The Rust template is authoritative;
 * this copy only exists so the `Chi tiết` dialog renders real prose without
 * Tauri. Placeholders and enum wording are copied verbatim from the spec.
 */
const SUMMARY_PLACEHOLDER = "[chưa có thông tin]";

const MOCK_LANGUAGE_LABELS: Record<string, string> = {
  VI: "tiếng Việt",
  EN: "tiếng Anh",
  JA: "tiếng Nhật",
  ZH_HANT: "chữ Hán phồn thể",
  ZH_HANS: "chữ Hán giản thể",
  ZH_CN: "chữ Hán giản thể",
};

const MOCK_RESOLUTION_LABELS: Record<string, string> = {
  RESOLVED: "kết thúc rõ ràng",
  OPEN: "để ngỏ",
  AMBIGUOUS: "mơ hồ",
};

const MOCK_SCALE_LABELS: Record<string, string> = {
  REALM: "cõi",
  WORLD: "thế giới",
  REGION: "vùng",
  COUNTRY: "quốc gia",
  CITY: "thành phố",
  TOWN: "thị trấn",
  FACILITY: "công trình",
  STREET: "đường phố",
  ROOM: "căn phòng",
};

/** `null` for a missing, blank or empty-array leaf; text otherwise. */
function mockTextAt(values: Map<string, unknown>, path: string): string | null {
  const value = values.get(path);
  if (value === null || value === undefined) return null;
  if (Array.isArray(value)) {
    const joined = value
      .filter((item): item is string => typeof item === "string" && item.trim() !== "")
      .join(", ");
    return joined || null;
  }
  if (typeof value === "string") return value.trim() || null;
  if (typeof value === "number") return String(value);
  return null;
}

function mockEnumAt(values: Map<string, unknown>, path: string, table: Record<string, string>): string | null {
  const code = mockTextAt(values, path);
  return code === null ? null : (table[code] ?? null);
}

function mockOr(value: string | null): string {
  return value ?? SUMMARY_PLACEHOLDER;
}

function mockTerminate(text: string): string {
  const trimmed = text.trimEnd();
  return /[.?!…]$/.test(trimmed) ? trimmed : `${trimmed}.`;
}

/** `90000` → `90.000` (the same rule as Rust's `with_thousands`, D5). */
function mockThousands(value: number): string {
  const digits = String(Math.abs(Math.trunc(value)));
  let out = "";
  for (let index = 0; index < digits.length; index += 1) {
    if (index > 0 && (digits.length - index) % 3 === 0) out += ".";
    out += digits[index];
  }
  return value < 0 ? `-${out}` : out;
}

function mockIntAt(values: Map<string, unknown>, path: string): number | null {
  const value = values.get(path);
  return typeof value === "number" && Number.isFinite(value) ? value : null;
}

function mockThousandOrPlaceholder(value: number | null): string {
  return value === null ? SUMMARY_PLACEHOLDER : mockThousands(value);
}

/**
 * Mirrors the Sprint 11.12 · D5 template: six paragraphs, label sentences when a
 * field is missing, no doubled punctuation and `.` thousands separators. The Rust
 * function is authoritative; this copy exists only so browser preview renders the
 * same prose.
 */
function mockSummary(values: Map<string, unknown> = new Map()): string {
  const secondary = mockTextAt(values, "optional.genre.secondary");
  const secondaryClause = secondary ? `, kèm ${secondary}` : "";

  const author = mockTextAt(values, "optional.overview.author");
  const authorSentence = author
    ? `Tác giả là ${mockTerminate(author)}`
    : `Tác giả: ${SUMMARY_PLACEHOLDER}.`;

  const trigger = mockTextAt(values, "require.overview.logline.trigger-event");
  const triggerSentence = trigger
    ? `Câu chuyện bắt đầu khi ${mockTerminate(trigger)}`
    : `Sự kiện mở đầu: ${SUMMARY_PLACEHOLDER}.`;

  const character = mockTextAt(values, "require.overview.logline.main-character");
  const action = mockTextAt(values, "require.overview.logline.action");
  let cast: string;
  if (character && action) {
    cast = mockTerminate(`Nhân vật chính là ${character}, ${action}`);
  } else if (character) {
    cast = `Nhân vật chính là ${mockTerminate(character)} Hành động chính: ${SUMMARY_PLACEHOLDER}.`;
  } else if (action) {
    cast = `Nhân vật chính: ${SUMMARY_PLACEHOLDER}. Hành động chính: ${mockTerminate(action)}`;
  } else {
    cast = `Nhân vật chính: ${SUMMARY_PLACEHOLDER}. Hành động chính: ${SUMMARY_PLACEHOLDER}.`;
  }

  const risk = mockTextAt(values, "require.overview.logline.risk");
  const loss = mockTextAt(values, "require.overview.logline.major-loss");
  let stake: string;
  if (risk && loss) {
    stake = mockTerminate(`Mối nguy là ${risk}, và cái giá có thể phải chịu là ${loss}`);
  } else if (risk) {
    stake = `Mối nguy là ${mockTerminate(risk)} Mất mát lớn: ${SUMMARY_PLACEHOLDER}.`;
  } else if (loss) {
    stake = `Rủi ro: ${SUMMARY_PLACEHOLDER}. Cái giá có thể phải chịu là ${mockTerminate(loss)}`;
  } else {
    stake = `Rủi ro: ${SUMMARY_PLACEHOLDER}. Mất mát lớn: ${SUMMARY_PLACEHOLDER}.`;
  }

  const message = mockTextAt(values, "optional.theme.message");
  const messageSentence = message
    ? `Thông điệp hướng tới là ${mockTerminate(message)}`
    : `Thông điệp hướng tới: ${SUMMARY_PLACEHOLDER}.`;

  const resolutionType = mockEnumAt(values, "optional.theme.resolution-type", MOCK_RESOLUTION_LABELS);
  const resolution = mockTextAt(values, "optional.theme.resolution") ?? SUMMARY_PLACEHOLDER;
  const answer = resolutionType
    ? `Cách trả lời được chọn là ${resolutionType}, với nội dung ${mockTerminate(resolution)}`
    : `Cách trả lời được chọn: ${SUMMARY_PLACEHOLDER}, với nội dung: ${mockTerminate(resolution)}`;

  const arc = mockIntAt(values, "require.overview.plan.arc.count");
  const chapter = mockIntAt(values, "require.overview.plan.chapter.count");
  const word = mockIntAt(values, "require.overview.plan.word.count");
  const plan =
    arc !== null && chapter !== null && word !== null
      ? `Dự kiến dài ${mockThousands(arc)} arc, ${mockThousands(chapter)} chương, khoảng ${mockThousands(word)} từ.`
      : `Số arc dự kiến: ${mockThousandOrPlaceholder(arc)}. Số chương dự kiến: ${mockThousandOrPlaceholder(chapter)}. Tổng số từ mục tiêu: ${mockThousandOrPlaceholder(word)}.`;
  const arcTolerance = mockIntAt(values, "optional.overview.plan.arc.tolerance");
  const chapterTolerance = mockIntAt(values, "optional.overview.plan.chapter.tolerance");
  const wordTolerance = mockIntAt(values, "optional.overview.plan.word.tolerance");
  const tolerance =
    arcTolerance !== null || chapterTolerance !== null || wordTolerance !== null
      ? ` Sai số chấp nhận: ${mockThousandOrPlaceholder(arcTolerance)} arc, ${mockThousandOrPlaceholder(chapterTolerance)} chương, ${mockThousandOrPlaceholder(wordTolerance)} từ.`
      : "";

  const power = values.get("optional.world.has-power-system");
  const powerSentence =
    power === true
      ? "Thế giới có hệ thống sức mạnh."
      : power === false
        ? "Thế giới không có hệ thống sức mạnh."
        : `Thế giới có hệ thống sức mạnh hay không: ${SUMMARY_PLACEHOLDER}.`;
  const scale = mockEnumAt(values, "optional.world.setting-scale", MOCK_SCALE_LABELS);
  const scaleSentence = scale
    ? `Phạm vi không gian sâu nhất cần dựng là ${mockTerminate(scale)}`
    : `Phạm vi không gian sâu nhất cần dựng: ${SUMMARY_PLACEHOLDER}.`;

  const description = mockTextAt(values, "optional.overview.description");

  return [
    `Dự án truyện có tên là ${mockOr(mockTextAt(values, "require.overview.name"))}, thuộc thể loại ${mockOr(mockTextAt(values, "require.genre.primary"))}${secondaryClause}. ${authorSentence} Truyện viết bằng ${mockOr(mockEnumAt(values, "optional.overview.language", MOCK_LANGUAGE_LABELS))}, hướng tới độc giả ${mockOr(mockTextAt(values, "optional.overview.target-audience"))}, độ tuổi ${mockTerminate(mockOr(mockTextAt(values, "optional.overview.age-rating")))}`,
    `${triggerSentence} ${cast} ${stake}`,
    `Bộ truyện đặt ra câu hỏi: ${mockTerminate(mockOr(mockTextAt(values, "require.theme.question")))} ${messageSentence} ${answer}`,
    `${plan}${tolerance}`,
    `${powerSentence} ${scaleSentence}`,
    `Tóm tắt nội dung: ${mockTerminate(mockOr(description))}`,
  ].join("\n\n");
}
/** Mirrors `super_json::progress` in Rust (D6/D7). */
function mockProgress(session: MockSession | undefined): SuperJsonProgress {
  const projectId = session?.projectId ?? "";
  const filled_required = projectId ? countMockFilled(session!.values, "require.") : 0;
  const filled_optional = projectId ? countMockFilled(session!.values, "optional.") : 0;
  const percent = projectId
    ? Math.round((70 * filled_required) / 11 + (30 * filled_optional) / 14)
    : 0;
  const label = !projectId
    ? "Chưa có project"
    : filled_required === 11 && filled_optional === 14
      ? "Độ mạch lạc: tốt"
      : filled_required === 11
        ? "Độ mạch lạc tạm chấp nhận"
        : "Đang thu thập thông tin";
  return {
    has_project: Boolean(projectId),
    project_id: projectId,
    percent,
    label,
    filled_required,
    required_total: 11,
    filled_optional,
    optional_total: 14,
  };
}

/**
 * The values the preview agent proposes when the author says they have no idea.
 * 13 fields; `require.overview.name` is only written when the author stated one.
 */
const MOCK_SUGGESTIONS: Array<[string, unknown]> = [
  ["require.overview.logline.trigger-event", "chiếc đồng hồ cuối cùng ngừng chạy"],
  ["require.overview.logline.main-character", "một người thợ đồng hồ mù"],
  ["require.overview.logline.action", "truy tìm người đã chế tạo ra chiếc đồng hồ"],
  ["require.overview.logline.risk", "đánh mất ký ức của chính mình"],
  ["require.overview.logline.major-loss", "đứa con gái duy nhất"],
  ["require.overview.plan.arc.count", 5],
  ["require.overview.plan.chapter.count", 120],
  ["require.overview.plan.word.count", 90000],
  ["optional.overview.plan.arc.tolerance", 1],
  ["optional.overview.plan.chapter.tolerance", 10],
  ["optional.overview.plan.word.tolerance", 5000],
  ["require.genre.primary", "trinh thám"],
  ["require.theme.question", "ký ức có làm nên con người hay không"],
];

/**
 * Mirrors `tools::PATH_LABELS` in Rust: the same Vietnamese wording is used for
 * the live phase messages and for the tool-log dialog (D3/D8).
 */
const MOCK_PATH_LABELS: Record<string, string> = {
  "require.overview.name": "tên truyện",
  "require.overview.logline.trigger-event": "sự kiện khởi động",
  "require.overview.logline.main-character": "nhân vật chính",
  "require.overview.logline.action": "hành động chính",
  "require.overview.logline.risk": "rủi ro",
  "require.overview.logline.major-loss": "mất mát lớn",
  "require.overview.plan.arc.count": "số arc",
  "require.overview.plan.chapter.count": "số chương",
  "require.overview.plan.word.count": "số từ",
  "require.genre.primary": "thể loại chính",
  "require.theme.question": "câu hỏi chủ đề",
  "optional.overview.description": "mô tả truyện",
  "optional.overview.author": "tác giả",
  "optional.overview.language": "ngôn ngữ",
  "optional.overview.target-audience": "đối tượng độc giả",
  "optional.overview.age-rating": "độ tuổi",
  "optional.overview.plan.arc.tolerance": "sai lệch số arc",
  "optional.overview.plan.chapter.tolerance": "sai lệch số chương",
  "optional.overview.plan.word.tolerance": "sai lệch số từ",
  "optional.genre.secondary": "thể loại phụ",
  "optional.theme.resolution-type": "kiểu giải quyết chủ đề",
  "optional.theme.resolution": "câu trả lời chủ đề",
  "optional.theme.message": "thông điệp",
  "optional.world.has-power-system": "có hệ thống sức mạnh",
  "optional.world.setting-scale": "quy mô thế giới",
};

function mockStatusMessage(name: string, args: Record<string, unknown>): string {
  switch (name) {
    case "project_create": {
      const named = typeof args.name === "string" && args.name.trim().length > 0;
      return named ? "Đang ghi nhận tên truyện và tạo project" : "Đang tạo project";
    }
    case "super_json_missing":
      return "Đang kiểm tra các trường còn thiếu";
    case "super_json_get":
      return "Đang đọc siêu JSON";
    case "super_json_set": {
      const path = typeof args.path === "string" ? args.path : "";
      return `Đang ghi nhận ${MOCK_PATH_LABELS[path] ?? path}`;
    }
    case "echo":
      return "Đang kiểm tra kết nối tool";
    default:
      return `Đang chạy ${name}`;
  }
}

/** Mirrors `tools::execute_tool` for the preview: same names, same errors. */
function mockExecuteTool(
  name: string,
  args: Record<string, unknown>,
  sessionId: string,
  projectId: string,
): unknown {
  const session = mockSession(sessionId);
  // Match the native IPC guard for document tools, without reimplementing validation.
  if (["super_json_get", "super_json_set", "super_json_missing"].includes(name)) {
    if (!session.projectId) return { error: "This chat is not bound to a project. Call project_create first." };
    if (projectId && projectId !== session.projectId) return { error: "Project does not belong to this session" };
  }
  switch (name) {
    case "project_create": {
      // D7: `name` is optional. Without one the project is created unnamed and
      // `require.overview.name` is left untouched — never invented.
      if (
        args.name !== undefined &&
        args.name !== null &&
        typeof args.name !== "string"
      ) {
        return { error: "project_create requires a non-empty string 'name' when it is provided" };
      }
      const title = typeof args.name === "string" ? args.name.trim() : "";
      if (session.projectId) {
        return { error: `This chat already belongs to project ${session.projectId}` };
      }
      if (title) {
        session.projectId = uniqueProjectId(mockSlug(title));
        session.values.set("require.overview.name", title);
        return {
          ok: true,
          project_id: session.projectId,
          name: title,
          missing_required: mockMissingRequired(session.values),
        };
      }
      session.unnamedCount += 1;
      session.projectId = uniqueProjectId(session.unnamedCount === 1 ? "du-an" : `du-an-${session.unnamedCount}`);
      return {
        ok: true,
        project_id: session.projectId,
        missing_required: mockMissingRequired(session.values),
      };
    }
    case "super_json_set": {
      const path = typeof args.path === "string" ? args.path : "";
      if (!path) return { error: "super_json_set requires a string 'path'" };
      if (!(path in MOCK_PATH_LABELS)) return { error: `Unknown path: ${path}` };
      session.values.set(path, args.value);
      return { ok: true, path, value: args.value };
    }
    case "super_json_get":
      return mockSuperJsonDocument(session.values);
    case "super_json_missing":
      return { missing: mockMissingRequired(session.values), required_total: 11 };
    case "echo":
      return args;
    default:
      return { error: `Unknown tool: ${name}` };
  }
}

/**
 * The title the author actually stated, or `""` when they named nothing (D7).
 * The preview must not invent a title either.
 */
function mockStatedTitle(text: string): string {
  const quoted = /["“']([^"”']{1,60})["”']/.exec(text)?.[1];
  const afterLa = /\blà\s+([^\n]{1,60})/i.exec(text)?.[1];
  return (quoted ?? afterLa ?? "").trim();
}

function mockSendChat(args: SendChatArgs): Promise<ChatTurn> {
  const session = mockSession(args.sessionId);
  if (args.projectId && session.projectId && args.projectId !== session.projectId) {
    return Promise.reject(new Error("Project does not belong to this session"));
  }
  // Native send_chat seeds a new session from a supplied project id.
  if (args.projectId && !session.projectId) {
    if (projectSession(args.projectId)) return Promise.reject(new Error("Project does not belong to this session"));
    session.projectId = args.projectId;
  }
  const providers = readBrowserProviders();
  const provider =
    providers.find((item) => item.id === args?.providerId) || providers[0];
  const model =
    provider?.models.find((item) => item.id === args?.modelId) ||
    provider?.models[0];
  const lastUser = [...(args?.messages || [])]
    .reverse()
    .find((message) => message.role === "user");
  const text = lastUser?.content ?? "";
  const stamp = Date.now();
  for (const message of args.messages) {
    if (message.role === "user" && !session.messages.some((stored) => stored.id === message.id)) {
      session.messages.push(message);
    }
  }

  const previewNote =
    "\n\nThis is a browser preview, so no AI service was contacted. " +
    "Run `pnpm tauri dev` and add an API key in Settings to reach a real model.";

  const wantsIdeas = /không có ý tưởng|gợi ý|điền hết|no idea|suggest/i.test(text);
  // D7: `project_create` only gets a name when the author actually stated one.
  const statedTitle = mockStatedTitle(text);

  // One round of tool calls per group, then a final round that only answers —
  // the same shape the backend produces, so the phase list grows the same way.
  const toolRounds: Array<Array<{ name: string; args: Record<string, unknown> }>> = [];
  if (!session.projectId) {
    // D7: no stated title -> call project_create without a name. The project is
    // still created, and require.overview.name stays empty.
    toolRounds.push([
      {
        name: "project_create",
        args: statedTitle ? { name: statedTitle } : {},
      },
    ]);
  }
  if (wantsIdeas) {
    // idea.md: when the author has no idea, the agent proposes and fills in.
    toolRounds.push(
      MOCK_SUGGESTIONS.map(([path, value]) => ({
        name: "super_json_set",
        args: { path, value },
      })),
    );
  } else if (statedTitle) {
    toolRounds.push([
      {
        name: "super_json_set",
        args: { path: "require.overview.name", value: statedTitle },
      },
    ]);
  }
  const rounds = [...toolRounds, []];

  // D10: emit the exact same event chain as the Rust backend, with a small delay
  // so the phases visibly grow instead of appearing all at once.
  const channel = args?.channel;
  const delay = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));
  const emitEvent = async (event: AgentEvent) => {
    channel?.onmessage?.(event);
    await delay(200);
  };

  const turnStarted = Date.now();
  const calls: ToolCall[] = [];
  const results: MessageEnvelope[] = [];
  const isError = (value: unknown) =>
    typeof value === "object" && value !== null && "error" in (value as Record<string, unknown>);

  return (async () => {
    await emitEvent({
      type: "turn_started",
      session_id: args.sessionId,
      round_budget: 5,
    });

    for (let index = 0; index < rounds.length; index += 1) {
      const round = index + 1;
      const roundStarted = Date.now();
      await emitEvent({ type: "round_started", round });

      for (const planned of rounds[index]) {
        const callId = `call-${planned.name}-${stamp}-${calls.length}`;
        calls.push({ id: callId, name: planned.name, arguments: planned.args });
        const status = mockStatusMessage(planned.name, planned.args);
        const path = typeof planned.args.path === "string" ? planned.args.path : null;
        await emitEvent({
          type: "tool_started",
          id: callId,
          name: planned.name,
          path,
          status,
        });

        const toolStarted = Date.now();
        const output = mockExecuteTool(planned.name, planned.args, args.sessionId, session.projectId);
        const durationMs = Date.now() - toolStarted;
        await emitEvent({
          type: "tool_finished",
          id: callId,
          name: planned.name,
          ok: !isError(output),
          duration_ms: durationMs,
          status,
        });
        results.push({
          id: `mock-tool-${callId}`,
          role: "tool",
          name: planned.name,
          tool_call_id: callId,
          content: JSON.stringify(output),
        });
      }

      await emitEvent({
        type: "round_finished",
        round,
        duration_ms: Date.now() - roundStarted,
        tool_calls: rounds[index].length,
      });
    }

    const turnDuration = Date.now() - turnStarted;
    await emitEvent({
      type: "turn_finished",
      project_id: session.projectId,
      duration_ms: turnDuration,
      rounds: rounds.length,
    });
    // Give the completed phase list a moment on screen (with its per-line times)
    // before `Chat.tsx` clears it when the turn resolves.
    await delay(700);

    const meta = {
      model: model?.name || args?.modelId || "mock-model",
      provider: provider?.name || args?.providerId || "mock-provider",
      session_id: args.sessionId,
      project_id: session.projectId,
      prompt_template_id: "harness_system.v1",
      prompt_hash: "0".repeat(64),
      tokens: {
        input_tokens: Math.max(1, Math.ceil(text.length / 4)),
        output_tokens: 42,
      },
      latency_ms: turnDuration,
      created_at: new Date().toISOString(),
    };

    const appended: MessageEnvelope[] = [];
    if (calls.length > 0) {
      appended.push({
        id: `mock-assistant-tools-${stamp}`,
        role: "assistant",
        content: "",
        tool_calls: calls,
      });
      appended.push(...results);
    }

    const markdownFixture = `# Markdown fixture\n\n## Heading two\n### Heading three\n\nParagraph **bold**, *italic*, and \`inline code\`.\nSoft line break.\n\n- bullet one\n- bullet two\n\n1. ordered one\n2. ordered two\n\n> quoted text\n\n---\n\n[Safe link](https://example.com)\n\n\`\`\`ts\nconst answer = 42;\n\`\`\``;
    const summary = wantsIdeas
      ? `[Browser preview] Đã đề xuất và điền các trường bắt buộc cho project "${session.projectId}".`
      : statedTitle
        ? `[Browser preview] Đã ghi các trường của project "${session.projectId}".`
        : `[Browser preview] Project "${session.projectId}" đã sẵn sàng.${previewNote}`;

    appended.push({
      id: `mock-assistant-${stamp}`,
      role: "assistant",
      content: `${markdownFixture}${text.includes("<img") ? `\n\n${text}` : ""}\n\n${summary}${previewNote}`,
      meta,
      // D9: reasoning is provider behaviour, exactly like the real adapters:
      // only a model with `reasoning_level` exposes thinking. gpt-4o therefore
      // renders no chevron, deepseek-reasoner does.
      ...(model?.reasoning_level
        ? {
            reasoning:
              `[Browser preview] Người dùng viết: "${text.trim().slice(0, 120)}". ` +
              "Tôi sẽ tạo project, ghi nhận đúng những trường người dùng đã nêu và không tự đặt tên cho truyện.",
          }
        : {}),
    });

    session.messages = [...session.messages, ...appended];
    return { project_id: session.projectId, messages: appended };
  })();
}

/**
 * Browser-preview answer for `fetch_models`.
 *
 * This is a *fixed preview list*, not a model catalog: in the desktop app the
 * command is implemented in Rust (`adapters::fetch_models`, Sprint 11.5 · D1),
 * which really GETs the provider's `/models` endpoint. Here there is no bridge to
 * Rust at all, so a small constant stands in — enough to exercise the UI's
 * merge + dedupe path (`pnpm dev` only).
 */
const BROWSER_FETCHED_MODELS: FetchedModel[] = [
  { id: "gpt-4o", name: "gpt-4o" },
  { id: "gpt-4o-mini", name: "GPT-4o mini" },
  { id: "o3-mini", name: "o3-mini" },
];

/** Preview transport only; the typed command map lives in ipc.ts. */
export async function browserInvoke(cmd: string, args?: unknown): Promise<unknown> {
  switch (cmd) {
    case "load_providers":
      return readBrowserProviders();
    case "save_providers": {
      const next = (args as { providers?: ProviderConfig[] } | undefined)?.providers;
      if (Array.isArray(next)) browserProviders = next;
      return undefined;
    }
    case "send_chat":
      return mockSendChat(args as SendChatArgs);
    case "list_messages": {
      const { sessionId } = args as { sessionId: string };
      return sessions.get(sessionId)?.messages ?? [];
    }
    case "fetch_models":
      // The browser never contacts a provider; the desktop adapter fetches real models.
      return BROWSER_FETCHED_MODELS;
    case "super_json_read": {
      const { projectId } = args as { projectId: string };
      return mockSuperJsonDocument(projectSession(projectId)?.values);
    }
    case "super_json_progress": {
      const { sessionId } = args as { sessionId: string };
      return mockProgress(sessions.get(sessionId));
    }
    case "super_json_summary": {
      const { projectId } = args as { projectId: string };
      return mockSummary(projectSession(projectId)?.values);
    }
    case "super_json_labels":
      // Mirrors the Rust label map for the same 25 preview paths.
      return MOCK_PATH_LABELS;
    case "context_usage": {
      const request = args as { providerId?: string; modelId?: string } | undefined;
      if (!request?.providerId || !request?.modelId) {
        return { has_model: false, total_tokens: 0, used_tokens: 0, percent: 0, system_tokens: 0, tool_tokens: 0, message_tokens: 0, measured_input_tokens: null } satisfies ContextUsage;
      }
      const system_tokens = 640, tool_tokens = 1800, message_tokens = 3560;
      const total_tokens = 128000, used_tokens = system_tokens + tool_tokens + message_tokens;
      return { has_model: true, total_tokens, used_tokens, percent: Math.round(100 * used_tokens / total_tokens), system_tokens, tool_tokens, message_tokens, measured_input_tokens: null } satisfies ContextUsage;
    }
    case "execute_tool": {
      const request = args as { toolName: string; args: Record<string, unknown>; sessionId: string; projectId: string };
      return mockExecuteTool(request.toolName, request.args, request.sessionId, request.projectId);
    }
    default:
      throw new Error(`Command "${cmd}" is not mocked in browser mode. Run the desktop app with \`pnpm tauri dev\`.`);
  }
}
