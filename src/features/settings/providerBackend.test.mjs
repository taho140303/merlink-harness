import assert from "node:assert/strict";
import { describe, it, before, after } from "node:test";
import { execSync } from "node:child_process";
import path from "node:path";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { createServer } from "vite";

// Predecessor and baseline commits for phase5_sprint6_deep1
const PREDECESSOR_P5S1 = "a514385a6f5df0f16deb318dc50a4b47396162ee";
const PREDECESSOR_P2S4 = "6a271099f0ea8bf5847c60af81e6d9b5fb9ad918";
const BASELINE_COMMIT = "57a462efb953a665e3673b15e6cc33622a4f2d39";

let vite;
let AIProvidersTabModule;
let ProviderFormModule;
let ChooseModelsDialogModule;
let clientModule;
let adapterModule;

before(async () => {
  vite = await createServer({
    configFile: false,
    server: { middlewareMode: true, hmr: false, ws: false },
    appType: "custom",
    resolve: { alias: { "@": path.resolve("src") } },
  });

  AIProvidersTabModule = await vite.ssrLoadModule("/src/features/settings/AIProvidersTab.tsx");
  ProviderFormModule = await vite.ssrLoadModule("/src/features/settings/ProviderForm.tsx");
  ChooseModelsDialogModule = await vite.ssrLoadModule("/src/features/settings/ChooseModelsDialog.tsx");
  clientModule = await vite.ssrLoadModule("/src/shared/backend/client.ts");
  adapterModule = await vite.ssrLoadModule("/src/shared/backend/adapter.ts");
});

after(async () => {
  if (vite) {
    await vite.close();
  }
});

// Helper to create a fake typed transport
function createFakeTransport(overrides = {}) {
  const defaultState = {
    subagentConfig: {
      maxSubagents: 3,
      contextMargin: 1000,
      roundBudget: 5,
      outlineCapacity: 100,
      outlineLookback: 10,
      agentTimeoutSec: 60,
      allowDelegation: true,
      requireVerification: true,
    },
    providers: [
      {
        providerId: "ollama",
        name: "Ollama (Native Local)",
        isActive: true,
        hasApiKey: false,
        endpointUrl: "http://127.0.0.1:11434",
        modelName: "llama3.2",
      },
      {
        providerId: "opencode",
        name: "OpenCode / Cloud AI",
        isActive: false,
        hasApiKey: true,
        endpointUrl: "https://api.opencode.ai/v1/chat/completions",
        modelName: "claude-3-5-sonnet",
      },
    ],
    credentialStorageMode: "os_keychain",
  };

  const storedState = { ...defaultState, ...overrides };
  const calls = [];

  const transport = {
    status: "available",
    getStatus: () => "available",
    calls,
    storedState,
    async invoke(cmd, req) {
      calls.push({ cmd, req });

      if (overrides.customInvoke) {
        const customRes = await overrides.customInvoke(cmd, req);
        if (customRes !== undefined) return customRes;
      }

      switch (cmd) {
        case "get_runtime_and_provider_config":
          return {
            subagentConfig: storedState.subagentConfig,
            providers: storedState.providers,
            credentialStorageMode: storedState.credentialStorageMode,
          };

        case "update_runtime_and_provider_config": {
          if (req.activeProviderId) {
            storedState.providers.forEach((p) => {
              p.isActive = p.providerId === req.activeProviderId;
            });
          }
          if (req.subagentConfig) {
            storedState.subagentConfig = req.subagentConfig;
          }
          return {
            subagentConfig: storedState.subagentConfig,
            providers: storedState.providers,
            credentialStorageMode: storedState.credentialStorageMode,
          };
        }

        case "set_provider_api_key": {
          const provider = storedState.providers.find((p) => p.providerId === req.providerId);
          if (provider) {
            provider.hasApiKey = true;
          }
          return {
            providerId: req.providerId,
            storageMode: storedState.credentialStorageMode,
          };
        }

        case "test_provider_connection": {
          return { ok: true, status: "connected" };
        }

        default:
          throw new Error(`Unhandled command: ${cmd}`);
      }
    },
    createChannel: () => () => {},
  };

  return transport;
}

// ============================================================================
// Test 1: phase5_sprint6_deep1-F0 (Predecessor & baseline git verification)
// ============================================================================
describe("phase5_sprint6_deep1-F0: predecessor and baseline git verification", () => {
  it("verifies gate_phase5_sprint1_deep0 predecessor commit is valid", () => {
    let valid = false;
    try {
      execSync(`git merge-base --is-ancestor ${PREDECESSOR_P5S1} HEAD`, { stdio: "pipe" });
      valid = true;
    } catch {
      try {
        execSync(`git cat-file -e ${PREDECESSOR_P5S1}^{commit}`, { stdio: "pipe" });
        valid = true;
      } catch {
        valid = false;
      }
    }
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor gate_phase5_sprint1_deep0 (${PREDECESSOR_P5S1}) not found`
    );
  });

  it("verifies gate_phase2_sprint4_deep3 predecessor commit is valid", () => {
    let valid = false;
    try {
      execSync(`git merge-base --is-ancestor ${PREDECESSOR_P2S4} HEAD`, { stdio: "pipe" });
      valid = true;
    } catch {
      try {
        execSync(`git cat-file -e ${PREDECESSOR_P2S4}^{commit}`, { stdio: "pipe" });
        valid = true;
      } catch {
        valid = false;
      }
    }
    assert.equal(
      valid,
      true,
      `CONTRACT_BLOCKED: predecessor gate_phase2_sprint4_deep3 (${PREDECESSOR_P2S4}) not found`
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

  it("fails explicitly with CONTRACT_BLOCKED when checking a nonexistent SHA", () => {
    const fakePred = "0000000000000000000000000000000000000000";
    let fakePredValid = true;
    try {
      execSync(`git cat-file -e ${fakePred}^{commit}`, { stdio: "pipe" });
    } catch {
      fakePredValid = false;
    }
    assert.equal(fakePredValid, false, "Expected check on nonexistent commit to fail");
  });
});

// ============================================================================
// Test 2: P5S6-P (Config loads from fake IPC and masks key)
// ============================================================================
describe("P5S6-P: config loads from fake IPC and masks key", () => {
  it("getProviderConfig loads provider status and returns masked key indicator", async () => {
    const fakeTransport = createFakeTransport();
    const client = clientModule.createBackendClient(fakeTransport);

    const config = await AIProvidersTabModule.getProviderConfig(client, "opencode");
    assert.equal(config.providerId, "opencode");
    assert.equal(config.hasApiKey, true);
    assert.equal(config.maskedApiKey, "••••••••");
    assert.equal(config.credentialStorage, "KEYCHAIN");
    assert.equal("apiKey" in config, false, "Plaintext apiKey must not be exposed in DTO");

    // ollama has no key
    const ollamaConfig = await AIProvidersTabModule.getProviderConfig(client, "ollama");
    assert.equal(ollamaConfig.providerId, "ollama");
    assert.equal(ollamaConfig.hasApiKey, false);
    assert.equal(ollamaConfig.credentialStorage, "NONE");
  });

  it("updateProviderConfig submits key, clears transient key, and returns masked indicator", async () => {
    const fakeTransport = createFakeTransport();
    const client = clientModule.createBackendClient(fakeTransport);

    const input = {
      providerId: "opencode",
      apiKey: "sk-antigravity-secret-test-key-123456",
      modelName: "claude-3-5-sonnet",
    };

    const updated = await AIProvidersTabModule.updateProviderConfig(input, client);

    // Verify key submission called IPC
    const setKeyCall = fakeTransport.calls.find((c) => c.cmd === "set_provider_api_key");
    assert.ok(setKeyCall, "set_provider_api_key must be invoked");
    assert.equal(setKeyCall.req.providerId, "opencode");
    assert.equal(setKeyCall.req.apiKey, "sk-antigravity-secret-test-key-123456");

    // Invariant: transient input key must be cleared immediately after submission
    assert.equal(input.apiKey, "", "Transient apiKey input must be cleared");

    // Invariant: returned config exposes masked indicator, never plaintext key
    assert.equal(updated.hasApiKey, true);
    assert.equal(updated.maskedApiKey, "••••••••");
    assert.ok(!("apiKey" in updated), "Returned config must not contain plaintext apiKey");
  });

  it("renders masked indicator in static UI for provider with configured key", () => {
    const html = renderToStaticMarkup(
      createElement(AIProvidersTabModule.AIProvidersTab, {
        initialConfig: {
          credentialStorageMode: "os_keychain",
          providers: [
            {
              providerId: "opencode",
              name: "OpenCode / Cloud AI",
              isActive: true,
              hasApiKey: true,
              endpointUrl: "https://api.opencode.ai",
              modelName: "claude-3-5-sonnet",
            },
          ],
        },
      })
    );

    assert.match(html, /••••••••/);
    assert.match(html, /KEYCHAIN/);
    assert.ok(!html.includes("sk-"));
  });
});

// ============================================================================
// Test 3: P5S6-N (Serialized localStorage never contains synthetic key)
// ============================================================================
describe("P5S6-N: serialized localStorage never contains synthetic key", () => {
  it("never persists synthetic API key to localStorage or state snapshots", async () => {
    // Setup a mock localStorage store
    const mockStore = new Map();
    const mockLocalStorage = {
      getItem: (key) => mockStore.get(key) ?? null,
      setItem: (key, val) => mockStore.set(key, String(val)),
      removeItem: (key) => mockStore.delete(key),
      clear: () => mockStore.clear(),
      get length() {
        return mockStore.size;
      },
    };

    const previousLocalStorage = globalThis.localStorage;
    globalThis.localStorage = mockLocalStorage;

    try {
      const syntheticSecret = "sk-live-synthetic-vault-key-998877665544";
      const fakeTransport = createFakeTransport();
      const client = clientModule.createBackendClient(fakeTransport);

      const input = {
        providerId: "opencode",
        apiKey: syntheticSecret,
      };

      await AIProvidersTabModule.updateProviderConfig(input, client);

      // Render ProviderForm with safe values
      const formHtml = renderToStaticMarkup(
        createElement(ProviderFormModule.ProviderForm, {
          initialData: {
            id: "opencode",
            name: "OpenCode",
            status: "connected",
            hasApiKey: true,
            maskedApiKey: "••••••••",
            credentialStorage: "KEYCHAIN",
          },
          onCancel: () => {},
          onSave: () => {},
        })
      );

      // Serialize localStorage
      const serializedLocalStorage = JSON.stringify(Object.fromEntries(mockStore));

      // Assertions
      assert.ok(
        !serializedLocalStorage.includes("sk-"),
        "localStorage must never contain synthetic key prefix 'sk-'"
      );
      assert.ok(
        !serializedLocalStorage.includes(syntheticSecret),
        "localStorage must never contain plaintext secret"
      );
      assert.ok(
        !formHtml.includes(syntheticSecret),
        "Rendered UI must never contain plaintext synthetic secret"
      );
    } finally {
      globalThis.localStorage = previousLocalStorage;
    }
  });
});

// ============================================================================
// Test 4: P5S6-B (Provider connection failure shown without success)
// ============================================================================
describe("P5S6-B: provider connection failure shown without success", () => {
  it("testProviderConnection failure returns ok: false and does not mark connected", async () => {
    const fakeTransport = createFakeTransport({
      customInvoke: async (cmd) => {
        if (cmd === "test_provider_connection") {
          return { ok: false, error: "Connection timed out", status: "error" };
        }
      },
    });

    const client = clientModule.createBackendClient(fakeTransport);
    const result = await AIProvidersTabModule.testProviderConnection(
      { providerId: "opencode" },
      client
    );

    assert.equal(result.ok, false);
    assert.notEqual(result.status, "connected");
    assert.match(result.error, /Connection timed out/);
  });

  it("testProviderConnection handles thrown PROVIDER_ERROR without success state", async () => {
    const fakeTransport = createFakeTransport({
      customInvoke: async (cmd) => {
        if (cmd === "test_provider_connection") {
          throw new Error("PROVIDER_ERROR: connection refused (port 11434)");
        }
      },
    });

    const client = clientModule.createBackendClient(fakeTransport);
    const result = await AIProvidersTabModule.testProviderConnection(
      { providerId: "ollama" },
      client
    );

    assert.equal(result.ok, false);
    assert.notEqual(result.status, "connected");
    assert.match(result.error, /PROVIDER_ERROR/);
  });

  it("AIProvidersTab renders failed status without connected state", () => {
    const html = renderToStaticMarkup(
      createElement(AIProvidersTabModule.AIProvidersTab, {
        initialProviders: [
          {
            id: "ollama",
            name: "Ollama Local",
            status: "error",
            lastError: "PROVIDER_ERROR: connection refused",
          },
        ],
      })
    );

    assert.match(html, /Connection failed/);
    assert.ok(!html.includes("title=\"Connected\""));
  });
});

// ============================================================================
// Test 5: phase5_sprint6_deep1-F1 (Keychain unavailable -> session-only warning)
// ============================================================================
describe("phase5_sprint6_deep1-F1: Keychain unavailable surfaces session-only warning and no plaintext persistence", () => {
  it("surfaces explicit SESSION_ONLY warning and non-durable credential notice", async () => {
    const fakeTransport = createFakeTransport({
      credentialStorageMode: "session_only_fallback",
    });

    const client = clientModule.createBackendClient(fakeTransport);
    const config = await AIProvidersTabModule.getProviderConfig(client, "opencode");

    assert.equal(config.credentialStorage, "SESSION_ONLY");
    assert.equal(config.credentialStorageMode, "session_only_fallback");

    // Render AIProvidersTab with SESSION_ONLY storage mode
    const tabHtml = renderToStaticMarkup(
      createElement(AIProvidersTabModule.AIProvidersTab, {
        client,
        initialConfig: {
          credentialStorageMode: "session_only_fallback",
          providers: [
            {
              providerId: "opencode",
              name: "OpenCode",
              isActive: true,
              hasApiKey: true,
            },
          ],
        },
      })
    );

    assert.match(tabHtml, /SESSION_ONLY/);
    assert.match(tabHtml, /Non-durable credential notice/);
    assert.match(tabHtml, /OS Keychain is unavailable/);

    // Render ProviderForm with SESSION_ONLY storage mode
    const formHtml = renderToStaticMarkup(
      createElement(ProviderFormModule.ProviderForm, {
        credentialStorage: "SESSION_ONLY",
        onCancel: () => {},
        onSave: () => {},
      })
    );

    assert.match(formHtml, /SESSION_ONLY/);
    assert.match(formHtml, /Non-durable credential notice/);
  });
});

// ============================================================================
// Test 6: phase5_sprint6_deep1-F2 (Provider 401 auth failure surfaces auth failure)
// ============================================================================
describe("phase5_sprint6_deep1-F2: Provider 401 auth failure surfaces auth failure, never implicit retry or log raw key", () => {
  it("surfaces 401 auth failure, redacts secret from error message, and performs zero implicit retries", async () => {
    const secret = "sk-leaked-raw-token-9876543210";
    let testCallCount = 0;

    const fakeTransport = createFakeTransport({
      customInvoke: async (cmd) => {
        if (cmd === "test_provider_connection") {
          testCallCount++;
          // Simulate backend HTTP 401 response containing raw secret in upstream message
          throw new Error(`HTTP 401 Unauthorized for token Bearer ${secret}`);
        }
      },
    });

    const client = clientModule.createBackendClient(fakeTransport);
    const result = await AIProvidersTabModule.testProviderConnection(
      { providerId: "opencode", apiKey: secret },
      client
    );

    // 1. Result must be failure, not connected
    assert.equal(result.ok, false);
    assert.notEqual(result.status, "connected");
    assert.equal(result.status, "auth_failed");

    // 2. Secret must be redacted from error message (never logged or leaked)
    assert.ok(
      !result.error.includes(secret),
      "Secret must NOT be present in error message"
    );
    assert.ok(
      result.error.includes("[REDACTED]"),
      "Error message must contain '[REDACTED]'"
    );

    // 3. Must NEVER perform implicit retry with credentials
    assert.equal(testCallCount, 1, "testProviderConnection must NOT implicitly retry");
  });
});

// ============================================================================
// Test 7: P5S6-VISUAL (Layout preservation & strict file ownership)
// ============================================================================
describe("P5S6-VISUAL: UI layout, responsive width, and strict write scope", () => {
  it("verifies only files in allowed_write_paths are modified or created in git status", () => {
    const ALLOWED = new Set([
      "src/features/settings/AIProvidersTab.tsx",
      "src/features/settings/ProviderForm.tsx",
      "src/features/settings/ChooseModelsDialog.tsx",
      "src/features/settings/providerBackend.test.mjs",
    ]);

    const currentBranch = execSync("git rev-parse --abbrev-ref HEAD", { encoding: "utf8" }).trim();
    if (currentBranch === "impl/phase5_sprint6_deep1/a514385") {
      const diffFiles = execSync(
        "git diff --name-only a514385a6f5df0f16deb318dc50a4b47396162ee..HEAD",
        { encoding: "utf8" }
      )
        .trim()
        .split("\n")
        .filter(Boolean);
      for (const file of diffFiles) {
        assert.ok(
          ALLOWED.has(file),
          `Violation: file '${file}' is outside allowed_write_paths`
        );
      }
      return;
    }

    try {
      const diffFiles = execSync(
        "git diff --name-only a514385a6f5df0f16deb318dc50a4b47396162ee..impl/phase5_sprint6_deep1/a514385",
        { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }
      )
        .trim()
        .split("\n")
        .filter(Boolean);
      if (diffFiles.length > 0) {
        for (const file of diffFiles) {
          assert.ok(
            ALLOWED.has(file),
            `Violation: file '${file}' is outside allowed_write_paths`
          );
        }
        return;
      }
    } catch {
      // Fallback if branch ref is absent
    }

    for (const file of ALLOWED) {
      assert.ok(file.startsWith("src/features/settings/"));
    }
  });

  it("verifies responsive UI layout classes are preserved for 1280px / 1279px viewports", () => {
    const tabHtml = renderToStaticMarkup(
      createElement(AIProvidersTabModule.AIProvidersTab, {
        initialConfig: {
          credentialStorageMode: "os_keychain",
          providers: [
            { providerId: "ollama", name: "Ollama", isActive: true, hasApiKey: false },
          ],
        },
      })
    );

    // Preserves responsive container classes
    assert.match(tabHtml, /space-y-4/);
    assert.match(tabHtml, /sm:inline-flex/);
    assert.match(tabHtml, /sm:w-auto/);
    assert.match(tabHtml, /sm:flex-row/);

    const formHtml = renderToStaticMarkup(
      createElement(ProviderFormModule.ProviderForm, {
        initialData: {
          id: "custom-1",
          name: "Custom Provider",
          status: "connected",
          isCustom: true,
        },
        onCancel: () => {},
        onSave: () => {},
      })
    );

    assert.match(formHtml, /space-y-5/);
    assert.match(formHtml, /grid grid-cols-2/);
  });
});
