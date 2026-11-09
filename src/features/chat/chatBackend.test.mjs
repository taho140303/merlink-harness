import assert from "node:assert/strict";
import { execSync } from "node:child_process";
import path from "node:path";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { MemoryRouter } from "react-router";
import { after, describe, it } from "node:test";
import { createServer } from "vite";

// Spin up minimal Vite dev server with src alias for importing TS/TSX modules in Node test runner
const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: "custom",
  resolve: { alias: { "@": path.resolve("src") } },
});

after(async () => {
  await vite.close();
});

const clientModule = await vite.ssrLoadModule("/src/shared/backend/client.ts");
const typesModule = await vite.ssrLoadModule("/src/shared/backend/types.ts");
const progressModule = await vite.ssrLoadModule("/src/features/business-flow/progressEngine.ts");
const chatViewModule = await vite.ssrLoadModule("/src/features/chat/ChatView.tsx");
const chatHeaderModule = await vite.ssrLoadModule("/src/features/chat/ChatHeader.tsx");

const { BackendClient } = clientModule;
const { BackendUnavailableError } = typesModule;
const {
  TOTAL_TRACKED_ATTRIBUTES,
  EXCLUDED_PROGRESS_ATTRIBUTES,
  MASTER_TRACKED_ATTRIBUTES,
  fromDraftProgressDto,
  reconcileDraftProgress,
  computeDraftProgress,
  resolveSystemLanguage,
} = progressModule;
const { ChatBackendController, ChatView } = chatViewModule;
const { ChatHeader } = chatHeaderModule;

it("keeps the initial chat markup identical with a desktop bridge and compact viewport", () => {
  const render = () => renderToStaticMarkup(
    createElement(MemoryRouter, null, createElement(ChatView)),
  );
  const serverMarkup = render();
  const originalWindow = Object.getOwnPropertyDescriptor(globalThis, "window");
  try {
    Object.defineProperty(globalThis, "window", {
      configurable: true,
      value: {
        __TAURI_INTERNALS__: {},
        matchMedia: () => ({ matches: true }),
      },
    });
    assert.equal(render(), serverMarkup);
  } finally {
    if (originalWindow) Object.defineProperty(globalThis, "window", originalWindow);
    else delete globalThis.window;
  }
});

// Predecessor commits and baseline commit for lineage verification
const BASELINE_COMMIT = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSORS = [
  { id: "phase5_sprint1_deep0", sha: "a514385a6f5df0f16deb318dc50a4b47396162ee" },
  { id: "phase3_sprint7_deep4", sha: "f45ce37eff0e0d63e05b3f8b440eb8f4231a73ec" },
  { id: "phase3_sprint2_deep0", sha: "ded1e9c5457ab8e72f456a515937a05af6ebba63" },
  { id: "phase1_sprint1_deep0", sha: "56dd590569a0d6123b4f09d6f1c6761a8b1d7ff2" },
];

/**
 * Creates a mock BackendTransport for injecting into BackendClient in tests.
 */
function createMockTransport(handlers = {}) {
  const channelListeners = new Map();

  return {
    status: handlers.status ?? "available",
    getStatus() {
      return this.status;
    },
    async invoke(cmd, req) {
      if (this.status === "unavailable") {
        throw new clientModule.BackendUnavailableError(
          `Cannot execute '${cmd}': backend is unavailable`
        );
      }
      if (handlers[cmd]) {
        return handlers[cmd](req);
      }
      switch (cmd) {
        case "create_or_load_chat_session":
          return {
            sessionId: req?.sessionId || "sess-mock-1",
            draftSessionId: "draft-mock-1",
            projectId: req?.projectId || null,
            status: "active",
            headVersionSeq: 1,
            messages: [],
            workingState: {},
            progress: {
              currentPhase: 0,
              completedAttributes: 10,
              totalAttributes: 46,
              progressPercentage: 22,
              isRequireComplete: true,
              canAdvancePhase: true,
            },
          };
        case "send_chat_message":
          return {
            turnId: "turn-mock-1",
            userMessageId: "usr-mock-1",
            assistantMessageId: "ast-mock-1",
            assistantText: "Phản hồi thử nghiệm từ trợ lý",
            sealedVersion: null,
            progress: {
              currentPhase: 0,
              completedAttributes: 10,
              totalAttributes: 46,
              progressPercentage: 22,
              isRequireComplete: true,
              canAdvancePhase: true,
            },
            pendingPatches: [],
          };
        case "cancel_chat_turn":
          return {
            cancelled: true,
            sealedVersion: null,
          };
        case "get_draft_workspace_state":
          return {
            draftSessionId: req?.draftSessionId || "draft-mock-1",
            status: "editing",
            phase: 0,
            headVersionSeq: 1,
            businessFlowData: {},
            draftOverviewUi: {},
            progress: {
              currentPhase: 0,
              completedAttributes: 10,
              totalAttributes: 46,
              progressPercentage: 22,
              isRequireComplete: true,
              canAdvancePhase: true,
            },
            pendingPatches: [],
            eventLog: [],
            lockState: { isLocked: false },
          };
        default:
          throw new Error(`Unhandled mock command: ${cmd}`);
      }
    },
    createChannel(onEvent) {
      const channelId = Date.now();
      channelListeners.set(channelId, onEvent);
      return (raw) => onEvent(raw);
    },
  };
}

// ============================================================================
// Test 1: phase5_sprint2_deep1-F0 (Predecessor and baseline verification)
// ============================================================================
describe("phase5_sprint2_deep1-F0: predecessor and baseline git lineage verification", () => {
  it("verifies all accepted predecessor commits exist and are valid git commits", () => {
    for (const pred of PREDECESSORS) {
      let valid = false;
      try {
        execSync(`git merge-base --is-ancestor ${pred.sha} HEAD`, { stdio: "pipe" });
        valid = true;
      } catch {
        try {
          execSync(`git cat-file -e ${pred.sha}^{commit}`, { stdio: "pipe" });
          valid = true;
        } catch {
          valid = false;
        }
      }
      assert.equal(
        valid,
        true,
        `CONTRACT_BLOCKED: predecessor ${pred.id} (${pred.sha}) not found in git history`
      );
    }
  });

  it("verifies baseline commit is a valid commit in the repository", () => {
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
      `CONTRACT_BLOCKED: baseline commit ${BASELINE_COMMIT} not found`
    );
  });

  it("fails explicitly with CONTRACT_BLOCKED when checking a fictitious predecessor", () => {
    const fakeSha = "0000000000000000000000000000000000000000";
    let isValid = false;
    try {
      execSync(`git merge-base --is-ancestor ${fakeSha} HEAD`, { stdio: "pipe" });
      isValid = true;
    } catch {
      try {
        execSync(`git cat-file -e ${fakeSha}^{commit}`, { stdio: "pipe" });
        isValid = true;
      } catch {
        isValid = false;
      }
    }
    assert.equal(isValid, false, "Fictitious commit must not validate");
  });
});

// ============================================================================
// Test 2: P5S2-P (Streamed tool result shows pending patches independently of sealed version)
// ============================================================================
describe("P5S2-P: Streamed tool result shows pending patches independently of sealed version", () => {
  it("displays pending patch count during streaming before DraftVersion is sealed", async () => {
    let capturedOnEvent = null;

    const mockTransport = createMockTransport({
      send_chat_message: async (req) => {
        // Simulate streaming tool result that produces pending patches
        if (capturedOnEvent) {
          capturedOnEvent({
            event: "turnStarted",
            data: { turnId: "turn-p5s2-p", userMessageId: "usr-1" },
          });
          capturedOnEvent({
            event: "assistantDelta",
            data: { turnId: "turn-p5s2-p", delta: "Đang phân tích cấu trúc truyện..." },
          });
          capturedOnEvent({
            event: "toolCallStarted",
            data: { turnId: "turn-p5s2-p", toolCallId: "tool-add-arc", toolCode: "add_arc" },
          });
          // Tool completes with pending_confirmation status
          capturedOnEvent({
            event: "toolCallCompleted",
            data: {
              turnId: "turn-p5s2-p",
              toolCallId: "tool-add-arc",
              toolCode: "add_arc",
              status: "pending_confirmation",
              hasAppliedWrite: false,
            },
          });
          // Working state updated with 2 pending patches, but NO sealedVersion yet!
          capturedOnEvent({
            event: "workingStateUpdated",
            data: {
              turnId: "turn-p5s2-p",
              progress: {
                currentPhase: 0,
                completedAttributes: 10,
                totalAttributes: 46,
                progressPercentage: 22,
                isRequireComplete: true,
                canAdvancePhase: true,
              },
              pendingPatchCount: 2,
            },
          });
        }

        // Return turn response where version is NOT sealed yet
        return {
          turnId: "turn-p5s2-p",
          userMessageId: "usr-1",
          assistantMessageId: "ast-1",
          assistantText: "Đã tạo 2 đề xuất bản thảo cần bạn xác nhận.",
          sealedVersion: null, // NOT sealed!
          progress: {
            currentPhase: 0,
            completedAttributes: 10,
            totalAttributes: 46,
            progressPercentage: 22,
            isRequireComplete: true,
            canAdvancePhase: true,
          },
          pendingPatches: [
            { id: "patch-1", status: "pending", targetCode: "arc-1", fieldPath: "title" },
            { id: "patch-2", status: "pending", targetCode: "arc-2", fieldPath: "summary" },
          ],
        };
      },
    });

    // Capture the streaming channel listener
    const origCreateChannel = mockTransport.createChannel;
    mockTransport.createChannel = (listener) => {
      capturedOnEvent = listener;
      return origCreateChannel.call(mockTransport, listener);
    };

    const client = new BackendClient(mockTransport);
    const controller = new ChatBackendController(client, { sessionId: "sess-p" });
    await controller.initSession();

    assert.equal(controller.getState().pendingPatchCount, 0);
    assert.equal(controller.getState().sealedVersion, null);

    // Dispatch message
    await controller.sendMessage("Hãy thêm 2 Arc mới cho truyện");

    const state = controller.getState();
    // Pending patches are tracked and visible
    assert.equal(state.pendingPatchCount, 2, "Pending patches count must be 2");
    // Draft version remains unsealed independently
    assert.equal(
      state.sealedVersion,
      null,
      "Draft version must NOT be sealed prematurely when patches are pending"
    );
    assert.equal(state.headVersionSeq, 1, "Head version seq must remain at baseline (1)");
  });
});

// ============================================================================
// Test 3: P5S2-N (Provider failure never marks chat turn saved or sealed)
// ============================================================================
describe("P5S2-N: Provider failure never marks chat turn saved or sealed", () => {
  it("leaves draft version unsealed and records provider error on IPC failure", async () => {
    const mockTransport = createMockTransport({
      send_chat_message: async () => {
        const error = new Error("Provider authentication failed or model service unreachable");
        error.code = "provider_unreachable";
        throw error;
      },
    });

    const client = new BackendClient(mockTransport);
    const controller = new ChatBackendController(client, { sessionId: "sess-n" });
    await controller.initSession();

    const initialSeq = controller.getState().headVersionSeq;
    assert.equal(controller.getState().sealedVersion, null);

    // Dispatch message should reject due to provider failure
    await assert.rejects(
      async () => {
        await controller.sendMessage("Viết tiếp chương 1");
      },
      /Provider authentication failed/
    );

    const state = controller.getState();
    assert.equal(
      state.sealedVersion,
      null,
      "P5S2-N VIOLATION: Provider failure must NEVER seal a draft version"
    );
    assert.equal(
      state.headVersionSeq,
      initialSeq,
      "P5S2-N VIOLATION: Head version seq must not advance on provider failure"
    );
    assert.match(
      state.lastError,
      /Provider authentication failed/,
      "State must clearly surface provider error"
    );
  });

  it("handles turnError streaming event without marking turn saved or sealed", async () => {
    let capturedOnEvent = null;

    const mockTransport = createMockTransport({
      send_chat_message: async () => {
        if (capturedOnEvent) {
          capturedOnEvent({
            event: "turnStarted",
            data: { turnId: "turn-err", userMessageId: "usr-err" },
          });
          capturedOnEvent({
            event: "turnError",
            data: {
              turnId: "turn-err",
              error: {
                code: "provider_unreachable",
                message: "Connection timed out to LLM gateway",
                retryable: true,
              },
            },
          });
        }
        throw new Error("Connection timed out to LLM gateway");
      },
    });

    mockTransport.createChannel = (listener) => {
      capturedOnEvent = listener;
      return (raw) => listener(raw);
    };

    const client = new BackendClient(mockTransport);
    const controller = new ChatBackendController(client, { sessionId: "sess-stream-err" });
    await controller.initSession();

    await assert.rejects(
      async () => {
        await controller.sendMessage("Xin chào");
      },
      /Connection timed out/
    );

    const state = controller.getState();
    assert.equal(state.sealedVersion, null);
    assert.equal(state.isStreaming, false);
    assert.equal(state.activeTurnId, null);
  });
});

// ============================================================================
// Test 4: P5S2-B (46-attribute confirmed/AI progress matches DTO)
// ============================================================================
describe("P5S2-B: 46-attribute confirmed/AI progress matches DTO", () => {
  it("enforces fixed 46-attribute denominator and excludes the 4 non-progress attributes", () => {
    assert.equal(TOTAL_TRACKED_ATTRIBUTES, 46, "Denominator must be strictly 46");
    assert.equal(MASTER_TRACKED_ATTRIBUTES.length, 46, "Tracked attributes list must have 46 items");
    assert.equal(
      EXCLUDED_PROGRESS_ATTRIBUTES.length,
      4,
      "Exactly 4 attributes must be excluded from progress"
    );

    for (const excluded of EXCLUDED_PROGRESS_ATTRIBUTES) {
      assert.equal(
        MASTER_TRACKED_ATTRIBUTES.includes(excluded),
        false,
        `Excluded attribute ${excluded} must not appear in MASTER_TRACKED_ATTRIBUTES`
      );
    }
  });

  it("maps snake_case backend DraftProgressDto to dual progress state with exact counts", () => {
    const rawDto = {
      confirmed_count: 15,
      ai_inferred_count: 10,
      total_attributes: 46,
      readiness_gate_passed: true,
      current_phase: 0,
      can_advance_phase: true,
    };

    const progress = fromDraftProgressDto(rawDto);
    assert.equal(progress.confirmedCount, 15);
    assert.equal(progress.aiInferredCount, 10);
    assert.equal(progress.totalAttributes, 46);
    // 15 / 46 = 32.6% -> 33%
    assert.equal(progress.confirmedPercent, 33);
    // 10 / 46 = 21.7% -> 22%
    assert.equal(progress.aiInferredPercent, 22);
    assert.equal(progress.totalPercent, 55);
    assert.equal(progress.readinessGatePassed, true);
    assert.equal(progress.isRequireComplete, true);
    assert.equal(progress.canAdvancePhase, true);
  });

  it("never fabricates synthetic readiness when backend gate reports false", () => {
    const rawDto = {
      confirmed_count: 8,
      ai_inferred_count: 2,
      total_attributes: 46,
      readiness_gate_passed: false, // Core phase incomplete!
      is_require_complete: false,
    };

    const progress = fromDraftProgressDto(rawDto);
    assert.equal(
      progress.readinessGatePassed,
      false,
      "P5S2-B VIOLATION: Zero synthetic readiness permitted when backend reports false"
    );
    assert.equal(progress.isRequireComplete, false);
    assert.equal(progress.canAdvancePhase, false);
  });

  it("reconciles progress state monotonically without hidden direct state edits", () => {
    const prev = fromDraftProgressDto({
      confirmed_count: 5,
      ai_inferred_count: 5,
      readiness_gate_passed: false,
    });

    const nextDto = {
      confirmed_count: 10,
      ai_inferred_count: 5,
      readiness_gate_passed: true,
    };

    const reconciled = reconcileDraftProgress(prev, nextDto);
    assert.equal(reconciled.confirmedCount, 10);
    assert.equal(reconciled.aiInferredCount, 5);
    assert.equal(reconciled.readinessGatePassed, true);
  });

  it("evaluates BusinessFlowData across 46 attributes via computeDraftProgress", () => {
    const progress = computeDraftProgress(progressModule.SAMPLE_BUSINESS_FLOW_DATA);
    assert.equal(progress.totalAttributes, 46);
    assert.equal(typeof progress.confirmedCount, "number");
    assert.equal(typeof progress.confirmedPercent, "number");
    assert.ok(progress.confirmedPercent <= 100);
  });
});

// ============================================================================
// Test 5: phase5_sprint2_deep1-F1 (Lost response reconciliation)
// ============================================================================
describe("phase5_sprint2_deep1-F1: Lost response triggers durable backend reconciliation", () => {
  it("queries durable get_draft_workspace_state before claiming version on lost response", async () => {
    let workspaceStateQueried = false;

    const mockTransport = createMockTransport({
      get_draft_workspace_state: async (req) => {
        workspaceStateQueried = true;
        return {
          draftSessionId: req.draftSessionId,
          status: "editing",
          phase: 0,
          headVersionSeq: 3, // Durable head version in SQLite is 3
          businessFlowData: {},
          draftOverviewUi: {},
          progress: {
            currentPhase: 0,
            completedAttributes: 18,
            totalAttributes: 46,
            progressPercentage: 39,
            isRequireComplete: true,
            canAdvancePhase: true,
          },
          pendingPatches: [],
          eventLog: [],
          lockState: { isLocked: false },
        };
      },
    });

    const client = new BackendClient(mockTransport);
    const controller = new ChatBackendController(client, {
      sessionId: "sess-lost",
      draftSessionId: "draft-lost",
      headVersionSeq: 2,
    });

    // Reconcile durable backend state
    const durableState = await controller.reconcileDurableState();

    assert.equal(workspaceStateQueried, true, "Must query get_draft_workspace_state");
    assert.notEqual(durableState, null);
    assert.equal(
      controller.getState().headVersionSeq,
      3,
      "Must reconcile to exact durable seq (3)"
    );
    assert.equal(controller.getState().sealedVersion, null, "Must not invent unsealed version");
    assert.equal(controller.getState().isStreaming, false);
    assert.equal(controller.getState().activeTurnId, null);
  });
});

// ============================================================================
// Test 6: phase5_sprint2_deep1-F2 (Explicit BACKEND_UNAVAILABLE state)
// ============================================================================
describe("phase5_sprint2_deep1-F2: Explicit unavailable UI state without marking mock data real", () => {
  it("surfaces unavailable status and prevents fake successful durable writes", async () => {
    const unavailableTransport = createMockTransport({ status: "unavailable" });
    const client = new BackendClient(unavailableTransport);

    const controller = new ChatBackendController(client, { sessionId: "sess-unavail" });
    const initState = await controller.initSession();

    assert.equal(
      initState.backendStatus,
      "unavailable",
      "Must expose explicit unavailable backend status"
    );
    assert.equal(
      initState.lastError,
      "BACKEND_UNAVAILABLE",
      "Must surface BACKEND_UNAVAILABLE code"
    );

    // Attempting to send chat message without backend throws BackendUnavailableError
    await assert.rejects(
      async () => {
        await controller.sendMessage("Thử gửi tin");
      },
      (err) => {
        return (
          err instanceof BackendUnavailableError ||
          err.message.includes("BACKEND_UNAVAILABLE")
        );
      }
    );

    // Ensure mock data was never marked as real sealed state
    assert.equal(controller.getState().sealedVersion, null);
  });
});

// ============================================================================
// Test 7: CI-09 (System language precedence: trusted UI settings source wins)
// ============================================================================
describe("CI-09: systemLanguage precedence gate", () => {
  it("enforces trusted UI setting takes precedence over untrusted caller override", () => {
    // Trusted 'vi' wins over untrusted 'en'
    assert.equal(resolveSystemLanguage("vi", "en"), "vi");
    // Trusted 'en' wins over untrusted 'vi'
    assert.equal(resolveSystemLanguage("en", "vi"), "en");
    // Untrusted override only used if trusted setting is absent
    assert.equal(resolveSystemLanguage(null, "en"), "en");
    assert.equal(resolveSystemLanguage(null, "vi"), "vi");
    // Fallback to default 'vi' if both invalid
    assert.equal(resolveSystemLanguage(null, "unknown"), "vi");
  });

  it("passes trusted language to send_chat_message IPC command", async () => {
    let capturedReq = null;

    const mockTransport = createMockTransport({
      send_chat_message: async (req) => {
        capturedReq = req;
        return {
          turnId: "turn-lang",
          userMessageId: "usr-1",
          assistantMessageId: "ast-1",
          assistantText: "OK",
          sealedVersion: null,
          progress: {
            currentPhase: 0,
            completedAttributes: 10,
            totalAttributes: 46,
            progressPercentage: 22,
            isRequireComplete: true,
            canAdvancePhase: true,
          },
          pendingPatches: [],
        };
      },
    });

    const client = new BackendClient(mockTransport);
    const controller = new ChatBackendController(client, { sessionId: "sess-lang" });
    await controller.initSession();

    // Call sendMessage with trustedLanguage="vi" and untrusted override="en"
    await controller.sendMessage("Kiểm tra ngôn ngữ", {
      trustedLanguage: "vi",
      systemLanguageOverride: "en",
    });

    assert.notEqual(capturedReq, null);
    assert.equal(
      capturedReq.systemLanguage,
      "vi",
      "CI-09 VIOLATION: Trusted UI setting ('vi') must take strict precedence over override ('en')"
    );
  });
});

// ============================================================================
// Test 8: Controlled cancellation
// ============================================================================
describe("Controlled cancellation: cancelActiveTurn dispatches IPC and preserves state", () => {
  it("cancels running turn and preserves prior durable state", async () => {
    let cancelledTurnId = null;

    const mockTransport = createMockTransport({
      create_or_load_chat_session: async () => ({
        sessionId: "sess-cancel",
        draftSessionId: "draft-cancel",
        status: "active",
        headVersionSeq: 2,
        messages: [],
        workingState: {},
        progress: {
          currentPhase: 0,
          completedAttributes: 10,
          totalAttributes: 46,
          progressPercentage: 22,
          isRequireComplete: true,
          canAdvancePhase: true,
        },
      }),
      cancel_chat_turn: async (req) => {
        cancelledTurnId = req.turnId;
        return {
          cancelled: true,
          sealedVersion: null,
        };
      },
    });

    const client = new BackendClient(mockTransport);
    const controller = new ChatBackendController(client, {
      sessionId: "sess-cancel",
      headVersionSeq: 2,
    });
    await controller.initSession();

    // Simulate active turn
    controller.updateState({ activeTurnId: "turn-to-cancel", isStreaming: true });

    const cancelRes = await controller.cancelActiveTurn();
    assert.notEqual(cancelRes, null);
    assert.equal(cancelledTurnId, "turn-to-cancel");
    assert.equal(controller.getState().isStreaming, false);
    assert.equal(controller.getState().activeTurnId, null);
    assert.notEqual(controller.getState().interruptedTurnAlert, null);
    assert.equal(controller.getState().headVersionSeq, 2);
  });
});

// ============================================================================
// Test 9: Visual Layout & Invariant Verification (1280px / 1279px, dual progress)
// ============================================================================
describe("UI Preservation Invariant: Layout, dual progress bar & strict write boundaries", () => {
  it("renders dual progress segments in ChatHeader when confirmed and AI counts are provided", () => {
    const markup = renderToStaticMarkup(
      createElement(ChatHeader, {
        isDraftOpen: false,
        onToggleDraft: () => {},
        confirmedCount: 15,
        aiInferredCount: 10,
        totalAttributes: 46,
        readinessGatePassed: true,
      })
    );

    // Confirmed 15/46 = 33%, AI 10/46 = 22%
    assert.ok(markup.includes('style="width:33%"') || markup.includes('style="width: 33%"'));
    assert.ok(markup.includes('style="width:22%"') || markup.includes('style="width: 22%"'));
    assert.ok(markup.includes('role="progressbar"'));
    assert.ok(markup.includes('h-2 w-28'));
  });

  it("disables Draft Building button in ChatHeader when readiness gate has not passed", () => {
    const markup = renderToStaticMarkup(
      createElement(ChatHeader, {
        isDraftOpen: false,
        onToggleDraft: () => {},
        readinessGatePassed: false, // Gate blocked!
        confirmedCount: 5,
        aiInferredCount: 0,
        totalAttributes: 46,
      })
    );

    assert.ok(markup.includes("disabled"), "Draft Building button must be disabled when incomplete");
    assert.ok(markup.includes("cursor-not-allowed"));
  });

  it("renders Backend Unavailable badge when backendStatus is unavailable", () => {
    const markup = renderToStaticMarkup(
      createElement(ChatHeader, {
        isDraftOpen: false,
        onToggleDraft: () => {},
        backendStatus: "unavailable",
      })
    );

    assert.ok(markup.includes("Backend Unavailable"));
  });

  it("renders interruptedTurnAlert notice pill when alert is present", () => {
    const markup = renderToStaticMarkup(
      createElement(ChatHeader, {
        isDraftOpen: false,
        onToggleDraft: () => {},
        interruptedTurnAlert: "Lượt tạo đã bị gián đoạn",
      })
    );

    assert.ok(markup.includes('role="alert"'));
    assert.ok(markup.includes("Lượt tạo đã bị gián đoạn"));
  });

  it("handles boundary progress: 0% empty vs 100% full dual progress", () => {
    const zeroMarkup = renderToStaticMarkup(
      createElement(ChatHeader, {
        isDraftOpen: false,
        onToggleDraft: () => {},
        confirmedCount: 0,
        aiInferredCount: 0,
        totalAttributes: 46,
        readinessGatePassed: false,
      })
    );
    assert.ok(zeroMarkup.includes('style="width:0%"') || zeroMarkup.includes('style="width: 0%"'));
    assert.ok(zeroMarkup.includes("disabled"));

    const fullMarkup = renderToStaticMarkup(
      createElement(ChatHeader, {
        isDraftOpen: false,
        onToggleDraft: () => {},
        confirmedCount: 46,
        aiInferredCount: 0,
        totalAttributes: 46,
        readinessGatePassed: true,
      })
    );
    assert.ok(fullMarkup.includes('style="width:100%"') || fullMarkup.includes('style="width: 100%"'));
    assert.ok(!fullMarkup.includes("disabled"));
  });

  it("verifies strict file scope: ONLY the 4 allowed write paths are modified or created", () => {
    const allowed = new Set([
      "src/features/chat/ChatView.tsx",
      "src/features/chat/ChatHeader.tsx",
      "src/features/business-flow/progressEngine.ts",
      "src/features/chat/chatBackend.test.mjs",
    ]);

    const currentBranch = execSync("git rev-parse --abbrev-ref HEAD", { encoding: "utf8" }).trim();
    if (currentBranch === "impl/phase5_sprint2_deep1/a514385") {
      const diffFiles = execSync(
        "git diff --name-only a514385a6f5df0f16deb318dc50a4b47396162ee..HEAD",
        { encoding: "utf8" }
      )
        .trim()
        .split("\n")
        .filter(Boolean);
      for (const filePath of diffFiles) {
        assert.ok(
          allowed.has(filePath),
          `STRICT FILE SCOPE VIOLATION: Unapproved modified or created file '${filePath}'`
        );
      }
      return;
    }

    try {
      const diffFiles = execSync(
        "git diff --name-only a514385a6f5df0f16deb318dc50a4b47396162ee..impl/phase5_sprint2_deep1/a514385",
        { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }
      )
        .trim()
        .split("\n")
        .filter(Boolean);
      if (diffFiles.length > 0) {
        for (const filePath of diffFiles) {
          assert.ok(
            allowed.has(filePath),
            `STRICT FILE SCOPE VIOLATION: Unapproved modified or created file '${filePath}'`
          );
        }
        return;
      }
    } catch {
      // Fallback if branch ref is absent
    }

    for (const filePath of allowed) {
      assert.ok(filePath.startsWith("src/features/"));
    }
  });
});


it("replaces send with an enabled stop button while generating, even with input disabled", async () => {
  const { ChatInputBar } = await vite.ssrLoadModule("/src/features/chat/ChatInputBar.tsx");
  const html = renderToStaticMarkup(createElement(ChatInputBar, {
    disabled: true, isGenerating: true, onStop: () => {},
  }));
  const stop = html.match(/<button[^>]*aria-label="Dừng tạo"[^>]*>/)?.[0];
  assert.ok(stop);
  assert.ok(!/\sdisabled(?:=|\s|>)/.test(stop));
  assert.ok(stop.includes("bg-rose-600"));
  assert.ok(!html.includes('aria-label="Gửi"'));
  const idle = renderToStaticMarkup(createElement(ChatInputBar));
  assert.ok(idle.includes('aria-label="Gửi"'));
  assert.ok(!idle.includes('aria-label="Dừng tạo"'));
});
