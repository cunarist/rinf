---
description: Per-project CLI settings belong in `pubspec.yaml` and structured options or in-schema annotations, not repeated flags or raw passthrough args
---

A PR added `-r/--rust-dir` and `-m/--msg-dir` flags; the maintainer asked for pubspec fields (`rinf.message.input_dir`, `rust_output_dir`, `dart_output_dir`) because retyping paths is cumbersome (#267, #250). Another PR proposed raw `extra_args` to protoc; the maintainer asked for a structured option and preferred an in-schema marker (#405). The preference for declarative, local configuration carries over to the current `gen_input_crates` style config. See [[cli-structured-paths]].

Evidence: #250, #267, #405, #332, #293
