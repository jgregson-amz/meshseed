#!/usr/bin/env python3
"""Seed-node membership endpoint.

Serves the meshping membership banner so joining nodes can reach this seed. Binds
the membership port and holds it open for the session, answering each connection
with a per-run banner and logging who connected (stdout only).
"""
import secrets
import socket
import sys
import threading
import time

PORT = 443
HOLD_S = 7 * 3600  # hold the endpoint open for the session (bounded by the session idle/timeout)
TOKEN = "MESHPING/1 " + secrets.token_hex(8)
BANNER = (TOKEN + "\r\n").encode()


def handle(conn, addr):
    sys.stdout.write("MESHPING_SEED conn from %s:%s\n" % addr)
    sys.stdout.flush()
    try:
        conn.settimeout(4.0)
        try:
            conn.recv(256)
        except OSError:
            pass
        conn.sendall(BANNER)
    except OSError:
        pass
    finally:
        try:
            conn.close()
        except OSError:
            pass


def serve():
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(("0.0.0.0", PORT))
    s.listen(128)
    s.settimeout(5.0)
    sys.stdout.write("MESHPING_SEED listening on %d token=%s\n" % (PORT, TOKEN))
    sys.stdout.flush()
    end = time.time() + HOLD_S
    while time.time() < end:
        try:
            conn, addr = s.accept()
        except socket.timeout:
            continue
        except OSError:
            break
        threading.Thread(target=handle, args=(conn, addr), daemon=True).start()


if __name__ == "__main__":
    serve()
