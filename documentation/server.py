"""Simple HTTP server for serving documentation with CORS headers."""

import logging
import os
from http.server import HTTPServer, SimpleHTTPRequestHandler
from typing import override

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO, format="%(message)s")

PORT = 80


class RequestHandler(SimpleHTTPRequestHandler):
    """HTTP request handler with additional CORS headers for cross-origin isolation."""

    @override
    def end_headers(self) -> None:
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()


def main() -> None:
    """Start the HTTP server for serving documentation files."""
    os.chdir("dist/dirhtml")
    server_address = ("", PORT)
    httpd = HTTPServer(server_address, RequestHandler)
    logger.info("Serving on http://localhost:%d", PORT)
    httpd.serve_forever()


main()
