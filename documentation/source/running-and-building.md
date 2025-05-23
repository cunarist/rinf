# Running and Building

## For Native Platforms

The following commands are just enough to run and build apps for native platforms. It's that simple.[^1]

[^1]: Rinf builds the native Rust library and links it to the Flutter app using [Cargokit](https://github.com/irondash/cargokit).

To run the app:

```{code-block} shell
:caption: CLI
flutter run
```

To build the app for a specific platform:

```{code-block} shell
:caption: CLI
flutter build [platform]
```

## For the Web

You need to manually build webassembly module from Rust before running or building the app for the web.[^2]

[^2]: Internally, Rinf uses `wasm-bindgen` and `wasm-pack` with the `web` [target](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target).

To serve the web application[^3]:

[^3]: Since repeatedly writing web header arguments during development can be overwhelming, Rinf provides a convenient command `rinf server` that prints the full Flutter web command.

```{code-block} shell
:caption: CLI
rinf wasm
flutter run --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```

To build the optimized release version of the web application:

```{code-block} shell
:caption: CLI
rinf wasm --release
flutter build web
```

When deploying your web app on a web server[^4], ensure that your web server is configured to include cross-origin-related HTTP headers in its responses. These headers enable web browsers using your website to gain access to `SharedArrayBuffer` web API, which is something similar to shared memory on the web.

[^4]: Rinf supports hosting a Flutter app at a [non-root location](https://docs.flutter.dev/ui/navigation/url-strategies#hosting-a-flutter-app-at-a-non-root-location). For example, you can place your Flutter app in `https://mywebsite.com/subpath/deeperpath/`.

- [`Cross-Origin-Opener-Policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy): `same-origin`
- [`Cross-Origin-Embedder-Policy`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy): `require-corp`.

Additionally, don't forget to specify the MIME type `application/wasm` for `.wasm` files within the server configuration to ensure optimal performance.
