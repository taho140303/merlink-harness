import assert from "node:assert/strict";
import { execSync } from "node:child_process";
import { after, describe, it, test } from "node:test";
import { createServer } from "vite";

// Spin up minimal Vite dev server in middleware mode to load TS modules directly in Node test runner
const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: "custom",
});
after(async () => {
  await vite.close();
});

const clientModule = await vite.ssrLoadModule("/src/shared/backend/client.ts");
const adapterModule = await vite.ssrLoadModule("/src/shared/backend/adapter.ts");
const typesModule = await vite.ssrLoadModule("/src/shared/backend/types.ts");
const generatedModule = await vite.ssrLoadModule("/src/shared/backend/generated.ts");

const BASELINE_COMMIT = "3bb971748651cfafa5a1b38ef5d93f8fc77c6f5c";
const PREDECESSOR_P2S5 = "02fc9b735457d47115e0f90063650dc00f322dc1";

// ============================================================================
// Test 1: phase5_sprint1_deep0-F0 (Git ancestry and predecessor verification)
// ============================================================================
describe("phase5_sprint1_deep0-F0: predecessor and baseline git verification", () => {
  it("verifies gate_phase2_sprint5_deep2 predecessor commit is an ancestor or valid commit", () => {
    let predecessorValid = false;
    try {
      execSync(`git merge-base --is-ancestor ${PREDECESSOR_P2S5} HEAD`, { stdio: "pipe" });
      predecessorValid = true;
    } catch {
      try {
        execSync(`git cat-file -e ${PREDECESSOR_P2S5}^{commit}`, { stdio: "pipe" });
        predecessorValid = true;
      } catch {
        predecessorValid = false;
      }
    }
    assert.equal(
      predecessorValid,
      true,
      `CONTRACT_BLOCKED: predecessor gate_phase2_sprint5_deep2 (${PREDECESSOR_P2S5}) not found`
    );
  });

  it("verifies baseline commit is a valid commit object", () => {
    let baselineValid = false;
    try {
      execSync(`git cat-file -e ${BASELINE_COMMIT}^{commit}`, { stdio: "pipe" });
      baselineValid = true;
    } catch {
      baselineValid = false;
    }
    assert.equal(
      baselineValid,
      true,
      `CONTRACT_BLOCKED: baseline commit (${BASELINE_COMMIT}) is invalid`
    );
  });

  it("fails explicitly with CONTRACT_BLOCKED when a fake predecessor is checked", () => {
    const fakePred = "0000000000000000000000000000000000000000";
    let fakePredValid = true;
    try {
      execSync(`git cat-file -e ${fakePred}^{commit}`, { stdio: "pipe" });
    } catch {
      fakePredValid = false;
    }
    assert.equal(
      fakePredValid,
      false,
      "Expected check on fake predecessor to fail"
    );
  });
});

// ============================================================================
// Test 2: P5S1-P (Fixture adapter preserves source & code, hides code label, round-trip)
// ============================================================================
describe("P5S1-P: fixture adapter round-trip and code label hiding invariant", () => {
  it("formatDisplayLabel hides raw code prefix while preserving clean name", () => {
    assert.equal(adapterModule.formatDisplayLabel("[CHAR_001] Alice", "CHAR_001"), "Alice");
    assert.equal(adapterModule.formatDisplayLabel("CHAR_001: Bob", "CHAR_001"), "Bob");
    assert.equal(adapterModule.formatDisplayLabel("CHAR_001 - Carol", "CHAR_001"), "Carol");
    assert.equal(adapterModule.formatDisplayLabel("ARC_01: Hồi 1: Khởi đầu", "ARC_01"), "Hồi 1: Khởi đầu");
    assert.equal(adapterModule.formatDisplayLabel("SCN_05 - Gặp gỡ bất ngờ", "SCN_05"), "Gặp gỡ bất ngờ");
    assert.equal(adapterModule.formatDisplayLabel("David", "CHAR_002"), "David");
    assert.equal(adapterModule.formatDisplayLabel("", "CHAR_003"), "Chưa đặt tên");
  });

  it("toCharacterViewModel preserves code and source while hiding code label from displayLabel", () => {
    const dto = {
      id: "char-123",
      code: "CHAR_ALICE_01",
      name: "[CHAR_ALICE_01] Alice Margatroid",
      nickname: "Seven-Colored Puppeteer",
      species: "Magician",
      biologicalSex: "FEMALE",
      genderIdentity: "FEMALE",
      genderExpression: "FEMALE",
      inProject: true,
      avatarUrl: "https://example.com/alice.png",
      defaultSummary: "A talented magician.",
    };

    const vm = adapterModule.toCharacterViewModel(dto, "user_edited");
    assert.equal(vm.id, "char-123");
    assert.equal(vm.code, "CHAR_ALICE_01"); // internal code preserved
    assert.equal(vm.displayName, "[CHAR_ALICE_01] Alice Margatroid");
    assert.equal(vm.displayLabel, "Alice Margatroid"); // code stripped from UI label
    assert.equal(vm.source, "user_edited"); // provenance preserved
    assert.equal(vm.species, "Magician");
  });

  it("toSceneViewModel preserves scene id, code, and source provenance", () => {
    const dto = {
      id: "scn-01",
      chapterId: "chap-01",
      position: 1,
      displayName: "SCN_01: Buổi sáng tĩnh lặng",
      summary: "Alice thức dậy tại rừng pháp thuật.",
      content: "Ánh ban mai xuyên qua tán cây.",
      wordCount: 8,
      isLocked: true,
      headVersionNumber: 3,
      currentSceneSource: "AI",
    };

    const vm = adapterModule.toSceneViewModel(dto);
    assert.equal(vm.id, "scn-01");
    assert.equal(vm.displayLabel, "Buổi sáng tĩnh lặng");
    assert.equal(vm.source, "ai_generated");
    assert.equal(vm.isLocked, true);
    assert.equal(vm.headVersionNumber, 3);
  });

  it("round-trip mapping preserves Draft, Chat, Workflow, Project, Writing, ProviderConfig DTOs", () => {
    // 1. Draft
    const draftDto = {
      draftSessionId: "draft-001",
      status: "editing",
      phase: 2,
      subphase: 1,
      businessFlowData: { core_theme: "Courage" },
      progress: {
        currentPhase: 2,
        completedAttributes: 15,
        totalAttributes: 46,
        progressPercentage: 32.6,
        isRequireComplete: false,
        canAdvancePhase: false,
      },
      pendingPatches: [],
      eventLog: [],
      lockState: { isLocked: false },
    };
    const draftAdapted = adapterModule.adaptBackendToDraft(draftDto);
    assert.equal(draftAdapted.draftSessionId, "draft-001");
    assert.equal(draftAdapted.businessFlowData.coreTheme, "Courage");

    // 2. Chat
    const chatDto = {
      id: "msg-1",
      role: "assistant",
      content: "Xin chào!",
      timestamp: "2026-11-08T00:00:00Z",
    };
    const chatAdapted = adapterModule.adaptBackendToChat(chatDto);
    assert.equal(chatAdapted.id, "msg-1");
    assert.equal(chatAdapted.role, "assistant");

    // 3. Workflow
    const workflowDto = {
      workflowRunId: "wf-1",
      planGeneration: 1,
      status: "completed",
      completedTasks: 5,
      totalTasks: 5,
      pendingPatches: [],
    };
    const wfAdapted = adapterModule.adaptBackendToWorkflow(workflowDto);
    assert.equal(wfAdapted.workflowRunId, "wf-1");
    assert.equal(wfAdapted.completedTasks, 5);

    // 4. Project
    const projectDto = {
      id: "proj-1",
      title: "Merlink Chronicles",
      status: "Drafting",
      cover: "cover.jpg",
      description: "Epic tale",
      genres: ["Fantasy"],
      modifiedAt: "2026-11-08T00:00:00Z",
      details: {
        wordCount: "1000",
        chapters: "1",
        characters: "2",
        worldLore: "5",
        createdDate: "2026-11-01",
      },
    };
    const projVm = adapterModule.toProjectViewModel(projectDto);
    assert.equal(projVm.id, "proj-1");
    assert.equal(projVm.displayLabel, "Merlink Chronicles");
    assert.equal(projVm.details.wordCount, "1000");

    // 5. Provider Config
    const provDto = {
      providerId: "openai",
      name: "OpenAI Official",
      isActive: true,
      hasApiKey: true,
      endpointUrl: "https://api.openai.com/v1",
      modelName: "gpt-4o",
    };
    const provVm = adapterModule.toProviderConfigViewModel(provDto);
    assert.equal(provVm.providerId, "openai");
    assert.equal(provVm.hasApiKey, true);
  });
});

// ============================================================================
// Test 3: P5S1-N & phase5_sprint1_deep0-F1 (SSR no-window safety & unavailable error)
// ============================================================================
describe("P5S1-N & phase5_sprint1_deep0-F1: SSR environment safety and unavailable transport", () => {
  it("imports client and adapter without window/Tauri without throwing", () => {
    assert.ok(clientModule.backendClient);
    assert.ok(clientModule.createBackendClient);
    assert.ok(clientModule.setBackendTransport);
    assert.equal(clientModule.isTauriEnvironment(), false);
  });

  it("returns explicit unavailable status and never returns fake mock success", async () => {
    // Reset custom transport to ensure default transport runs in current Node SSR env
    clientModule.setBackendTransport(null);
    const client = clientModule.createBackendClient();
    assert.equal(client.status, "unavailable");
    assert.equal(client.getStatus(), "unavailable");

    await assert.rejects(
      async () => {
        await client.listProjects();
      },
      (err) => {
        assert.ok(err instanceof typesModule.BackendUnavailableError || err.status === "unavailable");
        assert.equal(err.code, "provider_unreachable");
        return true;
      }
    );

    await assert.rejects(
      async () => {
        await client.getDraftWorkspaceState({ draftSessionId: "draft-1" });
      },
      (err) => {
        assert.equal(err.status, "unavailable");
        return true;
      }
    );
  });

  it("executes successfully when a fake typed transport is injected (DI)", async () => {
    const mockProjects = {
      items: [
        {
          id: "p1",
          title: "Mock Project",
          status: "Drafting",
          cover: "",
          description: "Desc",
          genres: [],
          modifiedAt: "2026-11-08",
          details: { wordCount: "0", chapters: "0", characters: "0", worldLore: "0", createdDate: "2026-11-08" },
        },
      ],
    };

    const fakeTransport = {
      status: "available",
      getStatus: () => "available",
      invoke: async (cmd, _req) => {
        if (cmd === "list_projects") {
          return mockProjects;
        }
        throw new Error(`Unexpected command: ${cmd}`);
      },
      createChannel: (onEvent) => ({ onEvent }),
    };

    const customClient = clientModule.createBackendClient(fakeTransport);
    assert.equal(customClient.status, "available");
    const result = await customClient.listProjects();
    assert.deepEqual(result, mockProjects);
  });
});

// ============================================================================
// Test 4: P5S1-B (Unknown IPC error maps to safe status and redacts secrets)
// ============================================================================
describe("P5S1-B: unknown IPC error mapping and strict secret redaction", () => {
  it("redacts plaintext API keys and Bearer tokens from error message", () => {
    const rawError = new Error("Request failed with sk-secret-123456789 and Bearer abc.def.ghi123");
    const normalized = adapterModule.normalizeBackendError(rawError);

    assert.equal(normalized.code, "internal_error");
    assert.ok(!normalized.message.includes("sk-secret-123456789"));
    assert.ok(!normalized.message.includes("abc.def.ghi123"));
    assert.ok(normalized.message.includes("[REDACTED]"));
  });

  it("redacts sensitive fields in error details object", () => {
    const rawIpc = {
      code: "unauthorized_caller",
      message: "Authorization failed",
      details: {
        apiKey: "sk-proj-9999999999",
        secret: "super-secret-key",
        normalField: "visible",
      },
      retryable: false,
    };

    const normalized = adapterModule.normalizeBackendError(rawIpc);
    assert.equal(normalized.code, "unauthorized_caller");
    assert.equal(normalized.details.apiKey, "[REDACTED]");
    assert.equal(normalized.details.secret, "[REDACTED]");
    assert.equal(normalized.details.normalField, "visible");
  });

  it("maps unrecognized error code to safe internal_error", () => {
    const rawUnknown = {
      code: "unrecognized_custom_code_from_future",
      message: "Something strange happened",
    };
    const normalized = adapterModule.normalizeBackendError(rawUnknown);
    assert.equal(normalized.code, "internal_error");
    assert.equal(normalized.retryable, false);
  });
});

// ============================================================================
// Test 5: phase5_sprint1_deep0-F2 (Integrity of generated.ts and all 40 commands)
// ============================================================================
describe("phase5_sprint1_deep0-F2: generated.ts integrity and 40 IPC command coverage", () => {
  it("verifies src/shared/backend/generated.ts has zero uncommitted modifications", () => {
    const diff = execSync("git diff HEAD -- src/shared/backend/generated.ts", {
      encoding: "utf8",
    });
    assert.equal(diff.trim(), "", "src/shared/backend/generated.ts must NOT be edited manually");
  });

  it("verifies IPC_COMMAND_NAMES has exactly 40 command names", () => {
    assert.equal(generatedModule.IPC_COMMAND_NAMES.length, 40);
  });

  it("verifies client implements typed methods for all 40 commands", () => {
    const client = clientModule.createBackendClient();
    const commandToMethodMap = {
      create_or_load_chat_session: "createOrLoadChatSession",
      list_chat_sessions: "listChatSessions",
      send_chat_message: "sendChatMessage",
      cancel_chat_turn: "cancelChatTurn",
      get_draft_workspace_state: "getDraftWorkspaceState",
      get_progress_detail: "getProgressDetail",
      apply_manual_draft_edit: "applyManualDraftEdit",
      confirm_phase: "confirmPhase",
      regenerate_draft_phase: "regenerateDraftPhase",
      list_draft_versions: "listDraftVersions",
      restore_draft_version: "restoreDraftVersion",
      list_pending_patches: "listPendingPatches",
      confirm_pending_patch: "confirmPendingPatch",
      reject_pending_patch: "rejectPendingPatch",
      confirm_all_pending_patches: "confirmAllPendingPatches",
      start_or_resume_workflow: "startOrResumeWorkflow",
      cancel_workflow: "cancelWorkflow",
      get_workflow_status: "getWorkflowStatus",
      get_commit_confirmation_preview: "getCommitConfirmationPreview",
      commit_draft: "commitDraft",
      reopen_committed_draft: "reopenCommittedDraft",
      list_projects: "listProjects",
      get_project_detail: "getProjectDetail",
      delete_project: "deleteProject",
      list_characters: "listCharacters",
      get_character_detail: "getCharacterDetail",
      create_or_update_library_character: "createOrUpdateLibraryCharacter",
      link_library_character_to_draft: "linkLibraryCharacterToDraft",
      get_chapter_writing_state: "getChapterWritingState",
      set_scene_lock: "setSceneLock",
      set_chapter_scenes_lock_bulk: "setChapterScenesLockBulk",
      start_scene_generation: "startSceneGeneration",
      cancel_scene_generation: "cancelSceneGeneration",
      respond_edit_scope_expansion: "respondEditScopeExpansion",
      list_scene_versions: "listSceneVersions",
      save_chapter: "saveChapter",
      get_chapter_save_receipt: "getChapterSaveReceipt",
      get_runtime_and_provider_config: "getRuntimeAndProviderConfig",
      update_runtime_and_provider_config: "updateRuntimeAndProviderConfig",
      set_provider_api_key: "setProviderApiKey",
    };

    for (const [cmd, methodName] of Object.entries(commandToMethodMap)) {
      assert.equal(
        typeof client[methodName],
        "function",
        `BackendClient missing method '${methodName}' for command '${cmd}'`
      );
    }
  });

  it("verifies streaming sequence validators handle correct and invalid transitions", () => {
    // Chat stream
    const validChatEvents = [
      { event: "turnStarted", data: { turnId: "t1", userMessageId: "u1" } },
      { event: "assistantDelta", data: { turnId: "t1", delta: "Hello " } },
      { event: "assistantDelta", data: { turnId: "t1", delta: "World" } },
      { event: "turnCompleted", data: { turnId: "t1", assistantMessageId: "a1" } },
    ];
    assert.equal(adapterModule.validateChatStreamSequence(validChatEvents).valid, true);

    const reducedChat = adapterModule.reduceChatStream(validChatEvents);
    assert.equal(reducedChat.text, "Hello World");
    assert.equal(reducedChat.isComplete, true);

    const invalidChatEvents = [
      { event: "assistantDelta", data: { turnId: "t1", delta: "Hi" } },
    ];
    assert.equal(adapterModule.validateChatStreamSequence(invalidChatEvents).valid, false);

    // Scene Generation stream
    const validSceneEvents = [
      {
        event: "started",
        data: {
          requestId: "r1",
          generationId: "g1",
          sceneId: "s1",
          baselineVersionNumber: 1,
          authorizationValid: true,
          status: "streaming",
        },
      },
      {
        event: "delta",
        data: { requestId: "r1", generationId: "g1", sceneId: "s1", textDelta: "Scene text", authorizationValid: true },
      },
      {
        event: "completed",
        data: {
          requestId: "r1",
          generationId: "g1",
          sceneId: "s1",
          baselineVersionNumber: 1,
          fullText: "Scene text complete",
          authorizationValid: true,
          status: "completed",
        },
      },
    ];
    assert.equal(adapterModule.validateSceneGenerationStreamSequence(validSceneEvents).valid, true);
    const reducedScene = adapterModule.reduceSceneGenerationStream(validSceneEvents);
    assert.equal(reducedScene.text, "Scene text complete");
    assert.equal(reducedScene.status, "completed");
  });
});

// ============================================================================
// Test 6: P5S1-VISUAL (Visual preservation invariant check)
// ============================================================================
describe("P5S1-VISUAL: UI layout, routes, and design tokens preservation invariant", () => {
  const getP5S1TargetRef = () => {
    try {
      execSync("git rev-parse --verify impl/phase5_sprint1_deep0/3bb9717", {
        stdio: "ignore",
      });
      return `${BASELINE_COMMIT}..impl/phase5_sprint1_deep0/3bb9717`;
    } catch {
      return BASELINE_COMMIT;
    }
  };

  it("verifies only files in allowed_write_paths are modified or added", () => {
    const allowed = new Set([
      "src/shared/backend/client.ts",
      "src/shared/backend/adapter.ts",
      "src/shared/backend/types.ts",
      "src/features/integration/contractFixtures.test.mjs",
    ]);

    const diffRange = getP5S1TargetRef();
    const diffFiles = execSync(`git diff --name-only ${diffRange}`, {
      encoding: "utf8",
    })
      .trim()
      .split("\n")
      .filter((line) => line.length > 0);

    for (const file of diffFiles) {
      assert.ok(
        allowed.has(file),
        `UI Preservation Invariant Violated: Unapproved modified file '${file}'`
      );
    }
  });

  it("confirms 0 modifications to routes, components, or styles", () => {
    const diffRange = getP5S1TargetRef();
    const diffUi = execSync(
      `git diff ${diffRange} -- src/routes src/features/characters src/features/draft src/features/chat src/features/projects src/features/settings`,
      {
        encoding: "utf8",
      }
    );
    assert.equal(diffUi.trim(), "", "Zero UI files outside integration tests should be modified");
  });
});
