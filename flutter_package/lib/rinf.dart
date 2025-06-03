/// This module supports communication with Rust.
library;

import 'dart:typed_data';
import 'dart:convert';
import 'src/structure.dart';
import 'src/interface.dart';

export 'src/structure.dart' show RustSignalPack;

/// Starts the `main` function in Rust.
Future<void> initializeRust(
  AssignRustSignal assignRustSignal, {
  String? compiledLibPath,
}) async {
  // Override the default library path if provided.
  if (compiledLibPath != null) {
    setCompiledLibPathReal(compiledLibPath);
  }

  // Add the print delegation endpoint.
  assignRustSignal['RinfOut'] = (messageBytes, binary) {
    final rustReport = utf8.decode(binary);
    print(rustReport);
  };

  // Prepare the interface with Rust.
  await prepareInterfaceReal(assignRustSignal);
  startRustLogicReal();
}

/// Terminates all Rust tasks by dropping the async runtime.
/// This function very briefly blocks the Dart thread
/// until the async runtime in Rust is completely dropped.
/// It's recommended to call this before closing your Flutter app
/// to prevent potential resource leaks from Rust.
/// On the web, this function has no effect as tasks are managed by
/// the JavaScript runtime rather than the Rust async runtime.
void finalizeRust() {
  stopRustLogicReal();
}

/// Sends a signal to Rust by using a symbol
/// that exists inside local space of the loaded dynamic library.
void sendDartSignal(
  String endpointSymbol,
  Uint8List messageBytes,
  Uint8List binary,
) {
  sendDartSignalReal(endpointSymbol, messageBytes, binary);
}
