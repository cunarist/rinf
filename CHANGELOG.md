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
