# Configuration

You can customize some Rinf behaviors by configuring the `pubspec.yaml` file. Rinf will change its behaviors by reading the fields below. All fields are optional and it's not necessary to write them.

```yaml title="pubspec.yaml"
rinf:
  message:
    input_dir: messages/
    rust_output_dir: native/hub/src/messages/
    dart_output_dir: lib/messages/
```

You can check the current configuration status by running the command below in the CLI.

```bash title="CLI"
rinf config
```
