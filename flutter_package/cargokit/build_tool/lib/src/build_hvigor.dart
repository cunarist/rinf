import 'dart:io';

import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'builder.dart';
import 'environment.dart';
import 'options.dart';
import 'target.dart';

final log = Logger('build_hvigor');

class BuildHvigor {
  BuildHvigor({required this.userOptions});

  final CargokitUserOptions userOptions;

  Future<void> build() async {
    if (userOptions.ohosSDKHome == null) {
      throw Exception("ohosSDKHome is not set");
    }

    final targets = Environment.targetPlatforms.map((arch) {
      final target = Target.forFlutterName(arch);
      if (target == null) {
        throw Exception("Unknown ohos target: $arch}");
      }
      return target;
    }).toList();

    final environment = BuildEnvironment.fromEnvironment(
        isAndroid: false, ohosSDKHome: userOptions.ohosSDKHome!);
    final provider =
        ArtifactProvider(environment: environment, userOptions: userOptions);
    final artifacts = await provider.getArtifacts(targets);

    for (final target in targets) {
      final libs = artifacts[target]!;
      final outputDir = path.join(Environment.outputDir, target.ohos);
      Directory(outputDir).createSync(recursive: true);

      for (final lib in libs) {
        if (lib.type == AritifactType.dylib) {
          File(lib.path).copySync(path.join(outputDir, lib.finalFileName));
        }
      }
    }
  }
}
