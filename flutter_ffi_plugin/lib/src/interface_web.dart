// ignore_for_file: avoid_web_libraries_in_flutter

import 'load_web.dart';
import 'dart:typed_data';
import 'dart:js' as js;
import 'interface.dart';
import 'dart:async';
import 'dart:convert';

Future<void> prepareNativeBridge(HandleRustSignal handleRustSignal) async {
  final isAlreadyPrepared = await loadJsFile();

  if (isAlreadyPrepared) {
    return;
  }

  // Listen to Rust via JavaScript
  js.context['rinf_send_rust_signal_extern'] = (
    int messageId,
    Uint8List messageBytes,
    bool blobValid,
    Uint8List blobBytes,
  ) {
    if (messageId == -1) {
      // -1 is a special message ID for Rust reports.
      String rustReport = utf8.decode(blobBytes);
      print(rustReport);
    }
    Uint8List? blob;
    if (blobValid) {
      blob = blobBytes;
    } else {
      blob = null;
    }
    handleRustSignal(messageId, messageBytes, blob);
  };

  startRustLogicExtern();
}

void startRustLogicExtern() {
  final jsObject = js.context['wasm_bindgen'] as js.JsObject;
  jsObject.callMethod('start_rust_logic_extern', []);
}

void stopRustLogicExtern() {
  // Dummy function to match that of the OS module.
}

void sendDartSignalExtern(
  int messageId,
  Uint8List messageBytes,
  bool blobValid,
  Uint8List blobBytes,
) {
  final jsObject = js.context['wasm_bindgen'] as js.JsObject;
  jsObject.callMethod('send_dart_signal_extern', [
    messageId,
    messageBytes,
    blobValid,
    blobBytes,
  ]);
}
