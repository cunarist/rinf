# CI/CD

Use this skill when fixing failing checks, workflows, or dependency automation.

## Failure Order

Triage in this order unless the failure is obviously platform-specific:

1. Rust clippy.
2. Ruff for Python lint failures.
3. Ty for Python type-check failures, when Python automation is involved.
4. Dart analyzer.
5. Platform build workflows.

Generated bindings can affect both Rust and Dart checks, so regenerate before chasing analyzer noise that may come from stale Dart output.

## Local Checks

For behavior changes, run:

```bash
cargo clippy --all-targets
dart analyze flutter_package --fatal-infos
```

If signal definitions or generator behavior changed, run `rinf gen` for the example app before analysis.

## Common CI Patterns

- Rust warnings are treated strictly in CI.
- Formatting workflows may apply automated fixes and then require a clean diff.
- Python automation uses Ruff plus Ty.
- Web build failures often come from wasm toolchain or header assumptions, not Dart analyzer issues.
- The CLI crate is intentionally checked/installed with its own lock file in some workflows; do not assume workspace lock behavior covers CLI installation.

## Dependabot

Minor dependency bumps can usually be approved when CI is green.

Major dependency bumps need manual review for breaking changes and platform implications. Rust dependency bumps should get at least clippy coverage; platform-sensitive bumps need affected-platform build coverage when practical.
