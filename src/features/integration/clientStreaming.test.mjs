import assert from "node:assert/strict";
import { after, test } from "node:test";
import { createServer } from "vite";

const vite = await createServer({
  configFile: false,
  server: { middlewareMode: true, watch: null, hmr: false, ws: false },
  appType: "custom",
});
after(async () => { delete globalThis.window; await vite.close(); });
const { BackendClient, BackendUnavailableError } = await vite.ssrLoadModule("/src/shared/backend/client.ts");

const commands = [
  ["sendChatMessage", "send_chat_message"],
  ["startOrResumeWorkflow", "start_or_resume_workflow"],
  ["startSceneGeneration", "start_scene_generation"],
];

test("all streaming commands attach a channel even without a callback", async () => {
  for (const [method, command] of commands) {
    let sent;
    const channel = { onEvent() {} };
    const client = new BackendClient({
      status: "available", getStatus: () => "available",
      createChannel: (callback) => { assert.equal(typeof callback, "function"); return channel; },
      invoke: async (name, payload) => { sent = [name, payload]; return {}; },
    });
    await client[method]({ marker: command });
    assert.equal(sent[0], command);
    assert.equal(sent[1].channel, channel);
    assert.doesNotThrow(() => channel.onEvent());
  }
});

test("native transport rejects unavailable Channel bridge instead of fabricating ID", async () => {
  globalThis.window = { __TAURI_INTERNALS__: { invoke: async () => ({}) } };
  for (const [method] of commands) {
    await assert.rejects(new BackendClient()[method]({}), BackendUnavailableError);
  }
});

test("native transport sends real Channel as onEvent separate from req", async () => {
  let invoked;
  globalThis.window = {
    __TAURI_INTERNALS__: {
      transformCallback: () => 123,
      invoke: async (command, args) => { invoked = [command, args]; return {}; },
    },
  };
  for (const [method, command] of commands) {
    const events = [];
    await new BackendClient()[method]({ marker: 7 }, (event) => events.push(event));
    assert.equal(invoked[0], command);
    assert.deepEqual(invoked[1].req, { marker: 7 });
    assert.equal(typeof invoked[1].onEvent.onmessage, "function");
    invoked[1].onEvent.onmessage("chunk");
    assert.deepEqual(events, ["chunk"]);
  }
});
