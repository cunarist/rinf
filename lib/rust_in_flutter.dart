/// This module supports communication with Rust.
/// More specifically, sending requests and
/// receiving responses updates are supported.
/// This `wrapper.dart` includes everything you need,
/// so do not import anything else inside the `bridge` folder.
/// DO NOT EDIT.

import 'dart:math';
import 'dart:async';
import 'dart:typed_data';
import 'src/bridge_definitions.dart';
import 'src/ffi.dart' if (dart.library.html) 'ffi_web.dart';

export 'src/bridge_definitions.dart';

/// Listens to a stream from Rust and broadcasts the data in Dart.
/// You can see the usage example at
/// https://pub.dev/packages/rust_in_flutter/example.
final rustBroadcaster = StreamController<RustSignal>.broadcast();
final _responseBroadcaster = StreamController<RustResponseUnique>.broadcast();
final _requestIdGenerator = _IdGenerator();

class RustInFlutter {
  static Future<void> ensureInitialized() async {
    api.prepareChannels();
    final rustSignalStream = api.prepareRustSignalStream();
    rustSignalStream.listen((rustSignal) {
      rustBroadcaster.add(rustSignal);
    });
    final responseStream = api.prepareRustResponseStream();
    responseStream.listen((responseUnique) {
      _responseBroadcaster.add(responseUnique);
    });
    await Future.delayed(const Duration(milliseconds: 100));
    api.startRustLogic();
  }
}

/// Sends bytes data to Rust wrapped in `RustRequest` object
/// with operation and address fields.
/// This concept is very similar to HTTP request.
/// Returns `RustResponse` object that is somehow calculated
/// from the Rust side.
/// You can see the usage example at
/// https://pub.dev/packages/rust_in_flutter/example.
Future<RustResponse> requestToRust(RustRequest rustRequest) async {
  final id = _requestIdGenerator.generateId();
  final requestUnique = RustRequestUnique(id: id, request: rustRequest);
  api.requestToRust(requestUnique: requestUnique);
  final future = _responseBroadcaster.stream.firstWhere((responseUnique) {
    return responseUnique.id == id;
  });
  final responseUnique = await future.timeout(
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
  final rustResponse = responseUnique.response;
  return rustResponse;
}

class _IdGenerator {
  int _counter = -pow(2, 31).toInt();
  final _maxLimit = pow(2, 31).toInt() - 1;
  int generateId() {
    final id = _counter;
    final increased = _counter + 1;
    _counter = increased <= _maxLimit ? increased : -pow(2, 31).toInt();
    return id;
  }
}
