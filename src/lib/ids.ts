let fallbackCounter = 0;

function newUuid(): string {
  if (typeof crypto !== "undefined" && typeof crypto.randomUUID === "function") {
    return crypto.randomUUID();
  }

  const bytes = new Uint8Array(16);
  if (typeof crypto !== "undefined" && typeof crypto.getRandomValues === "function") {
    crypto.getRandomValues(bytes);
  } else {
    // Last resort: combine wall-clock time and a counter with random bytes.
    // Math.random alone could repeat IDs across rapid sends or sessions.
    let timestamp = Date.now();
    for (let index = 5; index >= 0; index -= 1) {
      bytes[index] = timestamp % 256;
      timestamp = Math.floor(timestamp / 256);
    }
    const sequence = ++fallbackCounter;
    for (let index = 9; index >= 6; index -= 1) {
      bytes[index] = (sequence >>> ((9 - index) * 8)) & 0xff;
    }
    for (let index = 10; index < 16; index += 1) {
      bytes[index] = Math.floor(Math.random() * 256);
    }
  }
  bytes[6] = (bytes[6] & 0x0f) | 0x40;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;
  const hex = Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}

export function newSessionId(): string {
  return `sess-${newUuid()}`;
}

export function newMessageId(prefix = "msg"): string {
  return `${prefix}-${newUuid()}`;
}
