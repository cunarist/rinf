// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:js' as js;
import 'dart:html';
import 'dart:async';

Future<bool> loadJsFile() async {
  final completer = Completer<void>();
  js.context['rinf_load_completer'] = () {
    completer.complete();
  };

  final isAlreadyStarted = js.context.hasProperty("wasm_bindgen");
  if (isAlreadyStarted) {
    // When Dart performs hot restart,
    // the `wasm_bindgen` object is already defined
    // as a global JavaScript variable.
    return true;
  }

  final scriptElement = ScriptElement();
  scriptElement.type = "module";
  scriptElement.innerHtml = '''
import init, * as wasm_bindgen from "/pkg/hub.js";
await init();
window.wasm_bindgen = wasm_bindgen;
rinf_load_completer();
''';
  document.head!.append(scriptElement);

  await completer.future;
  return false;
}
