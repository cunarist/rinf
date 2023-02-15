#include "include/rust_in_flutter/rust_in_flutter_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "rust_in_flutter_plugin.h"

void RustInFlutterPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  rust_in_flutter::RustInFlutterPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
