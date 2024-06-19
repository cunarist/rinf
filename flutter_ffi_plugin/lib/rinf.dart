/// This module supports communication with Rust.
library;

import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'src/exports.dart';

export 'src/interface.dart' show RustSignal;

/// Sets the exact file path of the dynamic library
/// compiled from the `hub` crate.
/// On the web, this function sets the path to the JavaScript module
/// that needs to be loaded.
/// This function might not be necessary for major platforms
/// but can be useful when the app runs on embedded devices.
void setCompiledLibPath(String? path) {
  setCompiledLibPathReal(path);
}

/// This widget manages the lifecycle of the
/// async runtime on the Rust side.
/// In essense, this widget is responsible of
/// creation and graceful shutdown of the async runtime in Rust.
/// The Rust async runtime will be created
/// when the state of this widget is initialized,
/// and it will be shut down when this widget is disposed.
class Rusty extends StatefulWidget {
  final Widget child;
  final AssignRustSignal assignRustSignal;
  const Rusty({
    required this.child,
    required this.assignRustSignal,
  });

  @override
  State<Rusty> createState() => _RustyState();
}

class _RustyState extends State<Rusty> {
  late Future<void> initialLoad;

  @override
  void initState() {
    super.initState();
    initialLoad = () async {
      await prepareInterfaceReal(widget.assignRustSignal);
      startRustLogicReal();
      print("START DART (TEMP)");
    }();
  }

  @override
  void dispose() {
    print("STOP DART (TEMP)");
    stopRustLogicReal();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: initialLoad,
      builder: (context, snapshot) {
        return widget.child;
      },
    );
  }
}

/// Sends a signal to Rust.
void sendDartSignal(
  int messageId,
  Uint8List messageBytes,
  Uint8List binary,
) async {
  sendDartSignalReal(
    messageId,
    messageBytes,
    binary,
  );
}
