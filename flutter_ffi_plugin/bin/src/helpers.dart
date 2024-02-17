import 'dart:io';
import 'package:package_config/package_config.dart';
import 'config.dart';
import 'message.dart';
import 'package:yaml/yaml.dart';

/// Creates new folders and files to an existing Flutter project folder.
Future<void> applyRustTemplate({
  required RinfConfigMessage messageConfig,
}) async {
  // Get the path of the current project directory
  final flutterProjectPath = Directory.current.path;

  // Get the package directory path
  final packageConfig = await findPackageConfig(Directory.current);
  if (packageConfig == null) {
    return;
  }
  final packageName = 'rinf';
  final package = packageConfig.packages.firstWhere(
    (p) => p.name == packageName,
  );
  final packagePath = package.root.toFilePath();

  // Check if current folder is a Flutter app project.
  final specFile = File('$flutterProjectPath/pubspec.yaml');
  final isFlutterProject = await specFile.exists();
  if (!isFlutterProject) {
    print("This folder doesn't look like a Flutter project.");
    return;
  }
  final pubspec = loadYaml(await specFile.readAsString());
  final String? publishTo = pubspec['publish_to'];
  if (publishTo != "none") {
    print("Flutter package development is not supported by Rinf.");
    return;
  }

  // Copy basic folders needed for Rust to work
  final templateSource = Directory('$packagePath/example/native');
  final templateDestination = Directory('$flutterProjectPath/native');
  await copyDirectory(templateSource, templateDestination);
  final messagesSource = Directory('$packagePath/example/messages');
  final messagesDestination = Directory('$flutterProjectPath/messages');
  await copyDirectory(messagesSource, messagesDestination);

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
  final cargoFile = File('$flutterProjectPath/Cargo.toml');
  await cargoFile.writeAsString(cargoText);

  // Disable demonstrations in sample functions
  final sampleFunctionsFile =
      File('$flutterProjectPath/native/hub/src/sample_functions.rs');
  var sampleFunctionsContent = await sampleFunctionsFile.readAsString();
  sampleFunctionsContent = sampleFunctionsContent.replaceAll(
    'SHOULD_DEMONSTRATE: bool = true',
    'SHOULD_DEMONSTRATE: bool = false',
  );
  await sampleFunctionsFile.writeAsString(sampleFunctionsContent);

  // Add some lines to `.gitignore`
  final rustSectionTitle = '# Rust related';
  final messageSectionTitle = '# Generated messages';
  final gitignoreFile = File('$flutterProjectPath/.gitignore');
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
  final readmeFile = File('$flutterProjectPath/README.md');
  if (!(await readmeFile.exists())) {
    await readmeFile.create(recursive: true);
  }
  final readmeContent = await readmeFile.readAsString();
  var readmeSplitted = readmeContent.split('\n\n');
  readmeSplitted = readmeSplitted.map((s) => s.trim()).toList();
  if (!readmeContent.contains(guideSectionTitle)) {
    final text = '''
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

```bash
rustc --version
flutter doctor
```

You also need to have the CLI tool for Rinf ready.

```bash
cargo install rinf
```

Messages sent between Dart and Rust are implemented using Protobuf.
If you have newly cloned the project repository
or made changes to the `.proto` files in the `./messages` directory,
run the following command:

```bash
rinf message
```

Now you can run and build this app just like any other Flutter projects.

```bash
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rinf's [documentation](https://rinf.cunarist.com).
''';
    readmeSplitted.add(text);
  }
  await readmeFile.writeAsString(readmeSplitted.join('\n\n') + '\n');

  // Add Dart dependencies
  await Process.run('dart', ['pub', 'add', 'protobuf']);

  // Modify `./lib/main.dart`
  final mainFile = File('$flutterProjectPath/lib/main.dart');
  if (await mainFile.exists()) {
    await Process.run('dart', ['format', './lib/main.dart']);
    var mainText = await mainFile.readAsString();
    if (!mainText.contains('messages/generated.dart')) {
      final lines = mainText.split("\n");
      final lastImportIndex = lines.lastIndexWhere(
        (line) => line.startsWith('import '),
      );
      lines.insert(
        lastImportIndex + 1,
        "import './messages/generated.dart';",
      );
      mainText = lines.join("\n");
    }
    if (!mainText.contains('initializeRust()')) {
      mainText = mainText.replaceFirst(
        'main() {',
        'main() async {',
      );
      mainText = mainText.replaceFirst(
        'main() async {',
        'main() async { await initializeRust();',
      );
    }
    await mainFile.writeAsString(mainText);
    await Process.run('dart', ['format', './lib/main.dart']);
  }

  await generateMessageCode(silent: true, messageConfig: messageConfig);

  print("🎉 Rust template is now ready! 🎉");
}

Future<void> copyDirectory(Directory source, Directory destination) async {
  final newDirectory = Directory(destination.path);
  await newDirectory.create(recursive: true);
  await for (final entity in source.list(recursive: false)) {
    final entityName = entity.path.split(Platform.pathSeparator).last;
    if (entity is Directory) {
      final newDirectory = Directory(
        destination.uri.resolve(entityName).toFilePath(),
      );
      await newDirectory.create();
      await copyDirectory(entity.absolute, newDirectory);
    } else if (entity is File) {
      await entity.copy(
        destination.uri.resolve(entityName).toFilePath(),
      );
    }
  }
}

Future<void> buildWebassembly({bool isReleaseMode = false}) async {
  // Verify Rust toolchain.
  print("Verifying Rust toolchain for the web." +
      "\nThis might take a while if there are new updates.");
  await Process.run("rustup", ["toolchain", "install", "nightly"]);
  await Process.run("rustup", [
    "+nightly",
    "component",
    "add",
    "rust-src",
  ]);
  await Process.run("rustup", [
    "+nightly",
    "target",
    "add",
    "wasm32-unknown-unknown",
  ]); // For actual compilation
  await Process.run("rustup", [
    "target",
    "add",
    "wasm32-unknown-unknown",
  ]); // For Rust-analyzer
  await Process.run("cargo", ["install", "wasm-pack"]);
  await Process.run("cargo", ["install", "wasm-bindgen-cli"]);

  // Verify Flutter SDK web server's response headers.
  try {
    await patchServerHeaders();
    print("Patched Flutter SDK's web server with CORS HTTP headers.");
  } catch (error) {
    print("Failed patching Flutter's web server with CORS HTTP headers.");
    print("Try using the command below.");
    print('flutter run' +
        ' --web-header=Cross-Origin-Opener-Policy=same-origin' +
        ' --web-header=Cross-Origin-Embedder-Policy=require-corp');
  }

  // Prepare the webassembly output path.
  final flutterProjectPath = Directory.current;
  final subPath = './web/pkg';
  final outputPath = flutterProjectPath.uri.resolve(subPath).toFilePath();

  // Build the webassembly module.
  print("Compiling Rust with `wasm-pack` to `web` target...");
  final compileCommand = await Process.run(
    'wasm-pack',
    [
      '--quiet',
      'build',
      './native/hub',
      '--out-dir', outputPath,
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
    print(compileCommand.stderr.toString().trim());
    throw Exception('Unable to compile Rust into webassembly');
  }
  print("Saved `.wasm` and `.js` files to `$subPath`.");

  print("🎉 Webassembly module is now ready! 🎉");
}
