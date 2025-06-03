import 'dart:ffi';
import 'dart:typed_data';
import 'dart:async';
import 'dart:isolate';
import 'load_os.dart';
import 'structure.dart';

/// Sets the exact file path of the dynamic library
/// compiled from the `hub` crate.
void setCompiledLibPathReal(String path) {
  overrideLibPath(path);
}

Future<void> prepareInterfaceReal(AssignRustSignal assignRustSignal) async {
  // Prepare ports for communication over isolates.
  final rustSignalPort = ReceivePort();

  // Listen to Rust via isolate port.
  rustSignalPort.listen((rustSignalRaw) {
    String endpoint = rustSignalRaw[0];
    Uint8List? messageBytes = rustSignalRaw[1];
    Uint8List? binary = rustSignalRaw[2];

    // Rust will send null if the vector is empty.
    // Converting is needed on the Dart side.
    binary ??= Uint8List(0);
    // Rust will send null if the vector is empty.
    // Converting is needed on the Dart side.
    messageBytes ??= Uint8List(0);

    assignRustSignal[endpoint]!(messageBytes, binary);
  });

  // Make Rust prepare its isolate to send data to Dart.
  // This is handled by `allo_isolate`.
  rustLibrary.prepareIsolate(
    NativeApi.postCObject,
    rustSignalPort.sendPort.nativePort,
  );
}

void startRustLogicReal() {
  rustLibrary.startRustLogic();
}

void stopRustLogicReal() {
  rustLibrary.stopRustLogic();
}

void sendDartSignalReal(
  String endpointSymbol,
  Uint8List messageBytes,
  Uint8List binary,
) {
  rustLibrary.sendDartSignal(endpointSymbol, messageBytes, binary);
}
