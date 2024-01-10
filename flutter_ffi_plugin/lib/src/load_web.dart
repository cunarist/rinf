import 'package:universal_html/js.dart' as js;
import 'package:universal_html/html.dart';
import 'dart:async';

Future<void> loadJsFile() async {
  final completer = Completer<void>();
  js.context['rinf_load_completer'] = () {
    completer.complete();
  };

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
}
