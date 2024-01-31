# Running and Building

## 📱 For Native Platforms

The following commands are just enough to run and build apps for native platforms. It's that simple.[^1]

[^1]: Rinf builds the native Rust library and links it to the Flutter app using [Cargokit](https://github.com/irondash/cargokit).

To run the app:

```bash title="CLI"
flutter run
```

To build the app for a specific platform:

```bash title="CLI"
flutter build [platform] # Replace it with a platform name
```

## 🌍 For the Web

You need to manually build webassembly module from Rust before running or building the app for the web.[^2]

[^2]: Internally, Rinf uses `wasm-bindgen` and `wasm-pack` with the `web` [target](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target).

To serve[^3] the web application:

[^3]: Note that Flutter apps in debug mode are known to be quite slow on the web. It is recommended to use release mode when testing on a web browser. Debug mode can be used if you need to analyze the code more deeply, without the `--release` argument.

```bash title="CLI"
rinf wasm --release
flutter run --release  # Choose a browser
```

To build the optimized release version of the web application:

```bash title="CLI"
rinf wasm --release
flutter build web
```

### Deploying On a Web Server

When deploying your web app, ensure that your web server is configured to include cross-origin-related HTTP headers in its responses. These headers enable web browsers using your website to gain access to `SharedArrayBuffer` web API, which is something similar to shared memory on the web.

- [`cross-origin-opener-policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy): `same-origin`
- [`cross-origin-embedder-policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy): `require-corp`.

Additionally, don't forget to specify the MIME type `application/wasm` for `.wasm` files within the server configuration to ensure optimal performance.
