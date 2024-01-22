/// This module supports communication with Rust.

import 'dart:async';
import 'dart:typed_data';
import 'src/exports.dart';

export 'src/interface.dart' show RustSignal;

/// Makes sure that the Rust side is ready.
/// Don't forget to call this function in the `main` function of Dart.
Future<void> initializeRinf(HandleRustSignal handleRustSignal) async {
  await prepareNativeBridge(handleRustSignal);
}

/// Ensure that all Rust tasks are terminated
/// by calling this function before closing the Flutter app.
/// Doing so can prevent potential memory errors that may occur
/// when Rust attempts to send data after the Dart VM has been turned off.
/// Please note that on the web, this function does not have any effect,
/// as tasks are managed by the JavaScript runtime, not Rust.
Future<void> finalizeRinf() async {
  stopRustLogicExtern();
}

/// Send a signal to Rust.
Future<void> sendDartSignal(
  int messageId,
  Uint8List messageBytes,
  Uint8List? blob,
) async {
  bool blobValid;
  Uint8List blobBytes;
  if (blob == null) {
    blobValid = false;
    blobBytes = Uint8List(0);
  } else {
    blobValid = true;
    blobBytes = blob;
  }
  sendDartSignalExtern(messageId, messageBytes, blobValid, blobBytes);
}
