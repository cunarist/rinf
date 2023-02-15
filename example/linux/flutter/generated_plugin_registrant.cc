//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <rust_in_flutter/rust_in_flutter_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) rust_in_flutter_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "RustInFlutterPlugin");
  rust_in_flutter_plugin_register_with_registrar(rust_in_flutter_registrar);
}
