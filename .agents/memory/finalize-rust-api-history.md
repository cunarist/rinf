---
description: Shutdown API churn: 6.12 removed `finalizeRust`, 6.13 restored it, 8.3 made it block until the Rust runtime is dropped, and a non-blocking `settleRust` was proposed (#593)
---

6.12.0 (2024-06) removed `finalizeRust()`/`stopRustLogic` and relied on a thread-local shutdown sender (9bf6a7a6). Three weeks later 6.13.0 brought back a widget-managed `initializeRust` and `finalizeRust` (84893e75, 5fd1cbc5), with docs recommending `AppLifecycleListener.onExitRequested`. Lifecycle callbacks cannot always be relied on, so Rust `Drop` side effects must tolerate abrupt termination.

7.0 made the user's `main` bind to their own async runtime and await `rinf::dart_shutdown()`. 8.3.0 (a6141616) made `finalizeRust` block the Dart thread until the runtime is fully dropped. #591 reported a lint error from awaiting a function declared `void ... async`; #592 made the signature honest. #593 proposes an async `settleRust` (Rust notifies Dart over an Isolate after the drop) since blocking is a concern if shutdown takes more than a few ms; its status was not verified. Before this, `ensureFinalized()` (4.11.0) existed because Rust sending data after the Dart VM is gone caused memory errors, and tokio threads once lingered as a windowless process after closing the app (2.4.0, fixed with an `os_thread_local` runtime, 64648013).

See [[cooperative-shutdown]] and [[mobile-reopen-needs-channel-recreation]].

Evidence: #591, #592, #593, #594, commits 9bf6a7a6, 84893e75, 5fd1cbc5, a6141616, 2143821a, 64648013; CHANGELOG 2.4.0, 4.11.0, 6.12.0, 6.13.0, 8.3.0
