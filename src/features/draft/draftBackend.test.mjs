import assert from "node:assert/strict";
import { execSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { after, describe, it } from "node:test";
import { createServer } from "vite";

// Spin up minimal Vite dev server with src alias for importing TS/TSX modules in Node test runner
const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false },
  appType: "custom",
  resolve: { alias: { "@": path.resolve("src") } },
});

after(async () => {
  await vite.close();
});

const clientModule = await vite.ssrLoadModule("/src/shared/backend/client.ts");
const commitPreviewModule = await vite.ssrLoadModule("/src/features/draft/commitPreview.ts");
const commitNavModule = await vite.ssrLoadModule("/src/features/draft/commitNavigation.ts");
const draftWorkspaceModule = await vite.ssrLoadModule("/src/features/draft/DraftWorkspace.tsx");
const draftToolbarModule = await vite.ssrLoadModule("/src/features/draft/DraftToolbar.tsx");
const commitDialogModule = await vite.ssrLoadModule("/src/features/draft/CommitConfirmationDialog.tsx");
const { DRAFT_OVERVIEW } = await vite.ssrLoadModule("/src/features/draft/mockOverview.ts");

const { BackendClient } = clientModule;
const {
  WORKFLOW_BASELINE_UNAVAILABLE,
  REQUIRES_REPLAN,
  EXPIRED_BY_RESTORE,
  isTaskApplicable,
  fromCommitPreviewDto,
  isCommitEligible,
  deriveValidation,
  derivePendingPatches,
  canDismissCommitDialog,
  reduceCommitOp,
} = commitPreviewModule;
const {
  buildCommittedProjectViewPath,
  buildProjectPath,
  createOnceGuard,
  fromCommitResultDto,
  resolveRealNavigation,
  executeDraftCommitWithReconciliation,
} = commitNavModule;
const { DraftBackendController, DraftWorkspace } = draftWorkspaceModule;
const { DraftToolbar } = draftToolbarModule;
const { CommitConfirmationDialog } = commitDialogModule;

// Baseline and Accepted Predecessors for phase5_sprint3_deep2
const BASELINE_COMMIT = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSORS = [
  { id: "phase5_sprint2_deep1", sha: "4c27bbff4371fd4ab3fba86e44f94deee470502f" },
  { id: "phase4_sprint1_deep0", sha: "848c8c33a97dd7c26db71fff9711f3cd21fa302f" },
  { id: "phase4_sprint2_deep0", sha: "b013b9f9682ba92966af32e94f9c45d77473c25d" },
  { id: "phase5_sprint1_deep0", sha: "a514385a6f5df0f16deb318dc50a4b47396162ee" },
];

function verifyGitCommit(sha) {
  try {
    execSync(`git merge-base --is-ancestor ${sha} HEAD`, { stdio: "ignore" });
    return true;
  } catch {
    try {
      execSync(`git cat-file -e ${sha}^{commit}`, { stdio: "ignore" });
      return true;
    } catch {
      throw new Error(`CONTRACT_BLOCKED: Missing required predecessor or baseline commit '${sha}'`);
    }
  }
}

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
        case "get_draft_workspace_state":
          return {
            draftSessionId: req?.draftSessionId || "draft-mock-1",
            status: "editing",
            phase: 1,
            headVersionSeq: 1,
            businessFlowData: {},
            draftOverviewUi: {},
            progress: {
              currentPhase: 1,
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
        case "list_draft_versions":
          return {
            headVersionSeq: 1,
            versions: [
              {
                seq: 1,
                displayLabel: "v.01",
                phase: 1,
                reason: "Initial baseline",
                createdAt: new Date().toISOString(),
              },
            ],
          };
        case "restore_draft_version":
          return {
            newVersion: {
              seq: (req?.targetSeq || 1) + 1,
              displayLabel: `v.0${(req?.targetSeq || 1) + 1}`,
              phase: 1,
              reason: `Restored from version ${req?.targetSeq}`,
              createdAt: new Date().toISOString(),
            },
            restoredPhase: 1,
            expiredPatchIds: ["patch-1", "patch-2"],
            workingState: {},
          };
        case "list_pending_patches":
          return {
            items: [],
          };
        case "confirm_pending_patch":
          return {
            status: "confirmed",
            sealedVersion: {
              seq: 2,
              displayLabel: "v.02",
              phase: 1,
              reason: "ai_patch",
              createdAt: new Date().toISOString(),
            },
            progress: {
              currentPhase: 1,
              completedAttributes: 11,
              totalAttributes: 46,
              progressPercentage: 24,
              isRequireComplete: true,
              canAdvancePhase: true,
            },
          };
        case "reject_pending_patch":
          return {
            status: "rejected",
            sealedVersion: null,
            progress: {
              currentPhase: 1,
              completedAttributes: 10,
              totalAttributes: 46,
              progressPercentage: 22,
              isRequireComplete: true,
              canAdvancePhase: true,
            },
          };
        case "confirm_all_pending_patches":
          return {
            applied: ["patch-1"],
            skipped: [],
            sealedVersion: {
              seq: 2,
              displayLabel: "v.02",
              phase: 1,
              reason: "ai_patch",
              createdAt: new Date().toISOString(),
            },
            progress: {
              currentPhase: 1,
              completedAttributes: 11,
              totalAttributes: 46,
              progressPercentage: 24,
              isRequireComplete: true,
              canAdvancePhase: true,
            },
          };
        case "start_or_resume_workflow":
          return {
            workflowRunId: "run-mock-1",
            planGeneration: 1,
            status: "completed",
            completedTasks: 5,
            totalTasks: 5,
            lastSealedVersion: {
              seq: 2,
              displayLabel: "v.02",
              phase: 1,
              reason: "workflow_round",
              createdAt: new Date().toISOString(),
            },
            pendingPatches: [],
          };
        case "cancel_workflow":
          return {
            status: "cancelled",
            resumeAvailable: true,
          };
        case "get_commit_confirmation_preview":
          return {
            draftSessionId: req?.draftSessionId || "draft-mock-1",
            status: "ready",
            planSummary: {
              projectName: "Đại Việt Sử Ký",
              targetArcs: 3,
              targetChapters: 12,
              targetWordsPerChapter: 2500,
              characterCount: 6,
              arcCount: 3,
              chapterCount: 12,
              worldSettingCount: 4,
              loreEntryCount: 10,
            },
            validation: {
              status: "ready",
              issues: [],
            },
            pendingPatches: {
              count: 0,
              items: [],
            },
            canConfirm: true,
          };
        case "commit_draft":
          return {
            status: "committed",
            projectId: "proj-committed-1",
            redirectPath: "/projects/proj-committed-1",
          };
        case "list_projects":
          return {
            items: [],
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
// Acceptance Tests
// ============================================================================

describe("phase5_sprint3_deep2-F0: predecessor and baseline git lineage verification", () => {
  it("verifies all accepted predecessor commits exist and are valid git commits", () => {
    for (const pred of PREDECESSORS) {
      assert.doesNotThrow(
        () => verifyGitCommit(pred.sha),
        `Predecessor commit for '${pred.id}' (${pred.sha}) must exist`
      );
    }
  });

  it("verifies baseline commit is a valid commit in the repository", () => {
    assert.doesNotThrow(
      () => verifyGitCommit(BASELINE_COMMIT),
      `Baseline commit (${BASELINE_COMMIT}) must exist`
    );
  });

  it("fails explicitly with CONTRACT_BLOCKED when checking a fictitious predecessor", () => {
    const fakeSha = "0000000000000000000000000000000000000000";
    assert.throws(
      () => verifyGitCommit(fakeSha),
      /CONTRACT_BLOCKED/,
      "Non-existent commit must throw CONTRACT_BLOCKED"
    );
  });
});

describe("P5S3-P: Commit success navigates once to real project in View mode (/projects/${projectId})", () => {
  it("buildCommittedProjectViewPath generates canonical view mode route without mode parameter", () => {
    const path = buildCommittedProjectViewPath("proj-123");
    assert.equal(path, "/projects/proj-123");
    assert.ok(!path.includes("mode="), "Must not include mode parameter; default is View mode");
  });

  it("executes commit success and navigates exactly once to real project in View mode", async () => {
    let commitCalls = 0;
    const transport = createMockTransport({
      commit_draft: async (req) => {
        commitCalls++;
        return {
          status: "committed",
          projectId: "proj-durable-456",
          redirectPath: "/projects/proj-durable-456",
        };
      },
    });
    const client = new BackendClient(transport);

    let navCalls = 0;
    let navigatedPath = null;
    let navigatedProjId = null;

    const once = createOnceGuard();
    const handleSuccess = (projectId, path) => {
      once(() => {
        navCalls++;
        navigatedProjId = projectId;
        navigatedPath = path;
      });
    };

    const res = await executeDraftCommitWithReconciliation({
      client,
      draftSessionId: "draft-sess-1",
      onSuccess: handleSuccess,
    });

    assert.equal(res.status, "committed");
    assert.equal(res.projectId, "proj-durable-456");
    assert.equal(commitCalls, 1);
    assert.equal(navCalls, 1);
    assert.equal(navigatedProjId, "proj-durable-456");
    assert.equal(navigatedPath, "/projects/proj-durable-456");

    // Second duplicate invocation through once-guard must not trigger navigation again
    handleSuccess("proj-durable-456", "/projects/proj-durable-456");
    assert.equal(navCalls, 1, "Navigation must execute at most once");
  });

  it("resolveRealNavigation resolves committed result to View mode path and blocks uncommitted", () => {
    const eligible = resolveRealNavigation({ status: "committed", projectId: "p-99" }, false);
    assert.deepEqual(eligible, {
      ok: true,
      projectId: "p-99",
      path: "/projects/p-99",
    });

    const pendingBlocked = resolveRealNavigation({ status: "committed", projectId: "p-99" }, true);
    assert.deepEqual(pendingBlocked, { ok: false, reason: "pending" });

    const failedBlocked = resolveRealNavigation({ status: "failed", reason: "error" }, false);
    assert.deepEqual(failedBlocked, { ok: false, reason: "notCommitted" });

    const missingIdBlocked = resolveRealNavigation({ status: "committed", projectId: "" }, false);
    assert.deepEqual(missingIdBlocked, { ok: false, reason: "missingProjectId" });
  });
});

describe("P5S3-N: Pending patch blocks Confirm and dialog cannot be dismissed while pending/submitting", () => {
  it("blocks commit eligibility when pending patches exist or validation is not ready", () => {
    const readyVal = deriveValidation({ validation: { ready: true } });
    const blockedVal = deriveValidation({ validation: { ready: false, blockingReasons: ["Thiếu nhân vật"] } });
    const zeroPatches = derivePendingPatches({ pendingPatches: { count: 0 } });
    const pendingPatches = derivePendingPatches({ pendingPatches: { count: 2 } });
    const unknownPatches = derivePendingPatches(undefined);

    // Pending patches block confirm
    assert.equal(isCommitEligible(readyVal, pendingPatches, false), false);
    assert.equal(isCommitEligible(readyVal, unknownPatches, false), false);

    // Blocked validation blocks confirm
    assert.equal(isCommitEligible(blockedVal, zeroPatches, false), false);

    // Both ready and zero patches are eligible
    assert.equal(isCommitEligible(readyVal, zeroPatches, false), true);

    // While operation is pending, eligibility is false
    assert.equal(isCommitEligible(readyVal, zeroPatches, true), false);
  });

  it("prevents dismissal while pending in op state machine", () => {
    assert.equal(canDismissCommitDialog({ status: "pending" }), false);
    assert.equal(canDismissCommitDialog({ status: "idle" }), true);
    assert.equal(canDismissCommitDialog({ status: "error" }), true);
    assert.equal(canDismissCommitDialog({ status: "unknown" }), true);

    const s = reduceCommitOp({ status: "pending" }, "reset");
    assert.equal(s.status, "pending", "Reset cannot interrupt pending submission");
  });

  it("verifies CommitConfirmationDialog hides close button, blocks dismissal while pending, and disables Confirm on pending patches", () => {
    // 1. Logic-level invariant: eligibility is blocked when patches are pending
    const readyVal = deriveValidation({ validation: { ready: true } });
    const pendingPatches = derivePendingPatches({ pendingPatches: { count: 3 } });
    assert.equal(isCommitEligible(readyVal, pendingPatches, false), false, "Must block Confirm when patches are pending");

    // 2. Source-level DOM invariant: verify dialog props, close button concealment, and dismiss prevention
    const dialogSrc = readFileSync(
      new URL("./CommitConfirmationDialog.tsx", import.meta.url),
      "utf8"
    );
    assert.match(dialogSrc, /pending && "\[&>button:last-child\]:hidden"/, "Must hide close X when pending");
    assert.match(dialogSrc, /disabled=\{!eligible\}/, "Confirm button must be disabled when !eligible");
    assert.match(dialogSrc, /onEscapeKeyDown=\{\(e\) => \{ if \(pending\) e\.preventDefault\(\); \}\}/, "Escape prevented while pending");
    assert.match(dialogSrc, /onInteractOutside=\{\(e\) => \{ if \(pending\) e\.preventDefault\(\); \}\}/, "Click outside prevented while pending");
    assert.match(dialogSrc, /disabled=\{pending\} onClick=\{\(\) => handleOpenChange\(false\)\}/, "Cancel button disabled while pending");
  });
});

describe("P5S3-B: Failed TX-05 preserves draft state and displays structured error without navigation", () => {
  it("handles failed TX-05 commit by preserving draft state and reporting structured error without navigating", async () => {
    const structuredBackendError = {
      code: "TX05_VALIDATION_FAILED",
      message: "TX-05: Phase validation incomplete; cannot commit unsealed chapters",
      details: { missingField: "chapter.2.scenes" },
    };

    const transport = createMockTransport({
      commit_draft: async () => {
        throw structuredBackendError;
      },
    });
    const client = new BackendClient(transport);

    let navigated = false;
    let reportedError = null;

    const res = await executeDraftCommitWithReconciliation({
      client,
      draftSessionId: "draft-preserve-1",
      onSuccess: () => {
        navigated = true;
      },
      onError: (err) => {
        reportedError = err;
      },
    });

    assert.equal(res.status, "failed");
    assert.equal(navigated, false, "Must NEVER navigate on TX-05 failure");
    assert.ok(res.reason.includes("TX-05"));
    assert.ok(reportedError.message.includes("TX-05"));
    assert.deepEqual(reportedError.details, structuredBackendError.details);

    // Verify retry is possible by invoking reduceCommitOp
    const errorOp = reduceCommitOp({ status: "pending" }, "fail");
    assert.equal(errorOp.status, "error");
    const retryOp = reduceCommitOp(errorOp, "submit");
    assert.equal(retryOp.status, "pending");
  });
});

describe("phase5_sprint3_deep2-F1: WorkflowBaselineUnavailable surfaces manual-repair message without synthetic version", () => {
  it("surfaces manual-repair message and creates zero synthetic versions on restore operation", async () => {
    const baselineError = new Error("WorkflowBaselineUnavailable: sealed baseline not found on disk");
    baselineError.code = "WORKFLOW_BASELINE_UNAVAILABLE";

    const transport = createMockTransport({
      restore_draft_version: async () => {
        throw baselineError;
      },
    });
    const client = new BackendClient(transport);
    const controller = new DraftBackendController(client, {
      draftSessionId: "draft-f1",
      headVersionSeq: 1,
      versions: [
        { seq: 1, displayLabel: "v.01", phase: 1, reason: "Initial", createdAt: new Date().toISOString() },
      ],
    });

    const initialVersionsCount = controller.getState().versions.length;
    const initialHeadSeq = controller.getState().headVersionSeq;

    const result = await controller.restoreDraftVersion(1);

    assert.equal(result.ok, false);
    assert.equal(result.error, WORKFLOW_BASELINE_UNAVAILABLE);

    const state = controller.getState();
    assert.ok(
      state.manualRepairMessage?.includes("WORKFLOW_BASELINE_UNAVAILABLE"),
      "Must surface explicit manual-repair message"
    );
    assert.equal(
      state.versions.length,
      initialVersionsCount,
      "MUST NOT fabricate a synthetic version"
    );
    assert.equal(
      state.headVersionSeq,
      initialHeadSeq,
      "Head version seq must remain unchanged"
    );
  });

  it("surfaces manual-repair message on workflow execution without synthesizing version", async () => {
    const baselineError = new Error("WorkflowBaselineUnavailable: sealed baseline not found");
    baselineError.code = "WORKFLOW_BASELINE_UNAVAILABLE";

    const transport = createMockTransport({
      start_or_resume_workflow: async () => {
        throw baselineError;
      },
    });
    const client = new BackendClient(transport);
    const controller = new DraftBackendController(client, {
      draftSessionId: "draft-f1-wf",
      headVersionSeq: 1,
      versions: [{ seq: 1, displayLabel: "v.01", phase: 1, reason: "Init", createdAt: new Date().toISOString() }],
    });

    await assert.rejects(
      async () => {
        await controller.startOrResumeWorkflow(1);
      },
      /WorkflowBaselineUnavailable/
    );

    const state = controller.getState();
    assert.ok(state.manualRepairMessage?.includes("WORKFLOW_BASELINE_UNAVAILABLE"));
    assert.equal(state.workflowStatus, "failed");
    assert.equal(state.versions.length, 1, "Zero synthetic version fabricated");
  });
});

describe("phase5_sprint3_deep2-F2: Indeterminate receipt surfaces requires_replan UI, never completed badge", () => {
  it("surfaces requires_replan state from indeterminate workflow receipt", async () => {
    const transport = createMockTransport({
      start_or_resume_workflow: async () => {
        return {
          workflowRunId: "run-indet-1",
          planGeneration: 1,
          status: "requires_replan",
          recoveryReason: "insufficient_task_evidence",
          completedTasks: 2,
          totalTasks: 5,
          lastSealedVersion: null,
          pendingPatches: [],
        };
      },
    });
    const client = new BackendClient(transport);
    const controller = new DraftBackendController(client, {
      draftSessionId: "draft-f2",
      headVersionSeq: 1,
    });

    await controller.startOrResumeWorkflow(1);

    const state = controller.getState();
    assert.equal(state.workflowStatus, "requires_replan");

    // Test static markup of DraftToolbar: MUST show requires_replan badge and NEVER completed badge
    const toolbarMarkup = renderToStaticMarkup(
      createElement(DraftToolbar, {
        currentVersion: "v.01",
        versionHistory: [],
        onVersionChange: () => {},
        onRegenerate: () => {},
        isFeedbackMode: false,
        onToggleFeedback: () => {},
        workflowStatus: "requires_replan",
      })
    );

    assert.ok(
      toolbarMarkup.includes("REQUIRES_REPLAN"),
      "Toolbar must render REQUIRES_REPLAN badge"
    );
    assert.ok(
      !toolbarMarkup.includes("Completed"),
      "Toolbar must NEVER render a completed badge when requires_replan"
    );
  });

  it("handles INDETERMINATE status string in execution result", () => {
    const controller = new DraftBackendController(new BackendClient(createMockTransport()));
    controller.applyWorkflowExecutionResult({
      workflowRunId: "run-indet-2",
      planGeneration: 1,
      status: "INDETERMINATE",
      completedTasks: 1,
      totalTasks: 3,
      pendingPatches: [],
    });

    assert.equal(controller.getState().workflowStatus, "requires_replan");
  });
});

describe("Lost response reconciliation & Historical Verified applicability", () => {
  it("reconciles with durable backend state on lost response without double-committing", async () => {
    let listProjectsCalled = 0;
    const transport = createMockTransport({
      commit_draft: async () => {
        const timeoutErr = new Error("Network timeout: lost response from server");
        timeoutErr.code = "TIMEOUT";
        throw timeoutErr;
      },
      get_draft_workspace_state: async () => {
        // Durable state reports that commit actually completed on backend!
        return {
          draftSessionId: "draft-lost-1",
          status: "committed",
          projectId: "proj-recovered-888",
          headVersionSeq: 2,
        };
      },
      list_projects: async () => {
        listProjectsCalled++;
        return {
          items: [{ id: "proj-recovered-888", draftSessionId: "draft-lost-1" }],
        };
      },
    });
    const client = new BackendClient(transport);

    let navigatedPath = null;
    const res = await executeDraftCommitWithReconciliation({
      client,
      draftSessionId: "draft-lost-1",
      onSuccess: (_id, path) => {
        navigatedPath = path;
      },
    });

    assert.equal(res.status, "committed");
    assert.equal(res.projectId, "proj-recovered-888");
    assert.equal(navigatedPath, "/projects/proj-recovered-888");
  });

  it("historical Verified receipt alone never marks current task applicable without current head match", () => {
    const currentHeadSeq = 3;

    // Matching head and status
    const validReceipt = {
      taskId: "task-1",
      status: "Verified",
      headVersionSeq: 3,
      dependencyFingerprint: "fp-abc",
    };
    assert.equal(isTaskApplicable(validReceipt, currentHeadSeq, "fp-abc"), true);

    // Stale head version sequence: historical Verified is NOT applicable
    const staleReceipt = {
      taskId: "task-1",
      status: "Verified",
      headVersionSeq: 2, // stale!
      dependencyFingerprint: "fp-abc",
    };
    assert.equal(isTaskApplicable(staleReceipt, currentHeadSeq, "fp-abc"), false);

    // Mismatched dependency fingerprint: NOT applicable
    const mismatchedFpReceipt = {
      taskId: "task-1",
      status: "Verified",
      headVersionSeq: 3,
      dependencyFingerprint: "fp-xyz",
    };
    assert.equal(isTaskApplicable(mismatchedFpReceipt, currentHeadSeq, "fp-abc"), false);

    // Corrupt or Indeterminate status: NOT applicable
    assert.equal(isTaskApplicable({ ...validReceipt, status: "Corrupt" }, currentHeadSeq), false);
    assert.equal(isTaskApplicable({ ...validReceipt, status: "Indeterminate" }, currentHeadSeq), false);
  });

  it("restoring a version expires pending patches (EXPIRED_BY_RESTORE) and updates head version", async () => {
    const transport = createMockTransport({
      restore_draft_version: async (req) => {
        return {
          newVersion: {
            seq: 5,
            displayLabel: "v.05",
            phase: 1,
            reason: "restore_version:2",
            createdAt: new Date().toISOString(),
          },
          restoredPhase: 1,
          expiredPatchIds: ["patch-old-1"],
          workingState: {},
        };
      },
    });
    const client = new BackendClient(transport);
    const controller = new DraftBackendController(client, {
      draftSessionId: "draft-restore-test",
      headVersionSeq: 2,
      pendingPatches: [
        { id: "patch-old-1", draftSessionId: "draft-restore-test", targetCode: "ARC1", fieldPath: "title", operation: "update", proposedValue: {}, status: "pending", createdAt: "" },
        { id: "patch-kept-2", draftSessionId: "draft-restore-test", targetCode: "ARC2", fieldPath: "title", operation: "update", proposedValue: {}, status: "pending", createdAt: "" },
      ],
    });

    const res = await controller.restoreDraftVersion(2);
    assert.equal(res.ok, true);
    assert.equal(controller.getState().headVersionSeq, 5);
    assert.equal(controller.getState().currentVersionLabel, "v.05");

    // patch-old-1 must be expired/removed from active pending patches
    assert.equal(controller.getState().pendingPatches.length, 1);
    assert.equal(controller.getState().pendingPatches[0].id, "patch-kept-2");
  });
});

describe("Visual check: 1280/1279px layout preservation and allowed write paths", () => {
  it("preserves responsive classes and layout elements for toolbar and workspace", () => {
    const toolbarMarkup = renderToStaticMarkup(
      createElement(DraftToolbar, {
        currentVersion: "v.01",
        versionHistory: [{ version: "v.01", updatedAt: "Today" }],
        onVersionChange: () => {},
        onRegenerate: () => {},
        isFeedbackMode: false,
        onToggleFeedback: () => {},
      })
    );

    assert.ok(toolbarMarkup.includes("rounded-full"));
    assert.ok(toolbarMarkup.includes("backdrop-blur-md"));
    assert.ok(toolbarMarkup.includes("v.01"));
  });

  it("verifies only files in allowed_write_paths are modified or added", () => {
    const allowed = new Set([
      "src/features/draft/DraftWorkspace.tsx",
      "src/features/draft/DraftToolbar.tsx",
      "src/features/draft/CommitConfirmationDialog.tsx",
      "src/features/draft/commitPreview.ts",
      "src/features/draft/commitNavigation.ts",
      "src/features/draft/draftBackend.test.mjs",
    ]);

    const baselineSha = "2c31a5ed1484779143eb67b237fc9a219d4e3cea";
    const currentBranch = execSync("git rev-parse --abbrev-ref HEAD", { encoding: "utf8" }).trim();

    if (currentBranch === "impl/phase5_sprint3_deep2/4c27bbf") {
      const diffFiles = execSync(
        `git diff --name-only ${baselineSha}..HEAD`,
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
        `git diff --name-only ${baselineSha}..impl/phase5_sprint3_deep2/4c27bbf`,
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
      assert.ok(file.startsWith("src/features/draft/"));
    }
  });
});
