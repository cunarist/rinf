import 'dart:io';
import 'package:path/path.dart';
import 'package:watcher/watcher.dart';
import 'config.dart';
import 'common.dart';
import 'internet.dart';

enum MarkType {
  dartSignal,
  dartSignalBinary,
  rustSignal,
  rustSignalBinary,
  rustAttribute,
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
  final protoPath = flutterProjectPath.uri.join(messageConfig.inputDir);
  final rustOutputPath =
      flutterProjectPath.uri.join(messageConfig.rustOutputDir);
  final dartOutputPath =
      flutterProjectPath.uri.join(messageConfig.dartOutputDir);
  await Directory.fromUri(rustOutputPath).create(recursive: true);
  await emptyDirectory(rustOutputPath);
  await Directory.fromUri(dartOutputPath).create(recursive: true);
  await emptyDirectory(dartOutputPath);

  // Get the list of `.proto` files.
  final resourcesInFolders = <String, List<String>>{};
  await collectProtoFiles(
    Directory.fromUri(protoPath),
    Directory.fromUri(protoPath),
    resourcesInFolders,
  );

  // Analyze marked messages in `.proto` files.
  final markedMessagesAll = await analyzeMarkedMessages(
    protoPath,
    resourcesInFolders,
  );

  // Include `package` statement in `.proto` files.
  // Package name should be the same as the filename
  // because Rust filenames are written with package name
  // and Dart filenames are written with the `.proto` filename.
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    for (final resourceName in resourceNames) {
      final protoFile = File.fromUri(
        protoPath.join(subPath).join('$resourceName.proto'),
      );
      final lines = await protoFile.readAsLines();
      List<String> outputLines = [];
      for (var line in lines) {
        final packagePattern = r'^package\s+[a-zA-Z_][a-zA-Z0-9_\.]*\s*[^=];$';
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
  if (isInternetConnected) {
    if (!silent) {
      print("Ensuring `protoc-gen-prost` for Rust." +
          "\nThis is done by installing it globally on the system.");
    }
    final cargoInstallCommand = await Process.run('cargo', [
      'install',
      'protoc-gen-prost',
      ...(messageConfig.rustSerde ? ['protoc-gen-prost-serde'] : [])
    ]);
    if (cargoInstallCommand.exitCode != 0) {
      print(cargoInstallCommand.stderr.toString().trim());
      throw Exception('Cannot globally install `protoc-gen-prost` Rust crate');
    }
  } else {
    if (!silent) {
      print("Skipping ensurement of `protoc-gen-prost` for Rust.");
    }
  }
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    await Directory.fromUri(rustOutputPath.join(subPath))
        .create(recursive: true);
    if (resourceNames.isEmpty) {
      continue;
    }
    final protoPaths = <String>[];
    for (final key in resourcesInFolders.keys) {
      final joinedPath = protoPath.join(key).toFilePath();
      protoPaths.add('--proto_path=$joinedPath');
    }
    final rustFullPath = rustOutputPath.join(subPath).toFilePath();
    final protocRustResult = await Process.run('protoc', [
      ...protoPaths,
      '--prost_out=$rustFullPath',
      ...(messageConfig.rustSerde ? ['--prost-serde_out=$rustFullPath'] : []),
      ...resourceNames.map((name) => '$name.proto'),
      ...markedMessagesAll.values.fold<List<String>>([], (args, messages) {
        messages.values.forEach((messages) => args.addAll(messages
            .where((message) => message.markType == MarkType.rustAttribute)
            .map((message) => message.name)));
        return args;
      })
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
        final subPathSplitted = subPath
            .trim()
            .split("/")
            .where(
              (element) => element.isNotEmpty,
            )
            .toList();
        final otherSubPathSplitted = otherSubPath
            .split("/")
            .where(
              (element) => element.isNotEmpty,
            )
            .toList();
        ;
        if (subPathSplitted.length != otherSubPathSplitted.length - 1) {
          continue;
        }
        var isOtherChild = true;
        for (int i = 0; i < subPathSplitted.length; i++) {
          if (subPathSplitted[i] != subPathSplitted[i]) {
            isOtherChild = false;
            break;
          }
        }
        if (!isOtherChild) {
          continue;
        }
        final childName = otherSubPathSplitted.last;
        modRsLines.add('pub mod $childName;');
      }
    }
    if (subPath == "/") {
      modRsLines.add("pub mod generated;");
    }
    final modRsContent = modRsLines.join('\n');
    await File.fromUri(rustOutputPath.join(subPath).join('mod.rs'))
        .writeAsString(modRsContent);
  }

  // Generate Dart message files.
  if (isInternetConnected) {
    if (!silent) {
      print("Ensuring `protoc_plugin` for Dart." +
          "\nThis is done by installing it globally on the system.");
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
  } else {
    if (!silent) {
      print("Skipping ensurement of `protoc_plugin` for Dart.");
    }
  }
  for (final entry in resourcesInFolders.entries) {
    final subPath = entry.key;
    final resourceNames = entry.value;
    await Directory.fromUri(dartOutputPath.join(subPath))
        .create(recursive: true);
    if (resourceNames.isEmpty) {
      continue;
    }
    final protoPaths = <String>[];
    for (final key in resourcesInFolders.keys) {
      final joinedPath = protoPath.join(key).toFilePath();
      protoPaths.add('--proto_path=$joinedPath');
    }
    final dartFullPath = dartOutputPath.join(subPath).toFilePath();
    final protocDartResult = await Process.run(
      'protoc',
      [
        ...protoPaths,
        '--dart_out=$dartFullPath',
        ...resourceNames.map((name) => '$name.proto'),
      ],
    );
    if (protocDartResult.exitCode != 0) {
      print(protocDartResult.stderr.toString().trim());
      throw Exception('Could not compile `.proto` files into Dart');
    }
  }

  // Prepare communication channels between Dart and Rust.
  for (final entry in markedMessagesAll.entries) {
    final subPath = entry.key;
    final filesAndMarks = entry.value;
    for (final entry in filesAndMarks.entries) {
      if (entry.value.isEmpty) {
        continue;
      }
      final filename = entry.key;
      final dartPath = dartOutputPath.join(subPath).join('$filename.pb.dart');
      final dartFile = File.fromUri(dartPath);
      final dartContent = await dartFile.readAsString();
      final rustPath = rustOutputPath.join(subPath).join('$filename.rs');
      final rustFile = File.fromUri(rustPath);
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
use rinf::{debug_print, send_rust_signal, DartSignal, RinfError};
use std::sync::Mutex;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

''',
          atFront: true,
        );
      }
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        final messageName = markedMessage.name;
        final markType = markedMessage.markType;
        final camelName = pascalToCamel(messageName);
        final snakeName = pascalToSnake(messageName);
        if (markType == MarkType.dartSignal ||
            markType == MarkType.dartSignalBinary) {
          await insertTextToFile(
            rustPath,
            '''
type ${messageName}Cell = Mutex<Option<(
    UnboundedSender<DartSignal<${normalizePascal(messageName)}>>,
    Option<UnboundedReceiver<DartSignal<${normalizePascal(messageName)}>>>,
)>>;
pub static ${snakeName.toUpperCase()}_CHANNEL: ${messageName}Cell =
    Mutex::new(None);

impl ${normalizePascal(messageName)} {
    pub fn get_dart_signal_receiver()
        -> Result<UnboundedReceiver<DartSignal<Self>>, RinfError> 
    {       
        let mut guard = ${snakeName.toUpperCase()}_CHANNEL
            .lock()
            .map_err(|_| RinfError::LockMessageChannel)?;
        if guard.is_none() {
            let (sender, receiver) = unbounded_channel();
            guard.replace((sender, Some(receiver)));
        }
        let (mut sender, mut receiver_option) = guard
            .take()
            .ok_or(RinfError::NoMessageChannel)?;
        // After Dart's hot restart or app reopen on mobile devices,
        // a sender from the previous run already exists
        // which is now closed.
        if sender.is_closed() {
            let receiver;
            (sender, receiver) = unbounded_channel();
            receiver_option = Some(receiver);
        }
        let receiver = receiver_option.ok_or(RinfError::MessageReceiverTaken)?;
        guard.replace((sender, None));
        Ok(receiver)
    }
}
''',
          );
          if (markType == MarkType.dartSignal) {
            await insertTextToFile(
              dartPath,
              '''
extension ${messageName}Extension on $messageName{
  void sendSignalToRust() {
    sendDartSignal(
      ${markedMessage.id},
      this.writeToBuffer(),
      Uint8List(0),
    );
  }
}
''',
            );
          }
        }
        if (markType == MarkType.dartSignalBinary) {
          await insertTextToFile(
            dartPath,
            '''
extension ${messageName}Extension on $messageName{
  void sendSignalToRust(Uint8List binary) {
    sendDartSignal(
      ${markedMessage.id},
      this.writeToBuffer(),
      binary,
    );
  }
}
''',
          );
        }
        if (markType == MarkType.rustSignal ||
            markType == MarkType.rustSignalBinary) {
          await insertTextToFile(
            dartPath,
            '''
static final rustSignalStream =
    ${camelName}Controller.stream.asBroadcastStream();
''',
            after: "class $messageName extends \$pb.GeneratedMessage {",
          );
          await insertTextToFile(
            dartPath,
            '''
final ${camelName}Controller = StreamController<RustSignal<$messageName>>();
''',
          );
        }
        if (markType == MarkType.rustSignal) {
          await insertTextToFile(
            rustPath,
            '''
impl ${normalizePascal(messageName)} {
    pub fn send_signal_to_dart(&self) {
        let result = send_rust_signal(
            ${markedMessage.id},
            self.encode_to_vec(),
            Vec::new(),
        );
        if let Err(error) = result {
            debug_print!("{error}\\n{self:?}");
        }
    }
}
''',
          );
        }
        if (markType == MarkType.rustSignalBinary) {
          await insertTextToFile(
            rustPath,
            '''
impl ${normalizePascal(messageName)} {
    pub fn send_signal_to_dart(&self, binary: Vec<u8>) {
        let result = send_rust_signal(
            ${markedMessage.id},
            self.encode_to_vec(),
            binary,
        );
        if let Err(error) = result {
            debug_print!("{error}\\n{self:?}");
        }
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

use crate::tokio;
use prost::Message;
use rinf::{debug_print, DartSignal, RinfError};
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use tokio::sync::mpsc::unbounded_channel;

type Handler = dyn Fn(&[u8], &[u8]) -> Result<(), RinfError> + Send + Sync;
type DartSignalHandlers = HashMap<i32, Box<Handler>>;
static DART_SIGNAL_HANDLERS: OnceLock<DartSignalHandlers> = OnceLock::new();

pub fn assign_dart_signal(
    message_id: i32,
    message_bytes: &[u8],
    binary: &[u8]
) -> Result<(), RinfError> {    
    let hash_map = DART_SIGNAL_HANDLERS.get_or_init(|| {
        let mut new_hash_map: DartSignalHandlers = HashMap::new();
''';
  for (final entry in markedMessagesAll.entries) {
    final subpath = entry.key;
    final files = entry.value;
    for (final entry in files.entries) {
      final filename = entry.key;
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        final markType = markedMessage.markType;
        if (markType == MarkType.dartSignal ||
            markType == MarkType.dartSignalBinary) {
          final messageName = markedMessage.name;
          final snakeName = pascalToSnake(messageName);
          var modulePath = subpath.replaceAll("/", "::");
          modulePath = modulePath == "::" ? "" : modulePath;
          rustReceiveScript += '''
new_hash_map.insert(
    ${markedMessage.id},
    Box::new(|message_bytes: &[u8], binary: &[u8]| {
        use super::$modulePath$filename::*;
        let message =
            ${normalizePascal(messageName)}::decode(message_bytes)
            .map_err(|_| RinfError::DecodeMessage)?;
        let dart_signal = DartSignal {
            message,
            binary: binary.to_vec(),
        };
        let mut guard = ${snakeName.toUpperCase()}_CHANNEL
            .lock()
            .map_err(|_| RinfError::LockMessageChannel)?;
        if guard.is_none() {
            let (sender, receiver) = unbounded_channel();
            guard.replace((sender, Some(receiver)));
        }
        let mut pair = guard
            .as_ref()
            .ok_or(RinfError::NoMessageChannel)?;
        // After Dart's hot restart or app reopen on mobile devices,
        // a sender from the previous run already exists
        // which is now closed.
        if pair.0.is_closed() {
            let (sender, receiver) = unbounded_channel();
            guard.replace((sender, Some(receiver)));
            pair = guard
                .as_ref()
                .ok_or(RinfError::NoMessageChannel)?;
        }
        let sender = &pair.0;
        let _ = sender.send(dart_signal);
        Ok(())
    }),
);
''';
        }
      }
    }
  }
  rustReceiveScript += '''
        new_hash_map
    });

    let signal_handler = match hash_map.get(&message_id) {
        Some(inner) => inner,
        None => return Err(RinfError::NoSignalHandler),
    };
    signal_handler(message_bytes, binary)
}
''';
  await File.fromUri(rustOutputPath.join('generated.rs'))
      .writeAsString(rustReceiveScript);

  // Get ready to handle received signals in Dart.
  var dartReceiveScript = "";
  dartReceiveScript += '''
// ignore_for_file: unused_import

import 'dart:typed_data';
import 'package:rinf/rinf.dart';

final rustSignalHandlers = <int, void Function(Uint8List, Uint8List)>{
''';
  for (final entry in markedMessagesAll.entries) {
    final subpath = entry.key;
    final files = entry.value;
    for (final entry in files.entries) {
      final filename = entry.key;
      final markedMessages = entry.value;
      for (final markedMessage in markedMessages) {
        final markType = markedMessage.markType;
        if (markType == MarkType.rustSignal ||
            markType == MarkType.rustSignalBinary) {
          final messageName = markedMessage.name;
          final camelName = pascalToCamel(messageName);
          final importPath = subpath == "/"
              ? '$filename.pb.dart'
              : '$subpath$filename.pb.dart';
          if (!dartReceiveScript.contains(importPath)) {
            dartReceiveScript = '''
import './$importPath' as $filename;
''' +
                dartReceiveScript;
          }
          dartReceiveScript += '''
${markedMessage.id}: (Uint8List messageBytes, Uint8List binary) {
  final message = $filename.$messageName.fromBuffer(messageBytes);
  final rustSignal = RustSignal(
    message,
    binary,
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

void assignRustSignal(int messageId, Uint8List messageBytes, Uint8List binary) {
  rustSignalHandlers[messageId]!(messageBytes, binary);
}
''';
  await File.fromUri(dartOutputPath.join('generated.dart'))
      .writeAsString(dartReceiveScript);

  // Notify that it's done
  if (!silent) {
    print("🎉 Message code in Dart and Rust is now ready! 🎉");
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
  subPath = '$subPath/'; // Indicate that it's a folder, not a file
  resourcesInFolders[subPath] = resources;
}

Future<void> emptyDirectory(Uri directoryPath) async {
  final directory = Directory.fromUri(directoryPath);

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
  Uri filePath,
  String textToAppend, {
  bool atFront = false,
  String? after,
}) async {
  // Read the existing content of the file
  final file = File.fromUri(filePath);
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
  Uri protoPath,
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
    final subPath = entry.key;
    for (final filename in entry.value) {
      final protoFile = File.fromUri(
        protoPath.join(subPath).join('$filename.proto'),
      );
      final content = await protoFile.readAsString();
      final regExp = RegExp(r'{[^}]*}');
      final attrExp = RegExp(r"(?<=\[RINF:RUST-ATTRIBUTE\().*(?=\)\])");

      // Remove all { ... } blocks from the string
      final contentWithoutBlocks = content.replaceAll(regExp, ';');
      final statements = contentWithoutBlocks.split(";");
      for (final statementRaw in statements) {
        final statement = statementRaw.trim();
        // To find "}\n\n// [RINF:RUST-SIGNAL]",
        // `contains` is used instead of `startsWith`
        String? messageName = null;
        final lines = statement.split('\n');
        for (final line in lines) {
          final trimmed = line.trim();
          if (trimmed.startsWith("message")) {
            messageName = trimmed.replaceFirst("message", "").trim();
          }
        }
        if (messageName == null) {
          // When the statement is not a message
          continue;
        }
        MarkType? markType = null;
        if (statement.contains("[RINF:DART-SIGNAL]")) {
          markType = MarkType.dartSignal;
        } else if (statement.contains("[RINF:DART-SIGNAL-BINARY]")) {
          markType = MarkType.dartSignalBinary;
        } else if (statement.contains("[RINF:RUST-SIGNAL]")) {
          markType = MarkType.rustSignal;
        } else if (statement.contains("[RINF:RUST-SIGNAL-BINARY]")) {
          markType = MarkType.rustSignalBinary;
        }

        // find [RINF:RUST-ATTRIBUTE(...)]
        var attr = attrExp.stringMatch(statement);
        if (attr != null) {
          markedMessages[subPath]![filename]!.add(MarkedMessage(
            MarkType.rustAttribute,
            "--prost_opt=type_attribute=$filename.$messageName=${attr.replaceAll(",", "\\,")}",
            -1,
          ));
          continue;
        }

        if (markType == null) {
          // If there's no mark in the message, just ignore it
          continue;
        }
        markedMessages[subPath]![filename]!.add(MarkedMessage(
          markType,
          messageName,
          messageId,
        ));
        messageId += 1;
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
