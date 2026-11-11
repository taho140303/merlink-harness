import path from 'node:path';
import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { readFileSync } from "node:fs";
import { transformWithOxc } from "vite";

const source = readFileSync(path.resolve("src/features/chat/attachmentContent.ts"), "utf8");
const { code } = await transformWithOxc(source, "attachmentContent.ts");
const { serializeChatAttachments, splitChatAttachmentContent } = await import(`data:text/javascript,${encodeURIComponent(code)}`);

describe("chat attachments", () => {
  it("sends file contents to the model while showing only names in the user bubble", () => {
    const content = serializeChatAttachments("Read this", [{ name: "notes.md", content: "secret draft" }]);
    assert.match(content, /secret draft/);
    assert.deepEqual(splitChatAttachmentContent(content), { text: "Read this", names: ["notes.md"] });
  });

  it("keeps unrecognized user text visible", () => {
    const content = "Attached files (user supplied data):\ninvalid JSON";
    assert.deepEqual(splitChatAttachmentContent(content), { text: content, names: [] });
  });
});
