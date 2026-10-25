"""External installed-application verification; never bundled with mShogi.

W3C pointer/keyboard commands operate the actual native webview. Script execution
is read-only DOM observation, never Tauri IPC, injected events, or game mutation.
Native adapters operate real file dialogs and native close/quit requests.
"""
import argparse
import base64
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import hashlib
import importlib
import json
import os
from pathlib import Path
import platform
import signal
import shutil
import subprocess
import time
import traceback
import urllib.error
import urllib.request


def stamp():
    return datetime.now(timezone.utc).isoformat()


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def wait(predicate, label, seconds=25):
    deadline = time.monotonic() + seconds
    last = None
    while time.monotonic() < deadline:
        try:
            last = predicate()
            if last:
                return last
        except (AssertionError, RuntimeError, urllib.error.URLError) as error:
            last = str(error)
        time.sleep(0.1)
    raise AssertionError(f"Timed out: {label}; last observation: {last!r}")


class Webview:
    def __init__(self, endpoint, host):
        self.endpoint = endpoint
        self.session = None
        response = self.request("POST", "/session", {"capabilities": {
            "alwaysMatch": {"browserName": "wry", "tauri:options": {
                "application": str(host)}}}})
        self.session = response["sessionId"]
        self.capabilities = response.get("capabilities", {})

    def request(self, method, path, data=None):
        payload = None if data is None else json.dumps(data).encode()
        request = urllib.request.Request(self.endpoint + path, data=payload,
            headers={"Content-Type": "application/json"}, method=method)
        try:
            with urllib.request.urlopen(request, timeout=45) as response:
                reply = json.load(response)
        except urllib.error.HTTPError as error:
            raise RuntimeError(error.read().decode(errors="replace")) from error
        value = reply.get("value")
        if isinstance(value, dict) and "error" in value:
            raise RuntimeError(json.dumps(value))
        return value

    def command(self, method, path, data=None):
        return self.request(method, f"/session/{self.session}{path}", data)

    def find(self, selector, using="css selector"):
        value = self.command("POST", "/element", {"using": using, "value": selector})
        return value["element-6066-11e4-a52e-4f735466cecf"]

    def click(self, selector):
        if selector.startswith(".square["):
            # WebKit's element-click heuristic requires the SVG group itself
            # in the hit-test tree, although only its painted children occur
            # there. Observe its real geometry, then send actual pointer input.
            target = self.read("""(() => {
                const element = document.querySelector(SELECTOR);
                if (!element || element.getAttribute('aria-disabled') === 'true') return null;
                const rect = element.getBoundingClientRect();
                const left = Math.max(0, rect.left), right = Math.min(innerWidth, rect.right);
                const top = Math.max(0, rect.top), bottom = Math.min(innerHeight, rect.bottom);
                const x = Math.floor((left + right) / 2), y = Math.floor((top + bottom) / 2);
                const hit = document.elementFromPoint(x, y);
                return {x, y, visible: right > left && bottom > top,
                    hit: hit?.tagName, containsHit: hit !== null && element.contains(hit)};
            })()""".replace("SELECTOR", json.dumps(selector)))
            assert target and target["visible"] and target["containsHit"], (selector, target)
            self.command("POST", "/actions", {"actions": [{"type": "pointer", "id": "mouse",
                "parameters": {"pointerType": "mouse"}, "actions": [
                    {"type": "pointerMove", "duration": 0, "origin": "viewport",
                     "x": target["x"], "y": target["y"]},
                    {"type": "pointerDown", "button": 0},
                    {"type": "pointerUp", "button": 0}]}]})
            return
        self.command("POST", f"/element/{self.find(selector)}/click", {})

    def button(self, label):
        assert "'" not in label
        element = self.find(f"//button[normalize-space(.)='{label}']", "xpath")
        self.command("POST", f"/element/{element}/click", {})

    def set(self, selector, value):
        element = self.find(selector)
        self.command("POST", f"/element/{element}/click", {})
        # Native Select-all/Backspace generates the real input events React's
        # controlled number fields need; WebDriver clear alone can retain state.
        self.command("POST", "/actions", {"actions": [{"type": "key", "id": "keyboard",
            "actions": [{"type": "keyDown", "value": "\ue009"},
                        {"type": "keyDown", "value": "a"},
                        {"type": "keyUp", "value": "a"},
                        {"type": "keyUp", "value": "\ue009"},
                        {"type": "keyDown", "value": "\ue003"},
                        {"type": "keyUp", "value": "\ue003"}]}]})
        self.command("POST", f"/element/{element}/value", {"text": str(value)})
        observed = self.read(f"document.querySelector({json.dumps(selector)})?.value")
        assert observed == str(value), (selector, value, observed)

    def key(self, key):
        self.command("POST", "/actions", {"actions": [{"type": "key", "id": "keyboard",
            "actions": [{"type": "keyDown", "value": key},
                        {"type": "keyUp", "value": key}]}]})

    def read(self, expression):
        return self.command("POST", "/execute/sync", {
            "script": "return " + expression, "args": []})

    def text(self, selector="body"):
        return self.read(f"document.querySelector({json.dumps(selector)})?.innerText ?? ''")

    def has(self, selector):
        return self.read(f"document.querySelector({json.dumps(selector)}) !== null")

    def screenshot(self, path):
        path.write_bytes(base64.b64decode(self.command("GET", "/screenshot")))

    def close(self):
        if self.session is not None:
            self.command("DELETE", "")
            self.session = None


class Journey:
    def __init__(self, args, report):
        self.args, self.report = args, report
        self.native = None
        self.driver = self.web = self.log = None
        self.phase = None
        self.phase_report = None
        self.native_events = []

    def record(self, operation, observation):
        item = {"at_utc": stamp(), "operation": operation, "observation": observation}
        self.phase_report["events"].append(item)
        print(json.dumps(item), flush=True)

    def capture(self, label):
        prefix = self.args.output / f"{self.phase}-{label}"
        self.native.screenshot(prefix.with_suffix(".desktop.png"))
        self.web.screenshot(prefix.with_suffix(".webview.png"))
        prefix.with_suffix(".dom.json").write_text(json.dumps({
            "text": self.web.text(), "focus": self.web.read(
                "({tag:document.activeElement?.tagName,label:document.activeElement?.getAttribute('aria-label'),text:document.activeElement?.textContent})"),
            "dimensions": self.web.read("({width:innerWidth,height:innerHeight,dpr:devicePixelRatio})"),
            "inputs": self.web.read("[...document.querySelectorAll('input,select')].map(e=>({tag:e.tagName,type:e.type,value:e.value,min:e.min,max:e.max,disabled:e.disabled}))")
        }, indent=2), encoding="utf-8")

    def start(self, fixture=None):
        host = self.args.normal_host if fixture is None else self.args.verification_host
        engine = self.args.normal_engine if fixture is None else self.args.verification_engine
        self.phase = "normal" if fixture is None else f"verification-{fixture}"
        self.phase_report = {"phase": self.phase, "status": "running",
            "fixture": fixture, "host": str(host), "host_sha256": digest(host),
            "engine": str(engine), "engine_sha256": digest(engine),
            "events": [], "started_at_utc": stamp()}
        self.report["phases"].append(self.phase_report)
        self.native = importlib.import_module(
            "gui_windows" if platform.system() == "Windows" else "gui_linux").Native(
                self.args.output / self.phase)
        self.phase_report["native_environment"] = self.native.environment()
        env = os.environ.copy()
        env.pop("MSHOGI_DESKTOP_FIXTURE", None)
        if fixture is not None:
            env["MSHOGI_DESKTOP_FIXTURE"] = fixture
        command = [self.args.driver, "--port", "4444", "--native-port", "4445"]
        if self.args.native_driver:
            command += ["--native-driver", self.args.native_driver]
        self.log = (self.args.output / f"{self.phase}-driver.log").open("w", encoding="utf-8")
        self.driver = subprocess.Popen(command, stdout=self.log, stderr=subprocess.STDOUT, env=env,
            start_new_session=platform.system() != "Windows",
            creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
        self.phase_report["driver_command"] = command
        self.phase_report["driver_creation"] = {"windows_no_console": platform.system() == "Windows",
            "stdout_stderr": f"{self.phase}-driver.log"}
        endpoint = "http://127.0.0.1:4444"
        def ready():
            with urllib.request.urlopen(endpoint + "/status", timeout=2) as response:
                return response.status == 200
        wait(ready, "external native driver startup")
        self.web = Webview(endpoint, host)
        self.phase_report["capabilities"] = self.web.capabilities
        self.phase_report["native_window"] = self.native.attach(host, engine)
        wait(lambda: self.web.has(".synthetic-inspector" if fixture == "synthetic" else ".document-card"),
             "actual application frontend ready")
        self.record("Launch actual packaged application", self.phase_report["native_window"])

    def stopped(self):
        wait(lambda: not self.native.alive(), "host exits through native application guard")
        processes = self.native.processes()
        wait(lambda: not self.native.processes(), "bundled engine cleanup after native quit", 5)
        self.record("Observe native shutdown before driver teardown", {
            "host_alive": False, "immediate_matching_processes": processes,
            "final_matching_processes": self.native.processes()})
        self.phase_report["status"] = "passed"
        self.phase_report["finished_at_utc"] = stamp()

    def cleanup(self):
        # Cleanup is never used as evidence for graceful application exit.
        if self.web is not None:
            try:
                self.web.close()
            except Exception as error:
                self.phase_report["driver_teardown_observation"] = str(error)
            self.web = None
        if self.driver is not None:
            if platform.system() == "Windows":
                subprocess.run(["taskkill", "/PID", str(self.driver.pid), "/T", "/F"],
                    capture_output=True, timeout=10, check=False)
            else:
                try:
                    os.killpg(self.driver.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
            try:
                self.driver.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.driver.kill()
                self.driver.wait(timeout=5)
            self.driver = None
        if self.log is not None:
            self.log.close()
            self.log = None
        if self.native is not None:
            self.native.cleanup()
            self.native_events.extend(getattr(self.native, "events", []))
            self.native = None

    def plies(self, count, historical=False):
        text = f"{'Selected' if historical else 'Live'} position · {count} plies"
        wait(lambda: text in self.web.text(".game-footnote"), text)

    def square(self, name):
        return f'.square[aria-label^="{name},"]'

    def file(self, kind, path, cancel=False):
        with ThreadPoolExecutor(max_workers=1) as pool:
            picker = pool.submit(self.native.choose_file, kind, None if cancel else path)
            self.web.button("Open…" if kind == "open" else "Save")
            evidence = picker.result(timeout=35)
        self.record(f"Native {kind} {'cancel' if cancel else 'choose file'}", evidence)

    def save(self, path, observation_count=None):
        self.file("save", path)
        wait(lambda: self.web.text(".document-dirty") == "Saved", "complete document saved")
        wait(path.exists, "native save created file")
        archive = json.loads(path.read_text(encoding="utf-8"))
        assert archive["profile"] == "mshogi.desktop.archive.v1"
        if observation_count is not None:
            assert len(archive["observations"]) == observation_count
        self.record("Inspect actual saved canonical archive", {
            "path": str(path), "sha256": digest(path), "bytes": path.stat().st_size,
            "observations": len(archive["observations"]), "controls": len(archive["controls"]),
            "game_events": len(json.loads(archive["game_record_json"])["events"])})
        return archive

    def quit(self, synthetic=False):
        evidence = self.native.quit_app()
        self.record("Native Quit request", evidence)
        wait(lambda: self.web.has("[role=dialog]"), "native quit reaches application guard")
        self.web.button("Close fixture" if synthetic else "Discard & continue")
        self.stopped()

    def normal(self):
        self.start()
        w = self.web
        self.plies(0)
        w.click(self.square("e2")); w.click(self.square("e4"))
        self.plies(2)
        wait(lambda: "Move returned:" in w.text(".engine-choice"), "actual engine reply")
        assert not w.has(".outcome-card")
        self.capture("play")
        self.record("Play e2e4 against actual bundled engine", w.text(".engine-choice"))

        w.button("New game")
        wait(lambda: w.has(".document-guard"), "unsaved replacement guard")
        w.button("Cancel")
        wait(lambda: not w.has(".document-guard"), "cancel replacement")
        self.plies(2)
        w.button("Analyze position")
        w.set('.analysis-fields input[min="1"]', 8)
        w.set('.analysis-fields input[min="0"]', 5_000_000)
        w.click('[data-analysis-start="moves"]')
        wait(lambda: w.text(".analysis-card .engine-phase") == "Analyzing", "real active analysis")
        self.capture("active-analysis")
        w.button("Cancel analysis")
        wait(lambda: "cancelled" in w.text('[aria-label="Analysis status"]').lower(), "analysis cancellation")
        self.plies(2)
        self.record("Cancel demonstrably active analysis", w.text('[aria-label="Analysis status"]'))
        w.click('[data-analysis-start="moves"]')
        wait(lambda: w.text(".analysis-card .engine-phase") == "Analyzing", "replacement begins from active work")
        w.set('.analysis-fields input[min="1"]', 1)
        w.set('.analysis-fields input[min="0"]', 1_000)
        w.click('[data-analysis-start="moves"]')
        wait(lambda: "not played" in w.text(".analysis-suggestion"), "completed read-only replacement")
        self.plies(2)
        self.capture("replacement-complete")
        self.record("Replace with bounded read-only suggestion", w.text(".analysis-result"))

        path = self.args.output / "normal-complete.mshogi"
        archive = self.save(path)
        assert len(archive["observations"]) >= 2
        self.file("open", path)
        wait(lambda: "Opened the complete document with work stopped" in w.text(".document-card"), "reopen completed")
        self.plies(2)
        assert w.has(".play-paused")
        w.button("Play details")
        assert "No engine search has completed" in w.text(".sidebar")
        self.record("Reopen complete document without resuming work", w.text(".document-card"))
        w.click(".event-timeline li:nth-child(3) button")
        self.plies(1, historical=True)
        assert "read only" in w.text(".historical-notice")
        w.button("Analyze position")
        w.set('.analysis-fields input[min="1"]', 1)
        w.set('.analysis-fields input[min="0"]', 1_000)
        w.click('[data-analysis-start="moves"]')
        wait(lambda: "not played" in w.text(".analysis-suggestion"), "historical analysis")
        assert "Black" in w.text(".analysis-result")
        self.plies(1, historical=True)
        self.capture("historical-analysis")
        w.button("Return to live game")
        self.plies(2)
        assert not w.has(".historical-notice") and w.has(".play-paused")
        assert "Viewing an exact read-only history prefix" not in w.text(".document-card")
        w.button("Resume play")
        wait(lambda: not w.has(".play-paused"), "explicit Resume")
        self.plies(2)
        assert "Play remains paused" not in w.text(".document-card")
        # Save to the existing path uses no picker; the earlier observation is
        # canonical historical information, never resumable pending work.
        w.button("Save")
        wait(lambda: w.text(".document-dirty") == "Saved", "save history observation")
        archive = json.loads(path.read_text(encoding="utf-8"))
        observation = archive["observations"][-1]["response"]["context"]
        assert observation["purpose"]["kind"] == "move_analysis"
        assert len(observation["game"]["events"]) == 1
        self.record("History prefix provenance / Return live / Resume", {
            "historical_request": observation, "live_game_events":
                len(json.loads(archive["game_record_json"])["events"])})
        w.click(".flip-button")
        wait(lambda: w.text(".document-dirty") == "Unsaved", "orientation change is protected")
        self.record("Native window-close request", self.native.close_window())
        wait(lambda: w.has(".document-guard"), "native close protects unsaved work")
        self.capture("native-close-guard")
        w.button("Cancel")
        wait(lambda: not w.has(".document-guard"), "cancel native close")
        assert self.native.alive()
        self.plies(2)
        self.quit()

    def fixture(self, name):
        self.start(name)
        w = self.web
        if name == "standard":
            w.click(self.square("e2")); w.click(self.square("e4")); self.plies(2)
            # The extracted/installed verification package is writable. This
            # exercises the same actual supervisor without changing /usr/bin
            # permissions or running the installed Linux GUI as root.
            engine = self.args.verification_engine
            held = engine.with_name(engine.name + ".gui-verification-held")
            original = digest(engine)
            assert not held.exists()
            try:
                engine.rename(held)
                w.click(self.square("d2")); w.click(self.square("d4"))
                wait(lambda: "Engine failure" in w.text("[role=alert]"), "genuine missing engine failure")
                self.plies(3)
                assert not w.has(".outcome-card")
                self.capture("engine-failure")
                self.record("Genuine missing-engine startup failure preserves three plies", w.text("[role=alert]"))
            finally:
                if held.exists():
                    held.rename(engine)
            assert digest(engine) == original
            w.button("Retry engine turn")
            self.plies(4)
            assert not w.has(".outcome-card")
            self.record("Restore identical sidecar and Retry without losing history", {
                "engine_sha256": digest(engine), "result": w.text(".engine-choice")})
        elif name == "promotion":
            w.click(self.square("a7")); w.click(self.square("a8"))
            wait(lambda: w.has(".promotion-dialog"), "actual promotion choices")
            self.capture("promotion")
            w.key("\ue00c")  # Escape
            self.plies(0)
            w.click(self.square("a7")); w.click(self.square("a8")); w.click(".promotion-choices button:nth-child(4)")
            self.plies(1)
            wait(lambda: "proven dead position" in w.text(".outcome-card"), "native underpromotion outcome")
            self.record("Cancel promotion then commit complete Knight promotion", w.text(".outcome-card"))
        elif name == "claim":
            self.plies(0)
            assert "An opportunity to claim" in w.text()
            assert not w.has(".outcome-card")
            w.button("Claim selected draw")
            wait(lambda: "claimed fifty-move rule" in w.text(".outcome-card"), "native claim conclusion")
            self.plies(0)
            self.record("Optional intended-move claim concludes without playing intention", w.text(".outcome-card"))
        elif name == "capacity":
            assert "observations (255)" in w.text(".saved-observations")
            w.button("Analyze position"); w.click('[data-analysis-start="moves"]')
            wait(lambda: "observations (256)" in w.text(".saved-observations"), "accept observation 256")
            w.click('[data-analysis-start="moves"]')
            wait(lambda: "Application limit reached" in w.text(".capacity-notice"), "reject operation before observation 257")
            assert "observations (256)" in w.text(".saved-observations")
            assert not w.has(".outcome-card")
            self.capture("capacity-pause")
            self.save(self.args.output / "capacity-complete.mshogi", 256)
            # A preference change keeps this temporary saved document dirty for
            # the native lifecycle guard; it does not remove any observation.
            w.click(".flip-button")
        elif name == "synthetic":
            assert "not a real shogi variant" in w.text()
            assert not w.has(".document-card")
            w.button("↕ Flip board")
            w.key("\ue004")  # Tab from Flip enters the board's single stop.
            wait(lambda: str(w.read("document.activeElement?.getAttribute('aria-label')")).startswith("i9,"),
                 "keyboard focus exposes native synthetic origin")
            # Black orientation: left/down goes i9 -> j10. Native keyboard
            # focus scrolls the shared large board into view without JS input.
            for key in ("\ue007", "\ue012", "\ue015", "\ue007"):
                w.key(key)
            assert "No action submitted" in w.text(".route-selection")
            assert "Accepted actions 0" in w.text(".synthetic-inspector")
            w.key("\ue004"); w.key("\ue00c")  # Tab to completion, Escape cancels.
            wait(lambda: not w.has(".route-selection"), "cancel staged selection")
            for key in ("\ue007", "\ue012", "\ue015", "\ue007", "\ue014", "\ue013", "\ue007"):
                w.key(key)
            wait(lambda: "Accepted actions 1" in w.text(".synthetic-inspector"), "native complete route")
            assert "empty" in w.read(f"document.querySelector({json.dumps(self.square('j10'))}).getAttribute('aria-label')")
            w.click(".synthetic-inspector details summary")
            assert "136>153>136" in w.text(".synthetic-inspector pre")
            self.capture("capture-route")
            w.button("Reset synthetic fixture")
            wait(lambda: "Accepted actions 0" in w.text(".synthetic-inspector"), "native reset")
            w.button("↕ Flip board")  # White orientation for the alternate route.
            w.key("\ue004")
            wait(lambda: str(w.read("document.activeElement?.getAttribute('aria-label')")).startswith("i9,"),
                 "keyboard focus exposes reset synthetic origin")
            for key in ("\ue007", "\ue012", "\ue007", "\ue014", "\ue007"):
                w.key(key)
            wait(lambda: "136>135>136" in w.text(".synthetic-inspector pre"), "alternate complete native route")
            assert "stone" in w.read(f"document.querySelector({json.dumps(self.square('j10'))}).getAttribute('aria-label')")
            assert "mshogi.synthetic.strider16.v1" in w.text(".synthetic-inspector pre")
            self.capture("alternate-route")
            self.record("Distinct complete routes share endpoints but preserve different effects", w.text(".synthetic-inspector"))
        else:
            raise AssertionError(name)
        self.capture("result")
        self.quit(synthetic=name == "synthetic")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("normal-host", "normal-engine", "verification-host", "verification-engine"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--driver", default="tauri-driver")
    parser.add_argument("--native-driver")
    parser.add_argument("--phase", choices=["normal", "verification", "all"], default="all")
    args = parser.parse_args()
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=True)
    for name in ("normal_host", "normal_engine", "verification_host", "verification_engine"):
        setattr(args, name, getattr(args, name).resolve())
    report = {"schema_version": 1, "started_at_utc": stamp(), "status": "running",
        "source_revision": os.environ.get("GITHUB_SHA") or subprocess.check_output(
            ["git", "rev-parse", "HEAD"], text=True).strip(),
        "workflow_url": f"https://github.com/{os.environ.get('GITHUB_REPOSITORY')}/actions/runs/{os.environ.get('GITHUB_RUN_ID')}",
        "environment": {"platform": platform.platform(), "python": platform.python_version(),
            "runner_image": os.environ.get("ImageVersion"), "runner_os": os.environ.get("RUNNER_OS"),
            "renderer_environment": {name: os.environ.get(name) for name in
                ("GDK_BACKEND", "WEBKIT_DISABLE_COMPOSITING_MODE", "WEBKIT_DISABLE_DMABUF_RENDERER", "LIBGL_ALWAYS_SOFTWARE")}},
        "method": "Actual installed/packaged GUI via W3C pointer/keyboard and native OS dialogs/close, external to distributables. Read-only DOM inspection; no direct IPC or injected input events.",
        "driver_identity": {"path": shutil.which(args.driver) or args.driver,
            "version": subprocess.run([args.driver, "--version"], capture_output=True,
                text=True, timeout=5, check=False).stdout.strip()},
        "phases": []}
    journey = None
    try:
        journey = Journey(args, report)
        phases = ([None] if args.phase == "normal" else
                  ["standard", "promotion", "claim", "capacity", "synthetic"] if args.phase == "verification" else
                  [None, "standard", "promotion", "claim", "capacity", "synthetic"])
        for fixture in phases:
            try:
                journey.normal() if fixture is None else journey.fixture(fixture)
            except Exception as phase_error:
                if journey.phase_report is not None:
                    journey.phase_report.update(status="failed", error=str(phase_error))
                try:
                    journey.capture("failure")
                except Exception as capture_error:
                    report["failure_capture_error"] = str(capture_error)
                raise
            finally:
                journey.cleanup()
        report["status"] = "passed"
    except Exception as error:
        report.update(status="failed", error=str(error), traceback=traceback.format_exc())
        raise
    finally:
        if journey is not None:
            journey.cleanup()
            report["native_events"] = journey.native_events
        report["finished_at_utc"] = stamp()
        report["files"] = {str(path.relative_to(args.output)): digest(path)
            for path in sorted(args.output.rglob("*")) if path.is_file() and path.name != "gui-report.json"}
        (args.output / "gui-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
