# Upgrading Rinf

## 🐾 Minor Upgrades

Whenever upgrading Rinf, please ensure that the Rinf versions in `pubspec.yaml` and `native/hub/Cargo.toml` are identical.

## 7️⃣ Migrating from 6 to 7

The overall usage remains the same, but some changes were made to the API to make the code more readable and maintainable.

Explicitly bind the main function in Rust with the async runtime you choose to use. Also, don't forget to await for the `dart_shutdown` future provided by Rinf in the main function.

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

Import messages from the root `messages` module, not the inner module where it's declared.

```dart title="Dart"
import './messages/all.dart';
```

```rust title="Rust"
use crate::messages::*;
```

When you need to run a Flutter web server, use `rinf server` to get the whole Flutter command with necessary arguments

```bash title="CLI"
rinf server
```
