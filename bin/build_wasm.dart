import 'dart:convert';
import 'dart:io';
import 'package:path/path.dart';

void main(List<String> args) async {
  // Verify Rust toolchain.
  print("Verifying Rust toolchain for the web." +
      " This might take a while if there are new updates to be installed.");
  await Process.run("rustup", ["toolchain", "install", "nightly"]);
  Future.wait([
    Process.run("rustup", [
      "+nightly",
      "component",
      "add",
      "rust-src",
    ]),
    Process.run("rustup", [
      "+nightly",
      "target",
      "add",
      "wasm32-unknown-unknown",
    ]), // For actual compilation
    Process.run("rustup", [
      "target",
      "add",
      "wasm32-unknown-unknown",
    ]), // For Rust-analyzer
    Process.run("cargo", ["install", "wasm-pack"]),
    Process.run("cargo", ["install", "wasm-bindgen-cli"]),
  ]);

  // Verify Flutter SDK web server's response headers.
  await _verifyServerHeaders();

  // Build the webassembly module.
  await _compile(
    crateDir: './native/hub',
    wasmOutput: canonicalize('web/pkg'),
    isReleaseMode: args.contains("--release"),
  );
}

Future<void> _verifyServerHeaders() async {
  // Get the Flutter SDK's path.
  String flutterPath;
  if (Platform.isWindows) {
    // Windows
    final whereFlutterResult = await Process.run('where', ['flutter']);
    flutterPath = (whereFlutterResult.stdout as String).split('\n').first;
  } else {
    // macOS and Linux
    final whichFlutterResult = await Process.run('which', ['flutter']);
    flutterPath = whichFlutterResult.stdout as String;
  }
  flutterPath = flutterPath.trim();
  flutterPath = await File(flutterPath).resolveSymbolicLinks();
  flutterPath = File(flutterPath).parent.parent.path;

  // Get the server module file's path.
  final serverFile = File(
      '$flutterPath/packages/flutter_tools/lib/src/isolated/devfs_web.dart');
  var serverFileContent = await serverFile.readAsString();

  // Check if the server already includes cross-origin HTTP headers.
  if (serverFileContent.contains('cross-origin-opener-policy')) {
    return;
  }

  // Add the HTTP header code to the server file.
  final lines = serverFileContent.split('\n');
  final serverDeclaredIndex = lines.lastIndexWhere(
    (line) => line.contains('httpServer = await'),
  );
  lines.insert(serverDeclaredIndex + 1, """
httpServer.defaultResponseHeaders.add(
  'cross-origin-opener-policy',
  'same-origin',
);
httpServer.defaultResponseHeaders.add(
  'cross-origin-embedder-policy',
  'credentialless',
);""");
  serverFileContent = lines.join("\n");
  serverFile.writeAsStringSync(serverFileContent);

  // Remove the stamp file to make it re-generated.
  final flutterToolsStampPath = '$flutterPath/bin/cache/flutter_tools.stamp';
  if (await File(flutterToolsStampPath).exists()) {
    await File(flutterToolsStampPath).delete();
  }
}

Future<void> _compile({
  required String crateDir,
  required String wasmOutput,
  required bool isReleaseMode,
}) async {
  final String crateName = 'hub';
  await _runSystemCommand(
    'wasm-pack',
    [
      'build', '-t', 'no-modules', '-d', wasmOutput, '--no-typescript',
      '--out-name', crateName,
      if (!isReleaseMode) '--dev', crateDir,
      '--', // cargo build args
      '-Z', 'build-std=std,panic_abort',
    ],
    env: {
      'RUSTUP_TOOLCHAIN': 'nightly',
      'RUSTFLAGS': '-C target-feature=+atomics,+bulk-memory,+mutable-globals',
      if (stdout.supportsAnsiEscapes) 'CARGO_TERM_COLOR': 'always',
    },
  );
}

Future<void> _runSystemCommand(
  String command,
  List<String> arguments, {
  String? pwd,
  Map<String, String>? env,
  bool shell = true,
  bool silent = false,
}) async {
  print('> $command ${arguments.join(' ')}');
  final process = await Process.start(
    command,
    arguments,
    runInShell: shell,
    workingDirectory: pwd,
    environment: env,
  );
  final ret = <String>[];
  final err = <String>[];
  process.stdout.transform(utf8.decoder).listen((line) {
    if (!silent) stdout.write(line);
    ret.add(line);
  });
  process.stderr.transform(utf8.decoder).listen((line) {
    if (!silent) stderr.write(line);
    err.add(line);
  });
  final exitCode = await process.exitCode;
  if (exitCode != 0) {
    throw ProcessException(command, arguments, err.join(''), exitCode);
  }
}
