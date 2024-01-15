import 'dart:io';

import 'package:pubspec_parse/pubspec_parse.dart';
import 'package:yaml/yaml.dart';

const DEFAULT_RUST_TYPES_OUTPUT_DIR = "native/hub/src/messages";
const DEFAULT_MESSAGES_INPUT_DIR = "messages";

const KEY_MESSAGES_INPUT_DIR = "messagesInputDir";
const KEY_RUST_TYPES_OUTPUT_DIR = "rustTypesOutputDir";

const RINF_CONFIG_KEYS = [KEY_MESSAGES_INPUT_DIR, KEY_RUST_TYPES_OUTPUT_DIR];

class RinfConfig {
  final String messagesInputDir;
  final String rustTypesOutputDir;

  RinfConfig._({
    required this.messagesInputDir,
    required this.rustTypesOutputDir,
  });

  factory RinfConfig.defaultConfig() {
    return RinfConfig._(
      messagesInputDir: DEFAULT_MESSAGES_INPUT_DIR,
      rustTypesOutputDir: DEFAULT_RUST_TYPES_OUTPUT_DIR,
    );
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
    return RinfConfig._(
      messagesInputDir:
          yaml[KEY_MESSAGES_INPUT_DIR] ?? DEFAULT_MESSAGES_INPUT_DIR,
      rustTypesOutputDir:
          yaml[KEY_RUST_TYPES_OUTPUT_DIR] ?? DEFAULT_RUST_TYPES_OUTPUT_DIR,
    );
  }

  @override
  String toString() {
    return '''RinfConfig {
  messagesInputDir: $messagesInputDir
  rustTypesOutputDir: $rustTypesOutputDir
}''';
  }
}

/// Attempts to load the rinf configuration from the provided pubspec.yaml file.
///
/// If no rinf configuration is found, the default configuration is returned.
/// If the rinf configuration is invalid, an exception is thrown.
/// Otherwise it loads applies all values found in the config, using defaults for missing values.
///
/// Example:
///
/// ```yaml
/// flutter:
///   rinf:
///     messagesInputDir: messages
///     rustTypesOutputDir: native/hub/src/messages
/// ```
Future<RinfConfig> loadVerifiedRinfConfig(String pubspecYamlFile) async {
  final pubspec = Pubspec.parse(await File(pubspecYamlFile).readAsString());
  final YamlMap? rinfConfig = pubspec.flutter?['rinf'];
  return rinfConfig == null
      ? RinfConfig.defaultConfig()
      : RinfConfig.fromYaml(rinfConfig);
}
