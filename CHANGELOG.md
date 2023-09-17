## 4.1.4

- Fixed a sentence in the guides.

## 4.1.3

- Made `rifs message` command read `PUB_CACHE` enviornment variable if present. (Thanks @rabbitson87)

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

- Adopted Protobuf for message serialization. Now communication between Dart and Rust is much more type-safe and faster than before. Because the template has now changed, you need to run `dart run rust_in_flutter template` again when migrating from version 2. (Thanks @wheregmis and @bookshiyi)

## 2.9.0

- Removed `corrosion`. Now this package solely relies on `cargokit` and is much more slimmer. (Thanks @bookshiyi)
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

- Organized guides. (Thanks @bookshiyi)

## 2.7.0

- Stabilized web-related Rust toolchain's auto-installation. (Thanks @bookshiyi)

## 2.6.0

- Applied continuous integration for checking builds and improving project stability. (Thanks @bookshiyi)

## 2.5.6

- Updated Cargokit. (Thanks @bookshiyi)

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

- Added Chinese guides. (Thanks @moluopro)
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
