import 'package:native_toolchain_rust/native_toolchain_rust.dart';
import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

void main(List<String> args) async {
  // Get the crate path.
  Directory currentDirectory = Directory.current;
  final crateUri = currentDirectory.uri.resolve("native/hub");
  var cratePath = crateUri.toFilePath();
  if (!cratePath.startsWith("/")) {
    // Character `/` makes it act as an absolute path
    // even when passed into `Uri.resolve`.
    // Not needed on unix-based, but needed on Windows.
    cratePath = "/$cratePath";
  }

  await build(args, (BuildConfig buildConfig, BuildOutput output) async {
    if (buildConfig.dryRun) {
      return;
    }
    final builder = RustBuilder(
      // The ID of native assets consists of package name and crate name.
      package: 'rinf',
      cratePath: cratePath,
      buildConfig: buildConfig,
      release: buildConfig.buildMode == BuildMode.release,
    );
    await builder.run(output: output);
  });
}
