---
description: The CLI must not silently install or upgrade helper tools: unpinned `dart pub add` let a protobuf 4.0 release break every user, and offline or China users could not download tools
---

When protobuf 4.0.0 and protoc_plugin 22 came out, the 7.x generate command broke everywhere because the CLI ran `dart pub add protobuf` and resolved the new plugin. Fixes pinned `protobuf ^3.1.0` and `protoc_plugin ^21.0.0` (#541, #542) instead of migrating; 8.0 dropped Protobuf and the upgrade guide tells users to remove them. A dependabot major bump of bincode was rejected the same way ("ignore this major version"). Fewer pub.dev dependencies means fewer conflicts (#528).

Related stance from the same era: detect tools already on PATH rather than downloading (users behind the Great Firewall timed out fetching protoc; the maintainer chose "skip auto-install when it is on PATH" over a path flag), make commands work offline (#369), and at least print a notice when installing. Cargokit and rustup also run implicit network commands such as `rustup target add`. Related: [[signalpiece-replaced-protobuf]].

Evidence: #284, #285, #366, #369, #541, #542, #544, #528, #553, commit 7b54657d
