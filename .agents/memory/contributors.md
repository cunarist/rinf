# Contributors

## Contribution Patterns

External PRs usually cluster around platform support, Cargokit/build integration, and toolchain compatibility. Treat these changes as high-value but high-regression-risk because they often touch platforms the primary maintainer may not be actively using.

Cargokit updates have a long history and should preserve upstream context. Avoid flattening or hiding upstream history during sync work unless the maintainer explicitly asks for that shape.

## Notable Areas

- Early Cargokit, Xcode, and CI work established much of the platform-build foundation.
- Windows ARM64 support came through external platform expertise and should be kept covered when touching Cargokit or target lists.
- Dart Native Assets work has existed as external exploration; review it as an integration direction, not as established behavior unless it has been merged.
- File-based endpoint and Dart analyzer fixes are recurring examples of contributor-driven polish.

## Style

Commit history often mixes conventional prefixes with concise maintenance messages. For new work, prefer conventional commits, keep the subject descriptive, and keep self-review plus Clippy/Dart analyzer checks in the loop for behavior changes.
