// ignore_for_file: avoid_web_libraries_in_flutter

import 'load_web.dart';
import 'dart:typed_data';
import 'dart:js' as js;
import 'interface.dart';
import 'dart:async';
import 'dart:convert';

void setCompiledLibPathExtern(String? path) {
  setJsLibPath(path);
}

Future<void> prepareInterfaceExtern(
  HandleRustSignal handleRustSignal,
) async {
  await loadJsFile();

  // Listen to Rust via JavaScript
  final jsObject = js.context['rinf'] as js.JsObject;
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
    handleRustSignal(messageId, messageBytes, binary);
  };
}

void startRustLogicExtern() {
  if (wasAlreadyLoaded) {
    return;
  }
  final jsObject = js.context['rinf'] as js.JsObject;
  jsObject.callMethod('start_rust_logic_extern', []);
}

void sendDartSignalExtern(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) {
  final jsObject = js.context['rinf'] as js.JsObject;
  jsObject.callMethod('send_dart_signal_extern', [
    messageId,
    messageBytes,
    binary,
  ]);
}
