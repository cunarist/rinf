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

Origin: early issues asked not to spawn threads per task or use multithreading by default, so bridge tasks run on one thread with channels and the template's Tokio runtime is current-thread (#21, #73; worker pools removed in 50d9f169). Only the hub crate links into the binary. On web, async runtimes do not yield to JavaScript.

From 6.13 the bundled runtime is single-threaded (PR #372, #373) and multi-thread is the opt-in `rt-multi-thread` feature; backtraces are off unless the `backtrace` feature is on (#376). Engines such as Bevy should be driven from a loop that calls `app.update()`, sleeps, then `tokio::task::yield_now().await`, which also lets shutdown stop it (#349). `stop_rust_logic_extern` does nothing on web.

Evidence: #349, #372, #373, #376, #21, #73, commit 50d9f169
