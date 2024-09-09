import 'dart:ffi';
import 'dart:typed_data';
import 'load_os.dart';
import 'dart:async';
import 'dart:isolate';
import 'interface.dart';
import 'dart:convert';

/// Sets the exact file path of the dynamic library
/// compiled from the `hub` crate.
void setCompiledLibPathReal(String path) {
  setDynamicLibPath(path);
}

Future<void> prepareInterfaceReal(
  AssignRustSignal assignRustSignal,
) async {
  /// This should be called once at startup
  /// to enable `allo_isolate` to send data from the Rust side.
  storeDartPostCObjectReal(NativeApi.postCObject);

  // Prepare ports for communication over isolates.
  final rustSignalPort = ReceivePort();

  // Listen to Rust via isolate port.
  rustSignalPort.listen((rustSignalRaw) {
    final messageId = rustSignalRaw[0];
    var messageBytes = rustSignalRaw[1];
    var binary = rustSignalRaw[2];
    if (binary == null) {
      // Rust will send null if the vector is empty.
      // Converting is needed on the Dart side.
      binary = Uint8List(0);
    }
    if (rustSignalRaw[0] == -1) {
      // -1 is a special message ID for Rust reports.
      String rustReport = utf8.decode(binary);
      print(rustReport);
      return;
    }
    if (messageBytes == null) {
      // Rust will send null if the vector is empty.
      // Converting is needed on the Dart side.
      messageBytes = Uint8List(0);
    }
    assignRustSignal(messageId, messageBytes, binary);
  });

  // Make Rust prepare its isolate to send data to Dart.
  prepareIsolateReal(rustSignalPort.sendPort.nativePort);
}

@Native<Void Function()>(isLeaf: true, symbol: 'start_rust_logic_extern')
external void startRustLogicReal();

@Native<Void Function()>(isLeaf: true, symbol: 'stop_rust_logic_extern')
external void stopRustLogicReal();

typedef SendDartSignalReal = Void Function(
  Int32,
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);
@Native<SendDartSignalReal>(isLeaf: true, symbol: 'send_dart_signal_extern')
external void sendDartSignalExtern(
  int messageId,
  Pointer<Uint8> messageBytesAddress,
  int messageBytesLength,
  Pointer<Uint8> binaryAddress,
  int binaryLength,
);

void sendDartSignalReal(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) {
  sendDartSignalExtern(
    messageId,
    messageBytes.address,
    messageBytes.length,
    binary.address,
    binary.length,
  );
}

@Native<Void Function(Int64)>(isLeaf: true, symbol: 'prepare_isolate_extern')
external void prepareIsolateReal(
  int port,
);

typedef InnerFunction = Int8 Function(Int64, Pointer<Dart_CObject>);
@Native<Void Function(Pointer<NativeFunction<InnerFunction>>)>(
  isLeaf: true,
  symbol: 'store_dart_post_cobject',
)
external void storeDartPostCObjectReal(
  Pointer<NativeFunction<InnerFunction>> postCObject,
);
