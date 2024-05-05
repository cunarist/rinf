# Messaging

!!! warning

    If you are using Rinf version 5 or earlier, please refer to the [historical documentation](https://github.com/cunarist/rinf/blob/v5.4.0/documentation/docs/writing-code.md). With the introduction of Rinf version 6, a simpler way for communication between Dart and Rust has been implemented, and the system has undergone significant changes.

Marking Protobuf messages with special comments empower them with the capability to transmit signals across Dart and Rust. This is achieved by allowing Rinf's code generator to create the necessary channels for communication between Dart and Rust.

There are 2 types of special comments that you can mark messages with.

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
let mut receiver = MyDataInput::get_dart_signal_receiver();
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
Uint8List binary = Uint8List(64);
MyDataInput( ... ).sendSignalToRust(binary);
```

```rust title="Rust"
let mut receiver = MyDataInput::get_dart_signal_receiver();
while let Some(dart_signal) = receiver.recv().await {
    let message: MyDataInput = dart_signal.message;
    let binary: Vec<u8> = dart_signal.binary.unwrap();
    // Custom Rust logic here
}
```

## 📢 Rust Signal

`[RINF:RUST-SIGNAL]` generates a channel from Rust to Dart.

```proto title="Protobuf"
// [RINF:RUST-SIGNAL]
message MyDataOutput { ... }
```

```dart title="Dart"
final stream = MyDataOutput.rustSignalStream;
await for (final rustSignal in stream) {
    MyDataOutput message = rustSignal.message;
    // Custom Dart logic here
}
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
final stream = MyDataOutput.rustSignalStream;
await for (final rustSignal in stream) {
    MyDataOutput message = rustSignal.message;
    Uint8List binary = rustSignal.binary!;
    // Custom Dart logic here
}
```

```rust title="Rust"
let binary: Vec<u8> = vec![0; 64];
MyDataOutput { ... }.send_signal_to_dart(binary);
```
