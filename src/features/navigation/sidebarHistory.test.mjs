import assert from "node:assert/strict";
import path from "node:path";
import { after, describe, it } from "node:test";
import { createServer } from "vite";

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: "custom",
  resolve: { alias: { "@": path.resolve("src") } },
  ssr: { external: ["react", "react-router", "react-dom"] },
});
after(async () => vite.close());

const { mapSessionSummaryToChatItem } = await vite.ssrLoadModule(
  "/src/features/navigation/components/SidebarHistory.tsx",
);

describe("Sidebar chat history", () => {
  it("maps persisted sessions to chat routes and safe display metadata", () => {
    const item = mapSessionSummaryToChatItem({
      sessionId: "12345678-abcd-4000-8000-000000000000",
      draftSessionId: "22345678-abcd-4000-8000-000000000000",
      projectId: null,
      status: "editing",
      updatedAt: "2026-11-08T18:00:00Z",
    });

    assert.equal(item.id, "12345678-abcd-4000-8000-000000000000");
    assert.equal(item.title, "Chat 123456");
    assert.equal(item.projectId, "");
    assert.ok(item.time);
  });
});
