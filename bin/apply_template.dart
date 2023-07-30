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
  _copyDirectory(templateSource, templateDestination);

  // Copy `Cargo.toml`
  final cargoSource = File('$packagePath/example/Cargo.toml');
  final cargoDestination = File('$projectPath/Cargo.toml');
  cargoSource.copySync(cargoDestination.path);

  // Create `.cargo/config.toml` file
  final cargoConfigFile = File('$projectPath/.cargo/config.toml');
  if (!cargoConfigFile.existsSync()) {
    cargoConfigFile.createSync(recursive: true);
  }
  const cargoConfigContent = '''
[build]
# Uncomment the line below to switch Rust-analyzer to perform
# type checking and linting in webassembly mode, for the web target.
# target = "wasm32-unknown-unknown"
''';
  cargoConfigFile.writeAsStringSync(cargoConfigContent);

  // Add some lines to `.gitignore`
  final sectionTitle = '# Rust related';
  final gitignoreFile = File('$projectPath/.gitignore');
  if (!gitignoreFile.existsSync()) {
    gitignoreFile.createSync(recursive: true);
  }
  final gitignoreContent = gitignoreFile.readAsStringSync();
  var splitted = gitignoreContent.split('\n\n');
  splitted = splitted.map((s) => s.trim()).toList();
  if (!gitignoreContent.contains(sectionTitle)) {
    var text = sectionTitle;
    text += '\n' + '.cargo/';
    text += '\n' + 'target/';
    splitted.add(text);
  }
  gitignoreFile.writeAsStringSync(splitted.join('\n\n'));

  // Add `msgpack_dart` to Dart dependencies
  await Process.run('dart', ['pub', 'add', 'msgpack_dart']);

  // Modify `./lib/main.dart`
  await Process.run('dart', ['format', './lib/main.dart']);
  var mainText = mainFile.readAsStringSync();
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
  mainFile.writeAsStringSync(mainText);
  await Process.run('dart', ['format', './lib/main.dart']);

  print("\n🎉 Rust template is now ready! 🎉 \n");
}

void _copyDirectory(Directory source, Directory destination) {
  var newDirectory = Directory(destination.path);
  newDirectory.createSync();
  source.listSync(recursive: false).forEach(
    (var entity) {
      if (entity is Directory) {
        var newDirectory = Directory(
          path.join(destination.absolute.path, path.basename(entity.path)),
        );
        newDirectory.createSync();
        _copyDirectory(entity.absolute, newDirectory);
      } else if (entity is File) {
        entity.copySync(
          path.join(destination.path, path.basename(entity.path)),
        );
      }
    },
  );
}
