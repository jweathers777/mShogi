"""Canonical JSON, hashing and host description shared by the cloud strength tools."""
import hashlib
import importlib.metadata
import json
import os
import platform
import socket
from pathlib import Path

def encode(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False)+'\n').encode()

def decode(data):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError('duplicate JSON key: '+key)
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=unique)

def digest(data):
    return hashlib.sha256(data).hexdigest()

def file_hash(path):
    h = hashlib.sha256()
    with open(path, 'rb') as stream:
        for block in iter(lambda: stream.read(1024*1024), b''):
            h.update(block)
    return h.hexdigest()

def environment():
    value = {'platform': platform.platform(), 'architecture': platform.machine(), 'python': platform.python_version(), 'hostname': socket.gethostname(), 'cpu_count': os.cpu_count(), 'load': list(os.getloadavg())}
    value['packages']={}
    for name in ('google-cloud-storage','google-cloud-pubsub'):
        try: value['packages'][name]=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError: pass
    if hasattr(os, 'sched_getaffinity'):
        value['cpu_affinity'] = sorted(os.sched_getaffinity(0))
    if Path('/proc/cpuinfo').exists():
        value['cpuinfo'] = Path('/proc/cpuinfo').read_text()
    return value
