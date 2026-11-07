// Import the test package and Counter class
import 'dart:convert';
import 'dart:io';
import 'package:test/test.dart';
import 'package:rinf/rinf.dart';
import 'package:example_app/src/bindings/bindings.dart';

Future<void> main() async {
  // Build the dynamic library and load it.
  await Process.run('cargo', ['build'], runInShell: true);
  await initializeRust(assignRustSignal, compiledLibPath: getLibPath());

  // Start the test mechanism in Rust.
  const duration = Duration(milliseconds: 100);
  UnitTestStart().sendSignalToRust();
  await Future.delayed(duration);

  // Send signals of complex types back and forth.
  SerdeData.rustSignalStream.listen((signalPack) async {
    // Receive a signal from Rust and send it back.
    final serdeData = signalPack.message;
    serdeData.sendSignalToRust();
    final resultPack = await ComplexSignalTestResult.rustSignalStream.first;
    test('Signals should remain the same', () {
      expect(
        resultPack.message.value,
        true,
        reason: 'Signal data is different from the original:\n$serdeData',
      );
    });
  });

  // Wait for the test to be completed.
  await Future.delayed(duration);
  await UnitTestEnd.rustSignalStream.first;
}

/// Gets the path of the dynamic library file.
/// Cargo is asked for its target directory,
/// which can be changed by `CARGO_TARGET_DIR` or Cargo config files.
String getLibPath() {
  const args = ['metadata', '--format-version', '1', '--no-deps'];
  final metadata = Process.runSync('cargo', args, runInShell: true);
  final targetDir =
      jsonDecode(metadata.stdout as String)['target_directory'] as String;
  if (Platform.isMacOS) {
    return '$targetDir/debug/libhub.dylib';
  } else if (Platform.isLinux) {
    return '$targetDir/debug/libhub.so';
  } else if (Platform.isWindows) {
    return '$targetDir/debug/hub.dll';
  } else {
    throw UnsupportedError('This operating system is not for tests');
  }
}
