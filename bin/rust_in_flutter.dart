import 'dart:io';
import 'package:package_config/package_config.dart';
import 'dart:convert';

Future<void> main(List<String> args) async {
  if (args.length == 0) {
    print("No operation is provided.");
    print("Use `rifs --help` to see all available operations.");
    return;
  }
  switch (args[0]) {
    case "template":
      await _applyRustTemplate();
      break;
    case "message":
      await _generateMessageCode();
      break;
    case "wasm":
      if (args.contains("--release") || args.contains("-r")) {
        await _buildWebassembly(isReleaseMode: true);
      } else {
        await _buildWebassembly(isReleaseMode: false);
      }
      break;
    case "--help":
    case "-h":
      print("Usage: rifs [arguments]");
      print("Arguments:");
      print("  -h, --help    Shows this usage information.");
      print("  template      Applies Rust template to the current project.");
      print("  message       Generates message code from `.proto` files.");
      print("  wasm          Builds webassembly.");
    default:
      print("No such operation is available.");
      print("Use `rifs --help` to see all available operations.");
  }
}

/// Creates new folders and files to an existing Flutter project folder.
Future<void> _applyRustTemplate() async {
  // Get the path of the current project directory
  final flutterProjectPath = Directory.current.path;

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
  final mainFile = File('$flutterProjectPath/lib/main.dart');
  final isFlutterProject = await mainFile.exists();
  if (!isFlutterProject) {
    print("\nThis folder doesn't look like a Flutter project. Aborting...\n");
    return;
  }

  // Copy basic folders needed for Rust to work
  final templateSource = Directory('$packagePath/example/native');
  final templateDestination = Directory('$flutterProjectPath/native');
  await _copyDirectory(templateSource, templateDestination);
  final messagesSource = Directory('$packagePath/example/messages');
  final messagesDestination = Directory('$flutterProjectPath/messages');
  await _copyDirectory(messagesSource, messagesDestination);

  // Copy `Cargo.toml`
  final cargoSource = File('$packagePath/example/Cargo.toml');
  final cargoDestination = File('$flutterProjectPath/Cargo.toml');
  await cargoSource.copy(cargoDestination.path);

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
[Rust-In-Flutter](https://pub.dev/packages/rust_in_flutter) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install),
[Rust toolchain](https://www.rust-lang.org/tools/install),
and [Protobuf compiler](https://grpc.io/docs/protoc-installation)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```bash
rustc --version
protoc --version
flutter doctor
```

You also need to have the CLI tool for Rust-In-Flutter ready.

```bash
cargo install rifs
```

Messages sent between Dart and Rust are implemented using Protobuf.
If you have newly cloned the project repository
or made changes to the `.proto` files in the `./messages` directory,
run the following command:

```bash
rifs message
```

Now you can run and build this app just like any other Flutter projects.

```bash
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rust-In-Flutter's [documentation](https://rif-docs.cunarist.com).
''';
    readmeSplitted.add(text);
  }
  await readmeFile.writeAsString(readmeSplitted.join('\n\n') + '\n');

  // Add Dart dependencies
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
  await for (final entity in source.list(recursive: false)) {
    final entityName = entity.path.split(Platform.pathSeparator).last;
    if (entity is Directory) {
      final newDirectory = Directory(
        destination.uri.resolve(entityName).toFilePath(),
      );
      await newDirectory.create();
      await _copyDirectory(entity.absolute, newDirectory);
    } else if (entity is File) {
      await entity.copy(
        destination.uri.resolve(entityName).toFilePath(),
      );
    }
  }
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

  // Prepare the webassembly output path.
  final flutterProjectPath = Directory.current;
  final wasmOutputPath = flutterProjectPath.uri.resolve('web/pkg').toFilePath();

  // Build the webassembly module.
  print("Compiling Rust...");
  await _compile(
    crateDir: './native/hub',
    wasmOutput: wasmOutputPath,
    isReleaseMode: isReleaseMode,
  );

  print("🎉 Webassembly module is now ready! 🎉");
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
  'require-corp',
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

Future<void> _generateMessageCode() async {
  // Prepare paths.
  final flutterProjectPath = Directory.current;
  final protoPath = flutterProjectPath.uri.resolve('messages').toFilePath();
  final rustOutputPath =
      flutterProjectPath.uri.resolve('native/hub/src/messages').toFilePath();
  final dartOutputPath =
      flutterProjectPath.uri.resolve('lib/messages').toFilePath();
  await Directory(rustOutputPath).create(recursive: true);
  await _emptyDirectory(rustOutputPath);
  await Directory(dartOutputPath).create(recursive: true);
  await _emptyDirectory(dartOutputPath);

  // Get the list of `.proto` files.
  final resourcesInFolders = <String, List<String>>{};
  await _collectProtoFiles(
    Directory(protoPath),
    Directory(protoPath),
    resourcesInFolders,
  );

  // Verify `package` statement in `.proto` files.
  // Package name should be the same as the filename
  // because Rust filenames are written with package name
  // and Dart filenames are written with the `.proto` filename.
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    for (final resourceName in resourceNames) {
      final protoFile = File('$protoPath$subPath/$resourceName.proto');
      final lines = await protoFile.readAsLines();
      List<String> outputLines = [];
      for (var line in lines) {
        final packagePattern = r'^package\s+[a-zA-Z_][a-zA-Z0-9_]*\s*[^=];$';
        if (RegExp(packagePattern).hasMatch(line.trim())) {
          continue;
        } else if (line.trim().startsWith("syntax")) {
          continue;
        } else {
          outputLines.add(line);
        }
      }
      outputLines.insert(0, 'package $resourceName;');
      outputLines.insert(0, 'syntax = "proto3";');
      await protoFile.writeAsString(outputLines.join('\n') + '\n');
    }
  }

  // Generate Rust message files.
  print("Verifying `protoc-gen-prost` for Rust." +
      " This might take a while if there are new updates to be installed.");
  final cargoInstallCommand = await Process.run('cargo', [
    'install',
    'protoc-gen-prost',
  ]);
  if (cargoInstallCommand.exitCode != 0) {
    throw Exception('Cannot globally install `protoc-gen-prost` Rust crate');
  }
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    Directory('$rustOutputPath$subPath').create(recursive: true);
    final protocRustResult = await Process.run('protoc', [
      '--proto_path=$protoPath$subPath',
      '--prost_out=$rustOutputPath$subPath',
      ...resourceNames.map((name) => '$name.proto'),
    ]);
    if (protocRustResult.exitCode != 0) {
      throw Exception('Could not compile `.proto` files into Rust');
    }
  }

  // Generate `mod.rs` for `messages` module in Rust.
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    final modRsLines = resourceNames.map((resourceName) {
      return 'pub mod $resourceName;';
    }).toList();
    for (final otherSubPath in resourcesInFolders.keys) {
      if (otherSubPath != subPath && otherSubPath.contains(subPath)) {
        final relation = otherSubPath
            .replaceFirst(subPath, "")
            .replaceFirst(Platform.pathSeparator, '');
        if (!relation.contains(Platform.pathSeparator)) {
          modRsLines.add('pub mod $relation;');
        }
      }
    }
    final modRsContent = modRsLines.join('\n');
    await File('$rustOutputPath$subPath/mod.rs').writeAsString(modRsContent);
  }

  // Generate Dart message files.
  print("Verifying `protoc_plugin` for Dart." +
      " This might take a while if there are new updates to be installed.");
  final pubGlobalActivateCommand = await Process.run('dart', [
    'pub',
    'global',
    'activate',
    'protoc_plugin',
  ]);
  if (pubGlobalActivateCommand.exitCode != 0) {
    throw Exception('Cannot globally install `protoc_plugin` Dart package');
  }
  final newEnvironment = Map<String, String>.from(Platform.environment);
  final currentPathVariable = newEnvironment['PATH'];
  var pubCacheBinPath = Platform.isWindows
      ? '${Platform.environment['LOCALAPPDATA']}\\Pub\\Cache\\bin'
      : '${Platform.environment['HOME']}/.pub-cache/bin';
  if (Platform.environment["PUB_CACHE"] != null) {
    final binPath = Platform.isWindows ? '\\bin' : '/bin';
    pubCacheBinPath = '${Platform.environment["PUB_CACHE"]}$binPath';
  }
  final separator = Platform.isWindows ? ';' : ':';
  final newPathVariable = currentPathVariable != null
      ? '$currentPathVariable$separator$pubCacheBinPath'
      : pubCacheBinPath;
  newEnvironment['PATH'] = newPathVariable;
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    Directory('$dartOutputPath$subPath').create(recursive: true);
    final protocDartResult = await Process.run(
      'protoc',
      [
        '--proto_path=$protoPath$subPath',
        '--dart_out=$dartOutputPath$subPath',
        ...resourceNames.map((name) => '$name.proto'),
      ],
      environment: newEnvironment,
    );
    if (protocDartResult.exitCode != 0) {
      throw Exception('Could not compile `.proto` files into Dart');
    }
  }

  // Assign Rust resource index to each message module.
  var resourceIndex = 0;
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    for (final resourceName in resourceNames) {
      _appendLineToFile(
        '$rustOutputPath$subPath/$resourceName.rs',
        'pub const ID: i32 = $resourceIndex;',
      );
      _appendLineToFile(
        '$dartOutputPath$subPath/$resourceName.pb.dart',
        'const ID = $resourceIndex;',
      );
      resourceIndex += 1;
    }
  }

  // Notify that it's done
  print("🎉 Message code in Dart and Rust is now ready! 🎉");
}

Future<void> _emptyDirectory(String directoryPath) async {
  final directory = Directory(directoryPath);

  if (await directory.exists()) {
    await for (final entity in directory.list()) {
      if (entity is File) {
        await entity.delete();
      } else if (entity is Directory) {
        await entity.delete(recursive: true);
      }
    }
  }
}

Future<void> _appendLineToFile(String filePath, String textToAppend) async {
  // Read the existing content of the file
  final file = File(filePath);
  if (!(await file.exists())) {
    await file.create(recursive: true);
  }
  String fileContent = await file.readAsString();

  // Append the new text to the existing content
  fileContent += '\n';
  fileContent += textToAppend;

  // Write the updated content back to the file
  await file.writeAsString(fileContent);
}

Future<void> _collectProtoFiles(
  Directory rootDirectory,
  Directory directory,
  Map<String, List<String>> resourcesInFolders,
) async {
  final resources = <String>[];
  await for (final entity in directory.list()) {
    if (entity is File) {
      final filename = entity.uri.pathSegments.last;
      if (filename.endsWith('.proto')) {
        final parts = filename.split('.');
        parts.removeLast(); // Remove the extension from the filename.
        final fileNameWithoutExtension = parts.join('.');
        resources.add(fileNameWithoutExtension);
      }
    } else if (entity is Directory) {
      await _collectProtoFiles(
        rootDirectory,
        entity,
        resourcesInFolders,
      ); // Recursive call for subdirectories
    }
  }
  final folderPath = directory.path.replaceFirst(rootDirectory.path, '');
  if (resources.length > 0) {
    resourcesInFolders[folderPath] = resources;
  }
}
