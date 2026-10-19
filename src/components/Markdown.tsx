import type { ReactNode } from "react";
import { parseMarkdown, type MdBlock, type MdInline } from "../lib/markdown";

function inlines(nodes: MdInline[], key = "i"): ReactNode[] {
  return nodes.map((node, index) => {
    const k = `${key}-${index}`;
    if (node.type === "text") return node.value;
    if (node.type === "code") return <code key={k}>{node.value}</code>;
    if (node.type === "strong") return <strong key={k}>{inlines(node.children ?? [], k)}</strong>;
    if (node.type === "em") return <em key={k}>{inlines(node.children ?? [], k)}</em>;
    return <a key={k} href={node.url} target="_blank" rel="noreferrer noopener">{inlines(node.label, k)}</a>;
  });
}
function block(node: MdBlock, index: number) {
  const k = `b-${index}`;
  if (node.type === "hr") return <hr key={k} />;
  if (node.type === "codeblock") return <pre key={k}><code>{node.value}</code></pre>;
  if (node.type === "list") { const Tag = node.ordered ? "ol" : "ul"; return <Tag key={k}>{node.items.map((item, i) => <li key={i}>{inlines(item)}</li>)}</Tag>; }
  const content = node.children.map((line, i) => <span key={i}>{i > 0 && <br />}{inlines(line)}</span>);
  if (node.type === "quote") return <blockquote key={k}>{content}</blockquote>;
  if (node.type === "heading") { const Tag = (`h${node.level}`) as "h1" | "h2" | "h3"; return <Tag key={k}>{content}</Tag>; }
  return <p key={k}>{content}</p>;
}
export function Markdown({ content }: { content: string }) {
  return <div className="chat-markdown">{parseMarkdown(content).map(block)}</div>;
}
