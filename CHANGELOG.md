## 6.0.0-beta

- This is a beta version.

## 6.0.0-alpha

- This is an alpha version.

## 5.4.0

- Now users do not have to manually install `protoc` binary executable anymore. Protobuf compiler is now automatically installed. Note that you need to run `cargo install rinf` again to use this new version.

## 5.3.1

- Fixed a bug with `rinf message` that omits `mod.rs` inside a folder without any message.

## 5.3.0

- Now it is possible to use `import` statements in `.proto` files.
- Now you can properly use Flutter's hot restart while developing.
- Tutorials and guides are improved.
- Fixed a bug with `rinf message`, that might fail if some of the folders are empty.

## 5.2.0

- Unnecessary memory copy is now avoided.

## 5.1.3

- Fixed introductions.

## 5.1.2

- Fixed a bug with memory freeing.

## 5.1.1

- The codebase was organized further.
- Fixed a problem with Dart analysis.

## 5.1.0

- All the code from `flutter_rust_bridge` was removed. This was due to criticisms about Rinf from the community and FRB devs. Also, internal bridge and FFI code is now much smaller. User API remains unchanged.

## 5.0.0

- Now `requestToRust` Dart function will return `null` when the handler function in Rust cannot respond or has panicked. This is a breaking change, so when you upgrade to this version, you need to run `rinf template --bridge` in the terminal and refactor some of the code where IDE warns you about mismatches. You also need to modify `lib.rs` and `with_request.rs` modules of the `hub` crate. Also, the `timeout` parameter was removed from the `requestToRust` function because Dart will always get some return value from Rust in all cases. Please refer to the example code and documentation tutorials if you need detailed information.

## 4.20.0

- Added support for Android Gradle Plugin 8.

## 4.19.1

- Switched to the new official website `rinf.cunarist.com`.

## 4.19.0

- The mechanism for awaiting Rust's responses from Dart has become much more efficient.

## 4.18.0

- Now `null` can be provided to the `timeout` parameter in the `requestToRust()` Dart function. This enables awaiting a Rust response forever, but it should be used cautiously because it may potentially lead to resource leaks.

## 4.17.1

- Now `rinf message --watch` works on all platforms. Thanks @bookshiyi!

## 4.17.0

- New command `rinf message --watch` for automatic message code generation. Thanks @bookshiyi!

## 4.16.3

- Updated package descriptions.

## 4.16.2

- Improved guides to avoid misconceptions about the communication system. Rinf only uses FFI without any web server.

## 4.16.1

- Fixed the broken `rinf template --bridge` command

## 4.16.0

- Vastly organized files, dependencies, and code readability. Now the `hub` crate is much cleaner than before. If you already have an app using older Rinf versions, it is recommended to run `rinf template --bridge` and add `rinf = "4.16.0"` to `Cargo.toml` of the `hub` crate.

## 4.15.2

- Now the web API fetching example uses `http` instead of `https` in the example app.

## 4.15.1

- Now the `reqwest` crate will be disabled when compiling the example app for Android.

## 4.15.0

- Allowed setting custom timeout when using `requestToRust()`. Thanks @cipherchabon!

## 4.14.0

- Added a web API fetching example to the template.
- Fixed various things such as template comments, code linting issues.

## 4.13.2

- Fixed small things.

## 4.13.1

- Fixed formatting issues in Dart code.

## 4.13.0

- Improved the template code by disabling CPU-intensive tasks and removing unneeded dependency features.
- Improved CLI outputs from `rinf wasm`.
- Improved various guides and template comments.

## 4.12.5

- Automated publication.

## 4.12.4

- Fixed permission related problems with build script files.

## 4.12.3

- Fixed permission related problems with build script files.

## 4.12.2

- Fixed guide badges.

## 4.12.1

- Fixed publishing issues.

## 4.12.0

- Renamed the framework to Rinf.

## 4.11.8

- Improved first guides.

## 4.11.7

- Improved the example app's code and guides.

## 4.11.6

- Improved the shorthand command crate.

## 4.11.5

- Improved the shorthand command crate.

## 4.11.4

- Improved the first preview image and some comments.

## 4.11.3

- Improved the example app's code.

## 4.11.2

- Fixed a problem with compilation on macOS.

## 4.11.1

- Fixed a problem with compilation on macOS.

## 4.11.0

- New Dart function `ensureFinalized()`. This function ensures that all Rust tasks are terminated. Take a look at the example code to understand how to run this function before closing the Flutter app. Note that you have to run `rifs template --bridge` again to use this function.

## 4.10.0

- New default web alias `spawn_blocking()`. CPU-intensive blocking tasks are better to be executed on a separate thread pool.
- Improved the example app's performance and debug tests.

## 4.9.0

- New default web alias `yield_now()`. Inside a long-running code, calling this will help you avoid blocking the whole async event loop, by giving the flow back to other async tasks.
- Vastly improved comments inside the `web_alias` Rust module.
- Now Rust panics on the web will be printed to the CLI too.
- Improved the example app's performance and debug tests.

## 4.8.2

- Improved guide sentences.

## 4.8.1

- Improved the readability of the example code.
- Organized and clarified first guides.

## 4.8.0

- Now by running `rifs template --bridge`, you can apply and update only the `bridge` module inside the `hub` crate. This is useful when you've upgraded RIF but do not need to apply the whole template again.
- Improved `rifs --help` output.

## 4.7.0

- Now Rust stacktrace will be printed to the CLI when a panic occurs. The changes are mostly included in the template, so make sure you've run `rifs template` on this new version.

## 4.6.2

- Polished various aspects.

## 4.6.1

- Stabilized `debug_print!` logic.

## 4.6.0

- New `debug_print!` macro that works on all environments, including web and mobile emulators, with the power of Flutter debuggers. To use this, you need to run `rifs template` again.
- Now panic information in Rust will be properly printed to the CLI. Note that Rust panics don't crash the app and do not hinder stability.
- Improved docs. There are also more guides about well-known types in Protobuf. Thanks @LucaCoduriV!

## 4.5.0

- Added support for external symbols on iOS and macOS. This is needed for some Rust crates that depend on Apple's system frameworks.

## 4.4.2

- Updated docs and demo links.

## 4.4.1

- Updated docs and demo links.

## 4.4.0

- Improved various guides and comments.
- Fixed a bug that made the app crash when passing in an empty `Vec<u8>`.
- Fixed the formatting of Rust files.

## 4.3.0

- Now `flutter run` will use `require-corp` value for `cross-origin-embedder-policy` HTTP header that works on all web browsers.

## 4.2.1

- Fixed a bug with `RustResponse::default()`.

## 4.2.0

- New command `rifs --help`. Thanks @bookshiyi!

## 4.1.4

- Fixed a sentence in the guides.

## 4.1.3

- Made `rifs message` command read `PUB_CACHE` enviornment variable if present. Thanks @rabbitson87!

## 4.1.2

- Fixed `rifs template` command.

## 4.1.1

- Added some guides to the shorthand crate.
- Removed an unneeded dependency from the shorthand crate.

## 4.1.0

- Fixed `sleep()` on the web.
- Added demo link in the guides.

## 4.0.3

- Fixed bugs with `rifs template` on Windows.
- Fixed outdated comments.
- Organized sample code.

## 4.0.2

- Eliminated an unnecessary Dart dependency.

## 4.0.1

- Eliminated an unnecessary Dart dependency.

## 4.0.0

- Added support for sending large binaries between Dart and Rust. This is now possible by using the `blob` field in `RustRequest`, `RustResponse`, and `RustSignal`. Please make sure you've run `rifs template` before using this new version because the template has changed a little.
- Added support for nested message folders.
- Added support for Rust nightly.
- Eliminated unnecessary Dart dependencies.

## 3.7.4

- Updated `cargokit`, the build connector between Flutter and Rust.

## 3.7.3

- Fixed a bug with cargo.

## 3.7.2

- Fixed a bug with cargo.

## 3.7.1

- Organized descriptions and files.

## 3.7.0

- Now this framework provides a shorthand command `rifs ...` which is equivalent to `dart run rust_in_flutter ...`.

## 3.6.0

- Fixed a bug that prevents the app from running on Linux.
- Improved various texts exposed to developers for clarity.

## 3.5.1

- Bumped `prost` version to avoid snake case related warnings.

## 3.5.0

- Shortend some names that were unnecessarily long.

## 3.4.5

- Import statements became shorter in Dart.

## 3.4.4

- Cleaned up outdated dependencies in `Cargo.toml`.

## 3.4.3

- Now `syntax` and `package` statements in `.proto` files should be handled automatically.

## 3.4.2

- Now running `dart run rust_in_flutter message` verifies `package` statement in `.proto` files and mistakes are fixed automatically.

## 3.4.1

- Now match statement is used for handling requests. This improves code readability.

## 3.4.0

- Now each `.proto` file is treated as a Rust resource, which essentially becomes an API endpoint.

## 3.3.0

- `RustResource` enum has been added to `interaction.proto`. Now the list of available Rust resources are managed by Protobuf, which makes the project less error-prone. This new system also has less runtime overhead because interactions are distinguished by simple integers, not strings.

## 3.2.3

- Improved guides.

## 3.2.2

- Organized guides.

## 3.2.1

- Matched first guides with the docs.

## 3.2.0

- Now when applying the Rust template with `dart run rust_in_flutter template`, `README.md` file will get a new section explaining about this framework.

## 3.1.1

- Updated docs link.

## 3.1.0

- Now there's a new Dart command `message`. Developers can now generate Dart and Rust message code from `.proto` files with `dart run rust_in_flutter message`. `build.rs` file that used to do this is removed.

## 3.0.9

- Fixed a problem with pub.dev score.

## 3.0.8

- Fixed a problem with pub.dev score.

## 3.0.7

- Fixed a problem with pub.dev score.

## 3.0.6

- Fixed a problem with pub.dev score.

## 3.0.5

- Moved documentation to a dedicated website.
- Now `build.rs` will automatically modify PATH for `protoc-gen-dart`.
- Fixed an error appearing in Rust-analyzer's webassembly mode.

## 3.0.4

- Polished template code.

## 3.0.3

- Polished template code.

## 3.0.2

- Polished guides, comments and template code.

## 3.0.1

- Fixed and organized tutorials and comments.

## 3.0.0

- Adopted Protobuf for message serialization. Now communication between Dart and Rust is much more type-safe and faster than before. Because the template has now changed, you need to run `dart run rust_in_flutter template` again when migrating from version 2. Thanks @wheregmis` and @bookshiyi!

## 2.9.0

- Removed `corrosion`. Now this package solely relies on `cargokit` and is much more slimmer. Thanks @bookshiyi!
- Removed unneeded files from pub.dev publication.

## 2.8.5

- Fixed a problem with pub.dev score.

## 2.8.4

- Fixed a problem with pub.dev score.

## 2.8.3

- Wrote new catchphrase.

## 2.8.2

- Updated links.

## 2.8.1

- Updated links.

## 2.8.0

- Removed unneeded dependencies.

## 2.7.4

- Fixed CI badge showing rate limit error.

## 2.7.3

- Fixed wrong guides.

## 2.7.2

- Organized guides.

## 2.7.1

- Organized guides. Thanks @bookshiyi!

## 2.7.0

- Stabilized web-related Rust toolchain's auto-installation. Thanks @bookshiyi!

## 2.6.0

- Applied continuous integration for checking builds and improving project stability. Thanks @bookshiyi!

## 2.5.6

- Updated Cargokit. Thanks @bookshiyi!

## 2.5.5

- Improved guides about HTTP headers.

## 2.5.4

- Updated example code.

## 2.5.3

- Improved guides and CLI messages.

## 2.5.2

- Optimized web binary size.

## 2.5.1

- Optimized web performance.

## 2.5.0

- Now Rust logic will be restarted upon Dart's hot restart on the web too.
- CLI commands are shortened.

## 2.4.0

- Fixed the problem with dangling threads from the `tokio` runtime remaining after closing the Flutter app. Even after the app window was closed, `tokio` threads were still running, resulting in becoming a background process without a window. Now the `tokio` runtime will properly be shut down.

## 2.3.2

- Re-publishing due to `pub.dev`'s `[UNKNOWN PLATFORMS]` error.

## 2.3.1

- Restored the benefits section in the first guide.

## 2.3.0

- Improved Dart's hot restart process on native platforms.

## 2.2.0

- Improved the procedure of building for the web.
- Simplfied unneeded complexities.

## 2.1.2

- Improved web alias module.
- Fixed small things.

## 2.1.1

- Optimized the bridge thread on native platforms.
- Updated many minor errors in the guides.
- Fixed a problem with import statement not being written in `./lib/main.dart` when applying Rust template.

## 2.1.0

- Merged `frb_engine` crate into `hub`.
- Removed unneeded intermediate worker pools.
- Added `time` web alias import.
- Added many guides and comments.

## 2.0.1

- Improved guides.
- Added `print!` web alias macro.
- Organized exposed Dart APIs.

## 2.0.0

- Added web support.

## 1.6.6

- Improved guides.
- Now, the template application command will check if the current directory is a Flutter project first.

## 1.6.5

- Improved guides.

## 1.6.4

- Organized guide sections.

## 1.6.3

- Organized guide sections.

## 1.6.2

- Filled in missing translations.

## 1.6.1

- Slightly improved guide sections.

## 1.6.0

- Added step-by-step guides.

## 1.5.3

- Fixed some example app code.

## 1.5.2

- Improved the readability of example app code.

## 1.5.1

- Added Japanese translation.
- Fixed some sentences in Korean guides.

## 1.5.0

- Now the Android NDK version that the Flutter SDK expects will be used, not the version specified by this package.
- Fixed a bug saying `IntoDart` trait is not implemented.

## 1.4.1

- Improved various guides.

## 1.4.0

- Filled in various guides to help developers understand the structure more easily.

## 1.3.2

- Added Chinese guides. Thanks @moluopro!
- Added Korean guides.
- Added guides about build tool version issues.
- Added guides about library bundling.

## 1.3.1

- Fixed a problem with Rust crate path detection on Android.

## 1.3.0

- Changed the name of an exposed enum. Now `Operation` has changed to `RustOperation` so that it won't make confusions with other operations. All developers should update their code to match this new name, probably using the batch replace function in various IDEs.
- Updated code snippets.

## 1.2.8

- Fixed small things.

## 1.2.7

- Stabilized `main.dart` modifcation upon `dart run rust_in_flutter:apply_template`.

## 1.2.6

- Hid the information regarding the compilation of connector crates to avoid confusion with actual crates.

## 1.2.5

- Updated the guide about Android NDK version.

## 1.2.4

- Updated many outdated comments and guides.
- Decreased the time spent on `ensureInitialized`. Also, `ensureInitialized()` is automatically inserted in `main.dart` when doing `dart run rust_in_flutter:apply_template` from now on.
- Various code improvements were applied.

## 1.2.3

- Clarified template structure in guides.

## 1.2.2

- Hide more Dart APIs that are not meant to be used outside.

## 1.2.1

- Updated many comments.
- Fine-tuned the visibility of Dart APIs.
- Organized guides.

## 1.2.0

- Made the Rust request handler more future-proof, taking potential web support into account.

## 1.1.1

- Improved various guides to help understanding the features of this package.

## 1.1.0

- Now this package is a Flutter FFI plugin without dummy native code.
- Improved guides

## 1.0.4

- Fixed a problem with library bundling on Linux.
- Added comments.
- Added guides.
- Improved template application.

## 1.0.3

- Included code snippets in guides.

## 1.0.2

- Fixed typos.
- Organized inner code.

## 1.0.1

- Enforced bundling on macOS and iOS.
- Improved pub score.
- Make `apply_rust` modify `.gitignore`.

## 1.0.0

- Previously `flutter_rust_app_template`, now this is a small convenient framework that can be applied to existing Flutter projects.
