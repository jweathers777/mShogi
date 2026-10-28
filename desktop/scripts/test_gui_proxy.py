"""Real external-proxy boundary experiment, not packaged application evidence.

Run on Linux with the original and no-pool tauri-driver binaries. A controlled
native HTTP peer resets a reused socket or resets after accepting a mutation.
The peer logs each request and connection, independently of the proxy/client.
"""
import argparse
import http.client
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import importlib.util
import json
from pathlib import Path
import signal
import socket
import struct
import subprocess
import tempfile
import threading
import time
import urllib.error
import urllib.request

spec = importlib.util.spec_from_file_location('gui_smoke', Path(__file__).with_name('gui-smoke.py'))
gui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gui)


class Peer(ThreadingHTTPServer):
    daemon_threads = True

    def __init__(self, mode):
        super().__init__(('127.0.0.1', 0), Handler)
        self.mode, self.events, self.connections = mode, [], 0
        self.lock = threading.Lock()


class Handler(BaseHTTPRequestHandler):
    protocol_version = 'HTTP/1.1'

    def setup(self):
        super().setup()
        with self.server.lock:
            self.server.connections += 1
            self.connection_id = self.server.connections
        self.requests = 0

    def log_message(self, *args):
        pass

    def do_GET(self):
        self.handle_request()

    def do_POST(self):
        self.handle_request()

    def handle_request(self):
        body = self.rfile.read(int(self.headers.get('Content-Length', 0)))
        self.requests += 1
        reset = ((self.server.mode == 'reuse' and self.requests > 1)
                 or (self.server.mode == 'delivered' and self.path.endswith('/click')))
        event = {'connection': self.connection_id, 'on_connection': self.requests,
                 'method': self.command, 'path': self.path,
                 'body': body.decode(), 'reset_after_receipt': reset}
        with self.server.lock:
            self.server.events.append(event)
        if reset:
            # A peer RST after full receipt makes delivery ambiguous to the
            # caller. A new connection must never replay this command.
            self.close_connection = True
            self.connection.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack('ii', 1, 0))
            self.connection.close()
            return
        value = ({'sessionId': 'boundary', 'capabilities': {}} if self.path == '/session'
                 else {'error': 'no such element', 'message': 'deliberate semantic rejection'}
                 if self.server.mode == 'semantic' and self.path != '/status' else {'ready': True})
        payload = json.dumps({'value': value}).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)
        self.wfile.flush()


def unused_port():
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


def experiment(binary, mode, output):
    peer = Peer(mode)
    thread = threading.Thread(target=peer.serve_forever, daemon=True)
    thread.start()
    port = unused_port()
    endpoint = f'http://127.0.0.1:{port}'
    events, failure = [], None
    # tauri-driver still supervises its native child; our controlled HTTP peer
    # replaces WebKit for this experiment only. No application is launched.
    with tempfile.TemporaryDirectory() as temp:
        stub = Path(temp) / 'native-stub'
        stub.write_text('#!/usr/bin/env python3\nimport time\ntime.sleep(300)\n')
        stub.chmod(0o755)
        with (output / f'{binary.name}-{mode}.log').open('w') as log:
            driver = subprocess.Popen([str(binary), '--port', str(port), '--native-port',
                str(peer.server_port), '--native-driver', str(stub)], stdout=log,
                stderr=subprocess.STDOUT, start_new_session=True)
            try:
                deadline = time.monotonic() + 10
                while True:
                    # TCP readiness only: do not consume the seed HTTP request.
                    try:
                        with socket.create_connection(('127.0.0.1', port), timeout=0.1):
                            break
                    except OSError:
                        if driver.poll() is not None or time.monotonic() >= deadline:
                            raise RuntimeError('Proxy did not start')
                        time.sleep(0.05)
                web = object.__new__(gui.Webview)
                web.endpoint, web.session, web.record = endpoint, 'boundary', lambda *e: events.append(e)
                web.request('GET', '/status')
                # Give Hyper time to return the completed connection to its pool.
                time.sleep(0.1)
                try:
                    if mode == 'semantic':
                        gui.wait(lambda: web.read('document.body.innerText'), 'semantic must be fatal')
                    else:
                        web.command('POST', '/element/save/click', {})
                except (http.client.RemoteDisconnected, ConnectionResetError, urllib.error.URLError, RuntimeError) as error:
                    failure = repr(error)
                # Let accidental internal replays arrive before inspecting counts.
                time.sleep(0.15)
            finally:
                driver.send_signal(signal.SIGTERM)
                driver.wait(timeout=10)
                peer.shutdown()
                peer.server_close()
                thread.join(timeout=5)
    result = {'binary': str(binary), 'binary_sha256': gui.digest(binary), 'mode': mode,
              'requests': peer.events, 'client_events': events, 'failure': failure}
    (output / f'{binary.name}-{mode}.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--original', type=Path, required=True)
    parser.add_argument('--corrected', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    results = []
    for binary in (args.original.resolve(), args.corrected.resolve()):
        reuse = experiment(binary, 'reuse', args.output)
        results.append(reuse)
        assert len(reuse['requests']) == 2, reuse
        assert bool(reuse['failure']) == (binary == args.original.resolve()), reuse
        assert reuse['requests'][1]['on_connection'] == (2 if reuse['failure'] else 1), reuse
        delivered = experiment(binary, 'delivered', args.output)
        results.append(delivered)
        assert delivered['failure'] and len(delivered['requests']) == 2, delivered
        assert delivered['requests'][1]['reset_after_receipt'], delivered
        assert not delivered['client_events'], delivered
        semantic = experiment(binary, 'semantic', args.output)
        results.append(semantic)
        assert 'no such element' in semantic['failure'] and len(semantic['requests']) == 2, semantic
        assert not semantic['client_events'], semantic
    (args.output / 'summary.json').write_text(json.dumps({'status': 'passed', 'experiments': results}, indent=2) + '\n')
    print('Six real-proxy boundary experiments passed (not GUI evidence).')


if __name__ == '__main__':
    main()
