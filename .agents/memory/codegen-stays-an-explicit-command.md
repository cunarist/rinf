---
description: Code generation stays an explicit command, not part of every build, because PATH is unreliable under IDEs; change detection is the accepted improvement
---

PR #129 (3.1.0) removed `build.rs` generation: "The biggest problem is that the PATH environment variable might not be properly loaded, especially with IDE tools like Rust-analyzer." A later PR to run generation inside every platform build step was declined as aggressive: the explicit command is used after clone, schema edits and before runs (#169). Watch mode (#225) was accepted but later looped, see [[gen-watch-mode-loops]]. Generated Rust files are formatted by running `rustfmt` on explicit paths, because `cargo fmt` needs workspace context while the CLI can run anywhere and rustfmt's ignore option needs nightly (#500, #508).

In the Protobuf era a missing `protoc-gen-dart` made `build.rs` silently emit empty Dart output (macOS arm, Windows), while CI passed because its PATH differed; the lesson is that generation must fail loudly, never produce nothing (#123, #124, #129, #130). RPC-style services (prost/tonic) were also rejected in favor of one-way message channels, citing HTTP/2 overhead and the lack of Rust-initiated streaming; criteria for any new communication system were easy and maintainable (#250).

Evidence: #129, #169, #225, #500, #508, #682, #123, #124, #130, #250
