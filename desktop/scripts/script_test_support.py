"""Load command-line scripts with hyphenated names for unit tests."""

import importlib.util
from pathlib import Path


def load_script(name):
    path = Path(__file__).with_name(f"{name}.py")
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
