import 'dart:ffi';
import 'dart:typed_data';
import 'load_os.dart';
import 'package:ffi/ffi.dart';
import 'dart:async';
import 'dart:isolate';
import 'interface.dart';
import 'dart:convert';

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
    Uint8List? blob;
    if (rustSignalRaw[2]) {
      blob = rustSignalRaw[3];
    } else {
      blob = null;
    }
    if (rustSignalRaw[0] == -1) {
      // -1 is a special message ID for Rust reports.
      String rustReport = utf8.decode(rustSignalRaw[3]);
      print(rustReport);
      return;
    }
    final messageId = rustSignalRaw[0];
    final messageBytes = rustSignalRaw[1];
    handleRustSignal(messageId, messageBytes, blob);
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

void stopRustLogicExtern() {
  final rustFunction =
      rustLibrary.lookupFunction<Void Function(), void Function()>(
    'stop_rust_logic_extern',
  );
  rustFunction();
}

/// Sends bytes to Rust.
Future<void> sendDartSignalExtern(
  int messageId,
  Uint8List messageBytes,
  bool blobValid,
  Uint8List blobBytes,
) async {
  final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
  messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

  final Pointer<Uint8> blobMemory = malloc.allocate(blobBytes.length);
  blobMemory.asTypedList(blobBytes.length).setAll(0, blobBytes);

  final rustFunction = rustLibrary.lookupFunction<
      Void Function(
        IntPtr,
        Pointer<Uint8>,
        IntPtr,
        Bool,
        Pointer<Uint8>,
        IntPtr,
      ),
      void Function(
        int,
        Pointer<Uint8>,
        int,
        bool,
        Pointer<Uint8>,
        int,
      )>('send_dart_signal_extern');

  rustFunction(
    messageId,
    messageMemory.cast(),
    messageBytes.length,
    blobValid,
    blobMemory.cast(),
    blobBytes.length,
  );

  // Note that we do not free memory here with `malloc.free()`,
  // because Rust will take the ownership of the memory space
  // with `Vec::from_raw_parts()`.
  // Rust will properly deallocate the memory later
  // when `Vec<u8>` is dropped.
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
