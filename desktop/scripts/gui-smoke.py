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
import http.client
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


def terminal_transport_closed(error):
    """Only meaningful after dispatching the known native-close guard action."""
    if isinstance(error, (http.client.RemoteDisconnected, ConnectionResetError, BrokenPipeError)):
        return True
    if isinstance(error, urllib.error.URLError):
        return isinstance(error.reason, (ConnectionResetError, ConnectionRefusedError, BrokenPipeError))
    if isinstance(error, RuntimeError):
        try:
            response = json.loads(str(error))
        except (ValueError, TypeError):
            return False
        value = response.get("value", response) if isinstance(response, dict) else {}
        return (isinstance(value, dict) and
                (value.get("error") in ("invalid session id", "no such window") or
                 (value.get("error") == "unknown error" and
                  value.get("message") == "Session terminated without a reply")))
    return False


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
        if selector.startswith((".square[", ".shogi-square[")):
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
        try:
            self.web.screenshot(prefix.with_suffix(".webview.png"))
        finally:
            # DOM state can precede the X11 compositor's displayed frame. Give
            # native presentation a bounded settle after webview capture. Also
            # retain the raw desktop if a failed session cannot supply a view.
            time.sleep(0.15)
            self.native.screenshot(prefix.with_suffix(".desktop.png"))
        prefix.with_suffix(".dom.json").write_text(json.dumps({
            "text": self.web.text(), "focus": self.web.read(
                "({tag:document.activeElement?.tagName,label:document.activeElement?.getAttribute('aria-label'),text:document.activeElement?.textContent})"),
            "dimensions": self.web.read("({width:innerWidth,height:innerHeight,dpr:devicePixelRatio})"),
            "inputs": self.web.read("[...document.querySelectorAll('input,select')].map(e=>({tag:e.tagName,type:e.type,value:e.value,min:e.min,max:e.max,disabled:e.disabled}))")
        }, indent=2), encoding="utf-8")

    def start(self, fixture=None, product=None):
        host = self.args.normal_host if fixture is None else self.args.verification_host
        engine = self.args.normal_engine if fixture is None else self.args.verification_engine
        self.phase = ("normal" if product is None else f"normal-{product}") if fixture is None else f"verification-{fixture}"
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
        startup = {"attempts": 0, "last_transport_error": None}
        self.phase_report["driver_startup"] = startup
        def ready():
            startup["attempts"] += 1
            if self.driver.poll() is not None:
                raise ChildProcessError(f"Native driver exited during startup: {self.driver.returncode}")
            try:
                with urllib.request.urlopen(endpoint + "/status", timeout=2) as response:
                    return response.status == 200
            except (urllib.error.URLError, http.client.RemoteDisconnected,
                    ConnectionResetError, TimeoutError) as error:
                startup["last_transport_error"] = repr(error)
                return False
        wait(ready, "external native driver startup")
        self.web = Webview(endpoint, host)
        self.phase_report["capabilities"] = self.web.capabilities
        self.phase_report["native_window"] = self.native.attach(host, engine)
        wait(lambda: self.web.has(".synthetic-inspector" if fixture == "synthetic" else ".document-card")
             or (fixture is not None and fixture.startswith("shogi-") and self.web.has(".shogi-application")),
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
            residual = {}
            def isolated():
                residual.update(host_alive=self.native.alive(),
                                matching_processes=self.native.processes())
                return not residual["host_alive"] and not residual["matching_processes"]
            try:
                wait(isolated, "phase cleanup leaves no bound host or engine", 5)
            finally:
                if self.phase_report is not None:
                    self.phase_report["cleanup_isolation"] = {
                        **residual, "scope": "Failure containment only; never native Quit evidence."}
            self.native_events.extend(getattr(self.native, "events", []))
            self.native = None

    def plies(self, count, historical=False):
        text = f"{'Selected' if historical else 'Live'} position · {count} plies"
        wait(lambda: text in self.web.text(".game-footnote"), text)

    def square(self, name):
        return f'.square[aria-label^="{name},"]'

    def viewport(self, width, height):
        """Request and measure content size; OS decoration is never assumed."""
        requested_width, requested_height = width, height
        observations = []
        for _ in range(4):
            native = self.native.resize(requested_width, requested_height)
            time.sleep(0.2)
            observed = self.web.read("({width:innerWidth,height:innerHeight,dpr:devicePixelRatio})")
            observations.append({"native": native, "webview": observed})
            if abs(observed["width"] - width) <= 1 and abs(observed["height"] - height) <= 1:
                self.record("Resize actual application content", {
                    "target": {"width": width, "height": height}, "observations": observations})
                return observed
            requested_width += width - observed["width"]
            requested_height += height - observed["height"]
        raise AssertionError(f"Native resize did not establish requested content size {width}x{height}: {observations}")

    def shogi_board_fits(self):
        wait(lambda: self.web.read("[...document.fonts].some(f=>f.family.replaceAll('\"','').replaceAll(\"'\",'')==='mShogi Pieces' && f.status==='loaded')"),
             "bundled Japanese piece font loaded in actual platform webview")
        geometry = self.web.read("""(() => {
            const board = document.querySelector('.shogi-board');
            const squares = [...document.querySelectorAll('.shogi-square')];
            const players = [...document.querySelectorAll('.shogi-player')];
            const essential = [...document.querySelectorAll('.shogi-header button, .shogi-status-line')];
            const rect = e => {const r=e.getBoundingClientRect(); return {
                left:r.left, top:r.top, right:r.right, bottom:r.bottom, width:r.width, height:r.height};};
            const visible = e => {const r=e.getBoundingClientRect();
                return r.width>0 && r.height>0 && r.left>=-1 && r.top>=-1 && r.right<=innerWidth+1 && r.bottom<=innerHeight+1;};
            return {viewport:{width:innerWidth,height:innerHeight}, board:board ? rect(board):null,
                squares:squares.length, allSquaresVisible:squares.length===81 && squares.every(visible),
                players:players.map(rect), bothPlayerAreasVisible:players.length===2 && players.every(visible),
                essentialControlsVisible:essential.length>0 && essential.every(visible),
                pageWidth:document.documentElement.scrollWidth,
                fonts:document.fonts.status,
                loadedFontFaces:[...document.fonts].map(f=>({family:f.family,status:f.status})),
                pieceFonts:[...new Set([...document.querySelectorAll('.shogi-board text')].map(e=>getComputedStyle(e).fontFamily))]};
        })()""")
        assert geometry["allSquaresVisible"], geometry
        assert geometry["bothPlayerAreasVisible"] and geometry["essentialControlsVisible"], geometry
        assert geometry["pageWidth"] <= geometry["viewport"]["width"] + 1, geometry
        assert geometry["board"]["width"] > 300, geometry
        self.record("Observe complete 9x9 native product board without scrolling", geometry)
        return geometry

    def file(self, kind, path, cancel=False, button_label=None):
        with ThreadPoolExecutor(max_workers=1) as pool:
            picker = pool.submit(self.native.choose_file, kind, None if cancel else path)
            self.web.button(button_label or ("Open…" if kind == "open" else "Save"))
            if kind == "open":
                wait(lambda: picker.done() or self.web.has(".document-guard"), "native Open picker or unsaved guard")
                if self.web.has(".document-guard"):
                    self.web.button("Discard & continue")
            evidence = picker.result(timeout=35)
        self.record(f"Native {kind} {'cancel' if cancel else 'choose file'}", evidence)

    def shogi_square(self, square):
        return f'.shogi-square[data-square="{square}"]'

    def shogi_events(self, count, selected=None, historical=False):
        wait(lambda: self.web.read("document.querySelector('.shogi-application')?.dataset.liveEvents") == str(count),
             f"native shogi history has {count} events")
        if selected is not None:
            wait(lambda: self.web.read("document.querySelector('.shogi-application')?.dataset.currentEvent") == str(selected),
                 f"native shogi selected event {selected}")
        wait(lambda: self.web.read("document.querySelector('.shogi-application')?.dataset.historical") == str(historical).lower(),
             f"shogi historical viewing {historical}")

    def shogi_save(self, path, observation_count=None):
        self.file("save_as", path, button_label="Save as…")
        wait(lambda: self.web.text(".document-dirty") == "Saved", "complete shogi document saved")
        wait(path.exists, "native shogi Save As created exact path")
        archive = json.loads(path.read_text(encoding="utf-8"))
        assert archive["schema_version"] == 2 and archive["format"] == "mshogi.desktop.archive.v2", archive
        assert archive["game_kind"] == "shogi" and archive["rules_profile"] == "mshogi.shogi.csa36.v1", archive
        body = archive["payload"]
        assert body["current"]["game_kind"] == "shogi"
        if observation_count is not None:
            assert len(body["observations"]) == observation_count
        self.record("Inspect complete native shogi archive", {
            "path": str(path), "sha256": digest(path), "bytes": path.stat().st_size,
            "observations": len(body["observations"]), "controls": len(body["controls"]),
            "native_events": len(body["current"]["payload"]["events"]),
            "root": body["current"]["payload"].get("root")})
        return archive

    def shogi_analysis(self, depth=1, nodes=1000):
        self.web.set('.shogi-analysis input[name="depth"]', depth)
        self.web.set('.shogi-analysis input[name="nodes"]', nodes)
        self.web.click('[data-analysis-start="shogi"]')

    def shogi_complete_analysis(self):
        wait(lambda: "Analysis complete" in self.web.text(".shogi-analysis-state"), "actual read-only shogi analysis completion", 45)
        assert "No move or declaration was played" in self.web.text(".shogi-analysis")
        assert "perspective" in self.web.text(".analysis-perspective")

    def shogi_flip(self):
        previous = self.web.read("document.querySelector('.shogi-application')?.dataset.orientation")
        self.web.click(".shogi-flip")
        wait(lambda: self.web.read("document.querySelector('.shogi-application')?.dataset.orientation") != previous,
             "native shogi orientation changes")

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
        guard = '[aria-labelledby="synthetic-close-title"]' if synthetic else ".document-guard"
        wait(lambda: self.web.has(guard), "native quit reaches its specific application guard")
        label = "Close fixture" if synthetic else "Discard & continue"
        element = self.web.find(f"//button[normalize-space(.)='{label}']", "xpath")
        self.record("Dispatch final native-close guard action", {"label": label, "element": element})
        try:
            self.web.command("POST", f"/element/{element}/click", {})
        except Exception as error:
            if not terminal_transport_closed(error):
                raise
            self.record("Final guard click closed its WebDriver transport", {
                "error": str(error), "qualification": "Not exit proof; native host/engine absence is still required before teardown."})
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

    def new_shogi(self, side):
        w = self.web
        w.button("New game")
        wait(lambda: w.has('.document-guard') or w.has('[aria-label="New game"]'), "New Game guard or explicit game chooser")
        if w.has(".document-guard"):
            w.button("Discard & continue")
        wait(lambda: w.has('[aria-label="New game"]'), "explicit game selection dialog")
        w.click('[data-game="shogi"]')
        w.click(f'[data-side="{side}"]')
        w.click("[data-start-game]")
        wait(lambda: w.has(".shogi-application") and not w.has('[aria-label="New game"]'), "new native shogi product game")
        wait(lambda: w.read("document.querySelector('.shogi-application')?.dataset.orientation") == side,
             f"human {side} at bottom")
        self.record("Select Shogi and human side in normal New Game", {"side": side, "board_name":
            w.read("document.querySelector('.shogi-board')?.getAttribute('aria-label')")})

    def normal_shogi(self):
        self.start(product="shogi")
        w = self.web
        self.new_shogi("sente")
        self.viewport(1180, 820); self.shogi_board_fits()
        self.shogi_events(0)
        self.capture("initial-sente")
        self.viewport(760, 650); self.shogi_board_fits()
        self.capture("minimum-size")
        self.viewport(1180, 820)
        self.new_shogi("gote")
        self.shogi_events(1)
        self.shogi_board_fits()
        assert "Gote at bottom" in w.read("document.querySelector('.shogi-board').getAttribute('aria-label')")
        self.capture("initial-gote")
        self.new_shogi("sente")
        self.shogi_events(0)
        w.click(self.shogi_square("7g"))
        assert w.read(f"document.querySelector({json.dumps(self.shogi_square('7f'))}).classList.contains('target')")
        w.key("\ue00c")  # Escape retains board history and restores origin focus.
        self.shogi_events(0)
        assert w.read("document.activeElement?.dataset.square") == "7g"
        for key in ("\ue007", "\ue013", "\ue007"):  # Enter, ArrowUp, Enter.
            w.key(key)
        self.shogi_events(2)
        wait(lambda: w.has('[data-search-purpose="engine_turn"]'), "actual bundled shogi engine observation")
        self.capture("play")
        self.record("Pointer selection / Escape / keyboard 7g7f / actual engine reply", w.text(".shogi-history"))

        self.shogi_analysis(depth=4, nodes=20_000)
        wait(lambda: w.has('.shogi-analysis[data-phase="analyzing"]'), "actual active shogi analysis")
        self.capture("active-analysis")
        w.button("Cancel analysis")
        wait(lambda: "Search stopped" in w.text(".shogi-analysis-state"), "cancel active shogi analysis")
        self.shogi_events(2)
        self.shogi_analysis(depth=4, nodes=20_000)
        wait(lambda: w.has('.shogi-analysis[data-phase="analyzing"]'), "actual shogi analysis before replacement")
        w.set('.shogi-analysis input[name="depth"]', 1)
        w.set('.shogi-analysis input[name="nodes"]', 1000)
        assert w.has('.shogi-analysis[data-phase="analyzing"]'), "original work completed before replacement input; no active-replacement evidence"
        w.click('[data-analysis-start="shogi"]')
        self.shogi_complete_analysis()
        self.shogi_events(2)
        self.capture("read-only-analysis")
        self.record("Cancel and replace actual bounded read-only shogi analysis", w.text(".shogi-analysis"))
        self.shogi_save(self.args.output / "normal-shogi-complete.mshogi")
        self.shogi_flip()
        self.record("Native shogi window-close request", self.native.close_window())
        wait(lambda: w.has(".document-guard"), "native close protects shogi unsaved preferences")
        self.capture("native-close-guard")
        w.button("Cancel")
        wait(lambda: not w.has(".document-guard"), "cancel native shogi window close")
        assert self.native.alive()
        self.shogi_events(2)
        self.quit()

    def normal_shogi_reopened(self):
        self.start(product="shogi-reopened")
        w = self.web
        self.file("open", self.args.output / "normal-shogi-complete.mshogi")
        wait(lambda: w.has(".shogi-application"), "fresh normal Chess launch dispatches shogi archive")
        self.viewport(1180, 820); self.shogi_board_fits()
        self.shogi_events(2)
        assert w.read("document.querySelector('.shogi-application').dataset.paused") == "true"
        assert w.has(".play-paused") and not w.has('.shogi-analysis[data-phase="analyzing"]')
        w.click(".recorded-searches summary")
        assert "Saved observation" in w.text(".recorded-searches")
        w.click(".recorded-searches summary")
        self.capture("reopened-stopped")
        w.click('.shogi-history [data-event="1"]')
        self.shogi_events(2, selected=1, historical=True)
        assert "live game is unchanged" in w.text(".shogi-history-banner")
        self.shogi_analysis()
        self.shogi_complete_analysis()
        self.shogi_events(2, selected=1, historical=True)
        self.capture("historical-analysis")
        archive = self.shogi_save(self.args.output / "normal-shogi-history.mshogi")
        body = archive["payload"]
        context = body["observations"][-1]["response"]["context"]
        assert context["purpose"]["kind"] == "move_analysis"
        assert len(context["game"]["payload"]["events"]) == 1
        assert len(body["current"]["payload"]["events"]) == 2
        self.record("Saved historical analysis uses exact native history prefix", context)
        w.button("Return to live")
        self.shogi_events(2, selected=2)
        assert w.has(".play-paused")
        w.button("Resume play")
        wait(lambda: w.read("document.querySelector('.shogi-application').dataset.paused") == "false", "explicit resumed shogi live play")
        self.shogi_events(2)
        assert not w.has(".shogi-history-banner")
        self.capture("return-live-resumed")
        self.quit()

    def shogi_fixture(self, name):
        """Difficult native states, explicitly separate from normal packages."""
        self.start(name)
        w = self.web
        self.viewport(1180, 820)
        self.shogi_board_fits()
        assert not w.has(".shared-contract-application")
        if name == "shogi-hands":
            self.shogi_events(2)
            for side in ("sente", "gote"):
                assert "1" in w.text(f'.shogi-player[data-player="{side}"] .hand-piece[data-hand="pawn"]')
            self.capture("both-hands")
            w.click('.shogi-player[data-player="sente"] .hand-piece[data-hand="pawn"]')
            assert w.read(f"document.querySelector({json.dumps(self.shogi_square('5e'))}).classList.contains('target')")
            self.shogi_events(2)
            self.capture("drop-selection")
            w.key("\ue00c")  # Escape cancels without a native action.
            self.shogi_events(2)
            assert not w.has(".hand-piece.selected")
            # Exercise a genuine native process failure at a complete action,
            # retaining the same imported root and capture history for retry.
            engine = self.args.verification_engine
            held = engine.with_name(engine.name + ".gui-verification-held")
            original = digest(engine)
            assert not held.exists()
            try:
                engine.rename(held)
                w.click('.shogi-player[data-player="sente"] .hand-piece[data-hand="pawn"]')
                w.click(self.shogi_square("5e"))
                wait(lambda: "Engine connection failed" in w.text(".shogi-sidebar"), "genuine shogi bundled engine startup failure")
                self.shogi_events(3)
                assert "No game result is implied" in w.text(".shogi-sidebar")
                assert "Sente pawn" in w.read(f"document.querySelector({json.dumps(self.shogi_square('5e'))}).getAttribute('aria-label')")
                self.capture("engine-failure")
            finally:
                if held.exists():
                    held.rename(engine)
            assert digest(engine) == original
            w.button("Retry engine turn")
            self.shogi_events(4)
            self.record("Capture history / cancelled drop / complete native drop / process recovery", {
                "engine_sha256": original, "history": w.text(".shogi-history"),
                "status": w.text("[data-game-status]")})
        elif name == "shogi-promotion":
            self.shogi_events(0)
            w.click(self.shogi_square("5d")); w.click(self.shogi_square("5c"))
            wait(lambda: w.has(".shogi-promotion"), "optional promotion has two complete native choices")
            self.shogi_events(0)
            self.capture("optional-promotion")
            w.key("\ue00c")
            wait(lambda: not w.has(".shogi-promotion"), "cancel optional promotion")
            self.shogi_events(0)
            w.click(self.shogi_square("5d")); w.click(self.shogi_square("5c"))
            w.button("Promote")
            wait(lambda: "promoted pawn" in str(w.read(f"document.querySelector({json.dumps(self.shogi_square('5c'))})?.getAttribute('aria-label')")), "native promoted piece appears")
            self.shogi_events(2)
        elif name == "shogi-forced":
            self.shogi_events(0)
            w.click(self.shogi_square("5b")); w.click(self.shogi_square("5a"))
            wait(lambda: "promoted pawn" in str(w.read(f"document.querySelector({json.dumps(self.shogi_square('5a'))})?.getAttribute('aria-label')")), "mandatory native promotion commits directly")
            assert not w.has(".shogi-promotion")
            self.shogi_events(2)
        elif name == "shogi-declaration":
            self.shogi_events(0)
            assert "Eligibility alone does not conclude" in w.text(".shogi-game-actions")
            w.button("Declare entering king…")
            wait(lambda: w.has('[aria-label="Declare entering king?"]'), "formal declaration confirmation")
            self.shogi_events(0)
            self.capture("declaration-confirmation")
            w.button("Confirm declaration")
            self.shogi_events(1)
            wait(lambda: "Sente wins by entering-king declaration" in w.text("[data-game-status]"), "authoritative declaration conclusion")
            assert not w.has(".shogi-game-actions button")
        elif name == "shogi-concluded":
            self.shogi_events(1)
            assert "Sente resigned" in w.text("[data-game-status]")
            assert not w.has(".shogi-game-actions button")
            self.capture("concluded")
            w.click('.shogi-history [data-event="0"]')
            self.shogi_events(1, selected=0, historical=True)
            self.capture("before-terminal-event")
            w.click('.shogi-history [data-event="1"]')
            self.shogi_events(1, selected=1, historical=True)
            w.button("Return to live")
            self.shogi_events(1, selected=1)
            assert "Sente resigned" in w.text("[data-game-status]")
            self.record("Select distinct native terminal event at unchanged board ply", w.text(".shogi-history"))
        elif name == "shogi-capacity":
            assert "Recorded searches (255)" in w.text(".recorded-searches")
            self.shogi_analysis(nodes=0)
            wait(lambda: "Recorded searches (256)" in w.text(".recorded-searches"), "shogi observation256 accepted")
            self.shogi_analysis(nodes=0)
            wait(lambda: "Application limit reached" in w.text(".capacity-notice"), "shogi observation257 refused before commitment")
            assert "Recorded searches (256)" in w.text(".recorded-searches")
            assert "Saving and reopening does not reset" in w.text(".capacity-notice")
            self.shogi_events(0)
            self.capture("capacity-pause")
            self.shogi_save(self.args.output / "shogi-capacity-complete.mshogi", 256)
            self.shogi_flip()
        else:
            raise AssertionError(name)
        self.capture("result")
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
            wait(lambda: w.read("""(() => {
                const notice = document.querySelector('.capacity-notice');
                const sidebar = document.querySelector('.sidebar');
                if (!notice || !sidebar || document.activeElement !== notice) return false;
                const rect = notice.getBoundingClientRect(), box = sidebar.getBoundingClientRect();
                return rect.top >= Math.max(0, box.top) && rect.bottom <= Math.min(innerHeight, box.bottom);
            })()"""), "new capacity pause is focused and visible in the actual inspector")
            assert "observations (256)" in w.text(".saved-observations")
            assert not w.has(".outcome-card")
            self.capture("capacity-pause")
            self.save(self.args.output / "capacity-complete.mshogi", 256)
            # A preference change keeps this temporary saved document dirty for
            # the native lifecycle guard; it does not remove any observation.
            w.click(".flip-button")
            wait(lambda: w.text(".document-dirty") == "Unsaved", "protected preference change completes before Quit")
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
    parser.add_argument("--phase", choices=["normal", "normal-shogi", "verification", "verification-shogi", "all"], default="all")
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
        chess_verification = ["standard", "promotion", "claim", "capacity", "synthetic"]
        shogi_verification = ["shogi-hands", "shogi-promotion", "shogi-forced", "shogi-declaration", "shogi-concluded", "shogi-capacity"]
        shogi_normal = ["normal-shogi", "normal-shogi-reopened"]
        phases = ([None, *shogi_normal] if args.phase == "normal" else
                  shogi_normal if args.phase == "normal-shogi" else
                  [*chess_verification, *shogi_verification] if args.phase == "verification" else
                  shogi_verification if args.phase == "verification-shogi" else
                  [None, *shogi_normal, *chess_verification, *shogi_verification])
        failures = []
        report["phase_failures"] = failures
        for fixture in phases:
            journey.phase_report = None
            try:
                if fixture is None:
                    journey.normal()
                elif fixture == "normal-shogi":
                    journey.normal_shogi()
                elif fixture == "normal-shogi-reopened":
                    journey.normal_shogi_reopened()
                elif fixture.startswith("shogi-"):
                    journey.shogi_fixture(fixture)
                else:
                    journey.fixture(fixture)
            except Exception as phase_error:
                failure = {"phase": "normal" if fixture is None else fixture if fixture.startswith("normal-") else f"verification-{fixture}",
                           "error": str(phase_error), "traceback": traceback.format_exc()}
                failures.append(failure)
                if journey.phase_report is not None:
                    journey.phase_report.update(status="failed", error=str(phase_error),
                                                traceback=failure["traceback"])
                else:
                    report["phases"].append({**failure, "status": "failed"})
                try:
                    journey.capture("failure")
                except Exception as capture_error:
                    failure["capture_error"] = str(capture_error)
            finally:
                # Each phase owns fresh native processes, a driver and display
                # adapter. Finish failure cleanup before attempting the next;
                # cleanup is never graceful-shutdown evidence. A cleanup error
                # still aborts the run because isolation was not established.
                journey.cleanup()
        if failures:
            raise AssertionError("Required GUI phases failed: " +
                                 "; ".join(f"{item['phase']}: {item['error']}" for item in failures))
        report["status"] = "passed"
    except Exception as error:
        report.update(status="failed", error=str(error), traceback=traceback.format_exc())
        raise
    finally:
        cleanup_failure = None
        if journey is not None:
            try:
                journey.cleanup()
            except Exception as error:
                cleanup_failure = error
                report.update(status="failed", cleanup_error=str(error),
                              cleanup_traceback=traceback.format_exc())
            report["native_events"] = journey.native_events
        report["finished_at_utc"] = stamp()
        report["files"] = {str(path.relative_to(args.output)): digest(path)
            for path in sorted(args.output.rglob("*")) if path.is_file() and path.name != "gui-report.json"}
        (args.output / "gui-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
        if cleanup_failure is not None:
            raise RuntimeError("GUI failure cleanup did not complete; see retained report") from cleanup_failure


if __name__ == "__main__":
    main()
