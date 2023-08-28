import 'dart:convert';
import 'dart:io';

import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'logging.dart';

final log = Logger("process");

class CommandFailedException implements Exception {
  final String executable;
  final List<String> arguments;
  final ProcessResult result;

  CommandFailedException({
    required this.executable,
    required this.arguments,
    required this.result,
  });

  @override
  String toString() {
    final stdout = result.stdout.toString().trim();
    final stderr = result.stderr.toString().trim();
    return [
      "External Command: $executable ${arguments.join(' ')}",
      "Returned Exit Code: ${result.exitCode}",
      kSeparator,
      "STDOUT:",
      if (stdout.isNotEmpty) stdout,
      kSeparator,
      "STDERR:",
      if (stderr.isNotEmpty) stderr,
    ].join('\n');
  }
}

ProcessResult runCommand(
  String executable,
  List<String> arguments, {
  String? workingDirectory,
  Map<String, String>? environment,
  bool includeParentEnvironment = true,
  bool runInShell = false,
  Encoding? stdoutEncoding = systemEncoding,
  Encoding? stderrEncoding = systemEncoding,
}) {
  log.finer('Running command $executable ${arguments.join(' ')}');
  final res = Process.runSync(
    _resolveExecutable(executable),
    arguments,
    workingDirectory: workingDirectory,
    environment: environment,
    includeParentEnvironment: includeParentEnvironment,
    runInShell: runInShell,
    stderrEncoding: stderrEncoding,
    stdoutEncoding: stdoutEncoding,
  );
  if (res.exitCode != 0) {
    throw CommandFailedException(
      executable: executable,
      arguments: arguments,
      result: res,
    );
  } else {
    return res;
  }
}

class RustupNotFoundException implements Exception {
  @override
  String toString() {
    return [
      ' ',
      'rustup not found in PATH.',
      ' ',
      'Maybe you need to install Rust? It only takes a minute:',
      ' ',
      if (Platform.isWindows) 'https://www.rust-lang.org/tools/install',
      if (!Platform.isWindows)
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
      ' ',
    ].join('\n');
  }
}

String _resolveExecutable(String executable) {
  if (executable == 'rustup') {
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
    throw RustupNotFoundException();
  } else {
    return executable;
  }
}
