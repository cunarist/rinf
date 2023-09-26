# Running and Building

## 📱 For Native Platforms

The following commands are just enough to run and build apps for native platforms. It's that simple.

To run the app:

```bash
flutter run
```

To build the app for a specific platform:

```bash
flutter build (platform) # Replace it with a platform name
```

## 🌍 For the Web

You need to manually build webassembly module from Rust before running or building the app for the web.

To serve the web application:

```bash
rifs wasm
flutter run --profile  # Choose a browser
```

To build the optimized release version of the web application:

```bash
rifs wasm --release
flutter build web
```

> Note that Flutter apps in debug mode are known to be quite slow on the web. We recommend using profile mode when debugging on a web browser.
