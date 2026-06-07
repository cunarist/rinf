# AGENTS.md

## Project

- **Rinf** — Rust↔Flutter bridge via FFI/wasm
- Rust crate (`rust_crate`), proc macros (`rust_crate_proc`), CLI (`rust_crate_cli`), Flutter plugin (`flutter_package`)

## Rules

- Read `.agents/memory/INDEX.md` and `.agents/skills/INDEX.md` before acting
- **Actively maintain** memory and skills:
  - Create, update, or delete them whenever you learn something new
  - Even without explicit user command — if it matters, persist it
- Code changes must pass `cargo clippy --all-targets` + `dart analyze`
- Use conventional commits (`feat:`, `fix:`, `chore:`, `docs:`)

## Workflow

- **Signal changes:** Add `#[derive(SignalPiece)]` + `#[signal]`, run `rinf gen`, test target platform
- **Platform build issue:** Check `.agents/skills/platform-builds.md`
- **CI failure:** Clippy → Ruff → Dart analyzer, in that order
- **Dependabot PR:** Minor = auto-approve if CI green; Major = manual review

## Gotchas

- Web: no i64 in signals (use String/double)
- `rinf gen --watch` is broken (#682) — don't use it
- Serde `with` attribute is banned on signals
- Recursive types: use `Option<Box<T>>` pattern
