#ifndef FLUTTER_PLUGIN_RUST_IN_FLUTTER_PLUGIN_H_
#define FLUTTER_PLUGIN_RUST_IN_FLUTTER_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace rust_in_flutter {

class RustInFlutterPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  RustInFlutterPlugin();

  virtual ~RustInFlutterPlugin();

  // Disallow copy and assign.
  RustInFlutterPlugin(const RustInFlutterPlugin&) = delete;
  RustInFlutterPlugin& operator=(const RustInFlutterPlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace rust_in_flutter

#endif  // FLUTTER_PLUGIN_RUST_IN_FLUTTER_PLUGIN_H_
