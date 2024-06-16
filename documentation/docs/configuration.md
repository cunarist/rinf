# Configuration

## 📋 YAML File

You can customize the behavior of Rinf CLI commands by configuring the `pubspec.yaml` file. All fields are optional and it's not necessary to write them.

```yaml title="pubspec.yaml"
rinf:
  message:
    input_dir: messages/
    rust_output_dir: native/hub/src/messages/
    dart_output_dir: lib/messages/
    rust_serde: true
```

You can check the current configuration status by running the command below in the CLI.

```bash title="CLI"
rinf config
```

## 📦 Crate Features

```toml title="native/hub/Cargo.toml"
rinf = { version = "0.0.0", features = ["feature-name"] }
```

- `multi-worker`: Starts a worker thread for each CPU core available on the system within the `tokio` runtime by enabling its `rt-multi-thread` feature. By default, the `tokio` runtime uses only one thread. Enabling this feature allows the `tokio` runtime to utilize all the cores on your computer. This feature does not affect applications on the web platform.
- `show-backtrace`: Prints the full backtrace in the CLI when a panic occurs in debug mode. In general, backtrace is not very helpful when debugging async apps, so consider using [`tracing`](https://crates.io/crates/tracing) for logging purposes. Note that this feature does not affect debugging on the web platform.
