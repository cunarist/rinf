import 'dart:io';

import 'package:path/path.dart';

import 'target.dart';

class OhosEnvironment {
  OhosEnvironment(
      {required this.targetTempDir,
      required this.ohosSDKHome,
      required this.target});

  final String targetTempDir;
  final String ohosSDKHome;
  final Target target;

  _save(String fileName, String content) {
    final file = File(join(targetTempDir, fileName));
    file.createSync(recursive: true);
    file.writeAsStringSync(content);
    return file.path;
  }

  _addExecutablePermission(String path) {
    final result = Process.runSync('chmod', ['+x', path]);
    if (result.exitCode != 0) {
      throw Exception('chmod failed: ${result.stderr}');
    }
  }

  String _aarch64ClangContent() {
    return '''
#!/bin/sh
exec $ohosSDKHome/native/llvm/bin/clang \\
  -target aarch64-linux-ohos \\
  --sysroot=$ohosSDKHome/native/sysroot \\
  -D__MUSL__ \\
  "\$@"
''';
  }

  String _armv7ClangContent() {
    return '''
#!/bin/sh
exec $ohosSDKHome/native/llvm/bin/clang \\
  -target arm-linux-ohos \\
  --sysroot=$ohosSDKHome/native/sysroot \\
  -D__MUSL__ \\
  -march=armv7-a \\
  -mfloat-abi=softfp \\
  -mtune=generic-armv7-a \\
  -mthumb \\
  "\$@"
''';
  }

  String _x86_64ClangContent() {
    return '''
#!/bin/sh
exec $ohosSDKHome/native/llvm/bin/clang \\
  -target x86_64-linux-ohos \\
  --sysroot=$ohosSDKHome/native/sysroot \\
  -D__MUSL__ \\
  "\$@"
''';
  }

  String _saveClangToolchain() {
    switch (target.ohos) {
      case "arm64-v8a":
        {
          return _save(
              "aarch64-unknown-linux-ohos-clang.sh", _aarch64ClangContent());
        }

      case "armeabi-v7a":
        {
          return _save(
              "armv7-unknown-linux-ohos-clang.sh", _armv7ClangContent());
        }

      case "x86_64":
        {
          return _save(
              "x86_64-unknown-linux-ohos-clang.sh", _x86_64ClangContent());
        }

      default:
        {
          throw UnimplementedError("Unsupported ohos target: ${target.ohos}");
        }
    }
  }

  Future<Map<String, String>> buildEnvironment() async {
    final clangToolchainPath = _saveClangToolchain();
    _addExecutablePermission(clangToolchainPath);

    switch (target.ohos) {
      case "arm64-v8a":
        {
          return {
            "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_LINKER":
                clangToolchainPath,
            "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_AR":
                join(ohosSDKHome, "native", "llvm", "bin", "llvm-ar")
          };
        }

      case "armeabi-v7a":
        {
          return {
            "CARGO_TARGET_ARMV7_UNKNOWN_LINUX_OHOS_LINKER": clangToolchainPath,
            "CARGO_TARGET_ARMV7_UNKNOWN_LINUX_OHOS_AR":
                join(ohosSDKHome, "native", "llvm", "bin", "llvm-ar")
          };
        }

      case "x86_64":
        {
          return {
            "CARGO_TARGET_X86_64_UNKNOWN_LINUX_OHOS_LINKER": clangToolchainPath,
            "CARGO_TARGET_X86_64_UNKNOWN_LINUX_OHOS_AR":
                join(ohosSDKHome, "native", "llvm", "bin", "llvm-ar")
          };
        }

      default:
        {
          throw UnimplementedError("Unsupported ohos target: ${target.ohos}");
        }
    }
  }
}
