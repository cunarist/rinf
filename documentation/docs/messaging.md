# Messaging

There are special comments that you can mark messages with.

## 📢 Channels

`[RUST-SIGNAL]` generates a message channel from Rust to Dart.[^1] Use `[RUST-SIGNAL-BINARY]` to include binary data without the overhead of serialization.

[^1]: It’s important to note that when using `StreamBuilder`, it may only process the latest message from the stream to trigger a widget rebuild on the next render frame. Since widget builders are primarily focused on building widgets, they might skip some messages if multiple messages arrive within a single frame, typically around 16 milliseconds. To ensure that all messages from the stream are handled, you should consider using the `Stream.listen` method instead.

```proto title="Protobuf"
// [RUST-SIGNAL]
message MyDataOutput { ... }
```

```rust title="Rust"
MyDataOutput { ... }.send_signal_to_dart();
```

```dart title="Dart"
StreamBuilder(
  stream: MyDataOutput.rustSignalStream,
  builder: (context, snapshot) {
    final rustSignal = snapshot.data;
    if (rustSignal == null) {
      // Return an empty widget.
    }
    MyDataOutput message = rustSignal.message;
    // Below requires `[RUST-SIGNAL-BINARY]`.
    Uint8List binary = rustSignal.binary;
    // Return a filled widget.
  },
)
```

`[DART-SIGNAL]` generates a message channel from Dart to Rust. Use `[DART-SIGNAL-BINARY]` to include binary data without the overhead of serialization.

```proto title="Protobuf"
// [DART-SIGNAL]
message MyDataInput { ... }
```

```dart title="Dart"
MyDataInput( ... ).sendSignalToRust();
```

```rust title="Rust"
let receiver = MyDataInput::get_dart_signal_receiver();
while let Some(dart_signal) = receiver.recv().await {
    let message: MyDataInput = dart_signal.message;
    // Below requires `[DART-SIGNAL-BINARY]`.
    let binary: Vec<u8> = dart_signal.binary;
    // Custom Rust logic goes here.
}
```

## 🔖 Attributes

`[RUST-ATTRIBUTE(...)]` writes an attribute above the generated message struct in Rust. This is useful when you want to automatically implement a trait for the message struct in Rust.

```proto title="Protobuf"
// [RUST-ATTRIBUTE(#[derive(Hash)])]
message MyDataInput { ... }
```
