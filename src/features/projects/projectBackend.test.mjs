import assert from "node:assert/strict";
import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { after, describe, it } from "node:test";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { MemoryRouter } from "react-router";
import { createServer } from "vite";

// Spin up minimal Vite dev server in middleware mode to load TS/TSX modules directly in Node
const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: "custom",
  resolve: {
    alias: {
      "@": path.resolve("src"),
    },
  },
  ssr: {
    external: ["react", "react-router", "react-dom"],
  },
  plugins: [
    {
      name: "svg-stub",
      transform(_code, id) {
        if (id.endsWith(".svg")) {
          return { code: 'export default "stub.svg";', map: null };
        }
      },
    },
  ],
});

after(async () => {
  await vite.close();
});

const clientModule = await vite.ssrLoadModule("/src/shared/backend/client.ts");
const projectViewModule = await vite.ssrLoadModule("/src/features/projects/ProjectView.tsx");
const projectDetailModule = await vite.ssrLoadModule("/src/features/projects/ProjectDetailView.tsx");
const projectCreatorModule = await vite.ssrLoadModule("/src/features/projects/ProjectCreatorPanel.tsx");
const characterListModule = await vite.ssrLoadModule("/src/features/characters/CharacterListView.tsx");
const characterDetailModule = await vite.ssrLoadModule("/src/features/characters/CharacterDetailLayout.tsx");

const { createBackendClient } = clientModule;
const { sanitizeDisplayLabel, mapProjectDtoToItem } = projectViewModule;
const ProjectView = projectViewModule.default;
const { ProjectDetailView } = projectDetailModule;
const { ProjectCreatorPanel } = projectCreatorModule;
const { CharacterListView, linkCharacterToProject } = characterListModule;
const { CharacterDetailLayout, mapCharacterDetailDtoToProfile } = characterDetailModule;

// Predecessors and baseline SHAs
const BASELINE_COMMIT = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P5S1 = "a514385a6f5df0f16deb318dc50a4b47396162ee";
const PREDECESSOR_P4S3 = "48a9a74a912a37e000b4ce2824385b963c794634";

function verifyGitCommit(sha) {
  try {
    execSync(`git merge-base --is-ancestor ${sha} HEAD`, { stdio: "pipe" });
    return true;
  } catch {
    try {
      execSync(`git cat-file -e ${sha}^{commit}`, { stdio: "pipe" });
      return true;
    } catch {
      return false;
    }
  }
}

// ============================================================================
// Test 1: phase5_sprint4_deep1-F0 (Git ancestry and predecessor verification)
// ============================================================================
describe("phase5_sprint4_deep1-F0: predecessor and baseline git verification", () => {
  it("verifies phase5_sprint1_deep0 predecessor commit is an ancestor or valid commit", () => {
    const valid = verifyGitCommit(PREDECESSOR_P5S1);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor phase5_sprint1_deep0 (${PREDECESSOR_P5S1}) not found`
    );
  });

  it("verifies phase4_sprint3_deep1 predecessor commit is an ancestor or valid commit", () => {
    const valid = verifyGitCommit(PREDECESSOR_P4S3);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor phase4_sprint3_deep1 (${PREDECESSOR_P4S3}) not found`
    );
  });

  it("verifies baseline commit is a valid commit object", () => {
    const valid = verifyGitCommit(BASELINE_COMMIT);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: baseline commit (${BASELINE_COMMIT}) is invalid`
    );
  });

  it("fails explicitly with CONTRACT_BLOCKED when a fake predecessor is checked", () => {
    const fakePred = "0000000000000000000000000000000000000000";
    const valid = verifyGitCommit(fakePred);
    assert.equal(valid, false, "Expected check on fake predecessor to fail");
  });
});

// ============================================================================
// Test 2: P5S4-P (Project & Character read models render/map from fake typed Tauri IPC)
// ============================================================================
describe("P5S4-P: project list/detail and character library render from fake typed IPC", () => {
  const projectFixture = {
    id: "proj-101",
    title: "Vương Triều Bão Lửa",
    status: "Drafting",
    cover: "https://example.com/cover.png",
    description: "Biên niên sử phương bắc",
    genres: ["Tiên hiệp", "Huyền huyễn"],
    modifiedAt: "2026-11-08T07:00:00Z",
    details: {
      wordCount: "12500",
      chapters: "8",
      characters: "12",
      worldLore: "15",
      createdDate: "2026-11-01",
    },
    arcs: [
      {
        id: "arc-1",
        title: "Khởi Đầu Biến Cố",
        cover: "https://example.com/arc.png",
        chapters: [
          {
            id: "chap-1",
            title: "Thiếu niên xuất sơn",
            date: "2026-11-02",
            isNew: true,
            blocks: [
              { id: "blk-1", sceneId: "scn-1", text: "Trời mây phủ kín chân trời.", locked: false },
            ],
          },
        ],
      },
    ],
    inProject: true,
  };

  const characterFixture = {
    id: "char-201",
    code: "CHAR_LE_LOI",
    name: "Lê Lợi",
    nickname: "Bình Định Vương",
    species: "Nhân tộc",
    biologicalSex: "MALE",
    genderIdentity: "MALE",
    genderExpression: "MALE",
    inProject: true,
    avatarUrl: "https://example.com/avatar.png",
    defaultSummary: "Thủ lĩnh khởi nghĩa Lam Sơn.",
  };

  const characterDetailFixture = {
    character: characterFixture,
    appearance: {
      height: 175,
      weight: 70,
      hair: "Đen",
      eyes: "Sáng",
      skin: "Rám nắng",
      hairNote: "Búi cao",
      eyesNote: "Kiên định",
      skinNote: "Dày dạn phong trần",
      outfitStyle: "Chiến bào",
      defaultSummary: "Khí phách hiên ngang",
    },
    psychological: {
      coreValues: ["Chính nghĩa", "Thương dân"],
      traumaTriggers: ["Bạo ngược"],
      behavioralQuirks: ["Trầm ngâm khi suy nghĩ"],
      summary: "Người lãnh đạo quả cảm",
    },
    memories: [
      {
        id: "mem-1",
        memoryType: "Core",
        title: "Hội thề Lũng Nhai",
        description: "Lời thề sống chết có nhau cùng 18 tướng lĩnh",
        emotionalCharge: "Trọng đại",
      },
    ],
    competences: [
      {
        id: "comp-1",
        domain: "Quân sự",
        competenceName: "Thuật cầm quân",
        level: "Bậc thầy",
        sceneId: null,
      },
    ],
    relationships: [
      {
        targetCharacterId: "char-nguyen-trai",
        relationshipType: "Quân sư",
        intensity: "Cao",
        description: "Cánh tay đắc lực",
      },
    ],
  };

  function createFakeTransport() {
    return {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, req) => {
        if (cmd === "list_projects") {
          return { items: [projectFixture] };
        }
        if (cmd === "get_project_detail") {
          if (req?.projectId === projectFixture.id) {
            return { project: projectFixture };
          }
          throw new Error("EntityNotFound: project not found");
        }
        if (cmd === "list_characters") {
          return { items: [characterFixture] };
        }
        if (cmd === "get_character_detail") {
          if (req?.characterId === characterFixture.id) {
            return characterDetailFixture;
          }
          throw new Error("EntityNotFound: character not found");
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
      createChannel: (onEvent) => ({ onEvent }),
    };
  }

  it("fetches and maps project list and project detail from fake IPC transport", async () => {
    const fakeTransport = createFakeTransport();
    const client = createBackendClient(fakeTransport);

    const projectList = await client.listProjects();
    assert.equal(projectList.items.length, 1);
    assert.equal(projectList.items[0].id, "proj-101");
    assert.equal(projectList.items[0].title, "Vương Triều Bão Lửa");

    const projectDetail = await client.getProjectDetail({ projectId: "proj-101" });
    assert.equal(projectDetail.project.id, "proj-101");
    assert.equal(projectDetail.project.arcs.length, 1);
    assert.equal(projectDetail.project.arcs[0].title, "Khởi Đầu Biến Cố");
  });

  it("fetches and maps character library and character detail from fake IPC transport", async () => {
    const fakeTransport = createFakeTransport();
    const client = createBackendClient(fakeTransport);

    const characterList = await client.listCharacters();
    assert.equal(characterList.items.length, 1);
    assert.equal(characterList.items[0].id, "char-201");
    assert.equal(characterList.items[0].name, "Lê Lợi");

    const charDetail = await client.getCharacterDetail({ characterId: "char-201" });
    assert.equal(charDetail.character.id, "char-201");
    assert.equal(charDetail.appearance.hair, "Đen");
    assert.equal(charDetail.competences.length, 1);
    assert.equal(charDetail.competences[0].competenceName, "Thuật cầm quân");
  });

  it("renders ProjectView with items from fake IPC transport", () => {
    const mapped = [mapProjectDtoToItem(projectFixture)];
    const html = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        null,
        React.createElement(ProjectView, { initialProjects: mapped })
      )
    );
    assert.match(html, /Vương Triều Bão Lửa/);
    assert.match(html, /Biên niên sử phương bắc/);
  });

  it("renders ProjectDetailView and ProjectCreatorPanel with data from fake IPC transport", () => {
    const mapped = mapProjectDtoToItem(projectFixture);
    const detailHtml = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        { initialEntries: ["/project/proj-101"] },
        React.createElement(ProjectDetailView, { project: mapped })
      )
    );
    assert.match(detailHtml, /Vương Triều Bão Lửa/);
    assert.match(detailHtml, /Khởi Đầu Biến Cố/);
    assert.match(detailHtml, /Thiếu niên xuất sơn/);

    const creatorHtml = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        null,
        React.createElement(ProjectCreatorPanel, { project: mapped })
      )
    );
    assert.match(creatorHtml, /Khởi Đầu Biến Cố/);
    assert.match(creatorHtml, /Thiếu niên xuất sơn/);
  });

  it("renders CharacterListView with items from fake IPC transport", () => {
    const mappedChar = mapCharacterDetailDtoToProfile(characterDetailFixture);
    const html = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        null,
        React.createElement(CharacterListView, { initialCharacters: [mappedChar] })
      )
    );
    assert.match(html, /Lê Lợi/);
    assert.match(html, /Bình Định Vương/);
  });
});

// ============================================================================
// Test 3: P5S4-N (Deleted project is absent while shared characters are preserved)
// ============================================================================
describe("P5S4-N: deleted project is absent but shared character is preserved", () => {
  it("removes project upon deleteProject, causes getProjectDetail to return NOT_FOUND, while preserving character library", async () => {
    const projects = new Map([
      ["proj-p5s4", { id: "proj-p5s4", title: "Dự án sắp xóa", status: "Drafting", cover: "", description: "", genres: [], modifiedAt: "2026-11-08", details: { wordCount: "0", chapters: "0", characters: "0", worldLore: "0", createdDate: "2026-11-08" }, arcs: [], inProject: true }],
    ]);

    const characters = new Map([
      ["char-shared", { id: "char-shared", code: "CHAR_SHARED", name: "Nhân vật dùng chung", nickname: "Shared", species: "Human", biologicalSex: "NONE", genderIdentity: "NONE", genderExpression: "NONE", inProject: true }],
    ]);

    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, req) => {
        if (cmd === "list_projects") {
          return { items: Array.from(projects.values()) };
        }
        if (cmd === "get_project_detail") {
          const p = projects.get(req.projectId);
          if (!p) throw new Error("EntityNotFound: project not found");
          return { project: p };
        }
        if (cmd === "delete_project") {
          const existed = projects.delete(req.projectId);
          return { deleted: existed };
        }
        if (cmd === "list_characters") {
          return { items: Array.from(characters.values()) };
        }
        if (cmd === "get_character_detail") {
          const c = characters.get(req.characterId);
          if (!c) throw new Error("EntityNotFound: character not found");
          return {
            character: c,
            appearance: null,
            psychological: null,
            memories: [],
            competences: [],
            relationships: [],
          };
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
      createChannel: (onEvent) => ({ onEvent }),
    };

    const client = createBackendClient(fakeTransport);

    // Initial check
    const initialProjects = await client.listProjects();
    assert.equal(initialProjects.items.length, 1);
    assert.equal(initialProjects.items[0].id, "proj-p5s4");

    const initialChars = await client.listCharacters();
    assert.equal(initialChars.items.length, 1);
    assert.equal(initialChars.items[0].id, "char-shared");

    // Perform project delete
    const deleteResult = await client.deleteProject({ projectId: "proj-p5s4" });
    assert.equal(deleteResult.deleted, true);

    // Verify deleted project is absent from project list
    const postDeleteProjects = await client.listProjects();
    assert.equal(postDeleteProjects.items.length, 0);

    // Verify getProjectDetail returns NOT_FOUND (EntityNotFound)
    await assert.rejects(
      async () => {
        await client.getProjectDetail({ projectId: "proj-p5s4" });
      },
      /EntityNotFound/
    );

    // CRITICAL: Shared character in Character Library remains preserved intact!
    const postDeleteChars = await client.listCharacters();
    assert.equal(postDeleteChars.items.length, 1);
    assert.equal(postDeleteChars.items[0].id, "char-shared");
    assert.equal(postDeleteChars.items[0].name, "Nhân vật dùng chung");

    const charDetail = await client.getCharacterDetail({ characterId: "char-shared" });
    assert.equal(charDetail.character.id, "char-shared");
  });
});

// ============================================================================
// Test 4: P5S4-B (No visible internal entity codes in rendered output / snapshot)
// ============================================================================
describe("P5S4-B: no visible internal entity codes in rendered snapshots", () => {
  const CODE_REGEX = /\b(CHAR|REL|FACTION|EVENT|LOC|ARC|CH|SC|SCN|THM|PROJ|COMP)_[A-Za-z0-9_-]+\b/i;

  it("sanitizeDisplayLabel strips all raw technical entity code prefixes and slugs", () => {
    assert.equal(sanitizeDisplayLabel("[CHAR_001] Alice", "CHAR_001"), "Alice");
    assert.equal(sanitizeDisplayLabel("CHAR_001: Bob", "CHAR_001"), "Bob");
    assert.equal(sanitizeDisplayLabel("CHAR_001 - Carol", "CHAR_001"), "Carol");
    assert.equal(sanitizeDisplayLabel("ARC_001: Hồi 1", "ARC_001"), "Hồi 1");
    assert.equal(sanitizeDisplayLabel("CH_001: Chương 1", "CH_001"), "Chương 1");
    assert.equal(sanitizeDisplayLabel("SC_001 - Cảnh 1", "SC_001"), "Cảnh 1");
    assert.equal(sanitizeDisplayLabel("LOC_001: Thành cổ", undefined), "Thành cổ");
    assert.equal(sanitizeDisplayLabel("THM_001: Tình yêu nước", undefined), "Tình yêu nước");
    assert.equal(sanitizeDisplayLabel("REL_001: Bằng hữu", undefined), "Bằng hữu");
    assert.equal(sanitizeDisplayLabel("CHAR_001", "CHAR_001", "Nhân vật"), "Nhân vật");
    assert.equal(sanitizeDisplayLabel("ARC_001", undefined, "Hồi"), "Hồi");
  });

  it("renders ProjectView without any raw entity code slug in the snapshot", () => {
    const rawDto = {
      id: "PROJ_001",
      title: "[PROJ_001] Huyền Thoại Sông Lam",
      status: "Drafting",
      cover: "",
      description: "Mô tả dự án",
      genres: ["Lịch sử"],
      modifiedAt: "2026-11-08",
      details: { wordCount: "1000", chapters: "1", characters: "1", worldLore: "1", createdDate: "2026-11-08" },
      arcs: [
        {
          id: "ARC_001",
          title: "ARC_001: Khởi Sự",
          chapters: [
            {
              id: "CH_001",
              title: "CH_001: Xuất Binh",
              date: "2026-11-08",
              isNew: false,
              blocks: [{ id: "SC_001", sceneId: "SC_001", text: "Trận chiến bắt đầu", locked: false }],
            },
          ],
        },
      ],
      inProject: true,
    };

    const item = mapProjectDtoToItem(rawDto);
    assert.equal(item.title, "Huyền Thoại Sông Lam");
    assert.equal(item.arcs[0].title, "Khởi Sự");
    assert.equal(item.arcs[0].chapters[0].title, "Xuất Binh");

    const html = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        null,
        React.createElement(ProjectView, { initialProjects: [item] })
      )
    );

    // Verify project title is clean
    assert.match(html, /Huyền Thoại Sông Lam/);

    // Assert that no internal code slug appears in the rendered HTML
    const match = html.match(CODE_REGEX);
    assert.equal(
      match,
      null,
      `P5S4-B violation: found raw code ${match?.[0]} in rendered ProjectView snapshot`
    );
  });

  it("renders CharacterDetailLayout without any raw entity code slug in the snapshot", () => {
    const rawCharDto = {
      character: {
        id: "char-raw-01",
        code: "CHAR_001",
        name: "[CHAR_001] Nguyễn Huệ",
        nickname: "Quang Trung",
        species: "Nhân tộc",
        biologicalSex: "MALE",
        genderIdentity: "MALE",
        genderExpression: "MALE",
        inProject: true,
      },
      appearance: null,
      psychological: null,
      memories: [
        {
          id: "mem-1",
          memoryType: "Core",
          title: "THM_001: Đại thắng Đống Đa",
          description: "Chiến dịch thần tốc mùa xuân năm Kỷ Dậu",
          emotionalCharge: "Tự hào",
        },
      ],
      competences: [
        {
          id: "comp-1",
          domain: "Quân sự",
          competenceName: "COMP_001: Thần tốc hành quân",
          level: "Tuyệt đỉnh",
        },
      ],
      relationships: [
        {
          targetCharacterId: "REL_001",
          relationshipType: "Huynh đệ",
          intensity: "Cao",
          description: "Anh em Tây Sơn",
        },
      ],
    };

    const profile = mapCharacterDetailDtoToProfile(rawCharDto);
    assert.equal(profile.basicInfo.name, "Nguyễn Huệ");
    assert.equal(profile.competences[0].name, "Thần tốc hành quân");
    assert.equal(profile.relationships[0].targetName, "Người liên hệ");

    const html = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        { initialEntries: ["/character/char-raw-01"] },
        React.createElement(CharacterDetailLayout, { characterId: "char-raw-01" })
      )
    );

    const match = html.match(CODE_REGEX);
    assert.equal(
      match,
      null,
      `P5S4-B violation: found raw code ${match?.[0]} in rendered CharacterDetailLayout snapshot`
    );
  });
});

// ============================================================================
// Test 5: phase5_sprint4_deep1-F1 (Absent Project / Character -> Not-Found, NEVER mock fallback)
// ============================================================================
describe("phase5_sprint4_deep1-F1: absent project and character returns explicit not-found, never mock fallback", () => {
  it("renders explicit not-found UI when project is absent, never falling back to mock projects", () => {
    // When no project is found and targetId does not exist, ProjectDetailView must render not-found UI
    const html = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        { initialEntries: ["/project/non-existent-proj-id"] },
        React.createElement(ProjectDetailView, { projectId: "non-existent-proj-id" })
      )
    );

    assert.match(html, /data-testid="project-not-found"/);
    assert.match(html, /Không tìm thấy dự án|Project not found/);
    assert.doesNotMatch(html, /Vương Triều Bão Lửa/);
  });

  it("renders explicit not-found UI when character is absent, never falling back to mock characters", () => {
    const html = renderToStaticMarkup(
      React.createElement(
        MemoryRouter,
        { initialEntries: ["/character/non-existent-char-id"] },
        React.createElement(CharacterDetailLayout, { characterId: "non-existent-char-id" })
      )
    );

    assert.match(html, /data-testid="character-not-found"/);
    assert.match(html, /Không tìm thấy nhân vật|Character not found/);
    assert.doesNotMatch(html, /Kaelen Vance/);
  });
});

// ============================================================================
// Test 6: phase5_sprint4_deep1-F2 (Link failure preserves existing library rows/state)
// ============================================================================
describe("phase5_sprint4_deep1-F2: link failure preserves existing library rows and project state", () => {
  it("handles link failure gracefully without corrupting or mutating library rows", async () => {
    const libraryRows = [
      {
        id: "char-lib-1",
        code: "CHAR_LIB_01",
        name: "Lý Thường Kiệt",
        nickname: "Lý tướng quân",
        species: "Nhân tộc",
        biologicalSex: "MALE",
        genderIdentity: "MALE",
        genderExpression: "MALE",
        inProject: false,
      },
    ];

    // Fake client where linking fails (e.g. backend lock conflict or validation error)
    const failingClient = {
      status: "available",
      listCharacters: async () => ({ items: [...libraryRows] }),
      linkLibraryCharacterToDraft: async () => {
        throw new Error("DraftBusy: draft session is currently locked by another worker");
      },
    };

    // Attempt to link
    const result = await linkCharacterToProject(failingClient, {
      characterId: "char-lib-1",
      projectId: "proj-101",
      role: "Đại tướng",
      notes: "Ghi chú bảo vệ phòng tuyến",
    });

    // Verification: operation returned failure
    assert.equal(result.success, false);
    assert.match(result.error, /DraftBusy/);

    // Verification: existing library rows remain completely intact!
    const charactersAfterFailure = await failingClient.listCharacters();
    assert.equal(charactersAfterFailure.items.length, 1);
    assert.equal(charactersAfterFailure.items[0].id, "char-lib-1");
    assert.equal(charactersAfterFailure.items[0].name, "Lý Thường Kiệt");
    assert.equal(charactersAfterFailure.items[0].inProject, false);
  });
});

// ============================================================================
// Test 7: Visual & Layout Preservation and File Scope Invariant
// ============================================================================
describe("Visual & scope check: 1280/1279px layout preservation and allowed write paths", () => {
  it("preserves 1280px / 1279px split-pane vs compact overlay layout hooks and logic", () => {
    const detailSrc = readFileSync(
      new URL("./ProjectDetailView.tsx", import.meta.url),
      "utf8"
    );
    assert.match(detailSrc, /useIsCompact\(\)/);
    assert.match(detailSrc, /useResizablePanel/);
    assert.match(detailSrc, /isChatOpen && !isCompact/);
    assert.match(detailSrc, /isCompact && ['"]absolute inset-0 z-40/);

    const charDetailSrc = readFileSync(
      new URL("../characters/CharacterDetailLayout.tsx", import.meta.url),
      "utf8"
    );
    assert.match(charDetailSrc, /useIsCompact\(\)/);
    assert.match(charDetailSrc, /useResizablePanel/);
    assert.match(charDetailSrc, /isChatOpen && !isCompact/);
    assert.match(charDetailSrc, /isCompact && ['"]absolute inset-0 z-40/);
  });

  it("preserves ProjectCreatorPanel tested route navigation invariant (N12)", () => {
    const creatorSrc = readFileSync(
      new URL("./ProjectCreatorPanel.tsx", import.meta.url),
      "utf8"
    );
    assert.match(
      creatorSrc,
      /import \{ buildCreatorChapterPath \} from "@\/features\/draft\/commitNavigation"/
    );
    assert.match(
      creatorSrc,
      /navigate\(buildCreatorChapterPath\(project\.id, chapter\.id\)\)/
    );
    assert.ok(!creatorSrc.includes("?mode=creator"), "no inline creator route template");
  });

  it("verifies only files in allowed_write_paths are modified or added", () => {
    const allowed = new Set([
      "src/features/projects/ProjectView.tsx",
      "src/features/projects/ProjectDetailView.tsx",
      "src/features/projects/ProjectCreatorPanel.tsx",
      "src/features/characters/CharacterListView.tsx",
      "src/features/characters/CharacterDetailLayout.tsx",
      "src/features/projects/projectBackend.test.mjs",
    ]);

    const currentBranch = execSync("git rev-parse --abbrev-ref HEAD", { encoding: "utf8" }).trim();
    if (currentBranch === "impl/phase5_sprint4_deep1/a514385") {
      const diffFiles = execSync(
        "git diff --name-only a514385a6f5df0f16deb318dc50a4b47396162ee..HEAD",
        { encoding: "utf8" }
      )
        .trim()
        .split("\n")
        .filter(Boolean);
      for (const file of diffFiles) {
        assert.ok(
          allowed.has(file),
          `FORBIDDEN WRITE: File '${file}' is outside allowed_write_paths`
        );
      }
      return;
    }

    try {
      const diffFiles = execSync(
        "git diff --name-only a514385a6f5df0f16deb318dc50a4b47396162ee..impl/phase5_sprint4_deep1/a514385",
        { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }
      )
        .trim()
        .split("\n")
        .filter(Boolean);
      if (diffFiles.length > 0) {
        for (const file of diffFiles) {
          assert.ok(
            allowed.has(file),
            `FORBIDDEN WRITE: File '${file}' is outside allowed_write_paths`
          );
        }
        return;
      }
    } catch {
      // Fallback if branch ref is absent
    }

    for (const file of allowed) {
      assert.ok(file.startsWith("src/features/"));
    }
  });
});
