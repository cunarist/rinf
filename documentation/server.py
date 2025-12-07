"""Simple HTTP server for serving documentation with CORS headers."""

import os
from http.server import HTTPServer, SimpleHTTPRequestHandler
from typing import override


class RequestHandler(SimpleHTTPRequestHandler):
    """HTTP request handler with additional CORS headers for cross-origin isolation."""

    @override
    def end_headers(self) -> None:
        self.send_header("cross-origin-opener-policy", "same-origin")
        self.send_header("cross-origin-embedder-policy", "require-corp")
        super().end_headers()


def main() -> None:
    """Start the HTTP server for serving documentation files."""
    os.chdir("dist/dirhtml")
    address = ("", 80)
    HTTPServer(address, RequestHandler).serve_forever()


if __name__ == "__main__":
    main()
