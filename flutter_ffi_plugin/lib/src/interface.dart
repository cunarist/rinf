import 'dart:typed_data';

typedef ReceiveSignal = void Function(int, Uint8List, Uint8List?);

class RustSignal<T> {
  T message;
  Uint8List? blob;
  RustSignal(this.message, this.blob);
}
