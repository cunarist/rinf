import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:package_config/package_config.dart';

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

  final source = Directory('$packagePath/example/native');
  final destination = Directory('$projectPath/native');
  copyDirectory(source, destination);

  var sourceFile = File('$packagePath/example/Cargo.toml');
  var destinationFile = File('$projectPath/Cargo.toml');
  sourceFile.copySync(destinationFile.path);
}

void copyDirectory(Directory source, Directory destination) {
  var newDirectory = Directory(destination.path);
  newDirectory.createSync();
  source.listSync(recursive: false).forEach(
    (var entity) {
      if (entity is Directory) {
        var newDirectory = Directory(
          path.join(destination.absolute.path, path.basename(entity.path)),
        );
        newDirectory.createSync();
        copyDirectory(entity.absolute, newDirectory);
      } else if (entity is File) {
        entity.copySync(
          path.join(destination.path, path.basename(entity.path)),
        );
      }
    },
  );
}
