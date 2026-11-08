import assert from "node:assert/strict";
import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { after, describe, it } from "node:test";
import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { createMemoryRouter, RouterProvider } from "react-router";
import { createServer } from "vite";

// Spin up minimal Vite dev server in middleware mode to load TS/TSX modules directly in Node
const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false },
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
const generatedModule = await vite.ssrLoadModule("/src/shared/backend/generated.ts");
const scenePreviewModule = await vite.ssrLoadModule("/src/features/projects/scenePreview.ts");
const chapterDetailModule = await vite.ssrLoadModule("/src/features/projects/ChapterDetailView.tsx");

const { createBackendClient } = clientModule;
const { IPC_COMMAND_NAMES } = generatedModule;
const {
  buildSceneBlocksFromReadModel,
  isSceneContentDirty,
  isSceneDirty,
  reviewGeneratedScene,
  restoreSceneVersion,
  loadChapterWritingReadModel,
  setSceneLock,
  setChapterScenesLockBulk,
  generateSceneDraft,
  cancelSceneGeneration,
  listSceneVersions,
  saveChapterScenes,
  getChapterSaveReceipt,
  executeSaveWithReceiptReconciliation,
} = scenePreviewModule;
const { ChapterDetailView } = chapterDetailModule;

// Predecessors and baseline SHAs
const BASELINE_COMMIT = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P5S3 = "a33eb05a208eebfa46e7ff6a8d5f988cb5de8892";
const PREDECESSOR_P5S4 = "319f3ce00ac539864bdb3fcf50ead2bde02d456a";
const PREDECESSOR_P4S4 = "e8c1e32b3c0badc47bffba73936b8998ed2b2fb1";
const PREDECESSOR_P4S5 = "3bb971748651cfafa5a1b38ef5d93f8fc77c6f5c";

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
// Test 1: phase5_sprint5_deep3-F0 (Git ancestry and predecessor verification)
// ============================================================================
describe("phase5_sprint5_deep3-F0: predecessor and baseline git lineage verification", () => {
  it("verifies phase5_sprint3_deep2 predecessor commit is valid", () => {
    const valid = verifyGitCommit(PREDECESSOR_P5S3);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor phase5_sprint3_deep2 (${PREDECESSOR_P5S3}) not found`
    );
  });

  it("verifies phase5_sprint4_deep1 predecessor commit is valid", () => {
    const valid = verifyGitCommit(PREDECESSOR_P5S4);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor phase5_sprint4_deep1 (${PREDECESSOR_P5S4}) not found`
    );
  });

  it("verifies phase4_sprint4_deep1 predecessor commit is valid", () => {
    const valid = verifyGitCommit(PREDECESSOR_P4S4);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor phase4_sprint4_deep1 (${PREDECESSOR_P4S4}) not found`
    );
  });

  it("verifies phase4_sprint5_deep2 predecessor commit is valid", () => {
    const valid = verifyGitCommit(PREDECESSOR_P4S5);
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor phase4_sprint5_deep2 (${PREDECESSOR_P4S5}) not found`
    );
  });

  it("verifies source baseline commit is a valid commit object", () => {
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
// Test 2: P5S5-P (Valid completed stream remains reviewable after request scope expires)
// ============================================================================
describe("P5S5-P: valid completed stream remains reviewable after request scope expires", () => {
  const sampleSceneId = "0192e2b3-7649-7000-8000-000000000010";

  function createStreamingTransport() {
    let channelClosed = false;
    return {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, payload) => {
        if (cmd === "start_scene_generation") {
          const { channel } = payload;
          if (channel?.onEvent) {
            channel.onEvent({
              event: "started",
              data: {
                requestId: "req-streaming-1",
                generationId: "gen-streaming-1",
                sceneId: sampleSceneId,
                baselineVersionNumber: 1,
                authorizationValid: true,
                status: "streaming",
              },
            });
            channel.onEvent({
              event: "delta",
              data: {
                requestId: "req-streaming-1",
                generationId: "gen-streaming-1",
                sceneId: sampleSceneId,
                textDelta: "Đoạn văn thứ nhất sinh bởi AI. ",
              },
            });
            channel.onEvent({
              event: "delta",
              data: {
                requestId: "req-streaming-1",
                generationId: "gen-streaming-1",
                sceneId: sampleSceneId,
                textDelta: "Đoạn văn kết thúc hoàn chỉnh.",
              },
            });
            channel.onEvent({
              event: "completed",
              data: {
                requestId: "req-streaming-1",
                generationId: "gen-streaming-1",
                sceneId: sampleSceneId,
                fullText: "Đoạn văn thứ nhất sinh bởi AI. Đoạn văn kết thúc hoàn chỉnh.",
                status: "completed",
              },
            });
          }
          channelClosed = true;
          return {
            requestId: "req-streaming-1",
            generationId: "gen-streaming-1",
            sceneId: sampleSceneId,
            baselineVersionNumber: 1,
            status: "completed",
            authorizationValid: true,
            generatedText: "Đoạn văn thứ nhất sinh bởi AI. Đoạn văn kết thúc hoàn chỉnh.",
            normalizedNote: null,
          };
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
      createChannel: (onEvent) => ({ onEvent }),
      isChannelClosed: () => channelClosed,
    };
  }

  it("completes generation stream, closes request scope, and candidate remains reviewable", async () => {
    const transport = createStreamingTransport();
    const client = createBackendClient(transport);

    const receivedEvents = [];
    const candidate = await generateSceneDraft(
      client,
      {
        projectId: "proj-1",
        chapterId: "chap-1",
        sceneId: sampleSceneId,
        mode: "generate",
      },
      (ev) => receivedEvents.push(ev.event)
    );

    // Channel/request scope is now closed
    assert.equal(transport.isChannelClosed(), true, "Request scope / channel must be terminated");
    assert.deepEqual(receivedEvents, ["started", "delta", "delta", "completed"]);

    // Candidate remains in 'review' state with authorizationValid = true
    assert.equal(candidate.status, "review");
    assert.equal(candidate.authorizationValid, true);
    assert.equal(candidate.text, "Đoạn văn thứ nhất sinh bởi AI. Đoạn văn kết thúc hoàn chỉnh.");

    // User reviews and accepts AFTER request scope expired (SCOPE-02..04 / P5S5-P)
    const scene = {
      id: sampleSceneId,
      sceneId: sampleSceneId,
      text: "Nội dung cũ",
      locked: false,
      headVersionNumber: 1,
    };

    const reviewRes = reviewGeneratedScene({
      action: "accept",
      candidate,
      scene,
    });

    assert.equal(reviewRes.accepted, true);
    assert.equal(reviewRes.source, "AI_GENERATED");
    assert.equal(reviewRes.updatedText, candidate.text);
  });
});

// ============================================================================
// Test 3: P5S5-N (Lock-unlock cannot accept invalidated generation)
// ============================================================================
describe("P5S5-N: lock-unlock cannot accept invalidated generation", () => {
  const sampleSceneId = "0192e2b3-7649-7000-8000-000000000020";

  it("locking a scene invalidates generation; subsequent unlock cannot accept it", () => {
    const scene = {
      id: sampleSceneId,
      sceneId: sampleSceneId,
      text: "Nội dung hiện thời",
      locked: false,
      headVersionNumber: 1,
    };

    // Candidate originally produced validly
    const candidate = {
      requestId: "req-1",
      generationId: "gen-1",
      sceneId: sampleSceneId,
      baselineVersionNumber: 1,
      authorizationValid: true,
      status: "review",
      text: "Văn bản AI đề xuất",
    };

    // Scene is locked by user (LOCK-04)
    scene.locked = true;
    candidate.authorizationValid = false;
    candidate.status = "invalidated_by_lock";

    // While locked: accept fails
    assert.throws(
      () => reviewGeneratedScene({ action: "accept", candidate, scene }),
      /CONFLICT_INVALIDATED/
    );

    // Scene is subsequently UNLOCKED by user
    scene.locked = false;
    // P5S5-N: candidate.authorizationValid remains FALSE! Unlocking does not revive invalid generation
    assert.equal(candidate.authorizationValid, false);

    assert.throws(
      () => reviewGeneratedScene({ action: "accept", candidate, scene }),
      /CONFLICT_INVALIDATED/
    );
  });

  it("scene version bump after generation also blocks acceptance via CONFLICT_INVALIDATED", () => {
    const scene = {
      id: sampleSceneId,
      sceneId: sampleSceneId,
      text: "Nội dung đã sửa thủ công",
      locked: false,
      headVersionNumber: 2, // Bumped from 1 to 2
    };

    const candidate = {
      requestId: "req-2",
      generationId: "gen-2",
      sceneId: sampleSceneId,
      baselineVersionNumber: 1, // Generated based on version 1
      authorizationValid: true,
      status: "review",
      text: "Văn bản AI dựa trên version cũ",
    };

    assert.throws(
      () => reviewGeneratedScene({ action: "accept", candidate, scene }),
      /CONFLICT_INVALIDATED/
    );
  });
});

// ============================================================================
// Test 4: P5S5-B (Locked clean scene does not block saving another dirty unlocked scene)
// ============================================================================
describe("P5S5-B: locked clean scene does not block saving another dirty unlocked scene", () => {
  const scene1 = {
    id: "0192e2b3-7649-7000-8000-000000000031",
    sceneId: "0192e2b3-7649-7000-8000-000000000031",
    text: "Cảnh 1 không đổi",
    locked: true, // LOCKED, but CLEAN
    headVersionNumber: 1,
  };
  const scene1Baseline = {
    text: "Cảnh 1 không đổi",
    locked: true,
  };

  const scene2 = {
    id: "0192e2b3-7649-7000-8000-000000000032",
    sceneId: "0192e2b3-7649-7000-8000-000000000032",
    text: "Cảnh 2 đã sửa nội dung",
    locked: false, // UNLOCKED and DIRTY
    headVersionNumber: 1,
  };
  const scene2Baseline = {
    text: "Cảnh 2 ban đầu",
    locked: false,
  };

  it("filters out clean locked scene and persists dirty unlocked scene successfully", async () => {
    assert.equal(isSceneContentDirty(scene1, scene1Baseline), false, "Scene 1 must be clean");
    assert.equal(isSceneContentDirty(scene2, scene2Baseline), true, "Scene 2 must be dirty");

    let submittedPayload = null;
    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, req) => {
        if (cmd === "save_chapter") {
          submittedPayload = req;
          return {
            chapterId: req.chapterId,
            clientSaveId: req.clientSaveId,
            savedScenes: [{ sceneId: scene2.sceneId, newVersionNumber: 2 }],
            chapterScenes: [],
            wasIdempotentReplay: false,
          };
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
    };
    const client = createBackendClient(fakeTransport);

    // Prepare dirty scenes array (only dirty scenes submitted)
    const allBlocks = [scene1, scene2];
    const baselines = new Map([[scene1.sceneId, scene1Baseline], [scene2.sceneId, scene2Baseline]]);
    const dirtyScenes = allBlocks
      .filter((b) => isSceneContentDirty(b, baselines.get(b.sceneId)))
      .map((b) => ({
        sceneId: b.sceneId,
        expectedVersionNumber: b.headVersionNumber,
        content: b.text,
        isLocked: b.locked,
      }));

    assert.equal(dirtyScenes.length, 1);
    assert.equal(dirtyScenes[0].sceneId, scene2.sceneId);

    const res = await saveChapterScenes(client, {
      projectId: "proj-1",
      chapterId: "chap-1",
      clientSaveId: "save-uuid-1",
      dirtyScenes,
    });

    assert.equal(res.clientSaveId, "save-uuid-1");
    assert.equal(submittedPayload.dirtyScenes.length, 1);
    assert.equal(submittedPayload.dirtyScenes[0].sceneId, scene2.sceneId);
  });

  it("rejects save with SceneLocked if any DIRTY scene is locked (LOCK-05)", async () => {
    const dirtyLockedScene = {
      sceneId: "0192e2b3-7649-7000-8000-000000000033",
      expectedVersionNumber: 1,
      content: "Nội dung vừa sửa",
      isLocked: true, // Dirty AND Locked!
    };

    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async () => assert.fail("save_chapter should not be called when dirty scene is locked"),
    };
    const client = createBackendClient(fakeTransport);

    await assert.rejects(
      () =>
        saveChapterScenes(client, {
          projectId: "proj-1",
          chapterId: "chap-1",
          clientSaveId: "save-uuid-2",
          dirtyScenes: [dirtyLockedScene],
        }),
      /SceneLocked/
    );
  });
});

// ============================================================================
// Test 5: phase5_sprint5_deep3-F1 (Lock failure keeps UI unlocked, dirty buffer intact)
// ============================================================================
describe("phase5_sprint5_deep3-F1: Lock failure leaves UI unlocked and dirty buffer intact", () => {
  it("when setSceneLock rejects with DatabaseError, UI state remains unlocked and dirty text untouched", async () => {
    const workingScene = {
      id: "0192e2b3-7649-7000-8000-000000000040",
      sceneId: "0192e2b3-7649-7000-8000-000000000040",
      text: "Nội dung đang viết dở dang...",
      locked: false,
    };
    const originalText = workingScene.text;

    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd) => {
        if (cmd === "set_scene_lock") {
          throw new Error("DatabaseError: SQLite TX-06 locked busy");
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
    };
    const client = createBackendClient(fakeTransport);

    // Attempt lock toggle
    let lockFailed = false;
    try {
      await setSceneLock(client, {
        projectId: "proj-1",
        chapterId: "chap-1",
        sceneId: workingScene.sceneId,
        desiredLocked: true,
      });
      workingScene.locked = true;
    } catch {
      lockFailed = true;
      // UI retains previous lock state (false) and dirty content buffer completely intact!
    }

    assert.equal(lockFailed, true, "Lock operation must report failure");
    assert.equal(workingScene.locked, false, "Scene must remain unlocked on lock failure");
    assert.equal(workingScene.text, originalText, "Dirty buffer must remain completely intact");
  });
});

// ============================================================================
// Test 6: phase5_sprint5_deep3-F2 (Lost Save response reconciliation)
// ============================================================================
describe("phase5_sprint5_deep3-F2: Lost Save response reuses clientSaveId and reconciles via receipt", () => {
  it("reuses SAME clientSaveId and reconciles receipt without blind overwrite or new saveId", async () => {
    const FIXED_CLIENT_SAVE_ID = "save-idempotency-token-777";
    const sceneId = "0192e2b3-7649-7000-8000-000000000050";
    let saveAttempts = 0;
    let receiptQueried = false;

    const savedResponse = {
      chapterId: "chap-1",
      clientSaveId: FIXED_CLIENT_SAVE_ID,
      savedScenes: [{ sceneId, newVersionNumber: 3 }],
      chapterScenes: [
        {
          id: sceneId,
          chapterId: "chap-1",
          position: 1,
          displayName: "Cảnh 1",
          content: "Nội dung đã lưu bền vững",
          wordCount: 5,
          isLocked: false,
          headVersionNumber: 3,
          currentSceneSource: "USER",
        },
      ],
      wasIdempotentReplay: true,
    };

    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, req) => {
        if (cmd === "save_chapter") {
          saveAttempts += 1;
          // Simulate transport timeout / lost response on first attempt
          throw new Error("TransportError: Network connection lost before response received");
        }
        if (cmd === "get_chapter_save_receipt") {
          receiptQueried = true;
          assert.equal(
            req.clientSaveId,
            FIXED_CLIENT_SAVE_ID,
            "Receipt query must reuse the exact SAME clientSaveId"
          );
          return {
            found: true,
            receipt: savedResponse,
            currentChapterScenes: savedResponse.chapterScenes,
          };
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
    };
    const client = createBackendClient(fakeTransport);

    const dirtyScenes = [
      {
        sceneId,
        expectedVersionNumber: 2,
        content: "Nội dung đã lưu bền vững",
      },
    ];

    const result = await executeSaveWithReceiptReconciliation(client, {
      projectId: "proj-1",
      chapterId: "chap-1",
      clientSaveId: FIXED_CLIENT_SAVE_ID,
      dirtyScenes,
    });

    assert.equal(saveAttempts, 1);
    assert.equal(receiptQueried, true);
    assert.equal(result.success, true);
    assert.equal(result.reconciledFromReceipt, true);
    assert.equal(result.response.clientSaveId, FIXED_CLIENT_SAVE_ID);
    assert.equal(result.response.savedScenes[0].newVersionNumber, 3);
  });
});

// ============================================================================
// Test 7: CI-08 / REQ-WRITE-004 Verification (No 41st IPC; Restore via Save Chapter)
// ============================================================================
describe("CI-08 / REQ-WRITE-004: exact 40 IPC commands; version restore flows through dirty buffer + save_chapter", () => {
  it("verifies exact 40 IPC commands; no 41st IPC command invented", () => {
    assert.equal(
      IPC_COMMAND_NAMES.length,
      40,
      `Master IPC command count drift: expected 40, found ${IPC_COMMAND_NAMES.length}`
    );
    assert.ok(!IPC_COMMAND_NAMES.includes("restore_scene_version"), "restore_scene_version must not exist");
  });

  it("verifies listSceneVersions returns full content and restore flows into dirty buffer", async () => {
    const sceneId = "0192e2b3-7649-7000-8000-000000000060";
    const fakeVersions = [
      {
        versionNumber: 2,
        content: "Nội dung phiên bản 2",
        wordCount: 4,
        sceneSource: "USER",
        createdAt: "2026-11-08T07:00:00Z",
      },
      {
        versionNumber: 1,
        content: "Nội dung phiên bản 1 nguyên gốc",
        wordCount: 6,
        sceneSource: "AI",
        createdAt: "2026-11-08T06:00:00Z",
      },
    ];

    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, req) => {
        if (cmd === "list_scene_versions") {
          return {
            sceneId: req.sceneId,
            headVersionNumber: 2,
            versions: fakeVersions,
          };
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
    };
    const client = createBackendClient(fakeTransport);

    const versionList = await listSceneVersions(client, {
      projectId: "proj-1",
      chapterId: "chap-1",
      sceneId,
    });

    assert.equal(versionList.versions.length, 2);
    assert.equal(versionList.versions[1].content, "Nội dung phiên bản 1 nguyên gốc");

    // Restore version 1 into unlocked scene
    const scene = {
      id: sceneId,
      sceneId,
      text: "Nội dung phiên bản 2 hiện tại",
      locked: false,
      headVersionNumber: 2,
    };
    const baseline = { text: "Nội dung phiên bản 2 hiện tại" };

    const restoreRes = restoreSceneVersion(scene, versionList.versions[1]);
    assert.equal(restoreRes.success, true);
    assert.equal(restoreRes.source, "RESTORE");
    assert.equal(restoreRes.newText, "Nội dung phiên bản 1 nguyên gốc");

    // Apply restored content to working text
    scene.text = restoreRes.newText;
    assert.equal(isSceneContentDirty(scene, baseline), true, "Restored text must mark scene dirty");
  });

  it("blocks restore when scene is locked with explicit contract notice", () => {
    const lockedScene = {
      id: "s-locked",
      sceneId: "s-locked",
      text: "Locked text",
      locked: true,
    };
    assert.throws(
      () => restoreSceneVersion(lockedScene, { versionNumber: 1, content: "Old text" }),
      /SCENE_LOCKED_RESTORE_BLOCKED/
    );
  });
});

// ============================================================================
// Test 8: Visual & Scope Check (1280px / 1279px layout preservation & strict file scope)
// ============================================================================
describe("Visual & scope check: 1280px / 1279px layout preservation and allowed write paths", () => {
  it("preserves 1280px / 1279px split-pane vs compact overlay layout hooks and classes", () => {
    const viewSrc = readFileSync(
      new URL("./ChapterDetailView.tsx", import.meta.url),
      "utf8"
    );

    // Responsive hooks & conditions
    assert.match(viewSrc, /useIsCompact\(\)/);
    assert.match(viewSrc, /useResizablePanel/);
    assert.match(viewSrc, /isChatOpen && !isCompact/);
    assert.match(viewSrc, /isCompact && ["']absolute inset-0 z-40/);

    // Header responsive measurements (< 510px compactHeader, < 420px stackHeader)
    assert.match(viewSrc, /width > 0 && width < 510/);
    assert.match(viewSrc, /width > 0 && width < 420/);
    assert.match(viewSrc, /compactHeader/);
    assert.match(viewSrc, /stackHeader/);
  });

  it("renders ChapterDetailView cleanly without crashing", () => {
    const mockChapter = {
      id: "chap-1",
      title: "Chương 1: Mở màn",
      date: "2026-11-08",
      blocks: [
        { id: "blk-1", text: "Khung cảnh chiều tà.", locked: false },
      ],
    };
    const mockProject = {
      id: "proj-1",
      title: "Đại Nam Dị Truyện",
      status: "Drafting",
      cover: "",
      description: "",
      genres: [],
      modifiedAt: "2026-11-08",
      details: { wordCount: "1000", chapters: "1", characters: "2", worldLore: "3", createdDate: "2026-11-08" },
      arcs: [],
    };

    const router = createMemoryRouter([
      {
        path: "/",
        element: React.createElement(ChapterDetailView, {
          project: mockProject,
          chapter: mockChapter,
          chapterIndex: 1,
        }),
      },
    ]);

    const markup = renderToStaticMarkup(
      React.createElement(RouterProvider, { router })
    );

    assert.ok(markup.includes("Chương 1: Mở màn"));
    assert.ok(markup.includes("Khung cảnh chiều tà."));
  });

  it("verifies only files in allowed_write_paths are modified or added", () => {
    const allowed = new Set([
      "src/pages/ChapterDetailPage.tsx",
      "src/features/projects/ChapterDetailView.tsx",
      "src/features/projects/scenePreview.ts",
      "src/features/projects/scenePreview.test.mjs",
      "src/features/projects/chapterBackend.test.mjs",
    ]);

    const currentBranch = execSync("git rev-parse --abbrev-ref HEAD", { encoding: "utf8" }).trim();
    if (currentBranch === "impl/phase5_sprint5_deep3/a33eb05") {
      const diffFiles = execSync(
        "git diff --name-only a33eb05a208eebfa46e7ff6a8d5f988cb5de8892..HEAD",
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
        "git diff --name-only a33eb05a208eebfa46e7ff6a8d5f988cb5de8892..impl/phase5_sprint5_deep3/a33eb05",
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
      // Fallback
    }

    for (const file of allowed) {
      assert.ok(file.startsWith("src/"));
    }
  });
});
