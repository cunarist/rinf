# Cargokit

Use this skill when updating or reviewing vendored Cargokit changes.

## Principles

Cargokit is build infrastructure, so small-looking changes can affect many platforms. Preserve upstream context and review platform impact explicitly.

Do not flatten upstream history casually. Past sync work taught that losing context makes later platform regressions harder to explain and revert.

## Update Flow

1. Check upstream for relevant changes and platform notes.
2. Sync using the established vendoring/subtree pattern for the repository.
3. Keep the update isolated from unrelated Rinf refactors.
4. Test or request coverage for affected platforms.
5. Call out platform coverage gaps in the PR or commit notes.

## Review Focus

- Android/Gradle wrapper and NDK assumptions.
- Apple framework and pod integration behavior.
- Linux and Windows dynamic-library naming/loading behavior.
- Windows ARM64 target handling.
- ohos build directory and SDK assumptions.

If a Cargokit update also changes generated bindings or signal behavior, split those changes unless they are inseparable.
