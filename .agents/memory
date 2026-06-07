# Rinf Memory

## Project Overview

**Rinf** — "Rust in Flutter". Rust for native business logic, Flutter for GUI.
Cross-platform bridge via FFI (native) and wasm-bindgen (web).

- **Repo:** https://github.com/cunarist/rinf
- **Current Version:** 8.10.0 (2026-03-09)
- **License:** MIT
- **Author:** Danny Kim (temeddix) — sole maintainer, ~2257 commits
- **Started:** 2022-04-11 (cargokit fork), 2023-07 rinf as own project
- **Dart SDK:** >=3.5.0 <4.0.0 | **Flutter:** >=3.24.0
- **Total tags:** 219 | **PRs:** 300+ | **Issues:** 98

## Repository Evolution

### Phase 1 — Genesis (2022-04 ~ 2023-06) · 14 commits
- Forked from `cargokit`; focused on build system infrastructure
- NDK/Gradle compatibility fixes, cross-platform setup
- Pre-versioned — no tags yet

### Phase 2 — Protobuf Era (2023-07 ~ 2023-09) · v1.0.0 → v3.0.0
- **v1.0.0** (2023-07-09): Official rinf launch, Protobuf-based messaging
- CLI `template`/`generate` commands for code scaffolding
- `.proto` file-driven endpoint system; file-based resource mapping
- Early contributor PRs (bookshiyi, wheregmis): Cargokit subtree, Xcode, CI
- Peak activity: 267 main commits in July 2023 alone

### Phase 3 — SignalPiece Revolution (2023-10 ~ 2024-07) · v4.0.0 → v6.x
- **v4.0.0** (2023-09-16): SignalPiece trait introduced — replacing Protobuf with native Rust derives
  - `#[derive(SignalPiece)]` + `#[signal]` attribute
  - Landing page revamp, framework identity (`"this as framework"`)
- **v5.0.0** (2024-01-08): `spawn_blocking` multicore parallelism, `debug_print`, panic backtrace
- **v6.0.0** (2024-01-22): Event-driven system, `RinfError` type, `initializeRust`/`finalizeRust` lifecycle, no-unwrap policy
- **Biggest gap:** No main commits Apr–May 2024 (likely real-life break)

### Phase 4 — Hardening & Expansion (2024-08 ~ 2025-06) · v7.0.0 → v8.7.x
- **v7.0.0** (2024-09-21): eLinux experimental support, Rust version bump, template overhaul, async runtime guides
- **v8.0.0** (2025-04-02): Major derive overhaul (`derive-signals`), Rust signal instance borrowing, serde_generate upgrade
- Recursive SignalPiece via `Option<Box<T>>` (PR #615)
- Serde attribute control: `#[serde(skip)]` respected (PR #617), `#[serde(with = "...")]` banned (PR #618)
- Windows ARM64 support (PR #650 by ONEUI8)
- Docs/demo containerized (PR #653)
- Clippy strict mode + Ruff ALL rules (PR #644, #652)
- Ty for Python type checking (PR #676)

### Phase 5 — Current (2025-07 ~ 2026-06) · v8.8 ~ v8.10
- **v8.8.0** (2025-07): Windows ARM64 merge, strict linters
- **v8.9.0** (2025-10): Cargokit update, lower requirements
- **v8.10.0** (2026-03-09): HarmonyOS/ohos support (PR #665), wasm-pack nightly fix
- Activity slowed: ~6 main commits/month — maintenance/stabilization phase
- Rust toolchain: 1.88 minimum

## Architecture

```
rinf/
├── rust_crate/        # Core — SignalPiece trait, channel, interface, macros
├── rust_crate_proc/   # Proc macros — derive SignalPiece
├── rust_crate_cli/    # CLI — `rinf gen`, scaffolding (separate Cargo.lock)
└── flutter_package/   # Flutter plugin — FFI bindings, platform channels
    ├── cargokit/      # Vendored build system (Android/iOS/macOS/Windows/Linux/ohos)
    └── example/       # Example app with native/Rust code
```

### Core Concepts

- **SignalPiece** — Types that cross Dart↔Rust. Derive + `#[signal]` attribute.
  - Supported: primitives, String, Vec<T: SignalPiece>, Option<T: SignalPiece>, enums
  - Recursive via `Option<Box<T>>` (added 2025-06, PR #615)
  - `#[serde(skip)]` respected (PR #617)
  - `#[serde(with = "...")]` banned (PR #618)
- **`rinf gen`** — CLI generates Dart code from Rust `#[signal]` structs
  - `--watch` broken — perpetual changes (#682)
  - Must match rinf version in pubspec (#565)
- **Cargokit** — Handles platform build integration (vendored, updated separately)
  - Platforms: Android, iOS, macOS, Linux, Windows, Web, ohos (HarmonyOS)
- **interface_os.rs** — Native FFI (C bindings)
- **interface_web.rs** — WASM via wasm-bindgen

## Source-Level Architecture Notes

### Workspace and Versioning

- Root `Cargo.toml` uses `resolver = "3"` and includes `flutter_package/example/native/*`, `rust_crate`, and `rust_crate_proc`.
- `rust_crate_cli` is deliberately excluded from the workspace so it owns a separate `Cargo.lock`; install it with `cargo install --path rust_crate_cli`.
- `[patch.crates-io]` redirects `rinf` and `rinf_proc` to local paths for example/native workspace builds.
- Current published versions in source are `rinf` 8.10.0, `rinf_proc` 8.10.0, `rinf_cli` 8.10.0, Flutter package 8.10.0.
- Rust crate has `rust-version = "1.88"` and edition 2024. CI uses Flutter 3.24.0 and Rust 1.88 for repository checks.
- Note: `rust_crate/Cargo.toml` currently depends on `rinf_proc = { version = "8.9.1" }`; local workspace patch hides this in-repo, but publication/version-bump work should verify this is intentional.

### `rust_crate/src/lib.rs` Public Surface

- Internal modules: `channel`, `error`, `macros`, `shutdown`, `signal_trait`, `traits`, plus platform-selected `interface_os`/`interface_web`.
- Public API re-exports:
  - channel: `signal_channel`, `SignalSender`, `SignalReceiver`
  - lifecycle/interface: `start_rust_logic`, `send_rust_signal`, `DartSignalPack`
  - shutdown: `dart_shutdown`
  - traits: `DartSignal`, `DartSignalBinary`, `RustSignal`, `RustSignalBinary`, `SignalPiece`
  - proc macros with the same trait names from `rinf_proc`
  - hidden bincode helpers `serialize`/`deserialize` for generated macro code.
- `write_interface!()` emits `rinf_start_rust_logic_extern`; user `hub` crate must define `main` in scope and call the macro once at crate root.
- `debug_print!()` sends endpoint `RinfOut` with UTF-8 report in binary bytes in debug builds; if no Dart isolate/bindings exist, it falls back to `println!`.

### Signal Traits and Supported Types

- `RustSignal`/`RustSignalBinary` require `serde::Serialize` and expose `send_signal_to_dart`.
- `DartSignal`/`DartSignalBinary` require `for<'a> Deserialize<'a>` and expose `get_dart_signal_receiver() -> SignalReceiver<DartSignalPack<Self>>`.
- `SignalPiece` is a compile-time capability marker; `be_signal_piece()` is a no-op used by proc-macro generated checks.
- Built-in `SignalPiece` impls cover signed/unsigned ints through 128-bit, floats, bool, char, `String`, `&str`, `Box<T>`, `Option<T>`, arrays, `Vec<T>`, `HashSet`, `BTreeSet`, `HashMap<K,V>`, `BTreeMap<K,V>`, unit and tuples up to arity 4.
- Web remains a practical limitation for `i64`/large integer signal fields even though Rust trait support exists; use `String` or `double` for cross-platform web-safe IDs/timestamps.

### Channel Semantics

- `SignalSender<T>` and `SignalReceiver<T>` share an `Arc<Mutex<SignalChannel<T>>>` containing a FIFO `VecDeque`, one stored `Waker`, and an active receiver id.
- Cloning a receiver makes the clone the active receiver and wakes the previous receiver; previous receivers resolve `recv()` to `None`.
- `recv()` returns `impl Future<Output = Option<T>>`; consumers must handle `None` as "receiver superseded" rather than as a decoded signal.
- Queue entries are not broadcast among receivers. For Dart-to-Rust signals, calling `get_dart_signal_receiver()` in multiple tasks means only the newest clone remains live.

### Native Interface (`interface_os.rs`)

- Dart registers `NativeApi.postCObject` and a send port via `rinf_prepare_isolate_extern`; Rust stores an `allo_isolate::Isolate` in `DART_ISOLATE`.
- `start_rust_logic_real` installs a debug panic hook, prepares an `os_thread_local::ThreadLocal<ShutdownDropper>`, then spawns a Rust thread for the user main function.
- Hot restart/reopen handling: spawned thread sets `dart_stopped`, clears `dart_stopped` and `rust_stopped`, runs user main, then sets `rust_stopped`.
- `rinf_stop_rust_logic_extern` sets `dart_stopped` and blocks until `rust_stopped`.
- Rust-to-Dart payload is posted as `[endpoint, messageBytesOrNull, binaryOrNull]`; empty vectors are sent as Dart null because `ZeroCopyBuffer` over empty vectors can panic. Dart reconstructs empty `Uint8List`.

### Web Interface (`interface_web.rs` and Flutter web loader)

- `start_rust_logic_real` runs the user main function synchronously in the JavaScript environment; there is no blocking shutdown mechanism on web.
- Rust-to-Dart calls JS function `rinfBindings.rinf_send_rust_signal_extern(endpoint, messageBytes, binary)` via `wasm_bindgen(catch)`. Failure maps to `AppError::NoBindings`, commonly seen in workers before globals exist.
- Flutter `load_web.dart` creates `globalThis.rinfBindings`, injects a module script for `pkg/hub.js` unless `compiledLibPath` overrides it, imports `wasmBindings`, awaits `init()`, then calls `completeRinfLoad`.
- `interface_web.dart` registers `rinf_send_rust_signal_extern` on `rinfBindings`; it calls `assignRustSignal[endpoint]!(messageBytes, binary)`.
- `startRustLogicReal()` skips invoking `rinf_start_rust_logic_extern` if `wasAlreadyLoaded`, supporting hot restart.

### Dart Native Interface (`flutter_package/lib/src/load_os.dart`)

- Default dynamic library names:
  - Linux/Android/ohos: `libhub.so`
  - Windows: `hub.dll`
  - iOS/macOS: `rinf.framework/rinf`
- `compiledLibPath` in `initializeRust` overrides dynamic library path before `rustLibrary` is used.
- `RustLibraryGlobal` uses `@Native` for fixed symbols (`start`, `stop`, `prepareIsolate`) when symbols are global, but still looks up dynamic Dart-signal endpoints and caches them.
- `RustLibraryLocal` uses `DynamicLibrary.lookupFunction` for all fixed and dynamic symbols. Android and ohos always use local symbols; Linux tests use local symbols when `FLUTTER_TEST` exists.
- Dart-to-Rust native send allocates and copies `messageBytes` and `binary` with `malloc`, calls the endpoint symbol, then frees both allocations.

### Proc Macro Behavior (`rust_crate_proc/src/lib.rs`)

- Derives provided: `SignalPiece`, `DartSignal`, `DartSignalBinary`, `RustSignal`, `RustSignalBinary`.
- All foreign signal type names are rejected if their lowercase form starts with `rinf`; this avoids collisions with framework-reserved endpoints like `RinfOut`.
- Generic signal types are rejected with `A foreign signal type cannot be generic`.
- Banned serde field attributes: `skip_serializing`, `skip_serializing_if`, `skip_deserializing`, `with`, `serialize_with`, `deserialize_with`, `flatten`.
- `#[serde(skip)]` is supported and removes the field/variant from compile-time `SignalPiece` checks and generation tracing.
- `DartSignal*` derives create:
  - lazy static `(SignalSender<DartSignalPack<T>>, SignalReceiver<DartSignalPack<T>>)`
  - `get_dart_signal_receiver()` returning a clone of the receiver
  - native extern `rinf_send_dart_signal_<snake_type>` or wasm export with the same name
  - bincode deserialize path that logs `CannotDecodeMessage` via `debug_print!` and drops invalid messages.
- `RustSignal*` derives bincode serialize `self`, then call `send_rust_signal(type_name, message_bytes, binary)`; serialization and transport errors are logged and swallowed.

### CLI Internals (`rust_crate_cli`)

- Binary entry is `rust_crate_cli/src/bin/main.rs` (not `src/main.rs`); it delegates to `rinf_cli::run_command()` and prints red `Error: ...` on failure.
- Commands in `src/tool/entry.rs`: `config`, `template`, `gen [--watch]`, `wasm [--release]`, `server [--release]`.
- CLI refuses to run in package directories where `pubspec.yaml` has `publish_to` other than `none`; missing/unreadable `publish_to` is treated as a Flutter app project.
- `RinfConfig` is loaded from `pubspec.yaml` key `rinf`; defaults are `gen_input_crates: ["hub"]` and `gen_output_dir: "lib/src/bindings"`.
- `template` copies embedded files from `rust_crate_cli/template`, strips `.template` suffixes, appends Rust/generated entries to `.gitignore`, appends README guidance, runs `dart pub add meta`, `dart pub add tuple`, rewrites `lib/main.dart` to import Rinf/bindings and call `await initializeRust(assignRustSignal)`, runs `cargo fmt`, then `rinf gen`.
- `gen` recursively parses `native/<crate>/src/**/*.rs` with `syn`, traces structs/enums with signal derives into `serde_reflection` formats, deletes/recreates the output dir, installs serde/bincode Dart runtimes, writes generated Dart classes, adds stream/latest-signal helpers and `sendSignalToRust` extensions, then writes `signal_handlers.dart` and top-level export.
- Type generation maps `Box<T>` to `T`, `Option<T>` to nullable option, `Vec`/sets to sequences, maps to `Format::Map`, arrays with literal length to tuple arrays, tuples to Dart tuple/format equivalents, and unknown paths to type names.
- Generation collects doc comments into generated Dart comments and ignores `#[serde(skip)]` fields/variants.
- Known source wart: in `extract_signal_attributes`, `"RustSignal"` currently maps to `SignalAttribute::RustSignalBinary`; this makes regular `RustSignal` classes treated as Rust-signal-capable, but source readers should verify binary/non-binary handling before changing that logic.
- `gen --watch` uses `notify` recursive watches on `native/<crate>` and regenerates on any `.rs` event. It is known broken (#682) and should be avoided.
- `wasm` installs nightly, `rust-src`, `wasm32-unknown-unknown`, `wasm-pack`, and `wasm-bindgen-cli` when internet is reachable, then runs `wasm-pack build native/hub --target web --out-dir web/pkg --out-name hub --no-typescript` with nightly, `-Zbuild-std=std,panic_abort`, atomics/shared-memory flags, 1 GiB max memory, imported memory, and TLS exports.
- `server` copies to clipboard a `flutter run` command with COOP/COEP headers required for shared-memory wasm.

### CI Details From Current Workflows

- `quality_control.yaml` sets `RUSTFLAGS=-D warnings`, installs CLI from `rust_crate_cli`, runs `rinf gen` in `flutter_package/example`, then clippy for debug/release native, debug/release wasm target, all features in `rust_crate`, and CLI clippy.
- Dart analysis command is `dart analyze flutter_package --fatal-infos`; formatting check runs `dart format .`, `cargo fmt`, and `cargo clippy --fix --allow-dirty`, then requires `git diff --exit-code`.
- Python checks are `uv run ty check` then `uv run ruff check .`.
- Example/test/user app workflows build Android, web, Linux, Windows, macOS, and iOS no-codesign; web path runs `rinf wasm --release` before `flutter build web`.

### Commit Activity

| Year | All Commits | Main Branch | Phase |
|------|------------|-------------|-------|
| 2022 | 14 | 1 | Genesis (pre-v1) |
| 2023 | 1112 | 660 | Protobuf → SignalPiece |
| 2024 | 964 | 312 | Hardening (v5-v7) |
| 2025 | 667 | 130 | Expansion (v8) |
| 2026 | 27 | 13 | Maintenance |

**Peak month:** July 2023 (267 main commits) — launch sprint
**Longest gap:** April–May 2024 (zero main commits)

### Top Commit Patterns (across all branches)
1. `Organize code` (66) — frequent refactoring passes
2. `Update test file` (28)
3. `Update docs` (20)
4. `Fix a typo` (19)
5. `Fix a sentence` (18)
6. `Add a todo comment` (18) — lots of planned-but-not-yet-done items
7. `Improve comments` (14), `Fix a comment` (15)

## Dependencies

- **Rust:** serde, serde_json, tokio, reqwest, bevy_ecs, wasm-bindgen
- **Dart:** flutter, collection, meta, flutter_lints
- **CLI:** clap, serde, serde_generate, cargo_metadata
- **Python:** automate/ — Ty type checker (#676), Ruff ALL rules (#652)
- **CI:** GitHub Actions, Docker, dependabot (47 PRs)

## Known Issues & Patterns

### Recurring Bug Patterns

| Pattern | Issues |
|---------|--------|
| Code gen produces invalid Dart | #600, #601, #632, #666 |
| `rinf gen` reads unrelated files | #630, #682 |
| Build failures per platform | #579, #605, #647, #649 |
| Serialization edge cases | #597, #613, #619, #672 |
| Enum/type mapping Rust↔Dart | #638, #666 |

### Active (Open Issues)

- **#682** — `rinf gen --watch` perpetual changes (critical)
- **#672** — Web: i64 fields silently fail deserialization
- **#641** — Dart Native Assets support (PR #681 open by robertmsale)
- **#638** — Exhaustive enum matching in Dart
- **#637** — HarmonyOS NEXT (partial via #665)
- **#635** — Switch away from `serde_yml`
- **#620** — Don't assume rustup installed
- **#581** — Docs: guide Rust initialization
- **#565** — `rinf gen` respect version in manifest

### Resolved Highlights

- OHOS support (#665 + #678)
- Windows ARM64 (#650)
- Recursive SignalPiece (#615, #613)
- Serde skip respected (#617)
- Clippy strict + Ruff ALL (#652, #644)
- Docs/demo containerized (#653)
- Ty for Python (#676)

## Build & CI

- **Rust:** Clippy strict, ALL Ruff rules for Python
- **Workflows:** Build test (all platforms), docs gen+host, demo container, linting
- **Docker:** Containerized docs + demo (#653)
- **wasm-pack:** nightly required for some builds (#678)

## Dev Conventions

- Commits: `feat:`, `fix:`, `chore:` + English/Korean
- PR: self-review + Clippy mandatory
- Dependabot: minor auto if CI green, major manual review
- Tests: build tests per platform (no unit test suite yet)
