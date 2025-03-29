# Messaging

There are special comments that you can mark messages with.

## Channel Signals

The `RustSignal` trait generates a message channel from Rust to Dart.[^1] Use the `RustSignalBinary` trait to include binary data without the overhead of serialization.

[^1]: It’s important to note that when using [`StreamBuilder`](https://api.flutter.dev/flutter/widgets/StreamBuilder-class.html), it may only process the latest message from the stream to trigger a widget rebuild on the next render frame. Since widget builders are primarily focused on building widgets, they might skip some messages if multiple messages arrive within a single frame, typically around 16 milliseconds. To ensure that all messages from the stream are handled, you should consider using the [`Stream.listen`](https://api.flutter.dev/flutter/dart-async/Stream/listen.html) method instead.

```{code-block} rust
:caption: Rust
#[derive(Serialize, RustSignal)]
struct MyDataOutput {
  my_field: bool,
}
```

```{code-block} rust
:caption: Rust
MyDataOutput { my_field: true }.send_signal_to_dart();
```

```{code-block} dart
:caption: Dart
// Rebuild the widget from Rust signals on each render frame.
// Some Rust signals between frames may be ignored.
StreamBuilder(
  stream: MyDataOutput.rustSignalStream,
  builder: (context, snapshot) {
    final signalPack = snapshot.data;
    if (signalPack == null) {
      // Return an empty widget.
    }
    MyDataOutput message = signalPack.message;
    // Below requires `RustSignalBinary`.
    Uint8List binary = signalPack.binary;
    // Return a filled widget.
  },
);

// Alternatively, handle every Rust signal.
// Don't forget to cancel the subscription when it's no longer needed!
final subscription = MyDataOutput.rustSignalStream.listen((signalPack) {
  MyDataOutput message = signalPack.message;
})
```

The `DartSignal` trait generates a message channel from Dart to Rust. Use the `DartSignalBinary` trait to include binary data without the overhead of serialization.

```{code-block} rust
:caption: Rust
#[derive(Deserialize, DartSignal)]
struct MyDataInput {
  my_field: bool,
}
```

```{code-block} dart
:caption: Dart
MyDataInput(my_field: true).sendSignalToRust();
```

```{code-block} rust
:caption: Rust
let receiver = MyDataInput::get_dart_signal_receiver();
while let Some(signal_pack) = receiver.recv().await {
  let message: MyDataInput = signal_pack.message;
  // Below requires `DartSignalBinary`.
  let binary: Vec<u8> = signal_pack.binary;
  // Custom Rust logic goes here.
}
```

## Nested Signals

To nest a struct inside a signal struct, use the `SignalPiece` trait. A `SignalPiece` cannot be passed between languages independently, but it allows it to be nested inside a `RustSignal`, `DartSignal`, or another `SignalPiece`.

```{code-block} rust
:caption: Rust
#[derive(Serialize, Deserialize, RustSignal, DartSignal)]
struct Outer {
  middle: Middle,
}

#[derive(Serialize, Deserialize, SignalPiece)]
struct Middle {
  inner: Inner,
}

#[derive(Serialize, Deserialize, SignalPiece)]
struct Inner {
  my_field: bool,
}
```
