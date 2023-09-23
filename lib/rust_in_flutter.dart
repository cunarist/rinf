/// This module supports communication with Rust.
/// More specifically, sending requests to Rust and
/// receiving stream signals from Rust are possible.

import 'dart:math';
import 'dart:async';
import 'src/exports.dart';

export 'src/exports.dart' show RustOperation;
export 'src/exports.dart' show RustRequest;
export 'src/exports.dart' show RustResponse;
export 'src/exports.dart' show RustSignal;

/// Listens to a stream from Rust and broadcasts the data in Dart.
/// You can see the usage example at
/// https://pub.dev/packages/rust_in_flutter/example.
final rustBroadcaster = StreamController<RustSignal>.broadcast();
final _responseBroadcaster = StreamController<RustResponseUnique>.broadcast();
final _requestIdGenerator = _IdGenerator();

/// Contains basic functionalities of this framework.
class RustInFlutter            {
  /// Makes sure that the Rust side is ready.
  /// Don't forget to call this function in the `main` function of Dart.
  static Future<void> ensureInitialized() async {
    await api.prepareChannels();
    final rustSignalStream = api.prepareRustSignalStream();
    rustSignalStream.listen((rustSignal) {
      rustBroadcaster.add(rustSignal);
    });
    final rustResponseStream = api.prepareRustResponseStream();
    rustResponseStream.listen((responseUnique) {
      _responseBroadcaster.add(responseUnique);
    });
    while (!(await api.checkRustStreams())) {}
    api.startRustLogic();
  }
}

/// Sends bytes data to Rust wrapped in `RustRequest` object
/// with operation and address fields.
/// This system follows the definition of RESTful API
/// and is very similar to HTTP request.
/// Returns `RustResponse` object that is somehow calculated
/// from the Rust side.
/// If the Rust doesn't respond for 60 seconds,
/// this function will return a failed `RustResponse` object.
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
    const Duration(seconds: 60),
    onTimeout: () {
      return RustResponseUnique(
        id: id,
        response: RustResponse(
          successful: false,
          message: null,
          blob: null,
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
