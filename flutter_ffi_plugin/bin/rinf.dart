import 'src/helpers.dart';
import 'src/message.dart';

Future<void> main(List<String> args) async {
  if (args.length == 0) {
    print("No operation is provided.");
    print("Use `rinf --help` to see all available operations.");
    return;
  }
  switch (args[0]) {
    case "template":
      if (args.contains("--bridge") || args.contains("-b")) {
        await applyRustTemplate(onlyBridge: true);
      } else {
        await applyRustTemplate();
      }
      break;
    case "message":
      if (args.contains("--watch") || args.contains("-w")) {
        await watchAndGenerateMessageCode();
      } else {
        await generateMessageCode();
      }
      break;
    case "wasm":
      if (args.contains("--release") || args.contains("-r")) {
        await buildWebassembly(isReleaseMode: true);
      } else {
        await buildWebassembly();
      }
      break;
    case "--help":
    case "-h":
      print("Usage: rinf [arguments]");
      print("Arguments:");
      print("  -h, --help        Shows this usage information.");
      print("  template          Applies Rust template to current project.");
      print("    -b, --bridge    Only applies `bridge` Rust module.");
      print("  message           Generates message code from `.proto` files.");
      print("    -w, --watch     Continuously watches `.proto` files.");
      print("  wasm              Builds webassembly module.");
      print("    -r, --release   Builds in release mode.");
    default:
      print("No such operation is available.");
      print("Use `rinf --help` to see all available operations.");
  }
}
