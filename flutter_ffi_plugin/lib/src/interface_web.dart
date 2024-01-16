import 'load_web.dart';
import 'package:js/js.dart';
import 'dart:typed_data';
import 'package:universal_html/js.dart' as js;
import 'interface.dart';
import 'dart:async';

Future<void> prepareNativeBridge(ReceiveSignal handleSignal) async {
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
    Uint8List? blob;
    if (blobValid) {
      blob = blobBytes;
    } else {
      blob = null;
    }
    handleSignal(messageId, messageBytes, blob);
  };
  js.context['rinf_send_rust_report_extern'] = (String rustReport) {
    print(rustReport);
  };

  startRustLogicExtern();
}

@JS('wasm_bindgen.start_rust_logic_extern')
external void startRustLogicExtern();

@JS('wasm_bindgen.stop_rust_logic_extern')
external void stopRustLogicExtern();

@JS('wasm_bindgen.send_dart_signal_extern')
external void sendDartSignalExtern(
  int messageId,
  Uint8List messageBytes,
  bool blobValid,
  Uint8List blobBytes,
);
