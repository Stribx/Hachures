#!/usr/bin/env python3
"""Validate and smoke-test a generated Hachures web package."""

import http.client
import re
import sys
import threading
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


REQUIRED_FILES = (
    "index.html",
    "hachures.js",
    "hachures_bg.wasm",
    "assets/CREDITS.md",
    "assets/fonts/IMFellEnglish-Regular.ttf",
    "assets/models/tower-round-bottom-a.glb",
    "assets/shaders/hachure.wesl",
    "assets/ui/icon-cible.png",
)


class QuietHandler(SimpleHTTPRequestHandler):
    """HTTP handler that keeps smoke-test output concise."""

    def log_message(self, _format: str, *_args: object) -> None:
        return


def fail(message: str) -> int:
    print(f"web smoke test failed: {message}", file=sys.stderr)
    return 1


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else "website/play").resolve()
    if not root.is_dir():
        return fail(f"package directory does not exist: {root}")

    missing = [path for path in REQUIRED_FILES if not (root / path).is_file()]
    if missing:
        return fail(f"missing required files: {', '.join(missing)}")

    index = (root / "index.html").read_text(encoding="utf-8")
    script = (root / "hachures.js").read_text(encoding="utf-8")
    if "import init from './hachures.js'" not in index:
        return fail("index.html does not load hachures.js as a module")
    if not re.search(r"hachures_bg\.wasm", script):
        return fail("hachures.js does not reference hachures_bg.wasm")

    handler = partial(QuietHandler, directory=str(root))
    server = ThreadingHTTPServer(("127.0.0.1", 0), handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        connection = http.client.HTTPConnection("127.0.0.1", server.server_port, timeout=5)
        for path in ("/", "/index.html", "/hachures.js", "/hachures_bg.wasm"):
            connection.request("GET", path)
            response = connection.getresponse()
            response.read()
            if response.status != 200:
                return fail(f"HTTP {response.status} for {path}")
        connection.close()
    finally:
        server.shutdown()
        thread.join()

    print(f"web smoke test passed: {root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
