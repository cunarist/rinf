# Rinf Memory Index

This directory stores durable project memory: historical decisions, failure modes, and practical lessons that are not obvious from reading the source.

## Topics

- [architecture-decisions.md](architecture-decisions.md) — why major architecture choices were made, runtime ownership, symbol namespacing, and constraints they protect.
- [signal-system.md](signal-system.md) — SignalPiece boundaries, serde behavior, receiver semantics, stream state, and signal-design gotchas.
- [platform-notes.md](platform-notes.md) — platform-specific behavior for web/WASM, native loading, lifecycle, shutdown, and path/script hazards.
- [codegen-notes.md](codegen-notes.md) — `rinf gen` failure patterns, casing/version mismatch risks, path handling, and generation caveats.
- [contributors.md](contributors.md) — recurring contributor patterns, PR style, and areas external contributors often touch.

Read the relevant topic file before changing related behavior. Update these files when new project history or non-obvious operational knowledge is learned.
