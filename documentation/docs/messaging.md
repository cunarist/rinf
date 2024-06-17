# Messaging

There are 2 types of special comments that you can mark messages with.

## 📢 Rust Signal

`[RINF:RUST-SIGNAL]` generates a channel from Rust to Dart.

```proto title="Protobuf"
// [RINF:RUST-SIGNAL]
message MyDataOutput { ... }
```

```dart title="Dart"
StreamBuilder(
  stream: MyDataOutput.rustSignalStream,
  builder: (context, snapshot) {
    final rustSignal = snapshot.data;
    if (rustSignal == null) {
      // Return an empty widget.
    }
    final myDataOutput = rustSignal.message;
    // Return a filled widget.
  },
)
```

```rust title="Rust"
MyDataOutput { ... }.send_signal_to_dart();
```

Use `[RINF:RUST-SIGNAL-BINARY]` to include binary data without the overhead of serialization.

```proto title="Protobuf"
// [RINF:RUST-SIGNAL-BINARY]
message MyDataOutput { ... }
```

```dart title="Dart"
StreamBuilder(
  stream: MyDataOutput.rustSignalStream,
  builder: (context, snapshot) {
    final rustSignal = snapshot.data;
    if (rustSignal == null) {
      // Return an empty widget.
    }
    final myDataOutput = rustSignal.message;
    // Return a filled widget.
  },
)
```

```rust title="Rust"
let binary: Vec<u8> = vec![0; 64];
MyDataOutput { ... }.send_signal_to_dart(binary);
```

## 📭 Dart Signal

`[RINF:DART-SIGNAL]` generates a channel from Dart to Rust.

```proto title="Protobuf"
// [RINF:DART-SIGNAL]
message MyDataInput { ... }
```

```dart title="Dart"
MyDataInput( ... ).sendSignalToRust();
```

```rust title="Rust"
let mut receiver = MyDataInput::get_dart_signal_receiver()?;
while let Some(dart_signal) = receiver.recv().await {
    let message: MyDataInput = dart_signal.message;
    // Custom Rust logic here
}
```

Use `[RINF:DART-SIGNAL-BINARY]` to include binary data without the overhead of serialization.

```proto title="Protobuf"
// [RINF:DART-SIGNAL-BINARY]
message MyDataInput { ... }
```

```dart title="Dart"
final binary = Uint8List(64);
MyDataInput( ... ).sendSignalToRust(binary);
```

```rust title="Rust"
let mut receiver = MyDataInput::get_dart_signal_receiver()?;
while let Some(dart_signal) = receiver.recv().await {
    let message: MyDataInput = dart_signal.message;
    let binary: Vec<u8> = dart_signal.binary;
    // Custom Rust logic here
}
```
