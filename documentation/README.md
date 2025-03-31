# Documentation

In order to preview and build this documentation, you need to have [uv](https://docs.astral.sh/uv/getting-started/installation/) installed on your system.

Generate the static documentation files for publication on the web.

```shell
uv run sphinx-build -M dirhtml source dist
```

Serve the built files locally.

```shell
uv run python server.py
```

Automatically apply changes while writing.

```shell
uv run sphinx-autobuild source dist --builder dirhtml
```
