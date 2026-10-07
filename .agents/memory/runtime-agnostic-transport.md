---
description: Signal transport avoids Tokio channels so users may pick any async runtime; the template defaults to a current-thread Tokio runtime on purpose
---

Rinf moved its internal signal transport away from Tokio channels so the
framework is not tied to one async runtime. The template still uses Tokio by
default and binds the sample Rust entry point with
`#[tokio::main(flavor = "current_thread")]`, but users can choose a different
runtime as long as they keep the Dart shutdown future wired into their main
function.

The single-thread default is deliberate: Flutter already owns an event-driven
UI runtime, and a current-thread Rust runtime is usually enough for bridge
orchestration while using less memory. Multi-threaded or blocking work should
be an explicit application choice, especially on WASM, where shared memory and
atomics make the build and server headers much more sensitive
([[wasm-shared-memory-headers]]).

See [[cooperative-shutdown]].
