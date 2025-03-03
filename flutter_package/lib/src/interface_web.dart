// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:async';
import 'dart:js_interop';
import 'dart:js_interop_unsafe';
import 'dart:typed_data';
import 'dart:convert';
import 'load_web.dart';
import 'interface.dart';

/// Sets the path to the JavaScript module
/// that needs to be loaded.
void setCompiledLibPathReal(String path) {
  setJsLibPath(path);
}

Future<void> prepareInterfaceReal(
  AssignRustSignal assignRustSignal,
) async {
  // Load the JavaScript module.
  await loadJsFile();

  // Listen to Rust via JavaScript.
  rinfBindingsObject['send_rust_signal_extern'] = (
    int messageId,
    Uint8List messageBytes,
    Uint8List binary,
  ) {
    if (messageId == -1) {
      // -1 is a special message ID for Rust reports.
      String rustReport = utf8.decode(binary);
      print(rustReport);
      return;
    }
    assignRustSignal(messageId, messageBytes, binary);
  }.jsify();
}

void startRustLogicReal() {
  if (wasAlreadyLoaded) {
    return;
  }
  wasmBindingsObject.callMethod('start_rust_logic_extern'.toJS);
}

void stopRustLogicReal() {
  // Dummy function to match the structure of native platforms.
}
