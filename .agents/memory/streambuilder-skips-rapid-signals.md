---
description: `StreamBuilder`/`StreamProvider` may skip signals sent faster than one frame (about 16 ms); use `Stream.listen` to see every signal
---

Two Rust-to-Dart signals sent back to back lost the first one in the widget. The cause was not the bridge: widget-building stream consumers only rebuild with the latest event per frame. Handle all signals with `Stream.listen` and update state in the callback. A sleep between sends only masks it. Resolved with a docs guide on message interval (#469).

Evidence: #450, #469
