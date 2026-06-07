# Versioning

Use this skill for release version bumps and publish preparation.

## Release Flow

1. Update package and crate versions together.
2. Check proc-macro dependency versions explicitly.
3. Preserve declared minimum toolchain requirements unless the release intentionally raises them.
4. Update the changelog for user-visible changes.
5. Regenerate bindings if generated files depend on the release state.
6. Run required Rust and Dart checks.
7. Commit with a release/version subject and tag the release.

## Hidden Mismatch Risk

Local workspace patching can hide published dependency mismatches. Always inspect the proc-macro dependency version in the core crate during a version bump, even if local builds pass.

The CLI and package version also need to stay aligned for generation. A stale installed CLI can produce confusing output against a newer app package.

The CLI crate is separated from the main workspace and has lock-file-sensitive install behavior in CI. When changing CLI dependencies or CI installation, verify both local workspace use and installed-CLI use.

## Toolchain Requirements

Current public requirements are Rust 1.88 for the Rust crate and Dart SDK `>=3.5.0 <4.0.0` for the Flutter package. History includes reverted bumps to keep these requirements clear, so do not introduce newer Rust or Dart syntax just because CI happens to use a newer toolchain.

## Release Notes

Focus release notes on user-visible changes: signal syntax, generation behavior, platform support, lifecycle changes, and build requirements. Avoid listing internal source moves unless users need to act on them.
