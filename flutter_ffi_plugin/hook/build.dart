import 'package:native_toolchain_rust/native_toolchain_rust.dart';
import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

void main(List<String> args) async {
  // Get the crate path.
  Directory currentDirectory = Directory.current;
  final crateUri = currentDirectory.uri.resolve("native/hub");
  final cratePath = crateUri.toFilePath();

  await build(args, (BuildConfig buildConfig, BuildOutput output) async {
    final builder = RustBuilder(
      // The ID of native assets consists of package name and crate name.
      package: 'rinf',
      cratePath: '/$cratePath', // Character `/` makes it a absolute path.
      buildConfig: buildConfig,
    );
    await builder.run(output: output);
  });
}
