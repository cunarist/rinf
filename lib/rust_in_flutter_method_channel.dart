import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'rust_in_flutter_platform_interface.dart';

/// An implementation of [RustInFlutterPlatform] that uses method channels.
class MethodChannelRustInFlutter extends RustInFlutterPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('rust_in_flutter');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
