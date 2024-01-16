import 'load_web.dart';
import 'package:js/js.dart';
import 'dart:typed_data';
import 'package:universal_html/js.dart' as js;
import 'interface.dart';
import 'dart:async';

final rustSignalStream = StreamController<RustSignal>();
final rustResponseUniqueStream = StreamController<RustResponseUnique>();
final rustReportStream = StreamController<String>();

Future<void> prepareNativeBridge() async {
  final isAlreadyPrepared = await loadJsFile();

  if (isAlreadyPrepared) {
    return;
  }

  js.context['rinf_send_rust_signal_extern'] = (
    int resource,
    Uint8List message,
    Uint8List blob,
  ) {
    final rustSignal = RustSignal(
      resource: resource,
      message: message.length == 0 ? null : message,
      blob: blob.length == 0 ? null : blob,
    );
    rustSignalStream.add(rustSignal);
  };

  js.context['rinf_respond_to_dart_extern'] = (
    int id,
    bool successful,
    Uint8List message,
    Uint8List blob,
  ) {
    final RustResponse? rustResponse;
    if (successful) {
      rustResponse = RustResponse(
        message: message.length == 0 ? null : message,
        blob: blob.length == 0 ? null : blob,
      );
    } else {
      rustResponse = null;
    }
    final rustResponseUnique = RustResponseUnique(
      id: id,
      response: rustResponse,
    );
    rustResponseUniqueStream.add(rustResponseUnique);
  };

  js.context['rinf_send_rust_report_extern'] = (String rustReport) {
    rustReportStream.add(rustReport);
  };

  startRustLogicExtern();
}

@JS('wasm_bindgen.start_rust_logic_extern')
external void startRustLogicExtern();

@JS('wasm_bindgen.stop_rust_logic_extern')
external void stopRustLogicExtern();

@JS('wasm_bindgen.request_to_rust_extern')
external void requestToRustExternRaw(
  int interactionId,
  int resource,
  int operation,
  Uint8List messageRaw,
  Uint8List blobRaw,
);

void requestToRustExtern(RustRequestUnique rustRequestUnique) async {
  final interactionId = rustRequestUnique.id;
  final rustRequest = rustRequestUnique.request;

  final int rustOperation;
  if (rustRequest.operation == RustOperation.Create) {
    rustOperation = 0;
  } else if (rustRequest.operation == RustOperation.Read) {
    rustOperation = 1;
  } else if (rustRequest.operation == RustOperation.Update) {
    rustOperation = 2;
  } else {
    rustOperation = 3;
  }

  requestToRustExternRaw(
    interactionId,
    rustRequest.resource,
    rustOperation,
    rustRequest.message ?? Uint8List(0),
    rustRequest.blob ?? Uint8List(0),
  );
}

@JS('wasm_bindgen.prepare_isolates_extern')
external void prepareIsolatesExtern(
  int portSignal,
  int portResponse,
  int portReport,
);
