#!/usr/bin/env bash
# Sinh báo cáo: gộp Markdown nguồn -> PDF (XeLaTeX) và bản Markdown tự chứa.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/src"
BUILD="$ROOT/build"
OUT="$ROOT"
mkdir -p "$BUILD"

rm -f "$BUILD/combined.md"
for f in 00_title 00_mo_dau 01_chuong1 02_chuong2 03_chuong3 04_chuong4 05_ket_luan 06_tai_lieu_tham_khao; do
  cat "$SRC/$f.md" >> "$BUILD/combined.md"
  printf "\n\n" >> "$BUILD/combined.md"
done

python3 "$BUILD/md_build.py"

# Bản PDF qua XeLaTeX
pandoc "$BUILD/combined.md" \
  --from markdown+pipe_tables+implicit_figures \
  --pdf-engine=xelatex \
  --toc --toc-depth=3 \
  -V mainfont="Noto Serif" \
  -V sansfont="Noto Sans" \
  -V monofont="JetBrains Mono" \
  -V geometry:"a4paper,margin=2.3cm" \
  -V fontsize=12pt \
  -V linestretch=1.32 \
  -H "$BUILD/hdr.tex" \
  --lua-filter="$BUILD/br.lua" \
  --resource-path="$ROOT" \
  -o "$OUT/MERLINK_HARNESS_BAO_CAO.pdf"

echo "OK: $OUT/MERLINK_HARNESS_BAO_CAO.md + .pdf"
