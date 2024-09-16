import 'package:args/command_runner.dart';

import 'src/config.dart';
import 'src/helpers.dart';
import 'src/message.dart';
import 'src/internet.dart';

Future<void> main(List<String> args) async {
  await checkConnectivity();

  final runner = CommandRunner(
    'rinf',
    'Helper commands for building apps with Rust in Flutter.',
    usageLineLength: 80,
  )
    ..addCommand(ConfigCommand())
    ..addCommand(TemplateCommand())
    ..addCommand(MessageCommand())
    ..addCommand(WasmCommand());

  await runner.run(args);
}

class ConfigCommand extends Command {
  final name = 'config';
  final description = 'Shows current Rinf configuration' +
      ' resolved from `pubspec.yaml` with defaults applied.';

  ConfigCommand() {}

  Future<void> run() async {
    final rinfConfig = await loadVerifiedRinfConfig('pubspec.yaml');
    print(rinfConfig);
  }
}

class TemplateCommand extends Command {
  final name = 'template';
  final description = 'Applies Rust template to the current Flutter project.';

  TemplateCommand() {}

  Future<void> run() async {
    final rinfConfig = await loadVerifiedRinfConfig('pubspec.yaml');
    await applyRustTemplate(messageConfig: rinfConfig.message);
  }
}

class MessageCommand extends Command {
  final name = 'message';
  final description = 'Generates message code from `.proto` files.';

  MessageCommand() {
    argParser.addFlag(
      'watch',
      abbr: 'w',
      help: 'Continuously watches `.proto` files.',
    );
  }

  Future<void> run() async {
    final results = argResults;
    if (results == null) {
      return;
    }
    final watch = results.flag('watch');
    final rinfConfig = await loadVerifiedRinfConfig('pubspec.yaml');
    if (watch) {
      await watchAndGenerateMessageCode(messageConfig: rinfConfig.message);
    } else {
      await generateMessageCode(messageConfig: rinfConfig.message);
    }
  }
}

class WasmCommand extends Command {
  final name = 'wasm';
  final description = 'Builds the webassembly module for the web.';

  WasmCommand() {
    argParser.addFlag(
      'release',
      abbr: 'r',
      help: 'Builds in release mode.',
    );
  }

  Future<void> run() async {
    final results = argResults;
    if (results == null) {
      return;
    }
    final release = results.flag('release');
    await buildWebassembly(release);
  }
}
