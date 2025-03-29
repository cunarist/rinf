# Upgrading Rinf

## Minor Upgrades

Whenever upgrading Rinf, please ensure that the Rinf versions in `pubspec.yaml` and `native/hub/Cargo.toml` are identical.

## Migrating from 6 to 7

The overall usage remains the same, but some changes have been made to the API to improve code readability and flexibility.

Explicitly bind the `main` function in Rust with the async runtime of your choice. Also, don't forget to await the `dart_shutdown` future provided by Rinf in the `main` function.

```{code-block} rust
:caption: Rust
[tokio::main]
async fn main() {
  // Do whatever you want here.
  // ...
  rinf::dart_shutdown().await;
}
```

Remove `RINF:` from Protobuf message annotations. For example, `[RINF:DART-SIGNAL]` should become `[DART-SIGNAL]`.

```{code-block} proto
:caption: Protobuf
// [DART-SIGNAL]
message SomeMessage {}
```

Import messages from the root `generated` module, not from the inner module where they are declared.

```{code-block} dart
:caption: Dart
import 'generated.dart';
```

```{code-block} rust
:caption: Rust
use crate::messages::{MyMessage, AnotherMessage};
```

When you need to run a Flutter web server, use `rinf server` to get the complete Flutter command with the necessary arguments.

```{code-block} shell
:caption: CLI
rinf server
```

## Migrating from 7 to 8

The way signal messages are used remains unchanged, but their declaration has become much cleaner. Now, we define the structs in Rust and annotate them with signal trait attributes. Protobuf is no longer used.

Replace the Rinf CLI tool with the new one.

```{code-block} shell
:caption: CLI
cargo uninstall rinf
cargo install rinf_cli
```

Move all Protobuf messages into the `hub` crate. Placing them inside `native/hub/src/signals/mod.rs` can be a good starting point, though any location within the `hub` crate is acceptable.

```{code-block} proto
:caption: Protobuf (Before)
// [DART-SIGNAL]
message MessageA {}

// [RUST-SIGNAL]
message MessageB {}
```

```{code-block} rust
:caption: Rust (After)
use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
struct MessageA {}

#[derive(Serialize, RustSignal)]
struct MessageB {}
```

There are 5 signal attributes, as follows:

- `DartSignal`
- `DartSignalBinary`
- `RustSignal`
- `RustSignalBinary`
- `SignalPiece`

To generate the corresponding Dart code, use `rinf gen` instead of `rinf message`.

```{code-block} shell
:caption: CLI
rinf gen
```

Import generated classes in Dart from the `bindings` module:

```{code-block} dart
:caption: Dart (Before)
import 'package:my_app/messages/all.dart';
```

```{code-block} dart
:caption: Dart (After)
import 'package:my_app/src/bindings/bindings.dart';
```

The methods of signal structs are the same, but they have now become trait methods. You should explicitly import the traits to ensure the methods still work.

```{code-block} rust
:caption: Rust (Before)
SomeMessage {}.send_signal_to_dart();
let receiver = SomeMessage::get_dart_signal_receiver();
```

```{code-block} rust
:caption: Rust (After)
use rinf::{DartSignal, RustSignal};

SomeMessage {}.send_signal_to_dart();
let receiver = SomeMessage::get_dart_signal_receiver();
```

Also, Rinf now requires Rust 1.85 with the 2024 edition:

```{code-block} shell
:caption: CLI
rustup update
rustc --version
```

Finally, verify that your Rinf configurations in `pubspec.yaml` conform to the new format, if present.

```{code-block} shell
:caption: CLI
rinf config
```
