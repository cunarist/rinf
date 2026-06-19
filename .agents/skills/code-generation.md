# Code Generation

Use this skill for `rinf gen` and generated Dart binding problems.

## Running Generation

Run one-shot generation from the Flutter app that owns the `rinf` configuration:

```bash
rinf gen
```

Avoid watch mode unless it has been explicitly fixed and revalidated; it has a history of infinite regeneration.

## Configuration Expectations

Generation reads the app configuration from `pubspec.yaml`. If no generated classes appear, first confirm you are in a Flutter app root and that the package is treated as an app rather than a publishable package.

If generation output looks inconsistent with the current Rust code, check CLI/package version alignment before debugging parser behavior.

## Common Failures

- `DuplicatedSignal`: two signal-derived Rust types share the same generated endpoint name. Rename or isolate the DTOs.
- `CodeSyntax`: one Rust source file could not be parsed by the generator. Reduce newer or complex syntax near signal definitions and rerun.
- Missing Dart sender: the Rust type is probably only a `SignalPiece`; use the Dart-to-Rust signal derive for endpoint types.
- Missing Dart stream/latest field: use the Rust-to-Dart signal derive for endpoint types.
- Malformed Dart with extra braces: simplify trailing-comma or enum/struct edge cases until there is a small reproducer.
- Generator reads unrelated files: ignore package-manager or build metadata rather than reshaping valid signals.
- Reserved-name errors on ordinary helper structs: signal-name validation should only apply to actual signal endpoint/piece types.
- Wrong Dart casing: verify behavior against `serde-reflection`/`serde-generate` case conversion before adding local conversion rules.

## Review Generated Dart

Check that Dart analyzer passes, Rust-to-Dart assignment contains every endpoint, and Dart-to-Rust senders call the expected endpoint names.

For enum output, keep an eye on fallback behavior. Dart exhaustiveness has historically been a source of generated-code bugs.

For web, inspect and test signal schemas with 64-bit or wider integer fields even if generation succeeds.

## Updating Generation Logic

When adding support for a new Rust type shape, update the trait capability, proc-macro validation, generator type mapping, and an example signal together. Then regenerate and analyze the Flutter example before considering the change complete.

Be careful around binary versus non-binary Rust-to-Dart signal attribute handling. Generate examples of both paths before refactoring shared extraction logic.

Keep path handling structured. Past fixes covered nested folders, configured crates, custom generated-output paths, offline commands, and `pubspec.yaml` discovery; avoid string path rewrites that only work from one working directory.
