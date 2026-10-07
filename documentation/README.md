# Documentation

In order to preview this documentation, you need to have [uv](https://docs.astral.sh/uv/getting-started/installation/) installed on your system.

Write the documentation, applying changes as you go.

```shell
uv run sphinx-autobuild source dist --builder dirhtml
```

The landing page demo is the example app built for the web, so the command
above leaves the iframe empty. Building the image covers the Rust and Flutter
toolchains it needs, and serves the site as GitHub Pages does. Run this from
the repository root.

```shell
podman build -f documentation/Containerfile -t rinf-docs .
podman run --rm -p 8000:80 rinf-docs
```
