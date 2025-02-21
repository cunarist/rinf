# Documentation

In order to preview and build this documentation, you need to have [uv](https://docs.astral.sh/uv/getting-started/installation/) installed on your system.

Generate the static documentation files for publication on the web.

```shell
uv run sphinx-build -M dirhtml source dist
```

Activate a test server.

```shell
uv run -m http.server 8000 --directory dist/dirhtml
```
