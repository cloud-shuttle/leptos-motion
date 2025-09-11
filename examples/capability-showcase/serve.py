#!/usr/bin/env python3
"""
Simple HTTP server for the Leptos Motion Capability Showcase
"""

import http.server
import socketserver
import webbrowser
import os
import sys

PORT = 8000

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for local development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def log_message(self, format, *args):
        # Suppress broken pipe errors and other noise
        if "Broken pipe" not in str(args):
            super().log_message(format, *args)

def main():
    # Change to the directory containing this script
    os.chdir(os.path.dirname(os.path.abspath(__file__)))

    # Check if pkg directory exists
    if not os.path.exists('pkg'):
        print("❌ Error: pkg directory not found!")
        print("Please run 'wasm-pack build --target web --out-dir pkg' first")
        sys.exit(1)

    # Start the server
    with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
        print(f"🚀 Leptos Motion Capability Showcase")
        print(f"📡 Server running at http://localhost:{PORT}")
        print(f"🌐 Opening browser...")
        print(f"⏹️  Press Ctrl+C to stop")

        # Open browser
        webbrowser.open(f'http://localhost:{PORT}')

        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print(f"\n👋 Server stopped")

if __name__ == "__main__":
    main()
