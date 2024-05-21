import 'dart:io';
import 'package:yaml/yaml.dart';

class RinfConfigMessage {
  final String inputDir;
  final String rustOutputDir;
  final String dartOutputDir;
  final bool rustSerde;

  RinfConfigMessage({
    required this.inputDir,
    required this.rustOutputDir,
    required this.dartOutputDir,
    required this.rustSerde,
  });

  factory RinfConfigMessage.defaultConfig() {
    return RinfConfigMessage(
        inputDir: DEFAULT_INPUT_DIR,
        rustOutputDir: DEFAULT_RUST_OUTPUT_DIR,
        dartOutputDir: DEFAULT_DART_OUTPUT_DIR,
        rustSerde: DEFAULT_RUST_SERDE);
  }

  factory RinfConfigMessage.from(YamlMap yaml) {
    for (final key in yaml.keys) {
      if (!MESSAGE_CONFIG_KEYS.contains(key)) {
        throw Exception(
          "Unknown key '$key' in rinf message configuration.\n"
          "Available keys are: $MESSAGE_CONFIG_KEYS",
        );
      }
    }
    return RinfConfigMessage(
      inputDir: yaml[KEY_INPUT_DIR] ?? DEFAULT_INPUT_DIR,
      rustOutputDir: yaml[KEY_RUST_OUTPUT_DIR] ?? DEFAULT_RUST_OUTPUT_DIR,
      dartOutputDir: yaml[KEY_DART_OUTPUT_DIR] ?? DEFAULT_DART_OUTPUT_DIR,
      rustSerde: yaml[KEY_RUST_SERDE] ?? DEFAULT_RUST_SERDE,
    );
  }

  @override
  String toString() {
    return '''message:
    $KEY_INPUT_DIR: $inputDir
    $KEY_RUST_OUTPUT_DIR: $rustOutputDir
    $KEY_DART_OUTPUT_DIR: $dartOutputDir''';
  }

  static const KEY_INPUT_DIR = "input_dir";
  static const KEY_RUST_OUTPUT_DIR = "rust_output_dir";
  static const KEY_DART_OUTPUT_DIR = "dart_output_dir";
  static const KEY_RUST_SERDE = "rust_serde";

  static const DEFAULT_INPUT_DIR = "messages/";
  static const DEFAULT_RUST_OUTPUT_DIR = "native/hub/src/messages/";
  static const DEFAULT_DART_OUTPUT_DIR = "lib/messages/";
  static const DEFAULT_RUST_SERDE = false;

  static const MESSAGE_CONFIG_KEYS = [
    KEY_INPUT_DIR,
    KEY_RUST_OUTPUT_DIR,
    KEY_DART_OUTPUT_DIR,
    KEY_RUST_SERDE
  ];
}

class RinfConfig {
  final RinfConfigMessage message;

  RinfConfig({required this.message});

  factory RinfConfig.defaultConfig() {
    return RinfConfig(message: RinfConfigMessage.defaultConfig());
  }

  factory RinfConfig.fromYaml(YamlMap yaml) {
    for (final key in yaml.keys) {
      if (!RINF_CONFIG_KEYS.contains(key)) {
        throw Exception(
          "Unknown key '$key' in rinf configuration.\n"
          "Available keys are: $RINF_CONFIG_KEYS",
        );
      }
    }
    final YamlMap? messageYaml = yaml[KEY_MESSAGE];
    final message = messageYaml == null
        ? RinfConfigMessage.defaultConfig()
        : RinfConfigMessage.from(messageYaml);

    return RinfConfig(message: message);
  }

  @override
  String toString() {
    return '''
rinf:
  $message''';
  }

  static const KEY_MESSAGE = "message";
  static const RINF_CONFIG_KEYS = [KEY_MESSAGE];
}

/// Attempts to load the rinf configuration from the provided pubspec.yaml file.
///
/// If no rinf configuration is found, the default configuration is returned.
/// If the rinf configuration is invalid, an exception is thrown.
/// Otherwise it loads all values found in the config, using defaults for missing values.
///
/// Example:
///
/// ```yaml
/// rinf:
///   message: message:
///     input_dir: messages
///     rust_output_dir: native/hub/src/messages
///     dart_output_dir: lib/messages
/// ```
Future<RinfConfig> loadVerifiedRinfConfig(String pubspecYamlFile) async {
  final pubspec = loadYaml(await File(pubspecYamlFile).readAsString());
  final YamlMap? rinfConfig = pubspec['rinf'];
  return rinfConfig == null
      ? RinfConfig.defaultConfig()
      : RinfConfig.fromYaml(rinfConfig);
}
