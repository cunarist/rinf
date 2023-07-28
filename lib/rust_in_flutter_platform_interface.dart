import 'package:plugin_platform_interface/plugin_platform_interface.dart';
import 'rust_in_flutter_method_channel.dart';

abstract class RustInFlutterPlatform extends PlatformInterface {
  /// Constructs a RustInFlutterPlatform.
  RustInFlutterPlatform() : super(token: _token);

  static final Object _token = Object();

  static RustInFlutterPlatform _instance = MethodChannelRustInFlutter();

  /// The default instance of [RustInFlutterPlatform] to use.
  ///
  /// Defaults to [MethodChannelRustInFlutter].
  static RustInFlutterPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [RustInFlutterPlatform] when
  /// they register themselves.
  static set instance(RustInFlutterPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
