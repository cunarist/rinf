import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:package_config/package_config.dart';
import 'dart:convert';
import 'package:path/path.dart';

Future<void> main(List<String> args) async {
  if (args.length == 0) {
    print("No operation is provided");
  } else if (args[0] == "template") {
    await _applyTemplate();
  } else if (args[0] == "wasm") {
    if (args.contains("--release") || args.contains("-r")) {
      await _buildWebassembly(isReleaseMode: true);
    } else {
      await _buildWebassembly(isReleaseMode: false);
    }
  } else {
    print("No available operation is provided");
  }
}

/// Creates new folders and files to an existing Flutter project folder.
Future<void> _applyTemplate() async {
  // Get the path of the current project directory
  final projectPath = Directory.current.path;

  // Get the package directory path
  final packageConfig = await findPackageConfig(Directory.current);
  if (packageConfig == null) {
    return;
  }
  final packageName = 'rust_in_flutter';
  final package = packageConfig.packages.firstWhere(
    (p) => p.name == packageName,
  );
  final packagePath = package.root.toFilePath();

  // Check if current folder is a Flutter project.
  final mainFile = File('$projectPath/lib/main.dart');
  final isFlutterProject = await mainFile.exists();
  if (!isFlutterProject) {
    print("\nThis folder doesn't look like a Flutter project. Aborting...\n");
    return;
  }

  // Copy basic folders needed for Rust to work
  final templateSource = Directory('$packagePath/example/native');
  final templateDestination = Directory('$projectPath/native');
  await _copyDirectory(templateSource, templateDestination);
  final messagesSource = Directory('$packagePath/example/messages');
  final messagesDestination = Directory('$projectPath/messages');
  await _copyDirectory(messagesSource, messagesDestination);

  // Copy `Cargo.toml`
  final cargoSource = File('$packagePath/example/Cargo.toml');
  final cargoDestination = File('$projectPath/Cargo.toml');
  await cargoSource.copy(cargoDestination.path);

  // Create `.cargo/config.toml` file
  final cargoConfigFile = File('$projectPath/.cargo/config.toml');
  if (!(await cargoConfigFile.exists())) {
    await cargoConfigFile.create(recursive: true);
  }
  const cargoConfigContent = '''
[build]
# Uncomment the line below to switch Rust-analyzer to perform
# type checking and linting in webassembly mode, for the web target.
# You might have to restart Rust-analyzer for this change to take effect.
# target = "wasm32-unknown-unknown"
''';
  await cargoConfigFile.writeAsString(cargoConfigContent);

  // Add some lines to `.gitignore`
  final rustSectionTitle = '# Rust related';
  final messageSectionTitle = '# Generated messages';
  final gitignoreFile = File('$projectPath/.gitignore');
  if (!(await gitignoreFile.exists())) {
    await gitignoreFile.create(recursive: true);
  }
  final gitignoreContent = await gitignoreFile.readAsString();
  var splitted = gitignoreContent.split('\n\n');
  splitted = splitted.map((s) => s.trim()).toList();
  if (!gitignoreContent.contains(rustSectionTitle)) {
    var text = rustSectionTitle;
    text += '\n' + '.cargo/';
    text += '\n' + 'target/';
    splitted.add(text);
  }
  if (!gitignoreContent.contains(messageSectionTitle)) {
    var text = messageSectionTitle;
    text += '\n' + 'native/hub/src/messages';
    text += '\n' + 'lib/messages';
    splitted.add(text);
  }
  await gitignoreFile.writeAsString(splitted.join('\n\n'));

  // Add `msgpack_dart` to Dart dependencies
  await Process.run('dart', ['pub', 'add', 'protobuf']);

  // Modify `./lib/main.dart`
  await Process.run('dart', ['format', './lib/main.dart']);
  var mainText = await mainFile.readAsString();
  if (!mainText.contains('package:rust_in_flutter/rust_in_flutter.dart')) {
    final lines = mainText.split("\n");
    final lastImportIndex = lines.lastIndexWhere(
      (line) => line.startsWith('import '),
    );
    lines.insert(
      lastImportIndex + 1,
      "import 'package:rust_in_flutter/rust_in_flutter.dart';",
    );
    mainText = lines.join("\n");
  }
  if (mainText.contains('main() {')) {
    mainText = mainText.replaceFirst(
      'main() {',
      'main() async {',
    );
  }
  if (!mainText.contains('RustInFlutter.ensureInitialized()')) {
    mainText = mainText.replaceFirst(
      'main() async {',
      'main() async { await RustInFlutter.ensureInitialized();',
    );
  }
  await mainFile.writeAsString(mainText);
  await Process.run('dart', ['format', './lib/main.dart']);

  print("🎉 Rust template is now ready! 🎉");
}

Future<void> _copyDirectory(Directory source, Directory destination) async {
  final newDirectory = Directory(destination.path);
  await newDirectory.create();
  await source.list(recursive: false).forEach(
    (entity) async {
      if (entity is Directory) {
        final newDirectory = Directory(
          path.join(destination.absolute.path, path.basename(entity.path)),
        );
        await newDirectory.create();
        _copyDirectory(entity.absolute, newDirectory);
      } else if (entity is File) {
        await entity.copy(
          path.join(destination.path, path.basename(entity.path)),
        );
      }
    },
  );
}

Future<void> _buildWebassembly({bool isReleaseMode = false}) async {
  // Verify Rust toolchain.
  print("Verifying Rust toolchain for the web." +
      " This might take a while if there are new updates to be installed.");
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
  await _verifyServerHeaders();

  // Build the webassembly module.
  print("Compiling Rust...");
  await _compile(
    crateDir: './native/hub',
    wasmOutput: canonicalize('web/pkg'),
    isReleaseMode: isReleaseMode,
  );

  if (isReleaseMode) {
    print("Webassembly module for release is now ready!");
  } else {
    print("Webassembly module is now ready!");
  }
}

Future<void> _verifyServerHeaders() async {
  // Get the Flutter SDK's path.
  String flutterPath;
  if (Platform.isWindows) {
    // Windows
    final whereFlutterResult = await Process.run('where', ['flutter']);
    flutterPath = (whereFlutterResult.stdout as String).split('\n').first;
  } else {
    // macOS and Linux
    final whichFlutterResult = await Process.run('which', ['flutter']);
    flutterPath = whichFlutterResult.stdout as String;
  }
  flutterPath = flutterPath.trim();
  flutterPath = await File(flutterPath).resolveSymbolicLinks();
  flutterPath = File(flutterPath).parent.parent.path;

  // Get the server module file's path.
  final serverFile = File(
      '$flutterPath/packages/flutter_tools/lib/src/isolated/devfs_web.dart');
  var serverFileContent = await serverFile.readAsString();

  // Check if the server already includes cross-origin HTTP headers.
  if (serverFileContent.contains('cross-origin-opener-policy')) {
    return;
  }

  // Add the HTTP header code to the server file.
  final lines = serverFileContent.split('\n');
  final serverDeclaredIndex = lines.lastIndexWhere(
    (line) => line.contains('httpServer = await'),
  );
  lines.insert(serverDeclaredIndex + 1, """
httpServer.defaultResponseHeaders.add(
  'cross-origin-opener-policy',
  'same-origin',
);
httpServer.defaultResponseHeaders.add(
  'cross-origin-embedder-policy',
  'credentialless',
);""");
  serverFileContent = lines.join("\n");
  await serverFile.writeAsString(serverFileContent);

  // Remove the stamp file to make it re-generated.
  final flutterToolsStampPath = '$flutterPath/bin/cache/flutter_tools.stamp';
  if (await File(flutterToolsStampPath).exists()) {
    await File(flutterToolsStampPath).delete();
  }
}

Future<void> _compile({
  required String crateDir,
  required String wasmOutput,
  required bool isReleaseMode,
}) async {
  final String crateName = 'hub';
  await _runAdvancedCommand(
    'wasm-pack',
    [
      '--quiet',
      'build', '-t', 'no-modules', '-d', wasmOutput, '--no-typescript',
      '--out-name', crateName,
      if (!isReleaseMode) '--dev', crateDir,
      '--', // cargo build args
      '-Z', 'build-std=std,panic_abort',
    ],
    env: {
      'RUSTUP_TOOLCHAIN': 'nightly',
      'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
      if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
    },
  );
}

Future<void> _runAdvancedCommand(
  String command,
  List<String> arguments, {
  Map<String, String>? env,
  bool silent = false,
}) async {
  final process = await Process.start(
    command,
    arguments,
    environment: env,
  );
  final processOutput = <String>[];
  process.stderr.transform(utf8.decoder).listen((line) {
    if (!silent) stderr.write(line);
    processOutput.add(line);
  });
  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw ProcessException(
        command, arguments, processOutput.join(''), exitCode);
  }
}
