# Running and Building

## 📱 For Native Platforms

The following commands are just enough to run and build apps for native platforms. It's that simple.

To run the app:

```bash
flutter run
```

To build the app for a specific platform:

```bash
flutter build [platform] # Replace it with a platform name
```

## 🌍 For the Web

You need to manually build webassembly module from Rust before running or building the app for the web.

To serve the web application:

```bash
rinf wasm --release
flutter run --release  # Choose a browser
```

To build the optimized release version of the web application:

```bash
rinf wasm --release
flutter build web
```

> Note that Flutter apps in debug mode are known to be quite slow on the web. We recommend using release mode when testing on a web browser. You can use debug mode if you need to analyze the code deeper, without the `--release` argument.
