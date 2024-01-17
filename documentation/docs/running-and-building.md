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

### Deploying On a Web Server

When deploying your web app, ensure that your web server is configured to include cross-origin-related HTTP headers in its responses. These headers enable web browsers using your website to gain access to `SharedArrayBuffer` web API, which is something similar to shared memory on the web.

- [`cross-origin-opener-policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy): `same-origin`
- [`cross-origin-embedder-policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy): `require-corp`.

Additionally, don't forget to specify the MIME type `application/wasm` for `.wasm` files within the server configuration to ensure optimal performance.
