// ignore_for_file: avoid_web_libraries_in_flutter

import 'load_web.dart';
import 'dart:typed_data';
import 'dart:js' as js;
import 'interface.dart';
import 'dart:async';
import 'dart:convert';

Future<void> prepareInterfaceExtern(
  HandleRustSignal handleRustSignal,
) async {
  await loadJsFile();

  // Listen to Rust via JavaScript
  final jsObject = js.context['rinf'] as js.JsObject;
  jsObject['send_rust_signal_extern'] = (
    int messageId,
    Uint8List messageBytes,
    bool binaryIncluded,
    Uint8List binaryBytes,
  ) {
    if (messageId == -1) {
      // -1 is a special message ID for Rust reports.
      String rustReport = utf8.decode(binaryBytes);
      print(rustReport);
      return;
    }
    Uint8List? binary;
    if (binaryIncluded) {
      binary = binaryBytes;
    } else {
      binary = null;
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

void stopRustLogicExtern() {
  // Dummy function to match that of the OS module.
}

void sendDartSignalExtern(
  int messageId,
  Uint8List messageBytes,
  bool binaryIncluded,
  Uint8List binaryBytes,
) {
  final jsObject = js.context['rinf'] as js.JsObject;
  jsObject.callMethod('send_dart_signal_extern', [
    messageId,
    messageBytes,
    binaryIncluded,
    binaryBytes,
  ]);
}
