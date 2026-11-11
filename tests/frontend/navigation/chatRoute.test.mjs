import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { describe, it } from "node:test";

const routeSource = readFileSync("src/app/routes/chat.tsx", "utf8");
const pageSource = readFileSync("src/pages/ChatPage.tsx", "utf8");

describe("Chat route session restoration", () => {
  it("passes the URL session id through route and page to ChatView", () => {
    assert.match(routeSource, /const\s*\{\s*id\s*\}\s*=\s*useParams\(\)/);
    assert.match(routeSource, /<ChatPage\s+sessionId=\{id\}\s*\/>/);
    assert.match(pageSource, /<ChatView\s+sessionId=\{sessionId\}\s*\/>/);
  });
});
