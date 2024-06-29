/// This module supports communication with Rust.
library;

import 'dart:typed_data';
import 'src/exports.dart';

export 'src/interface.dart' show RustSignal;

/// Starts the `main` function in Rust.
Future<void> initializeRust(
  AssignRustSignal assignRustSignal, {
  String? compiledLibPath,
}) async {
  if (compiledLibPath != null) {
    setCompiledLibPathReal(compiledLibPath);
  }
  await prepareInterfaceReal(assignRustSignal);
  startRustLogicReal();
}

/// Terminates all Rust tasks.
/// Calling this function before closing the Flutter app
/// can prevent potential resource leaks that may occur
/// if the Rust side is abruptly terminated.
/// Please note that on the web, this function does not have any effect,
/// as tasks are managed by the JavaScript runtime, not Rust.
void finalizeRust() async {
  stopRustLogicReal();
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
