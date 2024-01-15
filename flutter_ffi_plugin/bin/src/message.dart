import 'package:path/path.dart';
import 'package:watcher/watcher.dart';
import 'dart:io';

Future<void> generateMessageCode({bool silent = false}) async {
  // Prepare paths.
  final flutterProjectPath = Directory.current;
  final protoPath = flutterProjectPath.uri.resolve('messages').toFilePath();
  final rustOutputPath =
      flutterProjectPath.uri.resolve('native/hub/src/messages').toFilePath();
  final dartOutputPath =
      flutterProjectPath.uri.resolve('lib/messages').toFilePath();
  await Directory(rustOutputPath).create(recursive: true);
  await emptyDirectory(rustOutputPath);
  await Directory(dartOutputPath).create(recursive: true);
  await emptyDirectory(dartOutputPath);

  // Get the list of `.proto` files.
  final resourcesInFolders = <String, List<String>>{};
  await collectProtoFiles(
    Directory(protoPath),
    Directory(protoPath),
    resourcesInFolders,
  );

  // Verify `package` statement in `.proto` files.
  // Package name should be the same as the filename
  // because Rust filenames are written with package name
  // and Dart filenames are written with the `.proto` filename.
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    for (final resourceName in resourceNames) {
      final protoFile = File('$protoPath$subPath/$resourceName.proto');
      final lines = await protoFile.readAsLines();
      List<String> outputLines = [];
      for (var line in lines) {
        final packagePattern = r'^package\s+[a-zA-Z_][a-zA-Z0-9_]*\s*[^=];$';
        if (RegExp(packagePattern).hasMatch(line.trim())) {
          continue;
        } else if (line.trim().startsWith("syntax")) {
          continue;
        } else {
          outputLines.add(line);
        }
      }
      outputLines.insert(0, 'package $resourceName;');
      outputLines.insert(0, 'syntax = "proto3";');
      await protoFile.writeAsString(outputLines.join('\n') + '\n');
    }
  }

  // Generate Rust message files.
  if (!silent) {
    print("Verifying `protoc-gen-prost` for Rust." +
        " This might take a while if there are new updates to be installed.");
  }
  final cargoInstallCommand = await Process.run('cargo', [
    'install',
    'protoc-gen-prost',
  ]);
  if (cargoInstallCommand.exitCode != 0) {
    print(cargoInstallCommand.stderr.toString().trim());
    throw Exception('Cannot globally install `protoc-gen-prost` Rust crate');
  }
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    await Directory('$rustOutputPath$subPath').create(recursive: true);
    if (resourceNames.length == 0) {
      continue;
    }
    final protoPaths = <String>[];
    for (final key in resourcesInFolders.keys) {
      protoPaths.add('--proto_path=$protoPath$key');
    }
    final protocRustResult = await Process.run('protoc', [
      ...protoPaths,
      '--prost_out=$rustOutputPath$subPath',
      ...resourceNames.map((name) => '$name.proto'),
    ]);
    if (protocRustResult.exitCode != 0) {
      print(protocRustResult.stderr.toString().trim());
      throw Exception('Could not compile `.proto` files into Rust');
    }
  }

  // Generate `mod.rs` for `messages` module in Rust.
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    final modRsLines = resourceNames.map((resourceName) {
      return 'pub mod $resourceName;';
    }).toList();
    for (final otherSubPath in resourcesInFolders.keys) {
      if (otherSubPath != subPath && otherSubPath.contains(subPath)) {
        final relation = otherSubPath
            .replaceFirst(subPath, "")
            .replaceFirst(Platform.pathSeparator, '');
        if (!relation.contains(Platform.pathSeparator)) {
          modRsLines.add('pub mod $relation;');
        }
      }
    }
    final modRsContent = modRsLines.join('\n');
    await File('$rustOutputPath$subPath/mod.rs').writeAsString(modRsContent);
  }

  // Generate Dart message files.
  if (!silent) {
    print("Verifying `protoc_plugin` for Dart." +
        " This might take a while if there are new updates to be installed.");
  }
  final pubGlobalActivateCommand = await Process.run('dart', [
    'pub',
    'global',
    'activate',
    'protoc_plugin',
  ]);
  if (pubGlobalActivateCommand.exitCode != 0) {
    print(pubGlobalActivateCommand.stderr.toString().trim());
    throw Exception('Cannot globally install `protoc_plugin` Dart package');
  }
  final newEnvironment = Map<String, String>.from(Platform.environment);
  final currentPathVariable = newEnvironment['PATH'];
  var pubCacheBinPath = Platform.isWindows
      ? '${Platform.environment['LOCALAPPDATA']}\\Pub\\Cache\\bin'
      : '${Platform.environment['HOME']}/.pub-cache/bin';
  if (Platform.environment["PUB_CACHE"] != null) {
    final binPath = Platform.isWindows ? '\\bin' : '/bin';
    pubCacheBinPath = '${Platform.environment["PUB_CACHE"]}$binPath';
  }
  final separator = Platform.isWindows ? ';' : ':';
  final newPathVariable = currentPathVariable != null
      ? '$currentPathVariable$separator$pubCacheBinPath'
      : pubCacheBinPath;
  newEnvironment['PATH'] = newPathVariable;
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    await Directory('$dartOutputPath$subPath').create(recursive: true);
    if (resourceNames.length == 0) {
      continue;
    }
    final protoPaths = <String>[];
    for (final key in resourcesInFolders.keys) {
      protoPaths.add('--proto_path=$protoPath$key');
    }
    final protocDartResult = await Process.run(
      'protoc',
      [
        ...protoPaths,
        '--dart_out=$dartOutputPath$subPath',
        ...resourceNames.map((name) => '$name.proto'),
      ],
      environment: newEnvironment,
    );
    if (protocDartResult.exitCode != 0) {
      print(protocDartResult.stderr.toString().trim());
      throw Exception('Could not compile `.proto` files into Dart');
    }
  }

  // Assign Rust resource index to each message module.
  var resourceIndex = 0;
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    for (final resourceName in resourceNames) {
      appendLineToFile(
        '$rustOutputPath$subPath/$resourceName.rs',
        'pub const ID: i32 = $resourceIndex;',
      );
      appendLineToFile(
        '$dartOutputPath$subPath/$resourceName.pb.dart',
        'const ID = $resourceIndex;',
      );
      resourceIndex += 1;
    }
  }

  // Notify that it's done
  if (!silent) {
    print("🎉 Message code in Dart and Rust is now ready! 🎉");
  }
}

Future<void> verifyServerHeaders() async {
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
  'require-corp',
);""");
  serverFileContent = lines.join("\n");
  await serverFile.writeAsString(serverFileContent);

  // Remove the stamp file to make it re-generated.
  final flutterToolsStampPath = '$flutterPath/bin/cache/flutter_tools.stamp';
  if (await File(flutterToolsStampPath).exists()) {
    await File(flutterToolsStampPath).delete();
  }
}

Future<void> watchAndGenerateMessageCode() async {
  final currentDirectory = Directory.current;
  final messagesPath = join(currentDirectory.path, "messages");
  final messagesDirectory = Directory(messagesPath);

  final watcher = PollingDirectoryWatcher(messagesDirectory.path);
  var generated = true;

  print("Started watching `.proto` files...\n${messagesDirectory.path}");

  watcher.events.listen((event) {
    if (event.path.endsWith(".proto") && generated) {
      var eventType = event.type.toString();
      eventType = eventType[0].toUpperCase() + eventType.substring(1);
      final fileRelativePath = relative(event.path, from: messagesPath);
      print("$eventType: $fileRelativePath");
      generated = false;
    }
  });

  while (true) {
    await Future.delayed(Duration(seconds: 1));
    if (!generated) {
      try {
        await generateMessageCode(silent: true);
        print("Message code generated");
      } catch (error) {
        // When message code generation has failed
      }
      generated = true;
    }
  }
}

Future<void> collectProtoFiles(
  Directory rootDirectory,
  Directory directory,
  Map<String, List<String>> resourcesInFolders,
) async {
  final resources = <String>[];
  await for (final entity in directory.list()) {
    if (entity is File) {
      final filename = entity.uri.pathSegments.last;
      if (filename.endsWith('.proto')) {
        final parts = filename.split('.');
        parts.removeLast(); // Remove the extension from the filename.
        final fileNameWithoutExtension = parts.join('.');
        resources.add(fileNameWithoutExtension);
      }
    } else if (entity is Directory) {
      await collectProtoFiles(
        rootDirectory,
        entity,
        resourcesInFolders,
      ); // Recursive call for subdirectories
    }
  }
  final folderPath = directory.path.replaceFirst(rootDirectory.path, '');
  resourcesInFolders[folderPath] = resources;
}

Future<void> emptyDirectory(String directoryPath) async {
  final directory = Directory(directoryPath);

  if (await directory.exists()) {
    await for (final entity in directory.list()) {
      if (entity is File) {
        await entity.delete();
      } else if (entity is Directory) {
        await entity.delete(recursive: true);
      }
    }
  }
}

Future<void> appendLineToFile(String filePath, String textToAppend) async {
  // Read the existing content of the file
  final file = File(filePath);
  if (!(await file.exists())) {
    await file.create(recursive: true);
  }
  String fileContent = await file.readAsString();

  // Append the new text to the existing content
  fileContent += '\n';
  fileContent += textToAppend;

  // Write the updated content back to the file
  await file.writeAsString(fileContent);
}
