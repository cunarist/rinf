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
  final packageName = "rust_in_flutter";
  final package = packageConfig.packages.firstWhere(
    (p) => p.name == packageName,
  );
  final packagePath = package.root.toFilePath();

  // Copy the `native` folder
  final source = Directory('$packagePath/example/native');
  final destination = Directory('$projectPath/native');
  _copyDirectory(source, destination);

  // Copy `Cargo.toml`
  final sourceFile = File('$packagePath/example/Cargo.toml');
  final destinationFile = File('$projectPath/Cargo.toml');
  sourceFile.copySync(destinationFile.path);

  // Add some lines to `.gitignore`
  final sectionTop = '# Rust related';
  final gitignoreFile = File('$projectPath/.gitignore');
  String contents;
  try {
    contents = gitignoreFile.readAsStringSync();
  } on FileSystemException {
    contents = "";
  }
  var doesRustSectionExist = false;
  var splitted = contents.split("\n\n");
  splitted = splitted.map((s) => s.trim()).toList();
  for (final piece in splitted) {
    if (piece.startsWith(sectionTop)) {
      doesRustSectionExist = true;
    }
  }
  if (!doesRustSectionExist) {
    var text = sectionTop;
    text += '\n' + '.cargo/';
    text += '\n' + 'target/';
    splitted.add(text);
  }
  gitignoreFile.writeAsStringSync(splitted.join('\n\n'));

  // Add `msgpack_dart` to Dart dependencies
  await Process.run('dart', ['pub', 'add', 'msgpack_dart']);

  // Modify `./lib/main.dart`
  final mainFile = File('$projectPath/lib/main.dart');
  final lines = mainFile.readAsLinesSync();
  final allLines = lines.join('\n');
  final modifiedLines = <String>[];
  for (final line in lines) {
    if (line.contains('main()')) {
      modifiedLines.add('void main() async {');
      if (!allLines.contains('RustInFlutter.ensureInitialized()')) {
        modifiedLines.add('  await RustInFlutter.ensureInitialized();');
      }
    } else {
      modifiedLines.add(line);
    }
  }
  if (!allLines.contains('package:rust_in_flutter/rust_in_flutter.dart')) {
    modifiedLines.insert(
      0,
      "import 'package:rust_in_flutter/rust_in_flutter.dart';",
    );
  }
  final modifiedContent = modifiedLines.join('\n');
  mainFile.writeAsStringSync(modifiedContent);
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
