---
description: Rust must listen for Dart shutdown and exit cooperatively; `finalizeRust()` matters on native, is a no-op on web, and hot restart relies on synced stopped flags
---

Native Rust logic should listen for Dart shutdown and exit cooperatively. Hot
restart relies on the Dart and Rust stopped flags staying synchronized.

`finalizeRust()` matters on native platforms because the Rust thread is
otherwise allowed to keep running. On web, finalization is effectively a no-op,
so web behavior is no proof that native shutdown is correct.

See [[runtime-agnostic-transport]].
