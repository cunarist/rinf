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
}
