import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:hooks/hooks.dart';
import 'package:logging/logging.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart';
import 'package:path/path.dart' as path;

const _assetName = 'rinf.dart';
const _hubPath = 'native/hub';

void main(List<String> args) async {
  await build(args, (input, output) async {
    final logger = Logger.detached('rinf.native_assets')
      ..level = Level.INFO
      ..onRecord.listen((record) => stdout.writeln(record.message));

    if (!input.config.buildCodeAssets) {
      logger.info('Skipping Rust build because code assets are disabled.');
      return;
    }

    final cratePath = _resolveCratePath(input);
    logger.info('Building Rust crate from $cratePath');

    await RustBuilder(
      assetName: _assetName,
      cratePath: cratePath,
      buildMode: input.config.linkingEnabled
          ? BuildMode.release
          : BuildMode.debug,
    ).run(input: input, output: output, logger: logger);
  });
}

String _resolveCratePath(BuildInput input) {
  final appRoot = _resolveAppRoot(input);
  final crateUri = appRoot.resolve(_hubPath);
  final cratePath = path.normalize(path.fromUri(crateUri));
  final packageRoot = path.normalize(path.fromUri(input.packageRoot));

  if (!Directory(cratePath).existsSync()) {
    throw StateError(
      'Could not find the Rinf hub crate at "$cratePath". '
      'Rinf currently expects the crate to be located at "$_hubPath".',
    );
  }

  final relative = path.relative(cratePath, from: packageRoot);
  if (relative.startsWith('..') || relative == '.') {
    return relative;
  }
  return './$relative';
}

Uri _resolveAppRoot(BuildInput input) {
  final outputFilePath = path.normalize(path.fromUri(input.outputFile));
  final appRootPath = path.dirname(
    path.dirname(path.dirname(path.dirname(path.dirname(outputFilePath)))),
  );
  return path.toUri(
    appRootPath.endsWith(path.separator)
        ? appRootPath
        : '$appRootPath${path.separator}',
  );
}
