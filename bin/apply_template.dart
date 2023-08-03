import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:package_config/package_config.dart';

/// Creates new folders and files to an existing Flutter project folder.
void main() async {
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

  // Copy the `native` folder
  final templateSource = Directory('$packagePath/example/native');
  final templateDestination = Directory('$projectPath/native');
  await _copyDirectory(templateSource, templateDestination);

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
  final sectionTitle = '# Rust related';
  final gitignoreFile = File('$projectPath/.gitignore');
  if (!(await gitignoreFile.exists())) {
    await gitignoreFile.create(recursive: true);
  }
  final gitignoreContent = await gitignoreFile.readAsString();
  var splitted = gitignoreContent.split('\n\n');
  splitted = splitted.map((s) => s.trim()).toList();
  if (!gitignoreContent.contains(sectionTitle)) {
    var text = sectionTitle;
    text += '\n' + '.cargo/';
    text += '\n' + 'target/';
    splitted.add(text);
  }
  await gitignoreFile.writeAsString(splitted.join('\n\n'));

  // Add `msgpack_dart` to Dart dependencies
  await Process.run('dart', ['pub', 'add', 'msgpack_dart']);

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

  print("\n🎉 Rust template is now ready! 🎉 \n");
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
