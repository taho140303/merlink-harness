# Refactor programme — Phase 1

Workflow: two independent reviewers per GoF group, then a synthesis that keeps the
best ideas, then one sprint per unit of work (one subagent per sprint, pushed).

## Skill

`../.agents/skills/design-patterns-gof/SKILL.md` (project-scoped install of
`caidanw/skills`, MIT-licensed upstream; read `SKILL.md` first, then the matching
file under `references/`).

## Report paths (one file per reviewer, never overwrite)

- `reports/creational/a.md`, `reports/creational/b.md`
- `reports/structural/a.md`, `reports/structural/b.md`
- `reports/behavioral/a.md`, `reports/behavioral/b.md`
- `synthesis/phase-0.md` — merged findings, ranked, with file:line evidence
- `sprints/` — the sprint specs produced from the synthesis

## Rules for reviewers

- Read the skill, then read the real source under `src/` and `src-tauri/src/`.
- Every finding needs `file:line`, a concrete failure/risk, and a pattern name
  from the skill (or an explicit "no pattern applies").
- Separate **bug** from **smell** from **over-abstraction**. Recommend deleting
  abstraction as loudly as adding it.
- No code changes in this phase — reports only.
