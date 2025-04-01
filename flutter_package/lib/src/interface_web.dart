// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:async';
import 'dart:js_interop';
import 'dart:js_interop_unsafe';
import 'dart:typed_data';
import 'dart:convert';
import 'load_web.dart';
import 'structure.dart';

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
  rinfBindingsObject['rinf_send_rust_signal_extern'] = ((
    String endpoint,
    JSObject messageBytesJS,
    JSObject binaryJS,
  ) {
    final messageBytes = (messageBytesJS as JSUint8Array).toDart;
    final binary = (binaryJS as JSUint8Array).toDart;

    if (endpoint == 'RinfPrint') {
      final rustReport = utf8.decode(binary);
      print(rustReport);
      return;
    }
    assignRustSignal[endpoint]!(messageBytes, binary);
  }).toJS;
}

void startRustLogicReal() {
  if (wasAlreadyLoaded) {
    return;
  }
  wasmBindingsObject.callMethod('rinf_start_rust_logic_extern'.toJS);
}

void stopRustLogicReal() {
  // Dummy function to match the structure of native platforms.
}

void sendDartSignalReal(
  String endpointSymbol,
  Uint8List messageBytes,
  Uint8List binary,
) {
  wasmBindingsObject.callMethod(
    endpointSymbol.toJS,
    messageBytes.toJS,
    binary.toJS,
  );
}
