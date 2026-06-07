# Architecture Decisions

## Protobuf to SignalPiece

Rinf moved away from Protobuf because the build pipeline had too much weight for the project shape: `protoc`, generated Rust/Dart artifacts, and regeneration overhead made the bridge feel heavier than the runtime needs justified.

The current `SignalPiece` derive model keeps the user-facing workflow closer to ordinary Rust types and lets Dart bindings be generated from Rust definitions. The lesson is to keep bridge code generation lightweight and predictable; Rinf does not need Protobuf-style runtime reflection for its core messaging model.

## CLI Independence

The CLI is intentionally versioned and installed independently from the core framework crates. This lets the tool evolve without forcing every framework dependency to move in lockstep.

The cost is version skew: if the installed `rinf gen` does not match the package version used by the Flutter app, generation can behave strangely. Whenever generation output looks inexplicable, check tool/package version alignment before debugging the AST logic.

## No Panic Across the Boundary

Rinf adopted a no-`unwrap`/`expect` posture because panic behavior is especially costly around FFI and WASM boundaries. Errors should be converted into project error types, logged, and swallowed when crossing the bridge would otherwise destabilize the host app.

For signal transport, a bad message should be dropped rather than killing the runtime thread. This is a deliberate reliability tradeoff: bridge errors are reported, but application lifecycles should remain intact.

## Runtime Ownership

Rinf moved its internal signal transport away from Tokio channels so the framework is not tied to one async runtime. The template still uses Tokio by default, and currently binds the sample Rust entry point with `#[tokio::main(flavor = "current_thread")]`, but users can choose a different runtime if they keep the Dart shutdown future wired into their main function.

The earlier single-thread default was deliberate: Flutter already owns an event-driven UI runtime, and a current-thread Rust runtime is usually enough for bridge orchestration while using less memory. Multi-threaded or blocking work should be an explicit application/runtime choice, especially on WASM where shared memory and atomics make the build and server headers much more sensitive.

## Symbol Namespacing

Rinf added a `rinf` prefix to exported symbols and moved `store_dart_post_cobject` setup into Rust to avoid symbol conflicts between multiple Flutter packages. User signal names beginning with `rinf` remain reserved; generator validation should only apply that reserved-name rule to actual signal types, not every Rust struct discovered while scanning source.
