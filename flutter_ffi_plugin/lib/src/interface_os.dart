import 'dart:ffi';
import 'dart:typed_data';
import 'load_os.dart';
import 'package:ffi/ffi.dart';
import 'dart:async';
import 'interface.dart';

typedef StoreDartPostCObject = Pointer Function(
    Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>);

/// This should be called once at startup
/// to enable `allo_isolate` to send data from the Rust side.
void storeDartPostCObject() {
  final rustFunction =
      rustLibrary.lookupFunction<StoreDartPostCObject, StoreDartPostCObject>(
    'store_dart_post_cobject',
  );
  // Call the Rust function
  rustFunction(NativeApi.postCObject);
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
Future<void> requestToRustExtern(RustRequestUnique rustRequestUnique) async {
  final interactionId = rustRequestUnique.id;
  final rustRequest = rustRequestUnique.request;

  final int rustOperation;
  if (rustRequest.operation == RustOperation.Create) {
    rustOperation = 0;
  } else if (rustRequest.operation == RustOperation.Read) {
    rustOperation = 1;
  } else if (rustRequest.operation == RustOperation.Update) {
    rustOperation = 2;
  } else {
    rustOperation = 3;
  }

  var messageBytes = rustRequest.message ?? Uint8List(0);
  final Pointer<Uint8> messageMemory = calloc<Uint8>(messageBytes.length);
  messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

  var blobBytes = rustRequest.blob ?? Uint8List(0);
  final Pointer<Uint8> blobMemory = calloc<Uint8>(blobBytes.length);
  blobMemory.asTypedList(blobBytes.length).setAll(0, blobBytes);

  // Look up the Rust function
  final rustFunction = rustLibrary.lookupFunction<
      Void Function(IntPtr, IntPtr, IntPtr, Pointer<Uint8>, IntPtr,
          Pointer<Uint8>, IntPtr),
      void Function(int, int, int, Pointer<Uint8>, int, Pointer<Uint8>,
          int)>('request_to_rust_extern');

  // Call the Rust function
  rustFunction(
    interactionId,
    rustRequest.resource,
    rustOperation,
    messageMemory.cast(),
    messageBytes.length,
    blobMemory.cast(),
    blobBytes.length,
  );
}

void prepareIsolatesExtern(int portSignal, int portResponse, int portReport) {
  // Look up the Rust function
  final rustFunction = rustLibrary.lookupFunction<
      Void Function(IntPtr, IntPtr, IntPtr),
      void Function(int, int, int)>('prepare_isolates_extern');
  // Call the Rust function
  rustFunction(portSignal, portResponse, portReport);
}

void prepareChannelsExtern() {
  // Look up the Rust function
  final rustFunction =
      rustLibrary.lookupFunction<Void Function(), void Function()>(
          'prepare_channels_extern');
  // Call the Rust function
  rustFunction();
}
