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
