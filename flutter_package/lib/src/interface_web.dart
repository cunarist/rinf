// ignore_for_file: avoid_web_libraries_in_flutter

import 'load_web.dart';
import 'dart:typed_data';
import 'dart:js' as js;
import 'interface.dart';
import 'dart:async';
import 'dart:convert';

/// Sets the path to the JavaScript module
/// that needs to be loaded.
void setCompiledLibPathReal(String path) {
  setJsLibPath(path);
}

Future<void> prepareInterfaceReal(
  AssignRustSignal assignRustSignal,
) async {
  // Check if this is a fresh webpage load.
  checkIfAlreadyLoaded();

  // Create the namespace JavaScript object.
  final jsObject = js.JsObject.jsify({});
  js.context['rinfBindings'] = jsObject;

  // Listen to Rust via JavaScript.
  jsObject['send_rust_signal_extern'] = (
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
  };

  // Load the JavaScript module.
  await loadJsFile();
}

void startRustLogicReal() {
  if (wasAlreadyLoaded) {
    return;
  }
  final jsObject = js.context['wasmBindings'] as js.JsObject;
  jsObject.callMethod('start_rust_logic_extern', []);
}

void stopRustLogicReal() {
  // Dummy function to match the structure of native platforms.
}

void sendDartSignalReal(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) {
  final jsObject = js.context['wasmBindings'] as js.JsObject;
  jsObject.callMethod('send_dart_signal_extern', [
    messageId,
    messageBytes,
    binary,
  ]);
}
