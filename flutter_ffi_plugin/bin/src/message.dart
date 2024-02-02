import 'dart:io';
import 'package:path/path.dart';
import 'package:watcher/watcher.dart';
import 'config.dart';

enum MarkType {
  fromDart,
  fromRust,
}

class MarkedMessage {
  MarkType markType;
  String name;
  int id;
  MarkedMessage(
    this.markType,
    this.name,
    this.id,
  );
}

Future<void> generateMessageCode({
  bool silent = false,
  required RinfConfigMessage messageConfig,
}) async {
  // Prepare paths.
  final flutterProjectPath = Directory.current;
  final protoPath =
      flutterProjectPath.uri.resolve(messageConfig.inputDir).toFilePath();
  final rustOutputPath =
      flutterProjectPath.uri.resolve(messageConfig.rustOutputDir).toFilePath();
  final dartOutputPath =
      flutterProjectPath.uri.resolve(messageConfig.dartOutputDir).toFilePath();
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
        "\nThis might take a while if there are new updates.");
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
        final relation =
            otherSubPath.replaceFirst(subPath, "").replaceFirst('/', '');
        if (!relation.contains('/')) {
          modRsLines.add('pub mod $relation;');
        }
      }
    }
    if (subPath == "") {
      modRsLines.add("pub mod generated;");
    }
    final modRsContent = modRsLines.join('\n');
    await File('$rustOutputPath$subPath/mod.rs').writeAsString(modRsContent);
  }

  // Generate Dart message files.
  if (!silent) {
    print("Verifying `protoc_plugin` for Dart." +
        "\nThis might take a while if there are new updates.");
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
    );
    if (protocDartResult.exitCode != 0) {
      print(protocDartResult.stderr.toString().trim());
      throw Exception('Could not compile `.proto` files into Dart');
    }
  }

  // Analyze marked messages in `.proto` files.
  final markedMessagesAll = await analyzeMarkedMessages(
    protoPath,
    resourcesInFolders,
  );

  // Prepare communication channels between Dart and Rust.
  for (final entry in markedMessagesAll.entries) {
    final subPath = entry.key;
    final filesAndMarks = entry.value;
    for (final entry in filesAndMarks.entries) {
      if (entry.value.length == 0) {
        continue;
      }
      final filename = entry.key;
      final dartPath = '$dartOutputPath$subPath/$filename.pb.dart';
      final dartFile = File(dartPath);
      final dartContent = await dartFile.readAsString();
      final rustPath = '$rustOutputPath$subPath/$filename.rs';
      final rustFile = File(rustPath);
      final rustContent = await rustFile.readAsString();
      if (!dartContent.contains("import 'dart:typed_data'")) {
        await insertTextToFile(
          dartPath,
          '''
// ignore_for_file: invalid_language_version_override

import 'dart:async';
import 'dart:typed_data';
import 'package:rinf/rinf.dart';
''',
          atFront: true,
        );
      }
      if (!rustContent.contains("use std::sync")) {
        await insertTextToFile(
          rustPath,
          '''
#![allow(unused_imports)]

use crate::tokio;
use prost::Message;
use rinf::send_rust_signal;
use rinf::DartSignal;
use rinf::SharedCell;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::OnceLock;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
''',
          atFront: true,
        );
      }
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        final messageName = markedMessage.name;
        final camelName = pascalToCamel(messageName);
        final snakeName = pascalToSnake(messageName);
        if (markedMessage.markType == MarkType.fromDart) {
          await insertTextToFile(
            dartPath,
            '''
void sendSignalToRust(Uint8List? blob) {
  sendDartSignal(
    ${markedMessage.id},
    this.writeToBuffer(),
    blob,
  );
}
''',
            after: "class $messageName extends \$pb.GeneratedMessage {",
          );
          await insertTextToFile(
            rustPath,
            '''
type ${messageName}Cell =
    SharedCell<Sender<DartSignal<${normalizePascal(messageName)}>>>;
pub static ${snakeName.toUpperCase()}_SENDER: ${messageName}Cell =
    OnceLock::new();

impl ${normalizePascal(messageName)} {
    pub fn get_dart_signal_receiver() -> Receiver<DartSignal<Self>> {
        let (sender, receiver) = tokio::sync::mpsc::channel(1024);
        let cell = ${snakeName.toUpperCase()}_SENDER
            .get_or_init(|| Mutex::new(RefCell::new(None)))
            .lock()
            .unwrap();
        cell.replace(Some(sender));
        receiver
    }
}
''',
          );
        }
        if (markedMessage.markType == MarkType.fromRust) {
          await insertTextToFile(
            dartPath,
            '''
static Stream<RustSignal<$messageName>> rustSignalStream =
    ${camelName}Controller.stream;
''',
            after: "class $messageName extends \$pb.GeneratedMessage {",
          );
          await insertTextToFile(
            dartPath,
            '''
final ${camelName}Controller = StreamController<RustSignal<$messageName>>.broadcast();
''',
          );
          await insertTextToFile(
            rustPath,
            '''
impl ${normalizePascal(messageName)} {
    pub fn send_signal_to_dart(&self, blob: Option<Vec<u8>>) {
        send_rust_signal(
            ${markedMessage.id},
            self.encode_to_vec(),
            blob
        );
    }
}
''',
          );
        }
      }
    }
  }

  // Get ready to handle received signals in Rust.
  var rustReceiveScript = "";
  rustReceiveScript += '''
#![allow(unused_imports)]
#![allow(unused_mut)]

use prost::Message;
use rinf::DartSignal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;

type SignalHandlers =
    OnceLock<Mutex<HashMap<i32, Box<dyn Fn(Vec<u8>, Option<Vec<u8>>) + Send>>>>;
static SIGNAL_HANDLERS: SignalHandlers = OnceLock::new();

pub fn handle_dart_signal(
    message_id: i32,
    message_bytes: Vec<u8>,
    blob: Option<Vec<u8>>
) {    
    let mutex = SIGNAL_HANDLERS.get_or_init(|| {
        let mut hash_map =
            HashMap
            ::<i32, Box<dyn Fn(Vec<u8>, Option<Vec<u8>>) + Send + 'static>>
            ::new();
''';
  for (final entry in markedMessagesAll.entries) {
    final subpath = entry.key;
    final files = entry.value;
    for (final entry in files.entries) {
      final filename = entry.key;
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        if (markedMessage.markType == MarkType.fromDart) {
          final messageName = markedMessage.name;
          final snakeName = pascalToSnake(messageName);
          var modulePath = subpath.replaceAll("/", "::");
          rustReceiveScript += '''
hash_map.insert(
    ${markedMessage.id},
    Box::new(|message_bytes: Vec<u8>, blob: Option<Vec<u8>>| {
        use super$modulePath::$filename::*;
        let message = ${normalizePascal(messageName)}::decode(
            message_bytes.as_slice()
        ).unwrap();
        let dart_signal = DartSignal {
            message,
            blob,
        };
        let cell = ${snakeName.toUpperCase()}_SENDER
            .get_or_init(|| Mutex::new(RefCell::new(None)))
            .lock()
            .unwrap();
        let sender = cell.clone().replace(None).expect(concat!(
            "Looks like the channel is not created yet.",
            "\\nTry using `$messageName::get_dart_signal_receiver()`."
        ));
        let _ = sender.try_send(dart_signal);
    }),
);
''';
        }
      }
    }
  }
  rustReceiveScript += '''
        Mutex::new(hash_map)
    });

    let guard = mutex.lock().unwrap();
    let signal_handler = guard.get(&message_id).unwrap();
    signal_handler(message_bytes, blob);
}
''';
  await File('$rustOutputPath/generated.rs').writeAsString(rustReceiveScript);

  // Get ready to handle received signals in Dart.
  var dartReceiveScript = "";
  dartReceiveScript += '''
// ignore_for_file: unused_import

import 'dart:typed_data';
import 'package:rinf/rinf.dart';

void initializeRust() async {
  prepareInterface(handleRustSignal);
  startRustLogic();
}

void finalizeRust() async {
  stopRustLogic();
}

final signalHandlers = <int, void Function(Uint8List, Uint8List?)>{
''';
  for (final entry in markedMessagesAll.entries) {
    final subpath = entry.key;
    final files = entry.value;
    for (final entry in files.entries) {
      final filename = entry.key;
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        if (markedMessage.markType == MarkType.fromRust) {
          final messageName = markedMessage.name;
          final camelName = pascalToCamel(messageName);
          final importPath = '$subpath/$filename.pb.dart';
          if (!dartReceiveScript.contains(importPath)) {
            dartReceiveScript = '''
import '.$importPath' as $filename;
''' +
                dartReceiveScript;
          }
          dartReceiveScript += '''
${markedMessage.id}: (Uint8List messageBytes, Uint8List? blob) {
  final message = $filename.$messageName.fromBuffer(messageBytes);
  final rustSignal = RustSignal(
    message,
    blob,
  );
  $filename.${camelName}Controller.add(rustSignal);
},
''';
        }
      }
    }
  }
  dartReceiveScript += '''
};

void handleRustSignal(int messageId, Uint8List messageBytes, Uint8List? blob) {
  signalHandlers[messageId]!(messageBytes, blob);
}
''';
  await File('$dartOutputPath/generated.dart').writeAsString(dartReceiveScript);

  // Notify that it's done
  if (!silent) {
    print("🎉 Message code in Dart and Rust is now ready! 🎉");
  }
}

Future<void> patchServerHeaders() async {
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

Future<void> watchAndGenerateMessageCode(
    {required RinfConfigMessage messageConfig}) async {
  final currentDirectory = Directory.current;
  final messagesPath = join(currentDirectory.path, "messages");
  final messagesDirectory = Directory(messagesPath);

  // Listen to keystrokes in the CLI.
  var shouldQuit = false;
  stdin.echoMode = false;
  stdin.lineMode = false;
  stdin.listen((keyCodes) {
    for (final keyCode in keyCodes) {
      final key = String.fromCharCode(keyCode);
      if (key.toLowerCase() == 'q') {
        shouldQuit = true;
      }
    }
  });

  // Watch `.proto` files.
  final watcher = PollingDirectoryWatcher(messagesDirectory.path);
  var generated = true;
  print("Watching `.proto` files...");
  print("Press `q` to stop watching.");
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
    if (shouldQuit) {
      exit(0);
    }
    if (!generated) {
      try {
        await generateMessageCode(silent: true, messageConfig: messageConfig);
        print("Message code generated.");
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
  var subPath = directory.path.replaceFirst(rootDirectory.path, '');
  subPath = subPath.replaceAll("\\", "/"); // For Windows
  resourcesInFolders[subPath] = resources;
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
  String? after,
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
  } else if (after != null) {
    fileContent = fileContent.replaceFirst(after, after + textToAppend);
  } else {
    fileContent = fileContent + '\n' + textToAppend;
  }

  // Write the updated content back to the file
  await file.writeAsString(fileContent);
}

Future<Map<String, Map<String, List<MarkedMessage>>>> analyzeMarkedMessages(
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
        if (statement.startsWith("// [RINF:DART-SIGNAL]")) {
          final lines = statement.split('\n');
          for (final line in lines) {
            final trimmed = line.trim();
            if (trimmed.startsWith("message")) {
              final messageName = trimmed.replaceFirst("message", "").trim();
              markedMessages[subpath]![filename]!.add(MarkedMessage(
                MarkType.fromDart,
                messageName,
                messageId,
              ));
              messageId += 1;
            }
          }
        } else if (statement.startsWith("// [RINF:RUST-SIGNAL]")) {
          final lines = statement.split('\n');
          for (final line in lines) {
            final trimmed = line.trim();
            if (trimmed.startsWith("message")) {
              final messageName = trimmed.replaceFirst("message", "").trim();
              markedMessages[subpath]![filename]!.add(MarkedMessage(
                MarkType.fromRust,
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

String pascalToSnake(String input) {
  if (input.isEmpty) {
    return input;
  }
  final camelCase = pascalToCamel(input);
  String snakeCase = camelCase.replaceAllMapped(
      RegExp(r'[A-Z]'), (Match match) => '_${match.group(0)?.toLowerCase()}');
  return snakeCase;
}

String snakeToCamel(String input) {
  List<String> parts = input.split('_');
  String camelCase = parts[0];
  for (int i = 1; i < parts.length; i++) {
    camelCase += parts[i][0].toUpperCase() + parts[i].substring(1);
  }
  return camelCase;
}

/// Converts a string `HeLLLLLLLo` to `HeLlllllLo`,
/// just like `protoc-gen-prost` does.
String normalizePascal(String input) {
  var upperStreak = "";
  var result = "";
  for (final character in input.split('')) {
    if (character.toUpperCase() == character) {
      upperStreak += character;
    } else {
      final fixedUpperStreak = lowerBetween(upperStreak);
      upperStreak = "";
      result += fixedUpperStreak;
      result += character;
    }
  }
  result += lowerExceptFirst(upperStreak);
  return result;
}

String lowerBetween(String input) {
  if (input.isEmpty) {
    return input;
  }
  if (input.length == 1) {
    return input.toUpperCase(); // Keep the single character in uppercase
  }
  String firstChar = input.substring(0, 1);
  String lastChar = input.substring(input.length - 1);
  String middleChars = input.substring(1, input.length - 1).toLowerCase();
  return '$firstChar$middleChars$lastChar';
}

String lowerExceptFirst(String input) {
  if (input.isEmpty) {
    return input;
  }
  String firstChar = input.substring(0, 1);
  String restOfString = input.substring(1).toLowerCase();
  return '$firstChar$restOfString';
}
