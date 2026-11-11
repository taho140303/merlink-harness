#!/usr/bin/env node
// Keep the three version sources and the git tag in sync.
//
// Merlink Harness versions live in package.json (source of truth), tauri.conf.json
// (what the bundler stamps into the app) and src-tauri/Cargo.toml (what the crate
// reports). A mismatch is silent at build time but produces releases whose tag,
// artwork filenames and "About" dialog disagree, so CI and the release workflow both
// run this check before anything is built.

import { readFileSync } from "node:fs";

const root = new URL("..", import.meta.url);

function readJson(path) {
  return JSON.parse(readFileSync(new URL(path, root), "utf8"));
}

function cargoVersion() {
  const manifest = readFileSync(new URL("src-tauri/Cargo.toml", root), "utf8");
  let inPackage = false;
  for (const line of manifest.split("\n")) {
    const section = line.match(/^\s*\[(.+)\]\s*$/);
    if (section) {
      inPackage = section[1].trim() === "package";
      continue;
    }
    if (!inPackage) continue;
    const version = line.match(/^\s*version\s*=\s*"([^"]+)"/);
    if (version) return version[1];
  }
  return null;
}

const pkg = readJson("package.json");
const tauri = readJson("src-tauri/tauri.conf.json");
const cargo = cargoVersion();

const problems = [];
const compares = [
  ["package.json", pkg.version],
  ["src-tauri/tauri.conf.json", tauri.version],
  ["src-tauri/Cargo.toml", cargo],
];

if (!compares[0][1]) problems.push("package.json has no version field");
for (const [label, value] of compares) {
  if (value !== compares[0][1]) {
    problems.push(`${label} is ${JSON.stringify(value)}, expected ${JSON.stringify(compares[0][1])}`);
  }
}

const tag = process.env.GITHUB_REF_NAME;
if (tag && tag.startsWith("v")) {
  const expected = `v${compares[0][1]}`;
  if (tag !== expected) {
    problems.push(`git tag ${tag} does not match the package version ${expected}`);
  }
}

if (problems.length > 0) {
  console.error("version check failed:");
  for (const problem of problems) console.error(`  - ${problem}`);
  process.exit(1);
}

console.log(`version ${compares[0][1]} is consistent across package.json, tauri.conf.json and Cargo.toml${tag ? ` (tag ${tag})` : ""}`);
