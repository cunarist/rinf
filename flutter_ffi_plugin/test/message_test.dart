import 'dart:io';

import 'package:test/test.dart';

import '../bin/src/config.dart';
import '../bin/src/message.dart';

void main() {
  group('generateMessageCode', () {
    test('should generate message code without error', () async {
      // Arrange
      final messageConfig = RinfConfigMessage(
          inputDir: 'test_data',
          rustOutputDir: 'build/test_out/rust',
          dartOutputDir: 'build/test_out/dart');

      // Act
      await generateMessageCode(messageConfig: messageConfig);

      // Assert
      // Verify issue #306
      final file = File('build/test_out/rust/enum_and_oneof.rs');
      final linesToFind = [
        'impl SampleOutput {',
        '    pub fn send_signal_to_dart(&self, blob: Option<Vec<u8>>) {',
      ];
      final fileContents = file.readAsStringSync();
      for (final line in linesToFind) {
        expect(fileContents.contains(line), isTrue);
      }
    });

    // Add more tests here if needed
  });
}
