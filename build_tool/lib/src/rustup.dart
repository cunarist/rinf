import 'dart:io';

import 'package:path/path.dart' as path;

import 'util.dart';

class Rustup {
  final List<String> installedTargets;
  final List<String> installedToolchains;

  Rustup()
      : installedTargets = _getInstalledTargets(),
        installedToolchains = _getInstalledToolchains();

  void installTarget(String target) {
    log.info("Installing Rust target: $target");
    runCommand("rustup", ['target', 'add', target]);
    installedTargets.add(target);
  }

  void installToolchain(String toolchain) {
    log.info("Installing Rust toolchain: $toolchain");
    runCommand("rustup", ['toolchain', 'install', toolchain]);
    installedToolchains.add(toolchain);
  }

  static List<String> _getInstalledToolchains() {
    final res = runCommand("rustup", ['toolchain', 'list']);
    final lines = res.stdout
        .toString()
        .split('\n')
        .where((e) => e.isNotEmpty)
        .toList(growable: true);
    return lines;
  }

  static List<String> _getInstalledTargets() {
    final res = runCommand("rustup", ['target', 'list', '--installed']);
    final lines = res.stdout
        .toString()
        .split('\n')
        .where((e) => e.isNotEmpty)
        .toList(growable: true);
    return lines;
  }

  bool _didInstallRustSrcForNightly = false;

  void installRustSrcForNightly() {
    if (_didInstallRustSrcForNightly) {
      return;
    }
    // Useful for -Z build-std
    runCommand(
      "rustup",
      ['component', 'add', 'rust-src', '--toolchain', 'nightly'],
    );
    _didInstallRustSrcForNightly = true;
  }

  static String? executablePath() {
    final envPath = Platform.environment['PATH'];
    final envPathSeparator = Platform.isWindows ? ';' : ':';
    final home = Platform.isWindows
        ? Platform.environment['USERPROFILE']
        : Platform.environment['HOME'];
    final paths = [
      if (home != null) path.join(home, '.cargo', 'bin'),
      if (envPath != null) ...envPath.split(envPathSeparator),
    ];
    for (final p in paths) {
      final rustup = Platform.isWindows ? 'rustup.exe' : 'rustup';
      final rustupPath = path.join(p, rustup);
      if (File(rustupPath).existsSync()) {
        return rustupPath;
      }
    }
    return null;
  }
}
