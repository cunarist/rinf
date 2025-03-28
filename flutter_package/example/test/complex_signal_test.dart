// Import the test package and Counter class
import 'dart:io';
import 'package:test/test.dart';
import 'package:rinf/rinf.dart';
import 'package:example_app/src/bindings/bindings.dart';

void main() async {
  // Build the dynamic library and load it.
  await Process.run('cargo', ['build'], runInShell: true);
  await initializeRust(assignRustSignal, compiledLibPath: getLibPath());

  // Start the test mechanism in Rust.
  final duration = Duration(milliseconds: 100);
  ComplexSignalTestStart().sendSignalToRust();
  await Future.delayed(duration);

  // Send signals of complex types back and forth.
  SerdeData.rustSignalStream.listen((signal) async {
    // Receive a signal from Rust and send it back.
    final serdeData = signal.message;
    serdeData.sendSignalToRust();
    final resultSignal = await ComplexSignalTestResult.rustSignalStream.first;
    test('Signals should remain the same', () {
      expect(
        resultSignal.message.value,
        true,
        reason: 'Signal data is different from the original:\n${serdeData}',
      );
    });
  });

  // Wait for the test to be completed.
  await Future.delayed(duration);
  await ComplexSignalTestEnd.rustSignalStream.first;
}

/// Gets the expected path of the dynamic library file.
/// The path should reflect the project folder structure.
String getLibPath() {
  if (Platform.isMacOS) {
    return '../../target/debug/libhub.dylib';
  } else if (Platform.isLinux) {
    return '../../target/debug/libhub.so';
  } else if (Platform.isWindows) {
    return '../../target/debug/hub.dll';
  } else {
    throw UnsupportedError('This operating system is not for tests');
  }
}
