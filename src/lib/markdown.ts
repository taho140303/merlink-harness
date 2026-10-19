export type MdInline =
  | { type: "text"; value: string }
  | { type: "strong"; children: MdInline[] }
  | { type: "em"; children: MdInline[] }
  | { type: "code"; value: string }
  | { type: "link"; label: MdInline[]; url: string };
export type MdBlock =
  | { type: "paragraph" | "heading" | "quote"; level?: number; children: MdInline[][] }
  | { type: "codeblock"; language: string; value: string }
  | { type: "list"; ordered: boolean; items: MdInline[][] }
  | { type: "hr" };

function inline(text: string): MdInline[] {
  const out: MdInline[] = [];
  const pattern = /\*\*([^*\n]+)\*\*|\*([^*\n]+)\*|`([^`\n]+)`|\[([^\]\n]+)\]\(([^)\s]+)\)/g;
  let pos = 0, match: RegExpExecArray | null;
  while ((match = pattern.exec(text))) {
    if (match.index > pos) out.push({ type: "text", value: text.slice(pos, match.index) });
    if (match[1] !== undefined) out.push({ type: "strong", children: inline(match[1]) });
    else if (match[2] !== undefined) out.push({ type: "em", children: inline(match[2]) });
    else if (match[3] !== undefined) out.push({ type: "code", value: match[3] });
    else {
      const url = match[5];
      try {
        const parsed = new URL(url, "https://invalid.local");
        if (["http:", "https:", "mailto:"].includes(parsed.protocol)) out.push({ type: "link", label: inline(match[4]), url });
        else out.push({ type: "text", value: match[0] });
      } catch { out.push({ type: "text", value: match[0] }); }
    }
    pos = pattern.lastIndex;
  }
  if (pos < text.length) out.push({ type: "text", value: text.slice(pos) });
  return out;
}

export function parseMarkdown(text: string): MdBlock[] {
  const lines = text.replace(/\r\n?/g, "\n").split("\n");
  const blocks: MdBlock[] = [];
  let i = 0;
  while (i < lines.length) {
    const line = lines[i];
    if (!line.trim()) { i++; continue; }
    if (/^```/.test(line)) {
      const language = line.slice(3).trim(); const body: string[] = []; i++;
      while (i < lines.length && !/^```\s*$/.test(lines[i])) body.push(lines[i++]);
      if (i < lines.length) i++;
      else { blocks.push({ type: "paragraph", children: [inline([line, ...body].join("\n"))] }); continue; }
      blocks.push({ type: "codeblock", language, value: body.join("\n") }); continue;
    }
    if (/^\s*---+\s*$/.test(line)) { blocks.push({ type: "hr" }); i++; continue; }
    const heading = /^(#{1,3})\s+(.+)$/.exec(line);
    if (heading) { blocks.push({ type: "heading", level: heading[1].length, children: [inline(heading[2])] }); i++; continue; }
    if (/^>\s?/.test(line)) {
      const quote: string[] = [];
      while (i < lines.length && /^>\s?/.test(lines[i])) quote.push(lines[i++].replace(/^>\s?/, ""));
      blocks.push({ type: "quote", children: quote.map(inline) }); continue;
    }
    const bullet = /^\s*[-*]\s+(.+)$/.exec(line), ordered = /^\s*\d+\.\s+(.+)$/.exec(line);
    if (bullet || ordered) {
      const isOrdered = !!ordered; const items: MdInline[][] = [];
      while (i < lines.length) {
        const m = isOrdered ? /^\s*\d+\.\s+(.+)$/.exec(lines[i]) : /^\s*[-*]\s+(.+)$/.exec(lines[i]);
        if (!m) break; items.push(inline(m[1])); i++;
      }
      blocks.push({ type: "list", ordered: isOrdered, items }); continue;
    }
    const para = [line]; i++;
    while (i < lines.length && lines[i].trim() && !/^(#{1,3}\s|```|>\s?|\s*[-*]\s+|\s*\d+\.\s+|\s*---+\s*$)/.test(lines[i])) para.push(lines[i++]);
    blocks.push({ type: "paragraph", children: para.map(inline) });
  }
  return blocks;
}
