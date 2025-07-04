# Messaging

You can use signal traits to mark a struct as a stream endpoint.[^1]

[^1]: Rinf relies solely on native FFI for communication, avoiding the use of web protocols or hidden threads. The goal is to minimize performance overhead as much as possible.

When you generate Dart signal class code from Rust structs using the `rinf gen` command, the `hub` crate is analyzed, and the resulting Dart modules are placed in `lib/src/bindings` folder by default.

```{code-block} shell
:caption: CLI
rinf gen
```

If you add the optional argument `-w` or `--watch` to the `rinf gen` command, the message code will be automatically generated when Rust files are modified. If you add this argument, the command will not exit on its own.

```{code-block} shell
:caption: CLI
rinf gen --watch
```

## Endpoint Signals

The `RustSignal` trait generates a signal stream from Rust to Dart.[^2] Use the `RustSignalBinary` trait to include binary data without the overhead of serialization.

[^2]: It’s important to note that when using [`StreamBuilder`](https://api.flutter.dev/flutter/widgets/StreamBuilder-class.html), it may only process the latest message from the stream to trigger a widget rebuild on the next render frame. Since widget builders are primarily focused on building widgets, they might skip some messages if multiple messages arrive within a single frame, typically around 16 milliseconds. To ensure that all messages from the stream are handled, you should consider using the [`Stream.listen`](https://api.flutter.dev/flutter/dart-async/Stream/listen.html) method instead.

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
    RustSignalPack<MyDataOutput>? signalPack = snapshot.data;
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

// You can also get the latest value received from the stream.
// This is useful when building and mounting a new widget.
RustSignalPack<MyDataOutput>? latestSignal = MyDataOutput.latestRustSignal;
```

The `DartSignal` trait generates a signal stream from Dart to Rust. Use the `DartSignalBinary` trait to include binary data without the overhead of serialization.

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

Now let's delve into the meaning of each field of a signal pack.

- **Field `message`:** It represents a message of a type annotated by a signal trait. This field is always filled.

- **Field `binary`:** This is a field designed to handle large binary data, potentially up to a few gigabytes. You can send any kind of binary data you wish, such as a high-resolution image or file data. This field carries empty `Uint8List` or `Vec<u8>` if the message is not marked as binary signal.

It's important to note that creating a signal larger than a few megabytes is not recommended. For large data, split it into multiple signals or use the `binary` field provided by the `RustSignalBinary` or `DartSignalBinary` traits instead.[^3]

[^3]: Sending a serialized message or binary data is a zero-copy operation from Rust to Dart, while it involves a copy operation from Dart to Rust in memory. Keep in mind that Serde's serialization and deserialization does involve memory copy.

## Nested Signals

To nest a struct inside a signal struct, use the `SignalPiece` trait. A `SignalPiece` cannot be passed between languages independently, but it can be nested inside a `RustSignal`, `DartSignal`, or another `SignalPiece`.

```{code-block} rust
:caption: Rust
#[derive(Serialize, RustSignal)]
struct Outer {
  middle: Middle,
}

#[derive(Serialize, SignalPiece)]
struct Middle {
  inner: Inner,
}

#[derive(Serialize, SignalPiece)]
struct Inner {
  my_field: bool,
}
```
