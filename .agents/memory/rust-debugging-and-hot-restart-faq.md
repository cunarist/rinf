---
description: Rust breakpoints work when the app is launched from the IDE debug panel; Rust changes need a rebuild and hot restart does not reload Rust
---

Breakpoints in Rust work when the Flutter app is started from the IDE debug panel (for example VS Code), as a later maintainer comment in #223 confirmed. Rust code is not hot reloaded because the binary must be relinked; hot restart restarts Dart, and Rust logic only restarts after a rebuild (the Rust side must exit cooperatively, see [[cooperative-shutdown]]). On native platforms hot restart restarts the Rust `async fn main()` (the old runtime and tasks are dropped); on web it has no effect on Rust logic because queued tasks in the JavaScript event loop cannot be cancelled (#188, #264). Printing and Rust unit tests are the fallback. Upstream irondash/cargokit#41 is related.

Evidence: #223
