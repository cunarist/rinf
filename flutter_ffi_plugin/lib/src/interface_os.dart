import 'dart:ffi';
import 'dart:typed_data';
import 'load_os.dart';
import 'package:ffi/ffi.dart';
import 'dart:async';
import 'dart:isolate';
import 'interface.dart';

typedef StoreDartPostCObject = Pointer Function(
    Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>);

/// This should be called once at startup
/// to enable `allo_isolate` to send data from the Rust side.
Future<void> prepareNativeBridge(ReceiveSignal receiveSignal) async {
  // Look up the Rust function
  final rustFunction =
      rustLibrary.lookupFunction<StoreDartPostCObject, StoreDartPostCObject>(
    'store_dart_post_cobject',
  );
  // Call the Rust function
  rustFunction(NativeApi.postCObject);

  // Prepare ports that can communicate over isolates
  final rustSignalPort = ReceivePort();
  final rustReportPort = ReceivePort();

  // Listen to Rust via isolate ports
  rustSignalPort.listen((rustSignalRaw) {
    Uint8List? blob;
    if (rustSignalRaw[2]) {
      blob = rustSignalRaw[3];
    } else {
      blob = null;
    }
    receiveSignal(
      rustSignalRaw[0],
      rustSignalRaw[1],
      blob,
    );
  });
  rustReportPort.listen((rustReport) {
    print(rustReport);
  });

  // Make Rust have its own isolates to send data to Dart
  prepareIsolatesExtern(
    rustSignalPort.sendPort.nativePort,
    rustReportPort.sendPort.nativePort,
  );
  startRustLogicExtern();
}

void startRustLogicExtern() {
  // Look up the Rust function
  final rustFunction =
      rustLibrary.lookupFunction<Void Function(), void Function()>(
          'start_rust_logic_extern');
  // Call the Rust function
  rustFunction();
}

void stopRustLogicExtern() {
  // Look up the Rust function
  final rustFunction =
      rustLibrary.lookupFunction<Void Function(), void Function()>(
          'stop_rust_logic_extern');
  // Call the Rust function
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

  // Look up the Rust function
  final rustFunction = rustLibrary.lookupFunction<
      Void Function(
          IntPtr, Pointer<Uint8>, IntPtr, Bool, Pointer<Uint8>, IntPtr),
      void Function(int, Pointer<Uint8>, int, bool, Pointer<Uint8>,
          int)>('send_dart_signal_extern');

  // Call the Rust function
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

void prepareIsolatesExtern(int portSignal, int portReport) {
  // Look up the Rust function
  final rustFunction = rustLibrary.lookupFunction<Void Function(IntPtr, IntPtr),
      void Function(int, int)>('prepare_isolates_extern');
  rustFunction(portSignal, portReport);
}
