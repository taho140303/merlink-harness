## What this changes

<!-- One paragraph: the problem, and the shape of the fix. -->

## How it was verified

<!-- Commands you ran, and what you exercised by hand. -->

- [ ] `pnpm test`
- [ ] `pnpm typecheck`
- [ ] `pnpm lint`
- [ ] `pnpm build`
- [ ] `cargo clippy --all-targets --locked` with no new warnings (in `src-tauri`)
- [ ] `cargo test --locked` (in `src-tauri`)

## Checklist

- [ ] Conventional Commit subject (`feat:`, `fix:`, `refactor:`, `docs:`, ...)
- [ ] New behaviour has a test that fails without the change
- [ ] Schema or seed changes append a new migration file and bump `MAX_SUPPORTED_SCHEMA_VERSION`
- [ ] User-visible copy lives in the i18n dictionary
- [ ] No secrets, local paths, `app-data/` or database files are staged
