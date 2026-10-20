#!/usr/bin/env node
// Generates the code-keyed catalogue JSON used by the Rust `catalog` module.
//
// Source of truth: `docs/schema/seed/**/*.sql` (PostgreSQL seed scripts).
// Output: `docs/schema/catalog/*.json`, committed so the app never parses SQL.
//
// Usage:
//   node scripts/gen-catalog.mjs           # (re)generate
//   node scripts/gen-catalog.mjs --check    # exit 1 when committed files drift
//
// No dependencies: only `node:fs` and `node:path`.

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const SEED = path.join(ROOT, "docs", "schema", "seed");
const OUT = path.join(ROOT, "docs", "schema", "catalog");
const CHECK = process.argv.includes("--check");

const read = (relative) => fs.readFileSync(path.join(SEED, relative), "utf8");
const stripComments = (sql) => sql.replace(/--[^\n]*/g, "");

/**
 * Extracts every top-level tuple that follows a `VALUES` keyword.
 * Tracks parentheses, single-quoted strings (with `''` and backslash escapes)
 * so commas and parens inside Vietnamese text never split a tuple.
 */
function tuples(sql) {
  const text = stripComments(sql);
  const upper = text.toUpperCase();
  const found = [];
  let cursor = 0;

  for (;;) {
    const at = upper.indexOf("VALUES", cursor);
    if (at === -1) break;
    let i = at + "VALUES".length;

    for (;;) {
      while (i < text.length && /[\s,]/.test(text[i])) i += 1;
      if (i >= text.length || text[i] !== "(") break;

      const start = i;
      let depth = 0;
      let inString = false;
      let escaped = false;
      for (; i < text.length; i += 1) {
        const char = text[i];
        if (inString) {
          if (escaped) {
            escaped = false;
            continue;
          }
          if (char === "\\") {
            escaped = true;
            continue;
          }
          if (char === "'") {
            if (text[i + 1] === "'") {
              i += 1;
              continue;
            }
            inString = false;
          }
          continue;
        }
        if (char === "'") inString = true;
        else if (char === "(") depth += 1;
        else if (char === ")") {
          depth -= 1;
          if (depth === 0) {
            i += 1;
            break;
          }
        }
      }

      found.push(text.slice(start, i));
      while (i < text.length && /\s/.test(text[i])) i += 1;
      if (text[i] === ",") {
        i += 1;
        continue;
      }
      break;
    }

    cursor = i;
  }

  return found;
}

/** Splits one tuple body into ordered literals: quoted strings and raw tokens. */
function literals(tuple) {
  const body = tuple.slice(1, -1);
  const out = [];
  let i = 0;

  while (i < body.length) {
    while (i < body.length && /[\s]/.test(body[i])) i += 1;
    if (i >= body.length) break;

    if (body[i] === "'") {
      let value = "";
      i += 1;
      for (; i < body.length; i += 1) {
        const char = body[i];
        if (char === "\\") {
          value += body[i + 1] ?? "";
          i += 1;
          continue;
        }
        if (char === "'") {
          if (body[i + 1] === "'") {
            value += "'";
            i += 2;
            continue;
          }
          i += 1;
          break;
        }
        value += char;
      }
      out.push({ kind: "str", value });
    } else {
      let token = "";
      while (i < body.length && body[i] !== ",") {
        token += body[i];
        i += 1;
      }
      const trimmed = token.trim();
      if (trimmed) out.push({ kind: "raw", value: trimmed });
    }

    while (i < body.length && body[i] !== ",") i += 1;
    i += 1;
  }

  return out;
}

/** All quoted strings inside an `unnest(ARRAY[...])` block fragment. */
function arrayStrings(fragment) {
  const open = fragment.indexOf("ARRAY[");
  if (open === -1) return [];
  const close = fragment.indexOf("]", open);
  if (close === -1) return [];
  return literalsInto(fragment.slice(open + "ARRAY[".length, close));
}

function literalsInto(body) {
  return literals(`(${body})`)
    .filter((entry) => entry.kind === "str")
    .map((entry) => entry.value);
}

const str = (list, index) =>
  list[index] && list[index].kind === "str" ? list[index].value : null;
const castOf = (list, index) =>
  list[index]?.value ? list[index].value.replace(/::\w+$/, "") : null;
const bool = (list, index) => list[index]?.value === "true";

function genres() {
  return tuples(read("genres.sql")).map((tuple) => {
    const list = literals(tuple);
    return { code: str(list, 0), name: str(list, 1), description: str(list, 2) };
  });
}

function motifs() {
  return tuples(read("motifs.sql")).map((tuple) => {
    const list = literals(tuple);
    return {
      code: str(list, 0),
      name: str(list, 1),
      category: str(list, 2),
      description: str(list, 3),
    };
  });
}

function narratorConfigs() {
  return tuples(read("narrator_configs.sql")).map((tuple) => {
    const list = literals(tuple);
    return {
      code: str(list, 0),
      name: str(list, 1),
      person: castOf(list, 2),
      person_subtype: castOf(list, 3),
      pov_scope: castOf(list, 4),
      is_reliable: bool(list, 5),
      breaks_fourth_wall: bool(list, 6),
    };
  });
}

function narrativeDirectives() {
  return tuples(read("narrative_directives.sql")).map((tuple) => {
    const list = literals(tuple);
    return {
      code: str(list, 0),
      description: str(list, 1),
      category: castOf(list, 2),
      is_system: true,
    };
  });
}

function relationshipCategories() {
  return tuples(read("relationship_categories/_seed.sql"))
    .map((tuple) => {
      const list = literals(tuple);
      const category = str(list, 1);
      const subType = str(list, 2);
      if (!category || !subType) return null;
      // The table has no `code`; the catalogue key is derived so every entry is
      // addressable the same way as the other catalogues.
      return { code: `${category}.${subType}`, category, sub_type: subType, name: subType };
    })
    .filter(Boolean);
}

/**
 * The style-preset seed is a sequence of `DO $$` blocks, each preceded by
 * `-- preset: <code> | name: "<name>"` and followed (inside the block) by
 * vocab / focus / rhetorical-device inserts that all point at the same `pid`.
 */
function stylePresets() {
  const raw = read("style_presets/_seed.sql");
  const chunks = raw.split(/^-- preset: /m).slice(1);
  const presets = [];

  for (const chunk of chunks) {
    const header = chunk.slice(0, chunk.indexOf("\n"));
    const match = header.match(/^(\S+?)\s*\|\s*name:\s*"([^"]*)"/);
    const commentCode = match ? match[1] : null;
    const commentName = match ? match[2] : null;

    const valuesTuple = tuples(
      chunk.slice(0, chunk.indexOf("RETURNING id INTO pid")),
    )[0];
    if (!valuesTuple) continue;
    const list = literals(valuesTuple);
    // (uuid, code, category, tone_pace, tone_emotion, is_system,
    //  visual, auditory, tactile, olfactory, gustatory,
    //  language_abstract, syntax_sentence, syntax_variation, syntax_structure)
    const strings = list.filter((entry) => entry.kind === "str").map((entry) => entry.value);
    const code = strings[0] ?? commentCode;
    if (!code) continue;

    const vocabBlock = chunk.split(/INSERT INTO style_preset_vocab_items/)[1] ?? "";
    const focusBlock = chunk.split(/INSERT INTO style_preset_focus_items/)[1] ?? "";
    const deviceBlock = chunk.split(/INSERT INTO style_preset_rhetorical_devices/)[1] ?? "";
    const deviceValues = deviceBlock.indexOf("JOIN (VALUES");
    const devices =
      deviceValues === -1
        ? []
        : tuples(deviceBlock.slice(deviceValues)).map((tuple) => {
            const fields = literals(tuple);
            return {
              code: str(fields, 0),
              usage_type: str(fields, 1),
              custom_instruction: str(fields, 2),
            };
          });

    const unmapped = {};
    for (const line of chunk.split("\n")) {
      const note = line.match(/^-- unmapped\.(\S+):\s*(.*)$/);
      if (note) unmapped[note[1]] = note[2].trim();
    }

    presets.push({
      code,
      category: strings[1] ?? code,
      name: commentName || code,
      tone_pace: strings[2] ?? null,
      tone_emotion: strings[3] ?? null,
      is_system: list.some((entry) => entry.kind === "raw" && entry.value === "true"),
      visual: strings[4] ?? null,
      auditory: strings[5] ?? null,
      tactile: strings[6] ?? null,
      olfactory: strings[7] ?? null,
      gustatory: strings[8] ?? null,
      language_abstract: strings[9] ?? null,
      syntax_sentence: strings[10] ?? null,
      syntax_variation: strings[11] ?? null,
      syntax_structure: strings[12] ?? null,
      vocab: arrayStrings(vocabBlock),
      focus: arrayStrings(focusBlock),
      devices,
      notes: unmapped,
    });
  }

  return presets;
}

function build() {
  const catalogues = {
    genres: genres(),
    motifs: motifs(),
    narrator_configs: narratorConfigs(),
    narrative_directives: narrativeDirectives(),
    relationship_categories: relationshipCategories(),
    style_presets: stylePresets(),
  };

  for (const entries of Object.values(catalogues)) {
    for (const entry of entries) {
      if (!entry.code) {
        throw new Error(`Catalogue entry without a code: ${JSON.stringify(entry)}`);
      }
    }
    entries.sort((a, b) => String(a.code).localeCompare(String(b.code)));
  }

  return catalogues;
}

function serialise(catalogues) {
  const files = {};
  for (const [name, entries] of Object.entries(catalogues)) {
    files[`${name}.json`] = `${JSON.stringify(entries, null, 2)}\n`;
  }
  return files;
}

function main() {
  let catalogues;
  try {
    catalogues = build();
  } catch (error) {
    console.error(`gen-catalog: ${error.message}`);
    process.exit(1);
  }

  const files = serialise(catalogues);
  let drifted = false;

  for (const [name, contents] of Object.entries(files)) {
    const target = path.join(OUT, name);
    if (CHECK) {
      const current = fs.existsSync(target) ? fs.readFileSync(target, "utf8") : null;
      if (current !== contents) {
        console.error(`drift: docs/schema/catalog/${name}`);
        drifted = true;
      }
      continue;
    }
    fs.mkdirSync(OUT, { recursive: true });
    fs.writeFileSync(target, contents);
    console.log(`${name.padEnd(28)} ${catalogues[name.replace(/\.json$/, "")].length} entries`);
  }

  if (CHECK) {
    if (drifted) {
      console.error("gen-catalog: committed catalogue is stale; run `node scripts/gen-catalog.mjs`");
      process.exit(1);
    }
    console.log("gen-catalog: committed catalogue is up to date");
  }
}

main();
