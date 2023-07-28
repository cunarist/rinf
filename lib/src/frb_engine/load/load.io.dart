import 'dart:ffi';
import 'dart:io';

/// agnostic dynamic library loading on native platforms
DynamicLibrary loadLibForFlutter(String path) => Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS && Abi.current() == Abi.macosX64
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
DynamicLibrary loadLibForDart(String path) =>
    Platform.isIOS ? DynamicLibrary.process() : DynamicLibrary.open(path);
