"""Regenerate current desktop test inputs with their Rust generators."""

import argparse
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
FRONTEND = ROOT / "desktop/src/__fixtures__"
DOCUMENTS = ROOT / "desktop/scripts/fixtures"
GENERATORS = (
    ("write_frontend_fixtures", FRONTEND),
    ("write_tori_frontend_fixtures", FRONTEND),
    ("write_chu_frontend_fixtures", FRONTEND),
    ("write_tenjiku_frontend_fixtures", FRONTEND),
    ("write_document_fixtures", DOCUMENTS),
    ("write_tori_gui_documents", DOCUMENTS),
    ("write_chu_gui_documents", DOCUMENTS),
    ("write_tenjiku_gui_fixtures", DOCUMENTS),
)


def regenerate(check):
    with tempfile.TemporaryDirectory(prefix="mshogi-desktop-fixtures-") as temporary:
        outputs = {
            FRONTEND: Path(temporary) / "frontend",
            DOCUMENTS: Path(temporary) / "documents",
        }
        for example, directory in GENERATORS:
            command = ["cargo", "run", "--locked", "-p", "mshogi-desktop-session", "--example", example]
            if directory == FRONTEND:
                command.extend(["--features", "desktop-verification"])
            subprocess.run([*command, "--", str(outputs[directory])], cwd=ROOT, check=True)

        current = {
            FRONTEND: set(FRONTEND.glob("*.json")),
            DOCUMENTS: set(DOCUMENTS.glob("*.mshogi")) | set(DOCUMENTS.glob("*-game-fixtures.json")),
        }
        errors = []
        count = 0
        for directory, generated in outputs.items():
            for path in sorted(generated.iterdir()):
                count += 1
                destination = directory / path.name
                current[directory].discard(destination)
                if destination.is_file() and destination.read_bytes() == path.read_bytes():
                    continue
                if check:
                    errors.append(f"Out of date: {destination.relative_to(ROOT)}")
                else:
                    shutil.copyfile(path, destination)
                    print(f"Updated {destination.relative_to(ROOT)}")
            for stale in sorted(current[directory]):
                errors.append(f"No generator produced {stale.relative_to(ROOT)}")
        if errors:
            raise SystemExit("\n".join(errors))
        print(f"Verified {count} generated desktop fixtures")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="compare generated files with the checkout")
    regenerate(parser.parse_args().check)


if __name__ == "__main__":
    main()
