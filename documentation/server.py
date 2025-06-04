from http.server import HTTPServer, SimpleHTTPRequestHandler
from os import chdir
from typing import override


class RequestHandler(SimpleHTTPRequestHandler):
    @override
    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()


# Serve files from the "dist" directory
def main():
    chdir("dist/dirhtml")
    server_address = ("", 8000)
    httpd = HTTPServer(server_address, RequestHandler)
    print("Serving on http://localhost:8000")
    httpd.serve_forever()


main()
