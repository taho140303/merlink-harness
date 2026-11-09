export interface ChatAttachment { name: string; content: string }

const ATTACHMENT_MARKER = "Attached files (user supplied data):\n";

export function serializeChatAttachments(prompt: string, files: ChatAttachment[]): string {
  if (!files.length) return prompt.trim();
  return `${prompt.trim()}\n\n${ATTACHMENT_MARKER}${JSON.stringify(files)}`.trim();
}

export function splitChatAttachmentContent(content: string): { text: string; names: string[] } {
  const markerIndex = content.lastIndexOf(ATTACHMENT_MARKER);
  if (markerIndex < 0) return { text: content, names: [] };
  try {
    const files: unknown = JSON.parse(content.slice(markerIndex + ATTACHMENT_MARKER.length));
    if (!Array.isArray(files) || !files.length ||
        files.some((file) => !file || typeof file.name !== "string" || typeof file.content !== "string")) {
      return { text: content, names: [] };
    }
    return { text: content.slice(0, markerIndex).trim(), names: files.map((file) => file.name) };
  } catch {
    return { text: content, names: [] };
  }
}
