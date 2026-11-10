#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Dựng bản Markdown tự chứa từ combined.md: bìa, mục lục, hình có chú thích."""
import re
import sys
import pathlib

root = pathlib.Path(__file__).resolve().parent.parent
combined = (root / "build" / "combined.md").read_text(encoding="utf-8")

# --- tách YAML front matter
meta = {}
body = combined
m = re.match(r"^---\n(.*?)\n---\n", combined, re.S)
if m:
    for line in m.group(1).splitlines():
        if ":" in line and not line.startswith((" ", "\t")):
            k, v = line.split(":", 1)
            meta[k.strip()] = v.strip().strip('"')
    body = combined[m.end():]

title = meta.get("title", "BÁO CÁO")
subtitle = meta.get("subtitle", "")
author = meta.get("author", "")
date = meta.get("date", "")

out = []
out.append(f"<div align=\"center\">\n\n# {title}\n\n### {subtitle}\n\n**{author}**\n\n*{date}*\n\n</div>\n")
out.append("\n---\n")

# --- mục lục dạng danh sách (không phụ thuộc anchor)
toc = []
for line in body.splitlines():
    hm = re.match(r"^(#{1,3})\s+(.*)$", line)
    if hm:
        level = len(hm.group(1))
        text = hm.group(2).strip()
        indent = "  " * (level - 1)
        toc.append(f"{indent}- {text}")
out.append("\n## MỤC LỤC\n")
out.append("\n".join(toc))
out.append("\n\n---\n")

# --- thân bài, thêm số hiệu và chú thích cho hình
fig_no = 0


def fig_repl(match):
    global fig_no
    fig_no += 1
    alt = match.group(1).strip()
    src = match.group(2).strip()
    return (
        "<figure>\n"
        f'  <img src="{src}" alt="{alt}" width="100%" />\n'
        f"  <figcaption><em>Hình {fig_no}. {alt}</em></figcaption>\n"
        "</figure>"
    )


body = re.sub(r"!\[([^\]]*)\]\(([^)]+)\)(?:\{[^}]*\})?", fig_repl, body)
out.append(body)

dest = root / "MERLINK_HARNESS_BAO_CAO.md"
dest.write_text("\n".join(out).replace("\n\n\n\n", "\n\n\n"), encoding="utf-8")
print(f"MD OK: {dest} ({fig_no} hình)")
