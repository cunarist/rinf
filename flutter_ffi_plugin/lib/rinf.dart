/// This module supports communication with Rust.
/// More specifically, sending requests to Rust and
/// receiving stream signals from Rust are possible.

import 'dart:async';
import 'src/exports.dart';

export 'src/exports.dart' show sendDartSignalExtern;
export 'src/exports.dart' show RustSignal;

/// Contains basic functionalities of this framework.
class Rinf {
  /// Makes sure that the Rust side is ready.
  /// Don't forget to call this function in the `main` function of Dart.
  static Future<void> ensureInitialized(ReceiveMessages receiveMessages) async {
    await prepareNativeBridge(receiveMessages);
  }

  /// Ensure that all Rust tasks are terminated
  /// by calling this function before closing the Flutter app.
  /// Doing so can prevent potential memory errors that may occur
  /// when Rust attempts to send data after the Dart VM has been turned off.
  /// Please note that on the web, this function does not have any effect,
  /// as tasks are managed by the JavaScript runtime, not Rust.
  static Future<void> ensureFinalized() async {
    stopRustLogicExtern();
  }
}
