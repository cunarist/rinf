/// This module supports communication with Rust.
library;

import 'dart:typed_data';
import 'src/exports.dart';

export 'src/interface.dart' show RustSignal;

/// Sets the exact file path of the dynamic library
/// compiled from the `hub` crate.
/// On the web, this function sets the path to the JavaScript module
/// that needs to be loaded.
/// This function might not be necessary for major platforms
/// but can be useful when the app runs on embedded devices.
void setCompiledLibPath(String? path) {
  setCompiledLibPathReal(path);
}

/// Prepares the native interface
/// needed to communicate with Rust.
Future<void> prepareInterface(HandleRustSignal handleRustSignal) async {
  await prepareInterfaceReal(handleRustSignal);
}

/// Starts the `main` function in Rust.
void startRustLogic() async {
  startRustLogicReal();
}

/// Sends a signal to Rust.
void sendDartSignal(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) async {
  sendDartSignalReal(
    messageId,
    messageBytes,
    binary,
  );
}
