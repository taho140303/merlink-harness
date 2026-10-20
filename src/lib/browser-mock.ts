import type { AgentEvent, ChatTurn, ContextUsage, FetchedModel, MessageEnvelope, ProjectSummary, ProviderConfig, SessionSummary, SuperJsonProgress, ToolCall } from "../types";
import type { SendChatArgs } from "./ipc";
import { mockProjects } from "../data/mock";

const outlineMockActive = new Set<string>();
const outlineMockCommitted = new Set<string>();
const outlineMockChapterRuns = new Set<string>();

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

/** Preview subset only: 81 paths, not a second full Rust validator. */
interface MockSession {
  projectId: string;
  messages: MessageEnvelope[];
  values: Map<string, unknown>;
  unnamedCount: number;
  title: string | null;
  createdAt: string;
}
const sessions = new Map<string, MockSession>();

function seedMockSessions(): void {
  if (sessions.size !== 0) return;
  const seed = (id: string, projectId: string, createdAt: string, userText?: string) => {
    const session: MockSession = { projectId, messages: [], values: new Map(), unnamedCount: 0, title: null, createdAt };
    if (userText) session.messages.push({ id: `${id}-user`, role: "user", content: userText });
    sessions.set(id, session);
  };
  seed("demo-session-moon", "glass-tide", "2025-01-03T10:00:00.000Z", "Viết về một đêm trăng bên sông.");
  seed("demo-session-garden", "midnight-orchard", "2025-01-02T10:00:00.000Z");
  seed("demo-session-unbound", "", "2025-01-01T10:00:00.000Z");
}

function mockProjectSummary(projectId: string): ProjectSummary | null {
  const seeded = mockProjects.find((project) => project.id === projectId);
  const projectSessions = [...sessions.entries()].filter(([, session]) => session.projectId === projectId);
  if (!seeded && projectSessions.length === 0) return null;
  const values = projectSessions[0]?.[1].values;
  const storedName = values?.get("require.project.name");
  const genreValues = seeded ? [seeded.genre] : mockRows(values, "project_genres")
    .map((row) => typeof row.genre_id === "string" ? row.genre_id.trim() : "")
    .filter(Boolean);
  const seenGenres = new Set<string>();
  const genres = genreValues.filter((genre) => {
    const key = genre.toLocaleLowerCase();
    if (!key || seenGenres.has(key)) return false;
    seenGenres.add(key);
    return true;
  });
  const createdAt = seeded
    ? new Date(seeded.createdAt).toISOString()
    : (projectSessions.map(([, session]) => session.createdAt).sort()[0] ?? new Date().toISOString());
  return {
    id: projectId,
    title: typeof storedName === "string" && storedName.trim() ? storedName.trim() : (seeded?.title ?? "Dự án chưa đặt tên"),
    genres,
    created_at: createdAt,
    session_count: projectSessions.length,
  };
}

function mockProjectTitle(projectId: string): string | null {
  if (!projectId) return null;
  const seeded = mockProjects.find((project) => project.id === projectId);
  if (seeded) return seeded.title;
  const owner = [...sessions.values()].find((session) => session.projectId === projectId);
  const name = owner?.values.get("require.project.name");
  return typeof name === "string" && name.trim() ? name.trim() : "Dự án chưa đặt tên";
}

function mockDisplayTitle(session: MockSession): string {
  if (session.title?.trim()) return session.title.trim();
  const firstUser = session.messages.find((message) => message.role === "user")?.content ?? "";
  const collapsed = firstUser.trim().replace(/\s+/g, " ");
  if (!collapsed) return "New conversation";
  return collapsed.length > 60 ? `${Array.from(collapsed).slice(0, 60).join("")}…` : collapsed;
}

function mockSessionSummary(id: string, session: MockSession): SessionSummary {
  return { id, project_id: session.projectId, project_title: mockProjectTitle(session.projectId), title: mockDisplayTitle(session), created_at: session.createdAt };
}

seedMockSessions();
function mockSession(sessionId: string): MockSession {
  let session = sessions.get(sessionId);
  if (!session) {
    session = { projectId: "", messages: [], values: new Map(), unnamedCount: 0, title: null, createdAt: new Date().toISOString() };
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

function mockIsFilled(value: unknown): boolean {
  if (value === null || value === undefined) return false;
  if (typeof value === "string") return value.trim().length > 0;
  if (Array.isArray(value)) return value.length > 0;
  return true;
}

const MOCK_PROJECT_LABELS: Record<string, string> = {
  "require.project.name": "tên truyện",
  "require.project.language": "ngôn ngữ",
  "require.project.plan.arc.count": "số arc",
  "require.project.plan.chapter.count": "số chương",
  "require.project.plan.scene.count": "số cảnh",
  "require.project.plan.word.count": "số từ",
  "optional.project.author": "tác giả",
  "optional.project.summarize": "tóm tắt",
  "optional.project.target-audience": "đối tượng độc giả",
  "optional.project.plan.arc.tolerance": "sai lệch số arc",
  "optional.project.plan.chapter.tolerance": "sai lệch số chương",
  "optional.project.plan.scene.tolerance": "sai lệch số cảnh",
  "optional.project.plan.word.tolerance": "sai lệch số từ",
};

const MOCK_REQUIRED_TABLES = [
  "project_genres", "arcs", "characters", "project_motifs", "themes", "theme_arguments",
] as const;

const MOCK_OPTIONAL_TABLES = [
  "relationship_motifs", "chapters", "scenes", "scene_characters", "scene_versions",
  "appearance_option_groups", "appearance_options", "character_appearances",
  "character_distinguishing_marks", "relationships", "relationship_categories",
  "relationship_roles", "relationship_states", "world_settings", "lore_entries",
  "power_systems", "power_tiers", "power_abilities", "memories", "memory_references",
  "memory_emotions", "memory_connections", "memory_revisions", "character_knowledge_sources",
  "competence_domains", "character_competences", "competence_capabilities",
  "competence_limitations",
] as const;

const MOCK_TABLE_LABELS: Record<string, string> = {
  relationship_motifs: "motif quan hệ",
  project_motifs: "motif dự án",
  project_genres: "thể loại dự án",
  arcs: "arc",
  chapters: "chương",
  scenes: "cảnh",
  scene_characters: "nhân vật trong cảnh",
  scene_versions: "phiên bản cảnh",
  characters: "nhân vật",
  appearance_option_groups: "nhóm ngoại hình",
  appearance_options: "lựa chọn ngoại hình",
  character_appearances: "ngoại hình nhân vật",
  character_distinguishing_marks: "dấu đặc biệt",
  relationships: "quan hệ",
  relationship_categories: "loại quan hệ",
  relationship_roles: "vai trò quan hệ",
  relationship_states: "trạng thái quan hệ",
  world_settings: "bối cảnh",
  lore_entries: "lore",
  power_systems: "hệ thống sức mạnh",
  power_tiers: "bậc sức mạnh",
  power_abilities: "năng lực",
  themes: "chủ đề",
  theme_arguments: "luận điểm chủ đề",
  memories: "ký ức",
  memory_references: "tham chiếu ký ức",
  memory_emotions: "cảm xúc ký ức",
  memory_connections: "liên kết ký ức",
  memory_revisions: "bản sửa ký ức",
  character_knowledge_sources: "nguồn tri thức",
  competence_domains: "lĩnh vực năng lực",
  character_competences: "năng lực nhân vật",
  competence_capabilities: "khả năng năng lực",
  competence_limitations: "giới hạn năng lực",
};

const MOCK_TABLE_FIELDS: Record<string, readonly string[]> = {
  project_genres: ["genre_id", "is_primary"],
  arcs: ["name", "position", "code", "summarize"],
  characters: ["name", "is_primary", "birth", "species", "biological_sex", "gender_identity", "gender_expression"],
  project_motifs: ["motif_id", "scope", "arc_id", "chapter_id"],
  themes: ["scope", "question", "code", "arc_id", "parent_theme_id", "resolution_type", "resolution", "message"],
  theme_arguments: ["theme_id", "character_id", "role", "answer_mode", "note", "required_knowledge", "required_competence"],
  relationship_motifs: ["motif_id", "relationship_id"],
  chapters: ["name", "position", "code", "arc_id", "summarize"],
  scenes: ["name", "chapter_id", "position", "code", "world_setting_id", "style_preset_id", "narrator_config_id", "summarize", "content"],
  scene_characters: ["scene_id", "character_id", "is_pov"],
  scene_versions: ["parent_id", "scene_source", "summarize", "content"],
  appearance_option_groups: ["code", "name"],
  appearance_options: ["group_id", "code", "label", "hex", "is_system"],
  character_appearances: ["character_id", "height", "height_unit", "weight", "weight_unit", "hair_color_id", "hair_length_id", "hair_style_id", "hair_texture_id", "pupil_shape_id", "iris_color_id", "sclera_color_id", "skin_tone_id", "skin_texture_id", "hair_note", "eyes_note", "skin_note", "outfit_style", "default_summary"],
  character_distinguishing_marks: ["character_id", "content"],
  relationships: ["from_character_id", "to_character_id"],
  relationship_categories: ["category", "sub_type"],
  relationship_roles: ["relationship_id", "category_id", "is_primary", "hierarchy"],
  relationship_states: ["parent_id", "trust", "affection", "hostility", "fear", "respect", "familiarity", "dependency"],
  world_settings: ["name", "type", "code", "parent_id", "description", "access", "spatial_features", "sensory_visual", "sensory_auditory", "sensory_tactile", "sensory_olfactory", "sensory_gustatory"],
  lore_entries: ["code", "type", "scope", "name", "description", "world_setting_id", "data"],
  power_systems: ["name", "source", "code", "scope", "description", "rules", "cost", "limitations"],
  power_tiers: ["power_system_id", "code", "name", "rank", "description"],
  power_abilities: ["power_system_id", "tier_id", "name", "description"],
  memories: ["owner_id", "type", "summary", "recollection", "interpretation", "certainty", "knowledge_source", "significance", "status", "occurred_at", "perceived_at", "source_scene_id"],
  memory_references: ["memory_id", "entity_type", "entity_id", "role"],
  memory_emotions: ["memory_id", "emotion", "intensity"],
  memory_connections: ["source_memory_id", "target_memory_id", "relation"],
  memory_revisions: ["memory_id", "recollection", "interpretation", "certainty", "source_scene_id", "reason"],
  character_knowledge_sources: ["character_id", "type", "path", "certainty"],
  competence_domains: ["code", "name", "description", "is_system"],
  character_competences: ["character_id", "domain_id", "name", "level", "description", "experience", "reliability", "development_potential", "development_current_focus", "development_growth_direction"],
  competence_capabilities: ["competence_id", "content"],
  competence_limitations: ["competence_id", "content"],
};

const MOCK_REQUIRED_UNITS = [
  "require.project.name",
  "require.project.language",
  "require.project.plan.arc.count",
  "require.project.plan.chapter.count",
  "require.project.plan.scene.count",
  "require.project.plan.word.count",
  ...MOCK_REQUIRED_TABLES.map((table) => `require.schema.${table}`),
];

const MOCK_OPTIONAL_UNITS = [
  "optional.project.author",
  "optional.project.summarize",
  "optional.project.target-audience",
  "optional.project.plan.arc.tolerance",
  "optional.project.plan.chapter.tolerance",
  "optional.project.plan.scene.tolerance",
  "optional.project.plan.word.tolerance",
  ...MOCK_OPTIONAL_TABLES.map((table) => `optional.schema.${table}`),
];

const MOCK_PATH_LABELS: Record<string, string> = (() => {
  const labels: Record<string, string> = { ...MOCK_PROJECT_LABELS };
  for (const table of MOCK_REQUIRED_TABLES) {
    labels[`require.schema.${table}.count`] = `số lượng ${MOCK_TABLE_LABELS[table]}`;
    labels[`require.schema.${table}.rows`] = `danh sách ${MOCK_TABLE_LABELS[table]}`;
  }
  for (const table of MOCK_OPTIONAL_TABLES) {
    labels[`optional.schema.${table}.count`] = `số lượng ${MOCK_TABLE_LABELS[table]}`;
    labels[`optional.schema.${table}.rows`] = `danh sách ${MOCK_TABLE_LABELS[table]}`;
  }
  return labels;
})();

const MOCK_PATHS = Object.keys(MOCK_PATH_LABELS);

function mockUnitFilled(values: Map<string, unknown>, unit: string): boolean {
  if (unit.includes(".project.")) return mockIsFilled(values.get(unit));
  const [, group, tail] = unit.split(".");
  const table = tail;
  const count = values.get(`${group}.schema.${table}.count`);
  const rows = values.get(`${group}.schema.${table}.rows`);
  return typeof count === "number" && Array.isArray(rows) && rows.length === count;
}

function mockMissingRequired(values: Map<string, unknown>): string[] {
  return MOCK_REQUIRED_UNITS.filter((unit) => !mockUnitFilled(values, unit));
}

function mockTableGroup(table: string): "require" | "optional" | null {
  if ((MOCK_REQUIRED_TABLES as readonly string[]).includes(table)) return "require";
  if ((MOCK_OPTIONAL_TABLES as readonly string[]).includes(table)) return "optional";
  return null;
}

/** Mirrors `tables::warnings` closely enough to exercise the result shape. */
function mockWarnings(values: Map<string, unknown>): string[] {
  const warnings: string[] = [];
  const scenes = mockRows(values, "scenes");
  if (scenes.length > 0 && !mockRows(values, "scene_characters").some((row) => row.is_pov === true)) {
    warnings.push(
      `scene_characters needs at least one is_pov row while scenes has ${scenes.length} row(s)`,
    );
  }
  if (!mockRows(values, "project_motifs").some((row) => row.scope === "project")) {
    warnings.push('project_motifs needs at least one row with scope = "project"');
  }
  return warnings;
}

/** Mirrors `io::add_row`: append, assign `_json_index`, follow count (D2). */
function mockAddRow(args: Record<string, unknown>, values: Map<string, unknown>): unknown {
  const table = typeof args.table === "string" ? args.table : "";
  const group = mockTableGroup(table);
  if (!group) return { error: `Unknown super json table: ${table}` };
  if (typeof args.row !== "object" || args.row === null || Array.isArray(args.row)) {
    return { error: `Invalid value for ${table}.rows: row must be an object` };
  }
  const rowsPath = `${group}.schema.${table}.rows`;
  const countPath = `${group}.schema.${table}.count`;
  const stored = values.get(rowsPath);
  const rows = Array.isArray(stored) ? [...(stored as Array<Record<string, unknown>>)] : [];
  const oldLength = rows.length;
  const current = values.get(countPath);
  const count =
    typeof current === "number" && current !== oldLength ? current : oldLength + 1;
  const jsonIndex = oldLength;
  const row = { ...(args.row as Record<string, unknown>), _json_index: jsonIndex };
  rows.push(row);
  values.set(rowsPath, rows);
  values.set(countPath, count);
  return {
    ok: true,
    table,
    json_index: jsonIndex,
    count,
    row,
    missing_required: mockMissingRequired(values),
    warnings: mockWarnings(values),
  };
}

/** Mirrors `io::add_field`: one field of one existing row. */
function mockAddField(args: Record<string, unknown>, values: Map<string, unknown>): unknown {
  const table = typeof args.table === "string" ? args.table : "";
  const group = mockTableGroup(table);
  if (!group) return { error: `Unknown super json table: ${table}` };
  const field = typeof args.field === "string" ? args.field : "";
  if (!field) return { error: "super_json_add_field requires a string 'field'" };
  if (field === "_json_index") {
    return { error: "_json_index is assigned by the tool and cannot be written" };
  }
  const fields = MOCK_TABLE_FIELDS[table] ?? [];
  const base = field.endsWith("_json_index")
    ? field.slice(0, -"_json_index".length)
    : "";
  if (!fields.includes(field) && !(base !== "" && fields.includes(base))) {
    return {
      error: `Unknown field "${field}" for ${table}; allowed fields are: ${fields.join(", ")}`,
    };
  }
  if (typeof args.json_index !== "number" || !Number.isInteger(args.json_index)) {
    return { error: "super_json_add_field requires an integer 'json_index'" };
  }
  const jsonIndex = args.json_index;
  const rowsPath = `${group}.schema.${table}.rows`;
  const stored = values.get(rowsPath);
  const rows = Array.isArray(stored) ? [...(stored as Array<Record<string, unknown>>)] : [];
  if (jsonIndex < 0 || jsonIndex >= rows.length) {
    return { error: `Row ${jsonIndex} not found in ${table}.rows` };
  }
  rows[jsonIndex] = { ...rows[jsonIndex], [field]: args.value };
  values.set(rowsPath, rows);
  return {
    ok: true,
    table,
    json_index: jsonIndex,
    field,
    value: args.value,
    missing_required: mockMissingRequired(values),
    warnings: mockWarnings(values),
  };
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

/** Inserts `value` at a dotted project path, creating intermediate objects. */
function assignPath(document: Record<string, unknown>, path: string, value: unknown): void {
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
  const document: Record<string, unknown> = {
    require: { project: {}, schema: [] },
    optional: { project: {}, schema: [] },
  };
  for (const group of ["require", "optional"] as const) {
    const tables = group === "require" ? MOCK_REQUIRED_TABLES : MOCK_OPTIONAL_TABLES;
    const schema = (document[group] as { schema: Array<Record<string, unknown>> }).schema;
    for (const table of tables) {
      schema.push({
        table,
        count: values.get(`${group}.schema.${table}.count`) ?? null,
        fields: {},
        rows: values.get(`${group}.schema.${table}.rows`) ?? [],
      });
    }
  }
  for (const [path, value] of values) {
    if (path.includes(".schema.")) continue;
    const [group, rest] = path.split(/\.(.*)/s);
    if (rest?.startsWith("project.")) assignPath(document, `${group}.${rest}`, value);
  }
  return document;
}

function mockRows(values: Map<string, unknown>, table: string): Array<Record<string, unknown>> {
  for (const group of ["require", "optional"]) {
    const rows = values.get(`${group}.schema.${table}.rows`);
    if (Array.isArray(rows)) return rows as Array<Record<string, unknown>>;
  }
  return [];
}

function mockText(row: Record<string, unknown> | undefined, field: string): string | undefined {
  const value = row?.[field];
  return typeof value === "string" && value.trim() ? value : undefined;
}

function mockOr(value: string | undefined): string {
  return value?.trim() ? value : "[chưa có thông tin]";
}

function mockSummary(values: Map<string, unknown> = new Map()): string {
  const primary = mockRows(values, "project_genres").find((row) => row.is_primary === true);
  const cast = mockRows(values, "characters");
  const theme = mockRows(values, "themes")[0];
  const world = mockRows(values, "world_settings")[0];
  const power = mockRows(values, "power_systems")[0];
  const castText = cast.length
    ? cast.map((row) => {
        const name = mockOr(mockText(row, "name"));
        return row.is_primary === true ? `${name} (chính)` : name;
      }).join(", ")
    : "[chưa có thông tin]";
  const filledRequired = MOCK_REQUIRED_UNITS.filter((unit) => mockUnitFilled(values, unit)).length;
  const filledOptional = MOCK_OPTIONAL_UNITS.filter((unit) => mockUnitFilled(values, unit)).length;
  return [
    `Tên dự án: ${mockOr(mockText({ name: values.get("require.project.name") }, "name"))}. ` +
      `Ngôn ngữ: ${mockOr(values.get("require.project.language") as string | undefined)}. ` +
      `Thể loại chính: ${mockOr(mockText(primary, "genre_id"))}. ` +
      `Tác giả: ${mockOr(values.get("optional.project.author") as string | undefined)}. ` +
      `Đối tượng độc giả: ${mockOr(values.get("optional.project.target-audience") as string | undefined)}.`,
    `Dàn nhân vật: ${castText}. Số quan hệ đã khai báo: ${mockRows(values, "relationships").length}.`,
    `Chủ đề: ${mockOr(mockText(theme, "question"))}. Thông điệp: ${mockOr(mockText(theme, "message"))}. ` +
      `Cách giải quyết: ${mockOr(mockText(theme, "resolution_type"))}, với nội dung: ${mockOr(mockText(theme, "resolution"))}.`,
    `Kế hoạch: ${mockOr(String(values.get("require.project.plan.arc.count") ?? ""))} arc, ` +
      `${mockOr(String(values.get("require.project.plan.chapter.count") ?? ""))} chương, ` +
      `${mockOr(String(values.get("require.project.plan.scene.count") ?? ""))} cảnh, ` +
      `${mockOr(String(values.get("require.project.plan.word.count") ?? ""))} từ.`,
    `Bối cảnh: ${mockOr(mockText(world, "name"))}, quy mô ${mockOr(mockText(world, "type"))}. ` +
      `Hệ thống sức mạnh: ${mockOr(mockText(power, "name"))}. Số lore: ${mockRows(values, "lore_entries").length}.`,
    `Tóm tắt: ${mockOr(values.get("optional.project.summarize") as string | undefined)}. ` +
      `Đã hoàn thành ${filledRequired}/${MOCK_REQUIRED_UNITS.length} mục bắt buộc và ` +
      `${filledOptional}/${MOCK_OPTIONAL_UNITS.length} mục tùy chọn.`,
  ].join("\n\n");
}

function mockProgress(session: MockSession | undefined): SuperJsonProgress {
  const projectId = session?.projectId ?? "";
  const values = session?.values ?? new Map<string, unknown>();
  const filledRequired = projectId
    ? MOCK_REQUIRED_UNITS.filter((unit) => mockUnitFilled(values, unit)).length
    : 0;
  const filledOptional = projectId
    ? MOCK_OPTIONAL_UNITS.filter((unit) => mockUnitFilled(values, unit)).length
    : 0;
  const percent = projectId
    ? Math.round((70 * filledRequired) / MOCK_REQUIRED_UNITS.length + (30 * filledOptional) / MOCK_OPTIONAL_UNITS.length)
    : 0;
  const label = !projectId
    ? "Chưa có project"
    : filledRequired === MOCK_REQUIRED_UNITS.length && filledOptional === MOCK_OPTIONAL_UNITS.length
      ? "Độ mạch lạc: tốt"
      : filledRequired === MOCK_REQUIRED_UNITS.length
        ? "Độ mạch lạc tạm chấp nhận"
        : "Đang thu thập thông tin";
  return {
    has_project: Boolean(projectId),
    project_id: projectId,
    percent,
    label,
    filled_required: filledRequired,
    required_total: MOCK_REQUIRED_UNITS.length,
    filled_optional: filledOptional,
    optional_total: MOCK_OPTIONAL_UNITS.length,
  };
}

/**
 * The values the preview agent proposes when the author says they have no idea.
 */
const MOCK_SUGGESTIONS: Array<[string, unknown]> = [
  ["require.project.plan.arc.count", 5],
  ["require.project.plan.chapter.count", 120],
  ["require.project.plan.scene.count", 360],
  ["require.project.plan.word.count", 90000],
  ["optional.project.plan.arc.tolerance", 1],
  ["optional.project.plan.chapter.tolerance", 10],
  ["optional.project.plan.scene.tolerance", 10],
  ["optional.project.plan.word.tolerance", 5000],
  ["require.schema.project_genres.count", 1],
  ["require.schema.project_genres.rows", [{ genre_id: "trinh thám", is_primary: true }]],
  ["require.schema.themes.count", 1],
  ["require.schema.themes.rows", [{ scope: "PROJECT", question: "ký ức có làm nên con người hay không" }]],
  ["require.project.language", "VI"],
];

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
    case "super_json_add_row": {
      const table = typeof args.table === "string" ? args.table : "";
      return `Đang thêm 1 ${MOCK_TABLE_LABELS[table] ?? table}`;
    }
    case "super_json_add_field": {
      const table = typeof args.table === "string" ? args.table : "";
      return `Đang cập nhật ${MOCK_TABLE_LABELS[table] ?? table}`;
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
  if (
    [
      "super_json_get",
      "super_json_set",
      "super_json_add_row",
      "super_json_add_field",
      "super_json_missing",
    ].includes(name)
  ) {
    if (!session.projectId) return { error: "This chat is not bound to a project. Call project_create first." };
    if (projectId && projectId !== session.projectId) return { error: "Project does not belong to this session" };
  }
  switch (name) {
    case "project_create": {
      // D7: `name` is optional. Without one the project is created unnamed and
      // `require.project.name` is left untouched — never invented.
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
        session.values.set("require.project.name", title);
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
      if (!MOCK_PATHS.includes(path)) return { error: `Unknown path: ${path}` };
      session.values.set(path, args.value);
      return {
        ok: true,
        path,
        value: args.value,
        missing_required: mockMissingRequired(session.values),
        warnings: mockWarnings(session.values),
      };
    }
    case "super_json_add_row":
      return mockAddRow(args, session.values);
    case "super_json_add_field":
      return mockAddField(args, session.values);
    case "super_json_get":
      return mockSuperJsonDocument(session.values);
    case "super_json_missing":
      return { missing: mockMissingRequired(session.values), required_total: MOCK_REQUIRED_UNITS.length };
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
  if (session.messages.length === 0 && !session.createdAt) session.createdAt = new Date().toISOString();
  const firstUser = args.messages.find((message) => message.role === "user");
  if (firstUser && !session.messages.some((message) => message.id === firstUser.id)) session.messages.push(firstUser);
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
    // still created, and require.project.name stays empty.
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
        args: { path: "require.project.name", value: statedTitle },
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

/** Mirrors `models::OutlineStatus`, including the per-stage rail state. */
function outlineMockStatus(sessionId: string) {
  const active = outlineMockActive.has(sessionId);
  const arcsCommitted = outlineMockCommitted.has(`${sessionId}:arcs`);
  const chaptersCommitted = outlineMockCommitted.has(`${sessionId}:chapters`);
  const chaptersStarted = outlineMockChapterRuns.has(sessionId);
  const stage = chaptersStarted ? "chapters" : active ? "arcs" : "seed";
  const stages = [
    { stage: "arcs", status: active ? (arcsCommitted ? "committed" : "review") : "empty", total: active ? 3 : 0 },
    { stage: "chapters", status: chaptersStarted ? (chaptersCommitted ? "committed" : "review") : "empty", total: chaptersStarted ? 12 : 0 },
    { stage: "cast", status: "empty", total: 0 },
    { stage: "world", status: "empty", total: 0 },
    { stage: "scenes", status: "empty", total: 0 },
  ];
  const committedTotal = (arcsCommitted ? 3 : 0) + (chaptersCommitted ? 12 : 0);
  const total = (active ? 3 : 0) + (chaptersStarted ? 12 : 0);
  return {
    ok: true,
    stage,
    active,
    run_status: active ? (arcsCommitted && chaptersCommitted ? "committed" : "review") : "idle",
    stage_status: active ? (arcsCommitted ? "committed" : "review") : "empty",
    counts: { total, pending: total - committedTotal, accepted: 0, committed: committedTotal, failed: 0 },
    next_stage: active ? "chapters" : "arcs",
    blockers: [],
    stages,
    usage: { calls: active ? 1 : 0, tokens_in: active ? 820 : 0, tokens_out: active ? 1450 : 0 },
  };
}

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
    case "list_projects": {
      const ids = new Set([...mockProjects.map((project) => project.id), ...[...sessions.values()].map((session) => session.projectId).filter(Boolean)]);
      return [...ids].map(mockProjectSummary).filter((project): project is ProjectSummary => project !== null)
        .sort((a, b) => b.created_at.localeCompare(a.created_at));
    }
    case "get_project": {
      const { projectId } = args as { projectId: string };
      return mockProjectSummary(projectId);
    }
    case "list_messages": {
      const { sessionId } = args as { sessionId: string };
      return sessions.get(sessionId)?.messages ?? [];
    }
    case "list_sessions":
      return [...sessions.entries()].map(([id, session]) => mockSessionSummary(id, session))
        .sort((a, b) => b.created_at.localeCompare(a.created_at));
    case "get_session": {
      const { sessionId } = args as { sessionId: string };
      const session = sessions.get(sessionId);
      return session ? mockSessionSummary(sessionId, session) : null;
    }
    case "rename_session": {
      const { sessionId, title } = args as { sessionId: string; title: string };
      const session = sessions.get(sessionId);
      if (!session) throw new Error(`Session not found: ${sessionId}`);
      const trimmed = title.trim();
      if (!trimmed) throw new Error("Session title cannot be empty");
      if (Array.from(trimmed).length > 120) throw new Error("Session title is too long (max 120 characters)");
      session.title = trimmed;
      return mockSessionSummary(sessionId, session);
    }
    case "delete_session": {
      const { sessionId } = args as { sessionId: string };
      return sessions.delete(sessionId);
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
    case "outline_anchor": {
      const { projectId } = args as { projectId: string };
      return {
        project_id: projectId,
        text: "Dự án: Mẫu [vi] | Thể loại: Tiên hiệp (xianxia) | Kế hoạch: 100k từ, 3 arc, 30 chương, 90 cảnh.\n\nArc 1 (seq 1): Khởi đầu -> [chưa có: require.schema.arcs.rows[0].outcome]\n\nNhân vật chính: Tiêu Viêm | Vai trò: protagonist | Archetype: chosen-one | Tóm tắt: Thiếu niên kiên định | Mô tả: Tóc đen áo vải. Tuyến đối trọng: [chưa có: require.schema.characters.rows].\n\nMotif chủ đạo: Nhẫn cổ | Phạm vi: global | Mô tả: Chiếc nhẫn chứa linh hồn dược lão | Ý nghĩa: Bàn đạp sức mạnh.\n\nTheme: Ý chí sinh tồn | Tuyên ngôn: Nghịch cảnh tôi luyện bản lĩnh | Trọng số: 0.9.\n\nXung đột tư tưởng: Tiêu Viêm | Quan điểm: Dựa vào chính mình [SUPPORT] phản biện: Định mệnh an bài.",
        json: {
          project: {
            name: "Mẫu",
            language: "vi",
            genre: "Tiên hiệp",
            genre_code: "xianxia",
            plan: { word: 100000, arc: 3, chapter: 30, scene: 90 },
          },
          arcs: [{ id: "arc-1", name: "Khởi đầu", sequence: 1, goal: "Tu luyện", outcome: null }],
          protagonist: { name: "Tiêu Viêm", role: "protagonist", archetype: "chosen-one", summary: "Thiếu niên kiên định", description: "Tóc đen áo vải" },
          cast: [],
          motif: { name: "Nhẫn cổ", scope: "global", description: "Chiếc nhẫn chứa linh hồn dược lão", significance: "Bàn đạp sức mạnh" },
          theme: { name: "Ý chí sinh tồn", statement: "Nghịch cảnh tôi luyện bản lĩnh", weight: 0.9 },
          argument: { character: "Tiêu Viêm", claim: "Dựa vào chính mình", polarity: "SUPPORT", counter_claim: "Định mệnh an bài" },
        },
      };
    }
    case "outline_status": {
      const request = args as { sessionId: string };
      return outlineMockStatus(request.sessionId);
    }
    case "outline_start": {
      const request = args as { sessionId: string };
      outlineMockActive.add(request.sessionId);
      return outlineMockStatus(request.sessionId);
    }
    case "outline_run": {
      const request = args as { sessionId: string; stage: string; channel: { onmessage?: (event: { type: string; stage?: string; key?: string; lane?: string; ok?: boolean; error?: string | null; count?: number; failures?: number }) => void } };
      const event = request.channel?.onmessage;
      const runArgs = args as { sessionId: string; stage: string; channel: { onmessage?: (event: { type: string; stage?: string; key?: string; lane?: string; ok?: boolean; error?: string | null; count?: number; failures?: number }) => void } };
      if (request.stage === "chapters") outlineMockChapterRuns.add(runArgs.sessionId);
      if (event) {
        event({ type: "run_started", stage: request.stage });
        event({ type: "shard_started", key: request.stage === "arcs" ? "arcs" : "chapters-1-10", lane: request.stage === "arcs" ? "arc_smith" : "chapter_smith" });
        event({ type: "shard_finished", key: request.stage === "arcs" ? "arcs" : "chapters-1-10", lane: request.stage === "arcs" ? "arc_smith" : "chapter_smith", ok: true, error: null });
        event({ type: "items_ready", stage: request.stage, count: request.stage === "arcs" ? 3 : 10 });
        event({ type: "run_finished", stage: request.stage, failures: 0 });
      }
      return { stage: request.stage, shards: 1, calls: 1, items: request.stage === "arcs" ? 3 : 10, failures: 0, tokens_in: 820, tokens_out: 1450 };
    }
    case "outline_regenerate":
      return { stage: "arcs", shards: 1, calls: 1, items: 1, failures: 0, tokens_in: 120, tokens_out: 240 };
    case "outline_items": {
      const request = args as { stage: string };
      if (request.stage === "arcs") return [
        { key: "arc-01", lane: "arc_smith", position: 1, status: "pending", title: "Lời thề dưới chân núi", name: "Lời thề dưới chân núi", goal: "Đặt nền móng cho hành trình và lời thề của nhân vật chính.", outcome: "Nhân vật rời quê nhà, mang theo bí mật của chiếc nhẫn.", summarize: "Tiêu Viêm đối mặt với biến cố làm đảo lộn cuộc sống. Một lời thề mới mở ra hành trình trưởng thành.", chapters: 4, error: null },
        { key: "arc-02", lane: "arc_smith", position: 2, status: "pending", title: "Thử lửa nơi viễn địa", name: "Thử lửa nơi viễn địa", goal: "Thử thách năng lực và lòng tin.", outcome: "Đồng minh được tìm thấy, mối đe dọa lộ diện.", summarize: "Hành trình đưa nhóm đến vùng đất xa lạ, nơi mỗi chiến thắng đều có cái giá riêng.", chapters: 4, error: null },
        { key: "arc-03", lane: "arc_smith", position: 3, status: "pending", title: "Ngọn lửa tái sinh", name: "Ngọn lửa tái sinh", goal: "Đưa xung đột đến lựa chọn cuối cùng.", outcome: "Nhân vật làm chủ sức mạnh và xác lập tương lai.", summarize: "Những bí mật được giải đáp trong cuộc đối đầu cuối. Nhân vật chọn con đường của riêng mình.", chapters: 4, error: null },
      ];
      const chapterRequest = args as { sessionId: string };
      if (!outlineMockChapterRuns.has(chapterRequest.sessionId)) return [];
      return Array.from({ length: 12 }, (_, index) => ({ key: `ch-${String(index + 1).padStart(3, "0")}`, lane: "chapter_smith", position: index + 1, status: "pending", title: `Chương ${index + 1}: ${["Tàn tro đầu mùa", "Người khách lạ", "Dấu ấn cổ", "Lời hẹn", "Cánh cửa đá", "Đêm truy đuổi", "Một món nợ", "Tin dữ", "Bước ngoặt", "Bóng dưới vực", "Trở về", "Khởi hành"][index]}`, summarize: "Nhân vật tiến gần hơn tới mục tiêu, đồng thời phát hiện một manh mối mới.", beats: ["Gặp gỡ", "Xung đột", "Lựa chọn"], scene_count: 3, arc: Math.floor(index / 4) + 1, error: null }));
    }
    case "outline_accept":
    case "outline_reject":
      return Array.isArray((args as { keys?: string[] }).keys) ? (args as { keys: string[] }).keys.length : 3;
    case "outline_commit": {
      const request = args as { sessionId: string; stage: string };
      outlineMockCommitted.add(`${request.sessionId}:${request.stage}`);
      return { committed: 3, skipped: 0, missing: [], warnings: [] };
    }
    case "outline_cancel":
      return undefined;
    case "writing_status":
      return { ok: true, scenes: 12, written: 3, words: 2700, locked: 0 };
    case "write_scene": {
      const request = args as { chapter: number; position: number };
      return {
        scene: `Cảnh ${request.position}`,
        path: `content/chapter-${String(request.chapter).padStart(3, "0")}/scene-${String(request.position).padStart(3, "0")}.md`,
        word_count: 912,
        within_budget: true,
        clean: true,
        violations: [],
      };
    }
    case "rewrite_scenes": {
      const request = args as { scenes: Array<{ chapter: number; position: number }> };
      return (request.scenes ?? []).map((scene) => ({
        scene: `Cảnh ${scene.position}`,
        path: `content/chapter-${String(scene.chapter).padStart(3, "0")}/scene-${String(scene.position).padStart(3, "0")}.md`,
        word_count: 880,
        within_budget: true,
        clean: true,
        violations: [],
      }));
    }
    case "lock_scene":
      return undefined;
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
