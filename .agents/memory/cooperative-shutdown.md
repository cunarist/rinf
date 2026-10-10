---
description: Rust must listen for Dart shutdown and exit cooperatively; `finalizeRust()` matters on native, is a no-op on web, and hot restart relies on synced stopped flags
---

Native Rust logic should listen for Dart shutdown and exit cooperatively. Hot
restart relies on the Dart and Rust stopped flags staying synchronized.

`finalizeRust()` matters on native platforms because the Rust thread is
otherwise allowed to keep running. On web, finalization is effectively a no-op,
so web behavior is no proof that native shutdown is correct.

See [[runtime-agnostic-transport]].

History: tokio threads once stayed alive as a windowless process after the window closed (2.4.0); `ensureFinalized()` arrived in 4.11.0 because Rust sending after the Dart VM is gone caused memory errors. On web, hot restart once spawned `main` twice until an `IS_MAIN_STARTED` guard (4.11.0). API churn since then is in [[finalize-rust-api-history]], and mobile reopen behavior in [[mobile-reopen-needs-channel-recreation]].

Evidence: commits 64648013, 2143821a, a1f06684, 19b78445; CHANGELOG 2.4.0, 2.5.0, 4.11.0
