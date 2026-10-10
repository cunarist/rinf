---
description: Rust cannot run on the platform main thread because the Dart event loop owns it; main-thread-only libraries (Bevy/winit on macOS) must run as a separate process
---

Dart's event loop starts on the main thread when the Flutter app launches and Rinf cannot change that. A winit event loop on macOS panics off the main thread. The maintainer's advice: a separate executable communicating over sockets, or two companion apps. GPU compute (wgpu) and multi-crate native workspaces work fine. The `bevy` feature (6.14.0) only turns Dart signals into Bevy ECS events and is experimental; a bevy example was declined because bevy's API is unstable.

Evidence: #513, #447, #454, #349, commits 1a065444, ce343968
