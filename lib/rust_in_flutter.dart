/// This module supports communication with Rust.
/// More specifically, sending requests and
/// receiving responses updates are supported.
/// This `wrapper.dart` includes everything you need,
/// so do not import anything else inside the `bridge` folder.
/// DO NOT EDIT.

import 'dart:math';
import 'dart:async';
import 'dart:typed_data';
import 'bridge_definitions.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

export 'bridge_definitions.dart';

var rustBroadcaster = StreamController<RustSignal>.broadcast();
var _responseBroadcaster = StreamController<RustResponseUnique>.broadcast();
var _requestIdGenerator = IdGenerator();

class RustInFlutter {
  static Future<void> ensureInitialized() async {
    api.prepareChannels();
    var rustSignalStream = api.prepareRustSignalStream();
    rustSignalStream.listen((rustSignal) {
      rustBroadcaster.add(rustSignal);
    });
    var responseStream = api.prepareRustResponseStream();
    responseStream.listen((responseUnique) {
      _responseBroadcaster.add(responseUnique);
    });
    await Future.delayed(const Duration(milliseconds: 100));
    api.startRustLogic();
  }
}

Future<RustResponse> requestToRust(RustRequest request) async {
  final id = _requestIdGenerator.generateId();
  final requestUnique = RustRequestUnique(id: id, request: request);
  api.requestToRust(requestUnique: requestUnique);
  var future = _responseBroadcaster.stream.firstWhere((responseUnique) {
    return responseUnique.id == id;
  });
  var responseUnique = await future.timeout(
    const Duration(seconds: 10),
    onTimeout: () {
      return RustResponseUnique(
        id: id,
        response: RustResponse(
          successful: false,
          bytes: Uint8List(0),
        ),
      );
    },
  );
  var response = responseUnique.response;
  return response;
}

class IdGenerator {
  int _counter = -pow(2, 31).toInt();
  final _maxLimit = pow(2, 31).toInt() - 1;
  int generateId() {
    var id = _counter;
    var increased = _counter + 1;
    _counter = increased <= _maxLimit ? increased : -pow(2, 31).toInt();
    return id;
  }
}
