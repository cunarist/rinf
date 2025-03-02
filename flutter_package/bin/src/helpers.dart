import 'dart:io';

import 'package:package_config/package_config.dart';
import 'package:yaml/yaml.dart';

import 'config.dart';
import 'message.dart';
import 'common.dart';
import 'internet.dart';
import 'progress.dart';

/// Creates new folders and files to an existing Flutter project folder.
Future<void> applyRustTemplate({
  required RinfConfigMessage messageConfig,
}) async {
  // Get the path of the current project directory
  final flutterProjectPath = Directory.current.uri;

  // Get the package directory path
  final packageConfig = await findPackageConfig(Directory.current);
  if (packageConfig == null) {
    return;
  }
  final packageName = 'rinf';
  final package = packageConfig.packages.firstWhere(
    (p) => p.name == packageName,
  );
  final packagePath = package.root;

  // Check if current folder is a Flutter app project.
  final specFile = File.fromUri(flutterProjectPath.join('pubspec.yaml'));
  final pubspec = loadYaml(await specFile.readAsString());
  final String? publishTo = pubspec['publish_to'];
  if (publishTo != 'none') {
    print('Flutter package development is not supported by Rinf.');
    return;
  }

  // Copy basic folders needed for Rust to work
  final templateSource = packagePath.join('template/native/');
  final templateDestination = flutterProjectPath.join('native/');
  await copyDirectory(templateSource, templateDestination);

  // Create workspace `Cargo.toml`
  final cargoText = '''
# This file is used for telling Rust-related tools
# where various Rust crates are.
# This also unifies `./target` output folder and
# various Rust configurations.

[workspace]
members = ["./native/*"]
resolver = "2"
''';
  final cargoFile = File.fromUri(flutterProjectPath.join('Cargo.toml'));
  await cargoFile.writeAsString(cargoText);

  // Add some lines to `.gitignore`
  final rustSectionTitle = '# Rust related';
  final messageSectionTitle = '# Generated messages';
  final gitignoreFile = File.fromUri(flutterProjectPath.join('.gitignore'));
  if (!(await gitignoreFile.exists())) {
    await gitignoreFile.create(recursive: true);
  }
  final gitignoreContent = await gitignoreFile.readAsString();
  var gitignoreSplitted = gitignoreContent.split('\n\n');
  gitignoreSplitted = gitignoreSplitted.map((s) => s.trim()).toList();
  if (!gitignoreContent.contains(rustSectionTitle)) {
    var text = rustSectionTitle;
    text += '\n' + '.cargo/';
    text += '\n' + 'target/';
    gitignoreSplitted.add(text);
  }
  if (!gitignoreContent.contains(messageSectionTitle)) {
    var text = messageSectionTitle;
    text += '\n' + '*/**/messages/';
    gitignoreSplitted.add(text);
  }
  await gitignoreFile.writeAsString(gitignoreSplitted.join('\n\n') + '\n');

  // Add some guides to `README.md`
  final guideSectionTitle = '## Using Rust Inside Flutter';
  final readmeFile = File.fromUri(flutterProjectPath.join('README.md'));
  if (!(await readmeFile.exists())) {
    await readmeFile.create(recursive: true);
  }
  final readmeContent = await readmeFile.readAsString();
  var readmeSplitted = readmeContent.split('\n\n');
  readmeSplitted = readmeSplitted.map((s) => s.trim()).toList();
  if (!readmeContent.contains(guideSectionTitle)) {
    final text = """
$guideSectionTitle

This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rinf](https://pub.dev/packages/rinf) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install)
and [Rust toolchain](https://www.rust-lang.org/tools/install)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```shell
rustc --version
flutter doctor
```

You also need to have the CLI tool for Rinf ready.

```shell
cargo install rinf
```

Signals sent between Dart and Rust are implemented using signal attributes.
If you've modified the signal structs, run the following command
to generate the corresponding Dart classes:

```shell
rinf gen
```

Now you can run and build this app just like any other Flutter projects.

```shell
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rinf's [documentation](https://rinf.cunarist.com).
""";
    readmeSplitted.add(text);
  }
  await readmeFile.writeAsString(readmeSplitted.join('\n\n') + '\n');

  // Add Dart dependencies
  await Process.run('dart', ['pub', 'add', 'meta']);
  await Process.run('dart', ['pub', 'add', 'tuple']);

  // Modify `./lib/main.dart`
  final mainFile = File.fromUri(flutterProjectPath.join('lib/main.dart'));
  if (await mainFile.exists()) {
    await Process.run('dart', ['format', './lib/main.dart']);
    var mainText = await mainFile.readAsString();
    if (!mainText.contains('messages/all.dart')) {
      final lines = mainText.split('\n');
      final lastImportIndex = lines.lastIndexWhere(
        (line) => line.startsWith('import '),
      );
      lines.insert(
        lastImportIndex + 1,
        "import 'package:rinf/rinf.dart';",
      );
      lines.insert(
        lastImportIndex + 2,
        "import './messages/all.dart';",
      );
      mainText = lines.join('\n');
    }
    if (!mainText.contains('initializeRust(assignRustSignal)')) {
      mainText = mainText.replaceFirst(
        'main() {',
        'main() async {',
      );
      mainText = mainText.replaceFirst(
        'main() async {',
        'main() async { await initializeRust(assignRustSignal);',
      );
    }
    await mainFile.writeAsString(mainText);
    await Process.run('dart', ['format', './lib/main.dart']);
  }

  await generateMessageCode(silent: true, messageConfig: messageConfig);

  print('Rust template is now ready 🎉');
}

Future<void> copyDirectory(Uri source, Uri destination) async {
  final sourceDir = Directory.fromUri(source);
  await Directory.fromUri(destination).create(recursive: true);
  await for (final entity in sourceDir.list()) {
    final entityName = entity.path.split(Platform.pathSeparator).last;
    if (entity is Directory) {
      final newDirectory = Directory.fromUri(
        destination.join('$entityName/'),
      );
      await newDirectory.create();
      await copyDirectory(entity.uri, newDirectory.uri);
    } else if (entity is File) {
      await entity.copy(
        destination.join(entityName).toFilePath(),
      );
    }
  }
}

Future<void> buildWebassembly(bool isReleaseMode) async {
  final fillingBar = ProgressBar(
    total: 3,
    width: 12,
  );
  // Ensure Rust toolchain.
  if (isInternetConnected) {
    fillingBar.desc = 'Installing Rust toolchain for the web';
    final processResults = <ProcessResult>[];
    processResults.add(await Process.run('rustup', [
      'toolchain',
      'install',
      'nightly',
    ]));
    processResults.add(await Process.run('rustup', [
      '+nightly',
      'component',
      'add',
      'rust-src',
    ]));
    processResults.add(await Process.run('rustup', [
      '+nightly',
      'target',
      'add',
      'wasm32-unknown-unknown',
    ])); // For actual compilation
    processResults.add(await Process.run('rustup', [
      'target',
      'add',
      'wasm32-unknown-unknown',
    ])); // For Rust-analyzer
    processResults.add(await Process.run('cargo', [
      'install',
      'wasm-pack',
    ]));
    processResults.add(await Process.run('cargo', [
      'install',
      'wasm-bindgen-cli',
    ]));
    processResults.forEach((processResult) {
      if (processResult.exitCode != 0) {
        throw Exception(processResult.stderr);
      }
    });
  } else {
    fillingBar.desc = 'Skipping ensurement of Rust toolchain for the web';
  }

  // Prepare the webassembly output path.
  final flutterProjectPath = Directory.current;
  final subPath = 'web/pkg/';
  final outputPath = flutterProjectPath.uri.join(subPath);

  // Build the webassembly module.
  fillingBar.desc = 'Compiling Rust with `wasm-pack` to `web` target';
  fillingBar.increment();
  final compileCommand = await Process.run(
    'wasm-pack',
    [
      '--quiet',
      'build',
      './native/hub',
      '--out-dir', outputPath.toFilePath(),
      '--out-name', 'hub',
      '--no-typescript',
      '--target', 'web',
      if (!isReleaseMode) '--dev',
      '--', // Cargo build args comes from here
      '-Z', 'build-std=std,panic_abort',
    ],
    environment: {
      'RUSTUP_TOOLCHAIN': 'nightly',
      'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
      if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
    },
  );
  if (compileCommand.exitCode != 0) {
    throw Exception(compileCommand.stderr);
  }
  fillingBar.desc = 'Saved `.wasm` and `.js` files to `$subPath`';
  fillingBar.increment();

  fillingBar.desc = 'Webassembly module is now ready 🎉';
  fillingBar.increment();

  // Guide the developer how to run Flutter web server with web headers.
  print('To get the Flutter web server command, run `rinf server`');
}
