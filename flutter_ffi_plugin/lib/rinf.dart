/// This module supports communication with Rust.

import 'dart:typed_data';
import 'src/exports.dart';

export 'src/interface.dart' show RustSignal;

/// Prepares the native interface
/// needed to communicate with Rust.
void prepareInterface(HandleRustSignal handleRustSignal) async {
  prepareInterfaceExtern(handleRustSignal);
}

/// Starts the `main` function in Rust.
void startRustLogic() async {
  startRustLogicExtern();
}

/// Terminates all Rust tasks.
/// Doing so before closing the Flutter app
/// can prevent potential memory errors that may occur
/// when Rust attempts to send data after the Dart VM has been turned off.
/// Please note that on the web, this function does not have any effect,
/// as tasks are managed by the JavaScript runtime, not Rust.
void stopRustLogic() async {
  stopRustLogicExtern();
}

/// Sends a signal to Rust.
void sendDartSignal(
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
  sendDartSignalExtern(
    messageId,
    messageBytes,
    blobValid,
    blobBytes,
  );
}
