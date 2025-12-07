"""Simple HTTP server with CORS headers for demo Rinf web app."""

from http.server import HTTPServer, SimpleHTTPRequestHandler
from typing import override


class RequestHandler(SimpleHTTPRequestHandler):
    """Request handler with CORS headers for SharedArrayBuffer support."""

    @override
    def end_headers(self) -> None:
        """Add required headers for cross-origin isolation."""
        self.send_header("cross-origin-opener-policy", "same-origin")
        self.send_header("cross-origin-embedder-policy", "require-corp")
        super().end_headers()


def main() -> None:
    """Start the HTTP server for serving documentation files."""
    address = ("", 80)
    HTTPServer(address, RequestHandler).serve_forever()


if __name__ == "__main__":
    main()
