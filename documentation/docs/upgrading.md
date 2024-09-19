# Upgrading Rinf

## 🐾 Minor Upgrades

Whenever upgrading Rinf, please ensure that the Rinf versions in `pubspec.yaml` and `native/hub/Cargo.toml` are identical.

## 7️⃣ Migrating from 6 to 7

The overall usage remains the same, but some changes have been made to the API to improve code readability and maintainability.

Explicitly bind the `main` function in Rust with the async runtime of your choice. Also, don't forget to await the `dart_shutdown` future provided by Rinf in the `main` function.

```rust title="Rust"
[tokio::main]
async fn main() {
    // Do whatever you want here.
    rinf::dart_shutdown().await;
}
```

Remove `RINF:` from Protobuf message annotations. For example, `[RINF:DART-SIGNAL]` should become `[DART-SIGNAL]`.

```proto title="Protobuf"
// [DART-SIGNAL]
message SomeMessage {}
```

Import messages from the root `messages` module, not from the inner module where they are declared.

```dart title="Dart"
import './messages/all.dart';
```

```rust title="Rust"
use crate::messages::*;
```

When you need to run a Flutter web server, use `rinf server` to get the complete Flutter command with the necessary arguments.

```bash title="CLI"
rinf server
```
