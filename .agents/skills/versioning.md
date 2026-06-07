# Versioning

Use this skill for release version bumps and publish preparation.

## Release Flow

1. Update package and crate versions together.
2. Check proc-macro dependency versions explicitly.
3. Update the changelog for user-visible changes.
4. Regenerate bindings if generated files depend on the release state.
5. Run required Rust and Dart checks.
6. Commit with a release/version subject and tag the release.

## Hidden Mismatch Risk

Local workspace patching can hide published dependency mismatches. Always inspect the proc-macro dependency version in the core crate during a version bump, even if local builds pass.

The CLI and package version also need to stay aligned for generation. A stale installed CLI can produce confusing output against a newer app package.

## Release Notes

Focus release notes on user-visible changes: signal syntax, generation behavior, platform support, lifecycle changes, and build requirements. Avoid listing internal source moves unless users need to act on them.
