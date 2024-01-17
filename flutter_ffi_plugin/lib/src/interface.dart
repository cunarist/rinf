import 'dart:typed_data';

typedef ReceiveSignal = void Function(int, Uint8List, Uint8List?);

/// This contains a message from Rust.
/// Optionally, a custom binary called `blob` can also be included.
/// This type is generic, and the message
/// can be of any type declared in Protobuf.
class RustSignal<T> {
  T message;
  Uint8List? blob;
  RustSignal(this.message, this.blob);
}
