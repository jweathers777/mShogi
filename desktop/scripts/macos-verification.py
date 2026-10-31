"""Prepare immutable macOS QA identities; select external startup fixtures.

No application launching, UI automation, or permission-store access. Automated
interaction must use CUA; an operator may use macOS open --env manually.
Run prepare once per build, then select/inspect without changing signed bytes.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
IDENTITIES = {
    "normal": ("mShogi.app", "org.mshogi.desktop"),
    "qa": ("mShogi Verification.app", "org.mshogi.desktop.verification.r6.shogipromotion"),
    "synthetic": ("mShogi Synthetic.app", "org.mshogi.desktop.verification.r6.synthetic"),
}
FIXTURES = frozenset("""standard shared-contract promotion claim current-claim
concluded incomplete capacity shogi-standard shogi-gote shogi-failure shogi-capacity
shogi-control-capacity shogi-hands shogi-promotion shogi-forced shogi-declaration
shogi-concluded xiangqi-standard xiangqi-failure xiangqi-capacity
xiangqi-control-capacity xiangqi-offer xiangqi-agreed xiangqi-resigned xiangqi-warning
xiangqi-fourth xiangqi-count xiangqi-fast-proof xiangqi-exact-evidence""".split())


def directory():
    return ROOT / "desktop/src-tauri/target/verification/macos"


def bundle(kind):
    name, _ = IDENTITIES[kind]
    parent = ROOT / "desktop/src-tauri/target/release/bundle/macos" if kind == "normal" else directory()
    return parent / name


def selector(kind):
    if kind not in ("qa", "synthetic"):
        raise ValueError("Normal packages have no fixture selector")
    return directory() / f"{kind}.fixture"


def validate(kind, fixture):
    valid = fixture == "synthetic" if kind == "synthetic" else kind == "qa" and fixture in FIXTURES
    if not valid or len(fixture.encode("utf-8")) + 1 > 128:
        raise ValueError("Unsupported fixture for this fixed QA identity")


def select(kind, fixture):
    validate(kind, fixture)
    if running(bundle(kind)):
        raise RuntimeError("Close the staged application natively before selecting its next fixture")
    target = selector(kind)
    target.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=f".{kind}-fixture-", dir=target.parent)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write((fixture + "\n").encode("ascii"))
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, target)
    finally:
        Path(temporary).unlink(missing_ok=True)
    return {"selector": str(target), "fixture": fixture, "effect": "Next QA launch only; signed app unchanged"}


def running(app):
    executable = str((app / "Contents/MacOS/mshogi-desktop").resolve())
    processes = subprocess.run(["ps", "-axo", "pid=,comm="], check=True, text=True, capture_output=True)
    return [line.split(None, 1)[0] for line in processes.stdout.splitlines()
            if len(line.split(None, 1)) == 2
            and str(Path(line.split(None, 1)[1].strip()).resolve()) == executable]


def inspect(kind):
    app = bundle(kind)
    info = plistlib.loads((app / "Contents/Info.plist").read_bytes())
    signatures = {}
    for label, arguments in (("display", ["-d", "--verbose=4", "-r-"]),
                             ("strict_verify", ["--verify", "--strict", "--verbose=2"])):
        result = subprocess.run(["codesign", *arguments, str(app)], text=True, capture_output=True)
        signatures[label] = {"exit_code": result.returncode, "output": result.stdout + result.stderr}
    files = {str(path.relative_to(app)): hashlib.sha256(
        os.readlink(path).encode() if path.is_symlink() else path.read_bytes()).hexdigest()
        for path in sorted(app.rglob("*")) if path.is_symlink() or path.is_file()}
    return {"path": str(app), "bundle_id": info.get("CFBundleIdentifier"),
            "expected_bundle_id": IDENTITIES[kind][1], "environment": info.get("LSEnvironment", {}),
            "files": files, "signatures": signatures,
            "qualification": "Ad-hoc signatures may change on rebuild; no approval persistence is inferred"}


def prepare(kind, gate=False):
    if kind not in ("qa", "synthetic") or gate and kind != "qa":
        raise ValueError("Only fixed QA/synthetic identities may be prepared; the gate is QA-only")
    destination = bundle(kind)
    if running(destination):
        raise RuntimeError("Close the staged application natively before preparing it again")
    source = ROOT / "desktop/src-tauri/target/release/bundle/macos/mShogi Verification.app"
    environment = {"MSHOGI_DESKTOP_FIXTURE_FILE": str(selector(kind))}
    if gate:
        external_gate = directory() / "r6-engine-gate"
        if not external_gate.is_file() or not os.access(external_gate, os.X_OK):
            raise RuntimeError("Compile the external engine gate at the fixed staging path first")
        environment.update(MSHOGI_VERIFICATION_ENGINE=str(external_gate),
                           MSHOGI_R6_REAL_ENGINE=str(destination / "Contents/MacOS/mshogi"),
                           MSHOGI_R6_DELAY_MS="5000")
    destination.parent.mkdir(parents=True, exist_ok=True)
    selection = selector(kind)
    if selection.exists():
        with selection.open("rb") as stream:
            selected = stream.read(129)
        if len(selected) > 128:
            raise ValueError("Fixture selector exceeds its 128-byte bound")
        validate(kind, selected.decode("ascii").strip())
    else:
        select(kind, "synthetic" if kind == "synthetic" else "standard")
    with tempfile.TemporaryDirectory(prefix=".prepare-", dir=destination.parent) as temporary:
        staged = Path(temporary) / destination.name
        shutil.copytree(source, staged, symlinks=True)
        plist = staged / "Contents/Info.plist"
        info = plistlib.loads(plist.read_bytes())
        name, identifier = IDENTITIES[kind]
        info.update(CFBundleIdentifier=identifier, CFBundleName=name[:-4],
                    CFBundleDisplayName=name[:-4], LSEnvironment=environment)
        plist.write_bytes(plistlib.dumps(info))
        subprocess.run(["codesign", "--force", "--deep", "--sign", "-", "--identifier", identifier, str(staged)],
                       check=True, capture_output=True, text=True)
        subprocess.run(["codesign", "--verify", "--strict", str(staged)], check=True, capture_output=True, text=True)
        if running(destination):
            raise RuntimeError("The staged application started during preparation; no replacement installed")
        backup = Path(temporary) / "previous.app"
        if destination.exists():
            destination.rename(backup)
        try:
            staged.rename(destination)
        except Exception:
            if backup.exists():
                backup.rename(destination)
            raise
    return inspect(kind)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    operations = parser.add_subparsers(dest="operation", required=True)
    build = operations.add_parser("prepare", help="Explicitly copy/sign once per build; never launch")
    build.add_argument("kind", choices=("qa", "synthetic"))
    build.add_argument("--gate", action="store_true", help="Use the fixed, separately compiled external QA engine gate")
    choose = operations.add_parser("select", help="Atomically select the next launch; do not modify app bytes")
    choose.add_argument("kind", choices=("qa", "synthetic"))
    choose.add_argument("fixture")
    show = operations.add_parser("inspect", help="Read identity, complete hashes and actual signing diagnostics")
    show.add_argument("kind", choices=tuple(IDENTITIES))
    args = parser.parse_args()
    result = prepare(args.kind, args.gate) if args.operation == "prepare" else select(args.kind, args.fixture) if args.operation == "select" else inspect(args.kind)
    print(json.dumps(result, indent=2))
    if args.operation != "select" and (result["signatures"]["strict_verify"]["exit_code"]
                                      or result["bundle_id"] != result["expected_bundle_id"]):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
