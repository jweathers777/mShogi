"""Common I/O for developer-only rules checks."""
import hashlib
import json
from pathlib import Path
import subprocess


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def git(source, *args):
    return subprocess.check_output(
        ["git", "-C", str(source), *args], text=True
    ).strip()


def strict_json(data):
    def unique_object(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError("duplicate JSON field: " + key)
            result[key] = value
        return result

    def reject_nonfinite(text):
        raise ValueError("nonfinite JSON: " + text)

    return json.loads(data, object_pairs_hook=unique_object, parse_constant=reject_nonfinite)


def run_json_probe(command, request, timeout, transport):
    return transport(command, (json.dumps(request) + "\n").encode("utf-8"), timeout)
