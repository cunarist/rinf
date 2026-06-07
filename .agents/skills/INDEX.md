# Rinf Agent Skills

## Rust Signal Development

### Adding a New Signal Type

```rust
use serde::{Deserialize, Serialize};
use rinf::{DartSignal, RustSignal, SignalPiece};

#[derive(Serialize, Deserialize, SignalPiece)]
pub struct MyData {
  pub id: u64,
  pub name: String,
  pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, DartSignal)]
pub struct StartJob {
  pub job: MyData,
}

#[derive(Serialize, Deserialize, RustSignal)]
pub struct JobProgress {
  pub id: u64,
  pub percent: f64,
}
```

**Rules:**
- Nested data derives `SignalPiece`; Dart→Rust endpoints derive `DartSignal`/`DartSignalBinary`; Rust→Dart endpoints derive `RustSignal`/`RustSignalBinary`
- Also derive matching Serde traits: `Deserialize` for Dart-originating signals, `Serialize` for Rust-originating signals; most shared types derive both
- All fields must be `pub`
- Supported field types in current source: primitives through 128-bit ints, floats, bool, char, String, &str, Box, Option, arrays, Vec, HashSet, BTreeSet, HashMap, BTreeMap, unit, tuples up to 4, and local structs/enums that derive `SignalPiece`
- Generic signal types are rejected by proc macros
- Names whose lowercase form starts with `rinf` are rejected; keep `Rinf*` reserved for framework internals
- **Not supported:** `#[serde(with = "...")]`, `serialize_with`, `deserialize_with`, `flatten`, `skip_serializing`, `skip_serializing_if`, `skip_deserializing`
- `#[serde(skip)]` is supported; skipped fields are excluded from compile-time SignalPiece checks and Dart generation
- Recursive types supported via `Option<Box<T>>` pattern; the CLI flattens `Box<T>` to `T` for generated type format

### Post-Add Checklist

- [ ] Run `cargo clippy --all-targets` — must pass
- [ ] Install/use local CLI if changing generation: `cargo install --path rust_crate_cli`
- [ ] Run `rinf gen` in `flutter_package/example` or the target Flutter app — verify generated Dart compiles
- [ ] Run `dart analyze flutter_package --fatal-infos` from repo root
- [ ] Test on target platform(s): `flutter run -d <platform>`
- [ ] If web target: verify no i64 fields (known limitation #672)

### Receiving Dart Signals in Rust

Only the most recently cloned receiver remains active. Treat `None` as "this receiver was superseded."

```rust
use rinf::{DartSignal, dart_shutdown};

async fn listen_for_jobs() {
  let receiver = StartJob::get_dart_signal_receiver();
  loop {
    tokio::select! {
      _ = dart_shutdown() => break,
      signal = receiver.recv() => {
        let Some(signal) = signal else {
          break;
        };
        let job = signal.message.job;
        let binary = signal.binary;
        // handle job and optional binary bytes
      }
    }
  }
}
```

### Sending Rust Signals to Dart

```rust
use rinf::RustSignal;

JobProgress {
  id: 7,
  percent: 42.5,
}.send_signal_to_dart();
```

For binary payloads:

```rust
use rinf::RustSignalBinary;

LargeFrame { frame_id: 3 }.send_signal_to_dart(bytes);
```

Generated Dart exposes `ClassName.rustSignalStream` and `ClassName.latestRustSignal` for Rust→Dart signals.

## Code Generation

### Running `rinf gen`

```bash
cd flutter_package/example
rinf gen
```

Generation reads `pubspec.yaml`:

```yaml
rinf:
  gen_input_crates:
    - hub
  gen_output_dir: lib/src/bindings
```

Default input is `native/hub/src/**/*.rs`; default output is `lib/src/bindings`.

**Common issues:**
- Reads `.npmrc`/`package.json` — add to `.rinfignore` if present (#630)
- `--watch` causes perpetual changes — avoid until #682 fixed; use one-shot `rinf gen`
- Extra curly brace in output — check for trailing commas (#601)
- Version mismatch — ensure `rinf` version in pubspec matches CLI version (#565)
- `DuplicatedSignal(name)` — two structs/enums with signal derives have the same Rust identifier across configured crates/modules
- `CodeSyntax(file)` — `syn` could not parse one Rust source file; reduce syntax to stable Rust 1.88-compatible syntax and rerun
- Missing Dart sender extension — verify the Rust type derives `DartSignal` or `DartSignalBinary`, not only `SignalPiece`
- Missing Dart stream/latest fields — verify the Rust type derives `RustSignal` or `RustSignalBinary`
- No generated classes — verify current directory is a Flutter app root; CLI rejects `pubspec.yaml` with `publish_to` not equal to `none`

### Generated Code Review

Check for:
1. Linter warnings (Dart analyzer) — fix via `// ignore:` or Rust side
2. Enum exhaustiveness — Dart doesn't have sealed classes, add fallback case (#638)
3. i64 on web — replace with `String` or `double` for cross-platform types
4. `assignRustSignal` contains one map entry per Rust→Dart signal endpoint, keyed by Rust type name
5. Dart→Rust sends use symbol `rinf_send_dart_signal_<snake_case_type>`

### Updating Generation Logic

Important source files:
- `rust_crate_cli/src/tool/generate.rs` — AST trace, type mapping, Dart extension/stream glue, watch mode
- `rust_crate_proc/src/lib.rs` — compile-time derives and exported symbols
- `flutter_package/lib/src/load_os.dart` and `interface_os.dart` — native symbol lookup and byte passing
- `flutter_package/lib/src/load_web.dart` and `interface_web.dart` — JS globals and wasm bindings

When adding a supported Rust type:
1. Add/confirm `SignalPiece` impl in `rust_crate/src/signal_trait.rs`.
2. Add matching `serde_reflection::Format` mapping in CLI `to_type_format`.
3. Add or update proc-macro where-clause/field validation if needed.
4. Add a signal using the type in the example native `hub` crate.
5. Run `cargo clippy --all-targets`, `rinf gen`, and `dart analyze flutter_package --fatal-infos`.

Watch for the current source wart in `extract_signal_attributes`: `"RustSignal"` maps to `SignalAttribute::RustSignalBinary`. Before refactoring that branch, inspect generated Dart for both binary and non-binary Rust→Dart signals.

## Platform Build Troubleshooting

### Android / Gradle

```bash
# Common error: "cannot exec"
# Fix: Ensure gradle wrapper is up to date
cd flutter_package/example/android
./gradlew wrapper --gradle-version 8.7
```

### iOS / macOS

```bash
# Pod install after adding new Rust dependency
cd flutter_package/example/ios
pod install --repo-update
```

### Linux

```bash
# Missing librust.so — check CMake paths
cd flutter_package/example/linux
rm -rf build/
flutter run -d linux
```

### Windows ARM64

- Requires VS2022 17.8+ with ARM64 build tools
- Cargokit build targets updated in #650

### Web (wasm)

```bash
# Ensure wasm32 target installed
rustup target add wasm32-unknown-unknown

# Repository/user app build path
cd flutter_package/example
rinf wasm --release
flutter build web --verbose
```

If wasm fails around nightly/std/shared memory, inspect `rust_crate_cli/src/tool/webassembly.rs`. Current CLI uses:
- `RUSTUP_TOOLCHAIN=nightly`
- `wasm-pack build native/hub --target web --out-dir web/pkg --out-name hub --no-typescript`
- `-Zbuild-std=std,panic_abort`
- atomics/shared-memory/mutable-globals flags
- COOP/COEP required for shared memory

Use `rinf server` to copy the correct run command:

```bash
rinf server
# copies:
# flutter run --web-header=cross-origin-opener-policy=same-origin --web-header=cross-origin-embedder-policy=require-corp
```

### ohos (HarmonyOS)

```bash
# Requires hvigor, ohos SDK
# Build directory resolution: see PR #678
cd flutter_package/example/ohos
hvigor build
```

### Native Library Loading

Default dynamic library names come from `flutter_package/lib/src/load_os.dart`:
- Linux/Android/ohos: `libhub.so`
- Windows: `hub.dll`
- iOS/macOS: `rinf.framework/rinf`

If a test or custom app cannot load the library:
1. Try `initializeRust(assignRustSignal, compiledLibPath: '/absolute/path/to/libhub.so')`.
2. Android and ohos use local symbols by design; Linux tests also use local symbols when `FLUTTER_TEST` exists.
3. Endpoint lookup failures usually mean generated Dart calls a symbol that the Rust proc macro did not export; verify the Rust type derives `DartSignal`/`DartSignalBinary` and rerun `rinf gen`.

### Shutdown and Hot Restart

Native Rust logic runs on a spawned thread and should keep its async runtime alive until Dart stops:

```rust
#[tokio::main]
async fn main() {
  tokio::select! {
    _ = dart_shutdown() => {}
    _ = run_app_logic() => {}
  }
}

rinf::write_interface!();
```

Call `finalizeRust()` before closing native apps when lifecycle control matters. On web it is a no-op.

## Cargokit Updates

Cargokit is vendored under `flutter_package/cargokit/`. It's periodically synced from upstream.

```bash
# Use the update script (historical pattern)
./scripts/update_cargokit.sh
```

**History of Cargokit integration:**
- Originally forked (2022), then subtree'd, then vendored with update script
- Key sync PRs: #65 (shell perms), #53 (Windows drive change), #65, #166, #320, #663
- Pattern: Create `cargokit-update` branch → merge upstream → PR as `cunarist/cargokit-update`

**When updating:**
1. Check upstream `cunarist/cargokit` for recent changes
2. Sync via subtree or manual merge
3. Ensure no squash of upstream history (#422c0ca1 lesson)
4. Test on affected platforms before merging

## CI/CD

### Fixing CI Failures

1. **Clippy errors** — `cargo clippy --fix --allow-dirty` then review changes
2. **Ruff errors** — `ruff check --fix` for Python files
3. **Dart analyzer** — `dart analyze` in flutter_package/
4. **GitHub Actions** — Check workflow files for deprecated runners/actions

Current quality workflow details:
- Repo root uses `RUSTFLAGS=-D warnings`
- Generate example signals before analysis: `cd flutter_package/example && rinf gen`
- Rust checks include native debug/release, wasm debug/release, `rust_crate --all-features`, and `rust_crate_cli`
- Dart check is `dart analyze flutter_package --fatal-infos`
- Python order in workflow is Ty then Ruff, but AGENTS.md says CI triage order is Clippy → Ruff → Dart analyzer

### Dependabot PRs

- Minor version bumps: Auto-approve if CI passes
- Major version bumps: Manual review required, check breaking changes
- Rust deps: Verify `cargo test` + `cargo clippy` after merge
- 47 dependabot PRs historically — mostly straightforward bumps

## Documentation

### Updating Guides

- Guides live in `documentation/` (32 files)
- Follow existing structure and tone
- Update `CHANGELOG.md` for user-facing changes
- Sync version numbers in `pubspec.yaml` + `Cargo.toml`

### Containerized Docs

- Docs are now containerized (PR #653)
- Domain: `cunarist.org`
- Build includes example web app

## Version Bumping

### Process

1. Update `pubspec.yaml` version
2. Update `Cargo.toml` version in `rust_crate/` and `rust_crate_cli/`
3. Update `CHANGELOG.md`
4. Commit as `Version X.Y.Z`
5. Tag: `git tag vX.Y.Z`
6. Push: `git push origin main vX.Y.Z`

### Version History Pattern

| Major | Period | Key Change |
|-------|--------|------------|
| 1.x | 2023-07 | Initial release, Protobuf |
| 2.x | 2023-07~08 | Early fixes, Cargokit integration |
| 3.x | 2023-08~09 | Proto resource system, CLI |
| 4.x | 2023-09~2024-01 | SignalPiece, framework identity |
| 5.x | 2024-01 | Multicore parallelism |
| 6.x | 2024-01~2024-09 | Event-driven, lifecycle, Bevy |
| 7.x | 2024-09~2025-04 | eLinux, Rust bump, templates |
| 8.x | 2025-04~now | Derive overhaul, Windows ARM64, ohos |

## Project Structure

```
rinf/
├── automate/          # Python scripts (Ty + Ruff)
├── demo/              # Containerized demo
├── documentation/     # 32 guide files
├── flutter_package/   # 265 files — Flutter plugin + cargokit + example
├── rust_crate/        # 12 files — core SignalPiece, interface
├── rust_crate_cli/    # 21 files — rinf CLI tool
├── rust_crate_proc/   # 2 files — proc macros
├── .github/           # 10 files — workflows + templates
└── .vscode/           # Dev settings
```

- **flutter_package/** dominates (265/336 files) — most work happens here
- **rust_crate/** is small but critical — core type system
- **rust_crate_cli/** has separate `Cargo.lock` — decoupled from main Rust crate
