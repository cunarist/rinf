// ignore_for_file: avoid_web_libraries_in_flutter

import "dart:js" as js;
import "dart:html";
import "dart:async";

String? jsLibPath;

// When Dart performs hot restart,
// the `rinf` object is already defined
// as a global JavaScript variable.
final wasAlreadyLoaded = js.context.hasProperty("rinf");

void setJsLibPath(String path) {
  jsLibPath = path;
}

Future<void> loadJsFile() async {
  if (wasAlreadyLoaded) {
    return;
  }

  final loadCompleter = Completer<void>();
  js.context["completeRinfLoad"] = loadCompleter.complete;

  // Flutter app doesn't always have the top-level path of the domain.
  // Sometimes, the flutter app might be placed in a lower path.
  // This variable includes domain and the base path.
  final baseHref = Uri.base;

  // Use the default JavaScript path unless provided.
  final path = jsLibPath ?? "pkg/hub.js";

  final fullUrl = baseHref.resolve(path);
  final scriptElement = ScriptElement();
  scriptElement.type = "module";
  scriptElement.innerHtml = '''
import init, * as wasmBindings from "$fullUrl";
await init();
window.rinf = { ...wasmBindings };
completeRinfLoad();
delete window.completeRinfLoad;
''';
  document.head!.append(scriptElement);

  await loadCompleter.future;
}
