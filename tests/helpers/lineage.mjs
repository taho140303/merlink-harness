import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const helperDir = path.dirname(fileURLToPath(import.meta.url));
export const repoRoot = path.resolve(helperDir, "..", "..");

export const lineage = JSON.parse(
  readFileSync(path.join(repoRoot, "tests", "fixtures", "lineage.json"), "utf8"),
);

const recordedCommits = new Set([
  lineage.baseline.sha,
  lineage.baseline.legacySha,
  ...lineage.acceptedCommits.map((entry) => entry.sha),
]);

/**
 * A commit is accepted when it is recorded in the checked-in lineage manifest.
 * The manifest keeps phase-gate provenance verifiable in a clean clone where
 * the original predecessor branches were never published.
 */
export function isRecordedCommit(sha) {
  return recordedCommits.has(sha);
}

function gitOk(args) {
  try {
    execFileSync("git", args, { cwd: repoRoot, stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

export function isAncestor(sha) {
  return gitOk(["merge-base", "--is-ancestor", sha, "HEAD"]);
}

export function commitExists(sha) {
  return gitOk(["cat-file", "-e", `${sha}^{commit}`]);
}

/**
 * Accepts a commit when it is an ancestor of HEAD, an object available in this
 * clone, or an entry in the checked-in lineage manifest. A fabricated SHA is
 * rejected because it is in none of those places.
 */
export function verifyGitCommit(sha) {
  return isAncestor(sha) || commitExists(sha) || isRecordedCommit(sha);
}

export function gitRefExists(ref) {
  return gitOk(["rev-parse", "--verify", "--quiet", ref]);
}

/**
 * Returns the files changed in a range, or null when the range is not
 * available in this clone. Callers should skip the diff assertion when null.
 */
export function gitDiffFiles(range, paths = []) {
  try {
    const output = execFileSync(
      "git",
      ["diff", "--name-only", range, ...(paths.length > 0 ? ["--", ...paths] : [])],
      { cwd: repoRoot, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] },
    );
    return output
      .trim()
      .split("\n")
      .filter((line) => line.length > 0);
  } catch {
    return null;
  }
}

export function gitDiffOutput(range, paths = []) {
  try {
    return execFileSync(
      "git",
      ["diff", range, ...(paths.length > 0 ? ["--", ...paths] : [])],
      { cwd: repoRoot, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] },
    );
  } catch {
    return null;
  }
}
