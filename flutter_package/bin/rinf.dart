import 'package:args/command_runner.dart';
import 'package:chalkdart/chalkstrings.dart';

import 'src/config.dart';
import 'src/helpers.dart';
import 'src/message.dart';
import 'src/internet.dart';
import 'src/common.dart';

Future<void> main(List<String> args) async {
  // When running `dart run rinf`,
  // Unnecessary two lines of
  //`Building package executable...\nBuilt rinf:rinf.` appear.
  // Remove those before proceeding.
  removeCliLines(2);

  // Check the internet connection status and rembember it.
  await checkConnectivity();

  // Parse CLI arguments and run the corresponding function.
  final runner = CommandRunner(
    'rinf',
    'Helper commands for building apps with Rust in Flutter',
    usageLineLength: 80,
  )
    ..addCommand(ConfigCommand())
    ..addCommand(TemplateCommand())
    ..addCommand(MessageCommand())
    ..addCommand(WasmCommand())
    ..addCommand(ServerCommand());

  try {
    await runner.run(args);
  } catch (error) {
    // Print the error gracefully without backtrace.
    print(error.toString().trim().red);
  }
}

class ConfigCommand extends Command {
  final name = 'config';
  final description = 'Shows Rinf configuration resolved from `pubspec.yaml`';

  ConfigCommand() {}

  Future<void> run() async {
    final rinfConfig = await loadVerifiedRinfConfig('pubspec.yaml');
    print(rinfConfig.toString().dim);
  }
}

class TemplateCommand extends Command {
  final name = 'template';
  final description = 'Applies Rust template to the current Flutter project';

  TemplateCommand() {}

  Future<void> run() async {
    final rinfConfig = await loadVerifiedRinfConfig('pubspec.yaml');
    await applyRustTemplate(messageConfig: rinfConfig.message);
  }
}

class MessageCommand extends Command {
  final name = 'message';
  final description = 'Generates message code from `.proto` files';

  MessageCommand() {
    argParser.addFlag(
      'watch',
      abbr: 'w',
      help: 'Continuously watches `.proto` files',
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
  final description = 'Builds the webassembly module for the web'
      ' with `wasm-pack`';

  WasmCommand() {
    argParser.addFlag(
      'release',
      abbr: 'r',
      help: 'Builds in release mode',
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

class ServerCommand extends Command {
  final name = 'server';
  final description = 'Shows how to run Flutter web server with web headers';

  ServerCommand() {}

  Future<void> run() async {
    final commandLines = [
      'flutter run',
      '--web-header=Cross-Origin-Opener-Policy=same-origin',
      '--web-header=Cross-Origin-Embedder-Policy=require-corp'
    ];
    print(commandLines.join(' ').dim);
  }
}
