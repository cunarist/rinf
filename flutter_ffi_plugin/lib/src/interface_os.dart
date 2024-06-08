import 'dart:ffi';
import 'dart:typed_data';
import 'load_os.dart';
import 'package:ffi/ffi.dart';
import 'dart:async';
import 'dart:isolate';
import 'interface.dart';
import 'dart:convert';

void setCompiledLibPathExtern(String? path) {
  setDynamicLibPath(path);
}

Future<void> prepareInterfaceExtern(
  HandleRustSignal handleRustSignal,
) async {
  /// This should be called once at startup
  /// to enable `allo_isolate` to send data from the Rust side.
  final rustFunction = rustLibrary.lookupFunction<
      Pointer Function(
        Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>,
      ),
      Pointer Function(
        Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>,
      )>('store_dart_post_cobject');
  rustFunction(NativeApi.postCObject);

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
    handleRustSignal(messageId, messageBytes, binary);
  });

  // Make Rust prepare its isolate to send data to Dart.
  prepareIsolateExtern(rustSignalPort.sendPort.nativePort);
}

void startRustLogicExtern() {
  final rustFunction =
      rustLibrary.lookupFunction<Void Function(), void Function()>(
    'start_rust_logic_extern',
  );
  rustFunction();
}

/// Sends bytes to Rust.
Future<void> sendDartSignalExtern(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) async {
  final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
  messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

  final Pointer<Uint8> binaryMemory = malloc.allocate(binary.length);
  binaryMemory.asTypedList(binary.length).setAll(0, binary);

  final rustFunction = rustLibrary.lookupFunction<
      Void Function(
        IntPtr,
        Pointer<Uint8>,
        IntPtr,
        Pointer<Uint8>,
        IntPtr,
      ),
      void Function(
        int,
        Pointer<Uint8>,
        int,
        Pointer<Uint8>,
        int,
      )>('send_dart_signal_extern');

  rustFunction(
    messageId,
    messageMemory,
    messageBytes.length,
    binaryMemory,
    binary.length,
  );

  malloc.free(messageMemory);
  malloc.free(binaryMemory);
}

void prepareIsolateExtern(int port) {
  final rustFunction = rustLibrary.lookupFunction<
      Void Function(
        IntPtr,
      ),
      void Function(
        int,
      )>('prepare_isolate_extern');
  rustFunction(port);
}
