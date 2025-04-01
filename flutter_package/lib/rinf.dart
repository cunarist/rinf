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
    String rustReport = utf8.decode(binary);
    print(rustReport);
  };

  // Prepare the interface with Rust.
  await prepareInterfaceReal(assignRustSignal);
  startRustLogicReal();
}

/// Terminates all Rust tasks by dropping the async runtime.
/// Calling this function before closing the Flutter app
/// can prevent potential resource leaks.
/// Please note that on the web, this function does not have any effect,
/// as tasks are managed by the JavaScript runtime, not Rust.
void finalizeRust() async {
  stopRustLogicReal();
}

/// Sends a signal to Rust by using a symbol
/// that exists inside local space of the loaded dynamic library.
void sendDartSignal(
  String endpointSymbol,
  Uint8List messageBytes,
  Uint8List binary,
) async {
  sendDartSignalReal(
    endpointSymbol,
    messageBytes,
    binary,
  );
}
