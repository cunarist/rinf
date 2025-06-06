import 'dart:typed_data';

/// This type represents a function
/// that can accept raw signal data from Rust
/// and handle it accordingly.
typedef AssignRustSignal = Map<String, Function(Uint8List, Uint8List)>;

/// This contains a message from Rust.
/// Optionally, a custom binary called `binary` can also be included.
class RustSignalPack<T> {
  /// The message instance.
  final T message;

  /// Binary data included in the signal.
  /// This field is useful for sending custom bytes
  /// without the overhead of serialization/deserialization.
  final Uint8List binary;

  /// Constructs a packed data transfer object that includes
  /// the message and the binary data from Rust.
  /// This constructor is not intended for external use outside the library.
  RustSignalPack(this.message, this.binary);
}
