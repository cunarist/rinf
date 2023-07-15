/// This module supports communication with Rust.
/// More specifically, sending requests to Rust and
/// receiving stream signals from Rust are possible.

import 'dart:math';
import 'dart:async';
import 'dart:typed_data';
import 'src/bridge_definitions.dart';
import 'src/ffi.dart' if (dart.library.html) 'ffi_web.dart';

export 'src/bridge_definitions.dart' show Operation;
export 'src/bridge_definitions.dart' show RustRequest;
export 'src/bridge_definitions.dart' show RustResponse;
export 'src/bridge_definitions.dart' show RustSignal;

/// Listens to a stream from Rust and broadcasts the data in Dart.
/// You can see the usage example at
/// https://pub.dev/packages/rust_in_flutter/example.
final rustBroadcaster = StreamController<RustSignal>.broadcast();
final _responseBroadcaster = StreamController<RustResponseUnique>.broadcast();
final _requestIdGenerator = _IdGenerator();

/// Contains basic functionalities of this package.
class RustInFlutter {
  /// Makes sure that the Rust side is ready.
  /// Don't forget to call this function in the `main` function of Dart.
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
/// This system follows the definition of RESTful API
/// and is very similar to HTTP request.
/// Returns `RustResponse` object that is somehow calculated
/// from the Rust side.
/// If the Rust doesn't respond for 10 seconds,
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
