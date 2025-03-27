// Import the test package and Counter class
import 'dart:io';
import 'package:test/test.dart';
import 'package:rinf/rinf.dart';
import 'package:example_app/src/bindings/bindings.dart';

void main() async {
  // Build the dynamic library and load it.
  await Process.run('cargo', ['build'], runInShell: true);
  await initializeRust(assignRustSignal, compiledLibPath: getLibPath());

  // Run the test.
  test('Numbers should be the same', () {
    expect(1, 2);
  });
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
