import 'package:path/path.dart';
import 'package:watcher/watcher.dart';
import 'dart:io';

enum RinfAttribute {
  fromDart,
  fromRust,
}

class MarkedMessage {
  RinfAttribute rinfAttribute;
  String name;
  int id;
  MarkedMessage(
    this.rinfAttribute,
    this.name,
    this.id,
  );
}

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

  // Get ready to prepare channels between Dart and Rust.
  final markedMessagesAll = await parseProtoFiles(
    protoPath,
    resourcesInFolders,
  );
  for (final entry in markedMessagesAll.entries) {
    final subPath = entry.key;
    final filesAndMarks = entry.value;
    for (final entry in filesAndMarks.entries) {
      final filename = entry.key;
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        final messageName = markedMessage.name;
        final camelName = pascalToCamel(messageName);
        final snakeName = pascalToSnake(messageName, true);
        final dartPath = '$dartOutputPath$subPath/$filename.pb.dart';
        final dartFile = File(dartPath);
        final dartContent = await dartFile.readAsString();
        if (markedMessage.rinfAttribute == RinfAttribute.fromDart) {
          if (!dartContent.contains("import 'dart:async'")) {
            await insertTextToFile(
              dartPath,
              '''
// ignore_for_file: invalid_language_version_override
import 'dart:async';
''',
              atFront: true,
            );
          }
          await insertTextToFile(
            dartPath,
            '''
final ${camelName}Controller = StreamController<$messageName>();
final ${camelName}Stream = ${camelName}Controller.stream;
''',
          );
          final rustPath = '$rustOutputPath$subPath/$filename.rs';
          final rustFile = File(rustPath);
          final rustContent = await rustFile.readAsString();
          if (!rustContent.contains("use std::sync")) {
            await insertTextToFile(
              rustPath,
              '''
#![allow(unused_imports)]
use crate::tokio;
use rinf::externs::lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
''',
              atFront: true,
            );
          }
          await insertTextToFile(
            rustPath,
            '''
lazy_static! {
    pub static ref ${snakeName}_RECEIVER: Arc<Mutex<RefCell<Option<Receiver<<$messageName>>>>>> =
        Arc::new(Mutex::new(RefCell::new(None)));
}
''',
          );
        }
        if (markedMessage.rinfAttribute == RinfAttribute.fromRust) {
          if (!dartContent.contains("import 'dart:async'")) {
            await insertTextToFile(
              dartPath,
              '''
// ignore_for_file: invalid_language_version_override
import 'dart:async';
''',
              atFront: true,
            );
          }
          await insertTextToFile(
            dartPath,
            '''
final ${camelName}Controller = StreamController<$messageName>();
final ${camelName}Stream = ${camelName}Controller.stream;
''',
          );
          final rustPath = '$rustOutputPath$subPath/$filename.rs';
          final rustFile = File(rustPath);
          final rustContent = await rustFile.readAsString();
          if (!rustContent.contains("use std::sync")) {
            await insertTextToFile(
              rustPath,
              '''
#![allow(unused_imports)]
use crate::tokio;
use rinf::externs::lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
''',
              atFront: true,
            );
          }
          await insertTextToFile(
            rustPath,
            '''
lazy_static! {
    pub static ref ${snakeName}_SENDER: Arc<Mutex<RefCell<Option<Sender<<$messageName>>>>>> =
        Arc::new(Mutex::new(RefCell::new(None)));
}
''',
          );
        }
      }
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

Future<void> insertTextToFile(
  String filePath,
  String textToAppend, {
  bool atFront = false,
}) async {
  // Read the existing content of the file
  final file = File(filePath);
  if (!(await file.exists())) {
    await file.create(recursive: true);
  }
  String fileContent = await file.readAsString();

  // Append the new text to the existing content
  if (atFront) {
    fileContent = textToAppend + '\n' + fileContent;
  } else {
    fileContent = fileContent + '\n' + textToAppend;
  }

  // Write the updated content back to the file
  await file.writeAsString(fileContent);
}

Future<Map<String, Map<String, List<MarkedMessage>>>> parseProtoFiles(
  String protoPath,
  Map<String, List<String>> resourcesInFolders,
) async {
  final markedMessages = <String, Map<String, List<MarkedMessage>>>{};
  for (final entry in resourcesInFolders.entries) {
    final subpath = entry.key;
    final filenames = entry.value;
    final markedMessagesInFiles = <String, List<MarkedMessage>>{};
    for (final filename in filenames) {
      markedMessagesInFiles[filename] = [];
    }
    markedMessages[subpath] = markedMessagesInFiles;
  }
  int messageId = 0;
  for (final entry in resourcesInFolders.entries) {
    final subpath = entry.key;
    for (final filename in entry.value) {
      final protoFile = File('$protoPath/$subpath/$filename.proto');
      final content = await protoFile.readAsString();
      final regExp = RegExp(r'{[^}]*}');
      // Remove all { ... } blocks from the string
      final contentWithoutBlocks = content.replaceAll(regExp, ';');
      final statements = contentWithoutBlocks.split(";");
      for (final statementRaw in statements) {
        final statement = statementRaw.trim();
        if (statement.startsWith("// FROM:DART")) {
          final lines = statement.split('\n');
          for (final line in lines) {
            final trimmed = line.trim();
            if (trimmed.startsWith("message")) {
              final messageName = trimmed.replaceFirst("message", "").trim();
              markedMessages[subpath]![filename]!.add(MarkedMessage(
                RinfAttribute.fromDart,
                messageName,
                messageId,
              ));
              messageId += 1;
            }
          }
        } else if (statement.startsWith("// FROM:RUST")) {
          final lines = statement.split('\n');
          for (final line in lines) {
            final trimmed = line.trim();
            if (trimmed.startsWith("message")) {
              final messageName = trimmed.replaceFirst("message", "").trim();
              markedMessages[subpath]![filename]!.add(MarkedMessage(
                RinfAttribute.fromRust,
                messageName,
                messageId,
              ));
              messageId += 1;
            }
          }
        }
      }
    }
  }
  return markedMessages;
}

String pascalToCamel(String input) {
  if (input.isEmpty) {
    return input;
  }

  return input[0].toLowerCase() + input.substring(1);
}

String pascalToSnake(String input, bool upperCase) {
  if (input.isEmpty) {
    return input;
  }

  final camelCase = pascalToCamel(input);
  String snakeCase = camelCase.replaceAllMapped(
      RegExp(r'[A-Z]'), (Match match) => '_${match.group(0)?.toLowerCase()}');

  if (upperCase) {
    snakeCase = snakeCase.toUpperCase();
  }

  return snakeCase;
}
