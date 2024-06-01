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
  setCompiledLibPathExtern(path);
}

/// Prepares the native interface
/// needed to communicate with Rust.
Future<void> prepareInterface(HandleRustSignal handleRustSignal) async {
  await prepareInterfaceExtern(handleRustSignal);
}

/// Starts the `main` function in Rust.
void startRustLogic() async {
  startRustLogicExtern();
}

/// Terminates all Rust tasks.
/// Calling this function before closing the Flutter app
/// can prevent potential resource leaks that may occur
/// if the Rust side is abruptly terminated.
/// Please note that on the web, this function does not have any effect,
/// as tasks are managed by the JavaScript runtime, not Rust.
void stopRustLogic() async {
  stopRustLogicExtern();
}

/// Sends a signal to Rust.
void sendDartSignal(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) async {
  sendDartSignalExtern(
    messageId,
    messageBytes,
    binary,
  );
}
