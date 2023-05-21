# 🆎 About This Template

This template provides instant capabilities to developers who want to embrace the power of **Rust** and **Flutter** together. Simply duplicate this template and you're ready to go!

![preview](https://github.com/cunarist/app-template/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

This template is primarily built using the [Flutter-Rust-Bridge](https://github.com/fzyzcjy/flutter_rust_bridge) library. It also incorporates several popular packages and modifications into the default Flutter template, ensuring optimal development process. It has been designed with future scalability and performance in mind.

Extra features added to the default Flutter template are:

- Rust integration with the ability to use an arbitrary number of library crates
- MVVM pattern with easy viewmodel binding from Dart and viewmodel update from Rust
- Convenient configuration management
- Restarting Rust logic on Dart's hot restart
- Convenient app naming and icon generation
- Setting desktop window properties

## Platform Support

This template is ready to be used without any additional setup. Structuring a Flutter project that targets multiple platforms can be a challenging task, especially when incorporating Rust. With this template, you don't have to start from scratch or face the added complexity of integrating Rust. It provides a ready-to-use solution, saving you time and effort.

- ✅ Windows: Tested and supported
- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ⏸️ Web: Not supported but will be in the near future

## Contribution

We love contributions! If you have any suggestions or want to report a bug, please leave it as an issue or a pull request. We will try to respond as quickly as possible.

# 🧱 Project Structure

**Flutter** deals with the cross-platform user interface while **Rust** handles the internal logic. The front-end and back-end are completely separated, which means that Dart code and Rust code should be detachable from each other. These two worlds communicate through signals.

Moreover, you can conveniently receive the latest commits from the [Cunarist App Template](https://github.com/cunarist/app-template) into your repository using the provided Python script below.

# 👜 System Preparation

Flutter and Rust are required for building the app itself. Python is needed to automate complicated procedures. Git is responsible for version control.

You can use an IDE of your choice. However, [Visual Studio Code](https://code.visualstudio.com/) is recommended because it has extensive support from Flutter and Rust communities.

## Preparing Git

Go to the [official downloads page](https://git-scm.com/downloads)

## Preparing Python

Download it from the app store if your system doesn't provide a pre-installed version. It's also available at [official downloads page](https://www.python.org/downloads/). Make sure `python` is available in the path environment variable. Version 3.11 or higher is recommended.

## Preparing Rust

Refer to the [official docs](https://doc.rust-lang.org/book/ch01-01-installation.html). Version 1.69 or higher is recommended.

## Preparing Flutter

Refer to the [official docs](https://docs.flutter.dev/get-started/install). Version 3.10 or higher is recommended.

## System Verification

You can make sure your system is ready for development in the terminal.

```
git --version
python --version
rustc --version
flutter doctor
```

## Extra Steps

If you are planning to compile your code for Windows, Linux, or macOS, you can skip this section.

If you intend to compile it for Android or iOS, there are additional steps involved. Please refer to the Flutter-Rust-Bridge docs for instructions on installing the necessary components on your system.

- Android: [Link 1](https://cjycode.com/flutter_rust_bridge/tutorial/setup_android.html), [Link 2](https://cjycode.com/flutter_rust_bridge/template/setup_android.html)
- iOS: [Link 1](https://cjycode.com/flutter_rust_bridge/tutorial_with_flutter.html#ios-app), [Link 2](https://cjycode.com/flutter_rust_bridge/template/setup_ios.html)

Setting up your system with extra build targets can sometimes present various issues. If you encounter any problems, feel free to visit [the discussions page](https://github.com/cunarist/app-template/discussions) and open a Q&A thread for assistance.

# 🗃️ Setting Up

Install Dart packages written in `./pubspec.yaml` from [Pub](https://pub.dev/).

```
flutter pub get
```

Install Python packages written in `./requirements.txt` from [PyPI](https://pypi.org/).

```
pip install -r requirements.txt
```

Generate configuration files or update them from template files if they already exist. Make sure to check the terminal output and fill in those necessary files manually after the generation process is complete.

```
python automate config-filling
```

# 🍳 Actual Development

You might need to dive into this section quite often.

Check and fix problems in `./native` Rust code and `./lib` Dart code.

```
python automate code-quality
```

Run the app in debug mode.

```
flutter run
```

Build the app in release mode.

```
flutter build (platform) --release
```

Check the actual sizes of compiled Rust libraries in release mode.

```
python automate size-check (platform)
```

Set the app name and domain. This only works once and you cannot revert this.

```
python automate app-naming
```

Apply `app_icon_full.png` file in `./assets` to multiple platforms with [Flutter Launcher Icons](https://pub.dev/packages/flutter_launcher_icons). Appropriate rounding and scaling are applied per platform with the power of Python. On Linux, you should include the icon manually in the distribution package.

```
python automate icon-gen
```

Apply `translations.csv` file in `./assets` to app profiles of multiple platforms. You need to run this extra command after changing the list of supported locales. Only modifying the CSV file is not enough on some platforms.

```
python automate translation
```

Receive the latest commits from [Cunarist App Template](https://github.com/cunarist/app-template).

```
python automate template-update
```

# ⛓️ MVVM Pattern

There are 3 layers of data flow.

1. View: Dart
1. Viewmodel: Bridge connecting Dart and Rust
1. Model: Rust

Rust logic updates the viewmodel. Dart listens to changes made in viewmodel and rebuilds the widgets accordingly. This system was designed to have minimal performance bottlenecks.

Details are explained as comments in the actual code. If there are things that are still unclear, please feel free to leave a question or start a discussion.

# 📁 Folder Structure

Basically, `./native/hub/src/lib.rs` is the entry point of your Rust logic while `./lib/main.dart` is the entry point of your Dart logic.

Most of the top-level folders come from the default Flutter template.

- `windows`: Platform-specific files
- `linux`: Platform-specific files
- `macos`: Platform-specific files
- `android`: Platform-specific files
- `ios`: Platform-specific files
- `web`: Platform-specific files
- `lib`: Dart modules empowering the Flutter application.

However, there are some extra folders created in Cunarist App Template to integrate other functionalities into development.

- `automate`: Python scripts for automating the development process. These scripts have nothing to do with the actual build and don't get included in the app release. Only for developers.
- `native`: A workspace Rust crate that includes many other library crates. Each crate inside this folder gets compiled into its own library binary(`.dll`/`.so`).
- `assets`: A place for asset files such as images.

In addition, there might be some other temporary folders generated by tools or IDE you are using. Those should not be version-controlled.

# 📜 Rules

## Division of Functionalities

Dart should only be used for the front-end user interface and Rust should handle all other back-end logic such as file handling, event handling, timer repetition, calculation, network communication, etc.

## Async over Multithreading

Rust provides a powerful way to utilize async functions which allows to you to achieve high level of concurrency using only a single thread. This template is focused on async ways of handling concurrent tasks. Avoid using multithreading unless high computing power is necessary.

## Multi-language Support

Always write user interface texts in `./assets/translations.csv`.

When an app gains popularity, there comes a need to support multiple human languages. However, manually replacing thousands of text widgets in the user interface is not a trivial task. Therefore it is a must to write texts that will be presented to normal users in translation files.

Refer to [Easy Localization](https://pub.dev/packages/easy_localization) docs for more details.

## Guiding Comments

Please write kind and readable comments and also attach doc comments to important elements of your code. You are probably not going to be developing on your own. Other developers should be able to get a grasp of the complex code that you wrote. Long and detailed comments are also welcomed.

## Development Automation

It is recommended to rely on Python scripts in `./automate` for faster and easier development.

## Modification Restictions

Be careful all the time! You shouldn't be editing any file without enough knowledge of how it works. Below are the top-level files and folders that are allowed to edit during app development:

- `lib`: Dart modules.
  - Do not modify the `bridge` folder inside.
- `pubspec.yaml`: Dart settings and dependencies.
- `assets`: Asset files.
- `native`: Rust's workspace containing multiple library crates. The name of the library crate folder should be the same as that of the library crate's name.
  - Do not modify `bridge` module inside `hub` crate.
