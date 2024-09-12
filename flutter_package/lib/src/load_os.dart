import 'dart:io' as io;
import 'dart:ffi';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';

String? dynamicLibPath;

void setDynamicLibPath(String path) {
  dynamicLibPath = path;
}

RustLibrary loadRustLibrary() {
  // Use provided dynamic library path if possible.
  // Otherewise, use the default path.
  final path = dynamicLibPath;
  DynamicLibrary lib;
  if (path != null) {
    lib = DynamicLibrary.open(path);
  } else if (io.Platform.isLinux) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isAndroid) {
    lib = DynamicLibrary.open('libhub.so');
  } else if (io.Platform.isWindows) {
    lib = DynamicLibrary.open('hub.dll');
  } else if (io.Platform.isIOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else if (io.Platform.isMacOS) {
    lib = DynamicLibrary.open('rinf.framework/rinf');
  } else {
    throw UnsupportedError('This operating system is not supported.');
  }

  if (io.Platform.isAndroid) {
    // On Android, native library symbols are loaded in local space
    // because of Flutter's `RTLD_LOCAL` behavior.
    // Therefore we cannot use the efficient `RustLibraryNew`.
    // - https://github.com/dart-lang/native/issues/923
    return RustLibraryOld(lib);
  } else {
    // Native library symbols are loaded in global space
    // thanks to Flutter's `RTLD_GLOBAL` behavior.
    return RustLibraryNew();
  }
}

// The central interface for calling native function.

final rustLibrary = loadRustLibrary();

// Common type aliases.
// This is for better readability of the code.

typedef PostCObjectInner = Int8 Function(Int64, Pointer<Dart_CObject>);
typedef PostCObjectPtr = Pointer<NativeFunction<PostCObjectInner>>;
typedef SendDartSignalExtern = Void Function(
  Int32,
  Pointer<Uint8>,
  UintPtr,
  Pointer<Uint8>,
  UintPtr,
);
typedef SendDartSignalWrap = void Function(
  int,
  Pointer<Uint8>,
  int,
  Pointer<Uint8>,
  int,
);

// Direct access to global function symbols loaded in the process.
// These are available only if the native library is
// loaded into global space with `RTLD_GLOBAL` configuration.

@Native<Void Function()>(
  isLeaf: true,
  symbol: 'start_rust_logic_extern',
)
external void startRustLogicExtern();

@Native<Void Function()>(
  isLeaf: true,
  symbol: 'stop_rust_logic_extern',
)
external void stopRustLogicExtern();

@Native<Void Function(Int64)>(
  isLeaf: true,
  symbol: 'prepare_isolate_extern',
)
external void prepareIsolateExtern(
  int port,
);

@Native<Void Function(PostCObjectPtr)>(
  isLeaf: true,
  symbol: 'store_dart_post_cobject',
)
external void storeDartPostCObjectExtern(
  PostCObjectPtr postCObject,
);

@Native<SendDartSignalExtern>(
  isLeaf: true,
  symbol: 'send_dart_signal_extern',
)
external void sendDartSignalExtern(
  int messageId,
  Pointer<Uint8> messageBytesAddress,
  int messageBytesLength,
  Pointer<Uint8> binaryAddress,
  int binaryLength,
);

/// Abstract class for unifying the interface
/// for calling native functions.
abstract class RustLibrary {
  void startRustLogic();
  void stopRustLogic();
  void prepareIsolate(int port);
  void storeDartPostCObject(PostCObjectPtr postCObject);
  void sendDartSignal(
    int messageId,
    Uint8List messageBytes,
    Uint8List binary,
  );
}

/// Class for global native library symbols loaded with `RTLD_GLOBAL`.
/// This is the efficient and ideal way to call native code.
/// `@Native` decorator with `isLeaf` parameter
/// that enables the `Uint8List.address` syntax
/// can only used on global native symbols.
/// - https://github.com/dart-lang/sdk/issues/44589
class RustLibraryNew extends RustLibrary {
  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void prepareIsolate(int port) {
    prepareIsolateExtern(port);
  }

  void storeDartPostCObject(PostCObjectPtr postCObject) {
    storeDartPostCObjectExtern(postCObject);
  }

  void sendDartSignal(
    int messageId,
    Uint8List messageBytes,
    Uint8List binary,
  ) {
    sendDartSignalExtern(
      messageId,
      messageBytes.address,
      messageBytes.length,
      binary.address,
      binary.length,
    );
  }
}

/// Class for local native library symbols loaded with `RTLD_LOCAL`.
/// This is relatively inefficient because `malloc.allocate` is required.
class RustLibraryOld extends RustLibrary {
  late DynamicLibrary lib;
  late void Function() startRustLogicExtern;
  late void Function() stopRustLogicExtern;
  late void Function(int) prepareIsolateExtern;
  late void Function(PostCObjectPtr) storeDartPostCObjectExtern;
  late void Function(int, Pointer<Uint8>, int, Pointer<Uint8>, int)
      sendDartSignalExtern;

  RustLibraryOld(DynamicLibrary lib) {
    this.lib = lib;
    this.startRustLogicExtern =
        lib.lookupFunction<Void Function(), void Function()>(
      'start_rust_logic_extern',
    );
    this.stopRustLogicExtern =
        lib.lookupFunction<Void Function(), void Function()>(
      'stop_rust_logic_extern',
    );
    this.prepareIsolateExtern =
        lib.lookupFunction<Void Function(Int64), void Function(int)>(
      'prepare_isolate_extern',
    );
    this.storeDartPostCObjectExtern = lib.lookupFunction<
        Void Function(PostCObjectPtr), void Function(PostCObjectPtr)>(
      'store_dart_post_cobject',
    );
    this.sendDartSignalExtern =
        lib.lookupFunction<SendDartSignalExtern, SendDartSignalWrap>(
      'send_dart_signal_extern',
    );
  }

  void startRustLogic() {
    startRustLogicExtern();
  }

  void stopRustLogic() {
    stopRustLogicExtern();
  }

  void prepareIsolate(int port) {
    prepareIsolateExtern(port);
  }

  void storeDartPostCObject(PostCObjectPtr postCObject) {
    storeDartPostCObjectExtern(postCObject);
  }

  void sendDartSignal(int messageId, Uint8List messageBytes, Uint8List binary) {
    final Pointer<Uint8> messageMemory = malloc.allocate(messageBytes.length);
    messageMemory.asTypedList(messageBytes.length).setAll(0, messageBytes);

    final Pointer<Uint8> binaryMemory = malloc.allocate(binary.length);
    binaryMemory.asTypedList(binary.length).setAll(0, binary);

    sendDartSignalExtern(
      messageId,
      messageMemory,
      messageBytes.length,
      binaryMemory,
      binary.length,
    );

    malloc.free(messageMemory);
    malloc.free(binaryMemory);
  }
}
