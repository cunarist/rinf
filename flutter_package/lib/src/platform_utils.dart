import 'dart:io';

class PlatformUtils {
  const PlatformUtils._();

  /// Whether the operating system is a version of
  /// [ohos](https://en.wikipedia.org/wiki/OpenHarmony).
  static final isOhos = Platform.operatingSystem == 'ohos';
}
