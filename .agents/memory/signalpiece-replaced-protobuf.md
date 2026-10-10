---
description: Rinf dropped Protobuf for the `SignalPiece` derive model because protoc and generated artifacts made the bridge too heavy
---

Rinf moved away from Protobuf because the build pipeline had too much weight
for the project shape: `protoc`, generated Rust/Dart artifacts, and
regeneration overhead made the bridge feel heavier than the runtime needs
justified.

The `SignalPiece` derive model keeps the user-facing workflow close to ordinary
Rust types and generates Dart bindings from Rust definitions. Keep bridge code
generation lightweight and predictable; Rinf does not need Protobuf-style
runtime reflection for its core messaging model.

History: Protobuf replaced MessagePack in 3.0.0 (PR #119, issue #106) for type safety without hand-installing protoc, but friction was constant: PATH not loaded for `protoc` and `protoc-gen-dart` under IDEs (hence [[codegen-stays-an-explicit-command]]), all `.proto` names in one namespace (#493), `cargo install` side effects (#366), and upstream majors breaking the 7.x line ([[cli-implicit-installs-break-users]]). Removal in 8.0 ended a long chain of toolchain friction, not a sudden reversal.

Evidence: #106, #119, #129, #366, #493, #542
