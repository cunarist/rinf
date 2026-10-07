---
description: Research thoroughly before every change, even small ones; the Flutter, Rust, and small open-source ecosystems Rinf depends on change day by day
---

Rinf sits on Flutter, Rust, Cargokit, wasm tooling, CI runner images, and many
small open-source projects. Any of them can change without notice, so a fix
that was right last month can be wrong today.

Before any change, however small, check the current state instead of assuming
it: read the actual failure log, check the upstream release notes or issue
tracker, confirm the installed tool versions, and look at what the CI runner
image ships today.

Examples of breakage that came from outside the repository:

- `cargo install --locked wasm-pack` started needing a newer Rust than the
  declared minimum, because a transitive dependency raised its own minimum.
- The `windows-latest` runner image stopped providing a Visual Studio version
  that the minimum supported Flutter recognizes.
- Gradle 9 removed `project.exec`, breaking the vendored Cargokit plugin, and
  upstream Cargokit is archived.

See [[external-prs-are-platform-risk]].
