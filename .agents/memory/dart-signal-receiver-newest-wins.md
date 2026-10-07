---
description: Dart-to-Rust receiving is FIFO but not broadcast; a new receiver clone supersedes the old one, whose `recv()` then returns `None`
---

Dart-to-Rust receiving is FIFO, but it is not broadcast. A receiver clone
becomes the active receiver, and the previous receiver resolves `recv()` to
`None`.

Treat `None` from `recv()` as "this receiver was superseded", not as a decoded
signal: stop that task or reacquire the receiver. Calling
`get_dart_signal_receiver()` from multiple tasks means only the newest consumer
remains live.
