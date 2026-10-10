---
description: Code generation stays an explicit command, not part of every build, because PATH is unreliable under IDEs; change detection is the accepted improvement
---

PR #129 (3.1.0) removed `build.rs` generation: "The biggest problem is that the PATH environment variable might not be properly loaded, especially with IDE tools like Rust-analyzer." A later PR to run generation inside every platform build step was declined as aggressive: the explicit command is used after clone, schema edits and before runs (#169). Watch mode (#225) was accepted but later looped, see [[gen-watch-mode-loops]]. Generated Rust files are formatted by running `rustfmt` on explicit paths, because `cargo fmt` needs workspace context while the CLI can run anywhere and rustfmt's ignore option needs nightly (#500, #508).

Evidence: #129, #169, #225, #500, #508, #682
