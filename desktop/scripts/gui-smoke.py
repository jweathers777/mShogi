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
import re
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


def desktop_document_payload(document, game_kind):
    """Inspect the one current native envelope; this is not a replay validator."""
    profiles = {
        "chess": ("mshogi.chess.match.v1", "FIDE-2023-01-01+mshogi.chess.match.v1"),
        "shogi": ("mshogi.shogi.csa36.v1", "JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026"),
        "xiangqi": ("mshogi.xiangqi.wxf2018.v1", "WXF-2018+XIANGQI-P1-2026-10-27"),
    }
    profile, edition = profiles[game_kind]
    assert set(document) == {
        "envelope_version", "format", "game_kind", "rules_profile",
        "profile_edition", "native_schema_version", "payload"}, document
    assert document["envelope_version"] == 1, document
    assert document["format"] == "mshogi.desktop.document.v1", document
    assert document["game_kind"] == game_kind, document
    assert document["rules_profile"] == profile and document["profile_edition"] == edition, document
    assert document["native_schema_version"] == 1, document
    assert isinstance(document["payload"], dict), document
    return document["payload"]


def xiangqi_retained_state(diagnostics):
    """Compare accepted native document state, excluding file-operation receipts.

    Snapshot revision and I/O status may advance to report a rejected Open. The
    document/session/selection authority and complete native semantics may not.
    This observer does not replay, classify, or reconstruct Xiangqi rules.
    """
    assert diagnostics["profile"] == "mshogi.xiangqi.wxf2018.v1", diagnostics
    context = diagnostics["context"]
    assert context["game_kind"] == "xiangqi", context
    assert context["rules_profile"] == diagnostics["profile"], context
    token = diagnostics["token"]
    return json.loads(json.dumps({
        "authority": {key: token[key] for key in ("document", "version", "selection")},
        "context": context, "xiangqi": diagnostics["xiangqi"],
        "document": diagnostics["document"], "observations": diagnostics["observations"],
    }))


def selected_gui_phases(choice):
    """Keep independent native-process journeys explicit and individually runnable."""
    chess = ["standard", "promotion", "claim", "capacity", "synthetic"]
    shogi = ["shogi-hands", "shogi-promotion", "shogi-forced", "shogi-declaration",
             "shogi-concluded", "shogi-capacity"]
    normal = [None, "normal-shogi", "normal-shogi-reopened", "normal-xiangqi"]
    return {"normal": normal, "normal-shogi": normal[1:3],
            "normal-xiangqi": ["normal-xiangqi"],
            "verification": [*chess, *shogi], "verification-shogi": shogi,
            "all": [*normal, *chess, *shogi]}[choice]


def saved_document_accepted(observation, expected_path):
    """One DOM observation must establish native acceptance, not an old Saved label."""
    return (observation["path"] == str(expected_path)
            and observation["dirty"].casefold() == "unmodified"
            and observation["buttons"] == [
                {"label": label, "enabled": True}
                for label in ("New game", "Save", "Save as…", "Open…")]
            and observation.get("io", {}).get("phase") == "complete"
            and observation["io"].get("id", 0) > observation.get("after_io_id", 0)
            and not any(word in status.casefold()
                        for status in observation["statuses"]
                        for word in ("saving", "opening")))


def opened_document_accepted(observation, expected_path):
    """Require the public native Open receipt as well as settled saved state."""
    return (saved_document_accepted(observation, expected_path)
            and observation.get("io", {}).get("phase") == "complete"
            and observation["io"].get("id", 0) > observation.get("after_io_id", 0)
            and (observation["io"].get("message") or "").startswith("Opened ")
            and "work stopped" in observation["io"]["message"])


def play_limit_matches(text, depth, nodes):
    """Compare actual displayed integers without imposing a WebView locale."""
    match = re.fullmatch(r"Play limit: depth ([0-9]+) · ([0-9,. \u00a0\u202f]+) nodes", text or "")
    return bool(match and int(match[1]) == depth
                and int(re.sub(r"[,. \u00a0\u202f]", "", match[2]) or "-1") == nodes)


def inspection_status_matches(text, native_label):
    """Exact owner-approved visible copy, independent of accessible explanations."""
    return text == f"Inspecting {native_label} · Esc clears"


def completed_action_matches(observed, action, event, origins, destinations, route=None):
    """Validate observed native geometry, never derive moves from board rules."""
    return (isinstance(observed, dict) and observed.get("action") == action
            and observed.get("event") == str(event)
            and observed.get("hidden") == "true" and observed.get("pointerEvents") == "none"
            and observed.get("origins") == origins and observed.get("destinations") == destinations
            and observed.get("destinationSignatures") == ["double-underline"] * len(destinations)
            and observed.get("route") == route)


def moves_controls_settled(observed, event, orientation=None, changed_from=None):
    """Selection alone can render before native capacity admission completes."""
    return (isinstance(observed, dict) and observed.get("event") == str(event)
            and observed.get("rootEnabled") is True and observed.get("flipEnabled") is True
            and observed.get("targetEnabled") is True and observed.get("checkingCapacity") is False
            and isinstance(observed.get("orientation"), str)
            and bool(observed["orientation"])
            and (orientation is None or observed["orientation"] == orientation)
            and (changed_from is None or observed["orientation"] != changed_from))


def focus_outline_visible(observed):
    """Require the actual focused outline inside every scroll clip and viewport."""
    if not isinstance(observed, dict) or not observed.get("focused") or not observed.get("focusVisible"):
        return False
    if observed.get("outlineStyle") in (None, "none", "hidden") or observed.get("outlineWidth", 0) <= 0:
        return False
    rect, clip = observed.get("outline", {}), observed.get("clip", {})
    return (all(isinstance(rect.get(k), (float, int)) and isinstance(clip.get(k), (float, int))
                for k in ("left", "right", "top", "bottom"))
            and rect["left"] >= clip["left"] - .5 and rect["right"] <= clip["right"] + .5
            and rect["top"] >= clip["top"] - .5 and rect["bottom"] <= clip["bottom"] + .5)


def historical_notice_matches(observed, event):
    """Validate the current shared copy together with actual native selection."""
    return (isinstance(observed, dict) and observed.get("heading") == "Historical position"
            and observed.get("meaning") == "Your live game is unchanged."
            and observed.get("historical") == "true" and observed.get("event") == str(event)
            and observed.get("returnLiveEnabled") is True)


def saved_chess_evidence_count(text, count):
    """Require the current player-facing summary and exact accepted count."""
    return text == f"Saved analysis evidence ({count})"


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
        except AssertionError as error:
            last = str(error)
        time.sleep(0.1)
    raise AssertionError(f"Timed out: {label}; last observation: {last!r}")


class Webview:
    def __init__(self, endpoint, host, record=None):
        self.endpoint = endpoint
        self.session = None
        self.record = record
        response = self.request("POST", "/session", {"capabilities": {
            "alwaysMatch": {"browserName": "wry", "tauri:options": {
                "application": str(host)}}}})
        self.session = response["sessionId"]
        self.capabilities = response.get("capabilities", {})

    def request(self, method, path, data=None):
        lookup = (self.session is not None and method == "POST" and path in {
            f"/session/{self.session}/element", f"/session/{self.session}/elements"})
        for attempt in range(2 if lookup else 1):
            try:
                result = self._request_once(method, path, data)
            except (http.client.RemoteDisconnected, ConnectionResetError, urllib.error.URLError) as error:
                if lookup and self.record is not None:
                    self.record("Read-only element lookup transport observation", {
                        "method": method, "path": path, "attempt": attempt + 1,
                        "error": repr(error), "retry": attempt == 0})
                if not lookup or attempt == 1:
                    raise
            else:
                if attempt == 1 and self.record is not None:
                    self.record("Read-only element lookup retry recovered", {
                        "method": method, "path": path, "attempt": 2})
                return result

    def _request_once(self, method, path, data=None):
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
        if selector.startswith((".square[", ".shogi-square[", ".xiangqi-point[")):
            # WebKit's element-click heuristic requires the SVG group itself
            # in the hit-test tree, although only its painted children occur
            # there. Observe its real geometry, then send actual pointer input.
            attempts = 0
            def ready_target():
                nonlocal attempts
                attempts += 1
                target = self.read("""(() => {
                    const element = document.querySelector(SELECTOR);
                    const context = {label: element?.getAttribute('aria-label') ?? null,
                        runtime: document.querySelector('.shell-runtime')?.textContent ?? null,
                        heading: document.querySelector('.shell-live-heading')?.textContent ?? null,
                        squareCount: document.querySelectorAll('.square,.shogi-square,.xiangqi-point').length};
                    if (!element) return {...context, present: false, enabled: false};
                    const enabled = element.getAttribute('aria-disabled') !== 'true';
                    const rect = element.getBoundingClientRect();
                    const left = Math.max(0, rect.left), right = Math.min(innerWidth, rect.right);
                    const top = Math.max(0, rect.top), bottom = Math.min(innerHeight, rect.bottom);
                    const x = Math.floor((left + right) / 2), y = Math.floor((top + bottom) / 2);
                    const hit = document.elementFromPoint(x, y);
                    return {...context, present: true, enabled, x, y,
                        visible: right > left && bottom > top,
                        hit: hit?.tagName, containsHit: hit !== null && element.contains(hit)};
                })()""".replace("SELECTOR", json.dumps(selector)))
                ready = (target["present"] and target["enabled"] and
                         target["visible"] and target["containsHit"])
                if self.record is not None and (attempts == 1 or ready):
                    self.record("Observe board pointer precondition", {
                        "selector": selector, "attempt": attempts, "ready": ready,
                        "target": target})
                assert ready, {"selector": selector, "attempt": attempts, "target": target}
                return target
            # Native operation admission may still be settling when a board
            # renders. Poll read-only target readiness within the existing
            # wait budget; dispatch pointer input exactly once, never in wait().
            target = wait(ready_target, f"board pointer target ready: {selector}")
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

    def focused_button(self, label, max_tabs=64):
        """Reach a clipped control with real keyboard navigation, then click once.

        WebKit element-click need not reveal controls inside a nested inspector.
        Read-only DOM observations establish unique/enabled/focused/visible state;
        only actual W3C Tab input may reveal it. No scroll/focus JS or IPC occurs.
        A failed final click is propagated without replaying any action.
        """
        assert isinstance(max_tabs, int) and max_tabs >= 0

        def observe():
            return self.read("""(() => {
                const matches=[...document.querySelectorAll('button')].filter(e=>e.textContent.trim()===LABEL);
                const e=matches.length===1?matches[0]:null;
                if(!e)return {matches:matches.length,enabled:false,focused:false,visible:false};
                const r=e.getBoundingClientRect();
                const clip={left:0,top:0,right:innerWidth,bottom:innerHeight};
                for(let p=e.parentElement;p;p=p.parentElement){
                    const c=getComputedStyle(p),b=p.getBoundingClientRect();
                    if(/^(auto|scroll|hidden|clip)$/.test(c.overflowX)){clip.left=Math.max(clip.left,b.left);clip.right=Math.min(clip.right,b.right);}
                    if(/^(auto|scroll|hidden|clip)$/.test(c.overflowY)){clip.top=Math.max(clip.top,b.top);clip.bottom=Math.min(clip.bottom,b.bottom);}
                }
                const hit=document.elementFromPoint((r.left+r.right)/2,(r.top+r.bottom)/2);
                return {matches:1,enabled:!e.disabled,focused:document.activeElement===e,
                    visible:r.width>0&&r.height>0&&r.left>=clip.left-1&&r.top>=clip.top-1&&r.right<=clip.right+1&&r.bottom<=clip.bottom+1&&!!hit&&e.contains(hit),
                    rect:{left:r.left,top:r.top,right:r.right,bottom:r.bottom},clip,
                    activeLabel:document.activeElement?.getAttribute('aria-label')??document.activeElement?.textContent?.trim()};
            })()""".replace("LABEL", json.dumps(label)))

        for tabs in range(max_tabs + 1):
            observed = observe()
            if self.record is not None:
                self.record("Observe keyboard control precondition", {"label": label, "tabs": tabs, "target": observed})
            assert observed["matches"] == 1 and observed["enabled"], {"label": label, "tabs": tabs, "target": observed}
            if observed["focused"]:
                if not observed["visible"]:
                    def revealed():
                        state = observe()
                        assert state["matches"] == 1 and state["enabled"] and state["focused"], state
                        return state if state["visible"] else None
                    observed = wait(revealed, f"keyboard-focused control revealed: {label}", 2)
                if self.record is not None:
                    self.record("Keyboard control focused and visible before one click", {"label": label, "tabs": tabs, "target": observed})
                self.button(label)
                return
            if tabs < max_tabs:
                self.key("\ue004")
        raise AssertionError(f"Control unreachable within {max_tabs} actual Tab inputs: {label}; last observation: {observed!r}")

    def keyboard_activate(self, selector, max_tabs=64):
        """Actual bounded Tab/Enter; DOM reads never reveal or focus a target."""
        def observe():
            return self.read("""(() => {
                const matches=[...document.querySelectorAll(SELECTOR)],e=matches.length===1?matches[0]:null;
                if(!e)return {matches:matches.length,enabled:false,focused:false,visible:false};
                const r=e.getBoundingClientRect(),clip={left:0,top:0,right:innerWidth,bottom:innerHeight};
                for(let p=e.parentElement;p;p=p.parentElement){const c=getComputedStyle(p),b=p.getBoundingClientRect();
                    if(/^(auto|scroll|hidden|clip)$/.test(c.overflowX)){clip.left=Math.max(clip.left,b.left);clip.right=Math.min(clip.right,b.right);}
                    if(/^(auto|scroll|hidden|clip)$/.test(c.overflowY)){clip.top=Math.max(clip.top,b.top);clip.bottom=Math.min(clip.bottom,b.bottom);}}
                return {matches:1,enabled:!e.disabled,focused:document.activeElement===e,
                    visible:r.width>0&&r.height>0&&r.left>=clip.left-1&&r.top>=clip.top-1&&r.right<=clip.right+1&&r.bottom<=clip.bottom+1};
            })()""".replace("SELECTOR", json.dumps(selector)))
        for tabs in range(max_tabs + 1):
            observed = observe()
            assert observed["matches"] == 1 and observed["enabled"], (selector, observed)
            if observed["focused"]:
                if not observed["visible"]:
                    def revealed():
                        state = observe()
                        assert state["matches"] == 1 and state["enabled"] and state["focused"], state
                        return state if state["visible"] else None
                    observed = wait(revealed, f"keyboard-focused move target revealed: {selector}", 2)
                if self.record is not None:
                    self.record("Actual Tab reaches visible target before one Enter", {"selector": selector, "tabs": tabs, "target": observed})
                self.key("\ue007")
                return
            if tabs < max_tabs:
                self.key("\ue004")
        raise AssertionError(f"Target unreachable within {max_tabs} actual Tabs: {selector}; {observed!r}")

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
        observed = self.read(f"""(() => {{ const e=document.querySelector({json.dumps(selector)});
            return e ? {{type:e.type,text:e.value,number:e.type==='number'?e.valueAsNumber:null}} : null;
        }})()""")
        # HTML number controls may retain a leading zero during real typing
        # after React renders Number('') as zero. Assert their actual submitted
        # numeric meaning rather than treating '04' as a different depth.
        assert observed is not None and (
            observed["number"] == float(value) if observed["type"] == "number"
            else observed["text"] == str(value)), (selector, value, observed)

    def key(self, key):
        self.command("POST", "/actions", {"actions": [{"type": "key", "id": "keyboard",
            "actions": [{"type": "keyDown", "value": key},
                        {"type": "keyUp", "value": key}]}]})

    def read(self, expression):
        # Callers supply DOM observations only. Generic script execution through
        # command() stays single-attempt, as do all input and native mutations.
        return self._observe("POST", "/execute/sync", {
            "script": "return " + expression, "args": []})

    def _observe(self, method, path, data=None):
        if (method, path) not in {("POST", "/execute/sync"), ("GET", "/screenshot")}:
            raise ValueError("Unsupported read-only observation")
        for attempt in range(2):
            try:
                result = self.command(method, path, data)
            except (http.client.RemoteDisconnected, ConnectionResetError, urllib.error.URLError) as error:
                if self.record is not None:
                    self.record("Read-only observation transport failure", {
                        "method": method, "path": path, "attempt": attempt + 1,
                        "error": repr(error), "retry": attempt == 0})
                if attempt == 1:
                    raise
            else:
                if attempt == 1 and self.record is not None:
                    self.record("Read-only observation retry recovered", {
                        "method": method, "path": path, "attempt": 2})
                return result

    def text(self, selector="body"):
        return self.read(f"document.querySelector({json.dumps(selector)})?.innerText ?? ''")

    def has(self, selector):
        return self.read(f"document.querySelector({json.dumps(selector)}) !== null")

    def screenshot(self, path):
        path.write_bytes(base64.b64decode(self._observe("GET", "/screenshot")))

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
        self.shell_layouts = {}

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
        common_actions = self.web.read("""[...document.querySelectorAll('button')]
            .filter(e=>['Cancel analysis','Return to live'].includes(e.textContent.trim()))
            .map(e=>{const h=e.closest('.shell-history-notice-heading'),a=e.closest('.shell-analysis-actions'),s=a?.querySelector('[data-analysis-start]');return {
                label:e.textContent.trim(),section:e.closest('[data-shell-section]')?.dataset.shellSection??null,
                analysisActionRow:!!a,sameRowWithStart:s?Math.abs(s.getBoundingClientRect().top-e.getBoundingClientRect().top)<=1:null,
                historyHeading:!!h,rightGap:h?h.getBoundingClientRect().right-e.getBoundingClientRect().right:null};})""")
        self.record('Observe shared analysis and historical action slots', common_actions)
        for action in common_actions:
            if action['label'] == 'Cancel analysis':
                assert action['section'] == 'analysis' and action['analysisActionRow'] and action['sameRowWithStart'], action
            else:
                assert action['historyHeading'] and abs(action['rightGap']) <= 3, action
        prefix.with_suffix(".dom.json").write_text(json.dumps({
            "text": self.web.text(), "focus": self.web.read(
                "({tag:document.activeElement?.tagName,label:document.activeElement?.getAttribute('aria-label'),text:document.activeElement?.textContent})"),
            "dimensions": self.web.read("({width:innerWidth,height:innerHeight,dpr:devicePixelRatio})"),
            "common_actions": common_actions,
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
        self.web = Webview(endpoint, host, self.record)
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
        text = f"{'Selected' if historical else 'Live'} position · {count} {'ply' if count == 1 else 'plies'}"
        wait(lambda: text in self.web.text(".game-footnote"), text)

    def square(self, name):
        return f'.square[aria-label^="{name},"]'

    def completed_cue(self):
        return self.web.read("""(() => {const e=document.querySelector('.completed-action-cue');if(!e)return null;
            return {action:e.dataset.nativeLastAction,event:e.dataset.lastEvent,hidden:e.getAttribute('aria-hidden'),
                pointerEvents:getComputedStyle(e).pointerEvents,
                origins:[...e.querySelectorAll('[data-last-move="origin"]')].map(p=>p.dataset.location),
                destinations:[...e.querySelectorAll('[data-last-move="destination"]')].map(p=>p.dataset.location),
                destinationSignatures:[...e.querySelectorAll('[data-last-move="destination"]')].map(p=>p.dataset.cueSignature),
                route:e.querySelector('.completed-action-route')?.dataset.route??null};})()""")

    def assert_completed_cue(self, action, event, origins, destinations, route=None):
        def observed():
            cue = self.completed_cue()
            assert completed_action_matches(cue, action, event, origins, destinations, route), cue
            return cue
        cue = wait(observed, f"exact native completed cue {action} at event {event}")
        self.record("Observe readonly completed action geometry", cue)
        return cue

    def wait_moves_controls(self, event, selector=".moves-root", orientation=None, changed_from=None):
        def observed():
            state = self.web.read("""(() => {const root=document.querySelector('.moves-root'),flip=document.querySelector('.flip-button'),
                target=document.querySelector(SELECTOR);
                return {event:document.querySelector('.moves-panel')?.dataset.currentEvent,
                    orientation:flip?.getAttribute('aria-label'),rootEnabled:!!root&&!root.disabled,
                    flipEnabled:!!flip&&!flip.disabled,targetEnabled:!!target&&!target.disabled,
                    checkingCapacity:[...document.querySelectorAll('[role="status"]')].some(e=>e.textContent.includes('Checking complete save capacity'))};})()""".replace("SELECTOR", json.dumps(selector)))
            assert moves_controls_settled(state, event, orientation, changed_from), state
            return state
        state = wait(observed, f"native Moves selection/controls settled at event {event}")
        self.record("Observe admitted Moves controls before next real input", state)
        return state

    def flip_moves_board(self, event, selector, orientation=None):
        before = self.wait_moves_controls(event, selector)
        self.web.click(".flip-button")
        return self.wait_moves_controls(event, selector, orientation, before["orientation"])

    def moves_navigation(self, game, action, notation, event, origin, destination):
        """Actual shared Moves pointer/Tab/Enter at both supported sizes."""
        w = self.web
        selector = f'.moves-table button[data-native-action="{action}"][data-event="{event}"]'
        wait(lambda: w.has(selector), f"native {game} standard notation target")
        assert w.text(selector).strip() == notation, (game, w.text(selector))
        live = self.completed_cue()
        assert live and live["action"] != action and live["event"] != str(event), live
        live_event = w.read("document.querySelector('.moves-panel').dataset.currentEvent")
        self.wait_moves_controls(live_event, selector)
        for width, height in ((1180, 820), (760, 650)):
            self.viewport(width, height)
            w.click(selector)
            selected = self.wait_moves_controls(event, selector)
            assert w.read(f"document.querySelector({json.dumps(selector)}).getAttribute('aria-current')") == "step"
            self.assert_completed_cue(action, event, [origin], [destination])
            self.capture(f"moves-{game}-{width}-pointer-history")
            self.flip_moves_board(event, selector)
            self.assert_completed_cue(action, event, [origin], [destination])
            self.capture(f"moves-{game}-{width}-flipped-history")
            self.flip_moves_board(event, selector, selected["orientation"])
            w.keyboard_activate(".moves-root")
            self.wait_moves_controls(0, selector, selected["orientation"])
            assert self.completed_cue() is None, "Known root cannot invent a last move"
            w.keyboard_activate(selector)
            self.wait_moves_controls(event, selector, selected["orientation"])
            self.assert_completed_cue(action, event, [origin], [destination])
            self.assert_focus_outline(selector, game)
            self.capture(f"moves-{game}-{width}-keyboard-history")
            w.button("Return to live")
            wait(lambda: w.read("document.querySelector('.game-shell').dataset.historical") == "false", "explicit Return to live")
            self.wait_moves_controls(live_event, selector, selected["orientation"])
            wait(lambda: self.completed_cue() == live, "Return to live restores genuine engine action cue")
            if w.has(".play-paused"):
                w.button("Resume play")
                wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "false", "explicit Resume after browsing")
                self.wait_moves_controls(live_event, selector, selected["orientation"])
        self.viewport(1180, 820)
        self.record("All-game shared Moves pointer and keyboard exact-prefix navigation", {"game": game, "action": action,
            "notation": notation, "event": event, "live_cue": live, "sizes": [[1180, 820], [760, 650]]})

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

    def synthetic_board_fits(self, orientation):
        """Observe all native 16x16 positions in place; never scroll to expose them."""
        geometry = self.web.read("""(() => {
            const board=document.querySelector('.synthetic-board .chess-board');
            const squares=[...document.querySelectorAll('.synthetic-board .square')];
            const rect=e=>{const r=e.getBoundingClientRect();return {left:r.left,top:r.top,right:r.right,bottom:r.bottom,width:r.width,height:r.height};};
            const fullyVisible=e=>{
                if(!e)return false;
                const r=e.getBoundingClientRect(),clip={left:0,top:0,right:innerWidth,bottom:innerHeight};
                for(let p=e.parentElement;p;p=p.parentElement){
                    const c=getComputedStyle(p),b=p.getBoundingClientRect();
                    if(/^(auto|scroll|hidden|clip)$/.test(c.overflowX)){clip.left=Math.max(clip.left,b.left);clip.right=Math.min(clip.right,b.right);}
                    if(/^(auto|scroll|hidden|clip)$/.test(c.overflowY)){clip.top=Math.max(clip.top,b.top);clip.bottom=Math.min(clip.bottom,b.bottom);}
                }
                return r.width>0&&r.height>0&&r.left>=clip.left-1&&r.top>=clip.top-1&&r.right<=clip.right+1&&r.bottom<=clip.bottom+1;
            };
            const key=e=>e.getAttribute('aria-label')?.split(',')[0]??'';
            const ancestors=[];
            for(let p=board?.parentElement;p;p=p.parentElement){const c=getComputedStyle(p);ancestors.push({tag:p.tagName,className:p.className,
                overflowX:c.overflowX,overflowY:c.overflowY,scrollLeft:p.scrollLeft,scrollTop:p.scrollTop,
                clientWidth:p.clientWidth,clientHeight:p.clientHeight,scrollWidth:p.scrollWidth,scrollHeight:p.scrollHeight});}
            return {viewport:{width:innerWidth,height:innerHeight},positionCount:squares.length,
                uniquePositions:new Set(squares.map(key)).size,positions:squares.map(key),
                fullyVisible:squares.every(fullyVisible),clippedPositions:squares.filter(e=>!fullyVisible(e)).map(key),
                board:board?rect(board):null,boardVisible:fullyVisible(board),
                orientationLabel:board?.getAttribute('aria-label')??'',pageWidth:document.documentElement.scrollWidth,
                boardAncestors:ancestors};
        })()""")
        assert geometry["positionCount"] == 256 and geometry["uniquePositions"] == 256, geometry
        assert len(geometry["positions"]) == 256 and len(set(geometry["positions"])) == 256, geometry
        assert geometry["fullyVisible"] and not geometry["clippedPositions"] and geometry["boardVisible"], geometry
        assert geometry["orientationLabel"].endswith(f", {orientation} at bottom"), geometry
        assert geometry["pageWidth"] <= geometry["viewport"]["width"] + 1, geometry
        self.record("Observe all256 synthetic positions fully visible without scrolling or zooming", geometry)
        return geometry

    def synthetic_record(self, actions):
        """Require the native record and visible acceptance count to agree."""
        def accepted():
            record = self.web.read("JSON.parse(document.querySelector('.synthetic-inspector pre').textContent)")
            if (record["rules_profile"] != "mshogi.synthetic.strider16.v1" or record["schema_version"] != 1
                    or record["initial_fixture"] != "strider16-capture-and-return-v1"):
                raise ValueError(f"Unexpected synthetic native record identity: {record!r}")
            return record if record["actions"] == actions else None
        record = wait(accepted, f"exact native synthetic actions {actions!r}")
        assert re.search(rf"\bAccepted actions {len(actions)}\b", self.web.text(".synthetic-inspector")), record
        self.record("Observe exact separate synthetic native acceptance", record)
        return record

    def assert_focus_outline(self, selector, game):
        observed=self.web.read("""(() => {
            const e=document.querySelector(SELECTOR);if(!e)return null;
            const r=e.getBoundingClientRect(),c=getComputedStyle(e),width=parseFloat(c.outlineWidth)||0,offset=parseFloat(c.outlineOffset)||0;
            const pad=Math.max(0,width+offset),clip={left:0,top:0,right:innerWidth,bottom:innerHeight},ancestors=[];
            for(let p=e.parentElement;p;p=p.parentElement){const s=getComputedStyle(p),b=p.getBoundingClientRect();
                const cx=/^(auto|scroll|hidden|clip)$/.test(s.overflowX),cy=/^(auto|scroll|hidden|clip)$/.test(s.overflowY);
                if(cx){clip.left=Math.max(clip.left,b.left+p.clientLeft);clip.right=Math.min(clip.right,b.left+p.clientLeft+p.clientWidth);}
                if(cy){clip.top=Math.max(clip.top,b.top+p.clientTop);clip.bottom=Math.min(clip.bottom,b.top+p.clientTop+p.clientHeight);}
                if(cx||cy)ancestors.push({tag:p.tagName,class:p.className,overflowX:s.overflowX,overflowY:s.overflowY,rect:{left:b.left,right:b.right,top:b.top,bottom:b.bottom}});
            }
            return {focused:document.activeElement===e,focusVisible:e.matches(':focus-visible'),outlineStyle:c.outlineStyle,outlineWidth:width,outlineOffset:offset,
                outline:{left:r.left-pad,right:r.right+pad,top:r.top-pad,bottom:r.bottom+pad},clip,ancestors,viewport:{width:innerWidth,height:innerHeight}};
        })()""".replace('SELECTOR',json.dumps(selector)))
        assert focus_outline_visible(observed), {'game':game,'selector':selector,'focus':observed}
        self.record('Actual keyboard Moves focus outline clears every overflow clip and viewport',{'game':game,'selector':selector,**observed})
        return observed

    def support_access(self, game):
        """Keyboard-opened ordinary About, disclosures and focus restoration."""
        w=self.web
        w.keyboard_activate('[data-support-trigger]')
        wait(lambda:w.has('.support-dialog'), 'keyboard About access')
        assert w.read("document.querySelector('.support-dialog')?.getAttribute('role')")=='dialog'
        assert w.read("document.querySelector('.support-dialog')?.contains(document.activeElement)")
        summaries=w.read("[...document.querySelectorAll('.support-dialog summary')].map(e=>e.textContent)")
        assert summaries==['Diagnostics','Font license'],summaries
        w.keyboard_activate('.support-dialog details:first-of-type summary')
        native=w.read("[JSON.parse(document.querySelector('[data-native-diagnostics]').textContent)]")[0]
        assert all(k in native for k in ('token','profile','context','document','capacity','diagnostic','io','error','analysis','observations')),native
        assert native['context']['game_kind']==game,native
        w.keyboard_activate('.support-dialog details:first-of-type summary')
        w.keyboard_activate('.support-dialog details:nth-of-type(2) summary')
        license=w.text('.support-dialog details:nth-of-type(2)')
        assert 'SIL OPEN FONT LICENSE' in license and 'mShogi' in license,license
        assert w.read("!!document.querySelector('.support-dialog a[download][href]')")
        size=w.read('({width:innerWidth,height:innerHeight})')
        self.capture(f'support-{game}-{size["width"]}x{size["height"]}')
        w.key('\ue00c')
        wait(lambda:not w.has('.support-dialog'),'Escape closes About')
        assert w.read("document.activeElement===document.querySelector('[data-support-trigger]')"),'About restores its invoker'
        assert not w.has('[data-native-diagnostics]'),'diagnostics absent outside About'
        self.record('Keyboard About / native diagnostics / font license / Escape / restored focus',{'game':game,'summaries':summaries,'profile':native['profile'],'token':native['token']})

    def choose_game(self, game):
        """Actual native-select/radio keys, at normal and minimum dimensions."""
        w=self.web
        expected=['chess','shogi','xiangqi']
        observed=w.read("[...document.querySelector('[data-game-select]').options].map(e=>({value:e.value,label:e.textContent,disabled:e.disabled}))")
        assert observed==[{'value':k,'label':label,'disabled':False} for k,label in zip(expected,['Chess','Shogi','Xiangqi'])],observed
        def select(kind):
            w.click('[data-game-select]');w.key('\ue011')  # Home
            for _ in range(expected.index(kind)):w.key('\ue015')  # ArrowDown
            w.key('\ue007')
            wait(lambda:w.read("document.querySelector('[data-game-select]').value")==kind,'native game dropdown selection')
        for width,height in ((1180,820),(760,650)):
            self.viewport(width,height)
            for kind in expected:
                select(kind)
                sides={'chess':['White','Black','Random'],'shogi':['Sente','Gote','Random'],'xiangqi':['Red','Black','Random']}[kind]
                buttons=w.read("[...document.querySelectorAll('.side-choices button')].map(e=>({label:e.textContent,role:e.getAttribute('role'),selected:e.getAttribute('aria-checked'),chosen:e.classList.contains('chosen')}))")
                assert [e['label'] for e in buttons]==sides and all(e['role']=='radio' for e in buttons),buttons
                assert sum(e['chosen'] and e['selected']=='true' for e in buttons)==1,buttons
                w.click(f'[data-side="{sides[0].lower()}"]')
                w.key('\ue014')  # Right: existing native side radio behavior
                wait(lambda:w.read("document.activeElement?.textContent")==sides[1],'side radio arrow selects second side')
                assert w.read("document.activeElement?.getAttribute('aria-checked')")=='true'
                w.key('\ue012')  # Left restores first side
                wait(lambda:w.read("document.activeElement?.textContent")==sides[0],'side radio arrow returns first side')
                assert w.read("document.activeElement?.classList.contains('chosen')")
                self.capture(f'chooser-{kind}-{width}')
                self.record('Supported-only native game dropdown preserves side radio keys and styling',{'game':kind,'options':observed,'sides':buttons,'viewport':[width,height]})
        self.viewport(1180,820)
        select(game)

    def shared_shell(self, game):
        layout = self.web.read("""(() => {
            const root=document.querySelector('.game-shell');
            const rectElement=e=>{if(!e)return null;const r=e.getBoundingClientRect();return {left:r.left,top:r.top,width:r.width,height:r.height};};
            const rect=s=>rectElement(document.querySelector(s));
            const theme=s=>{const e=document.querySelector(s);if(!e)return null;const c=getComputedStyle(e);return Object.fromEntries(['backgroundColor','color','borderTopColor','borderTopWidth','borderRadius','fontSize','fontFamily','colorScheme'].map(k=>[k,c[k]]));};
            const visible=e=>{if(!e)return false;const r=e.getBoundingClientRect();return r.width>0&&r.height>0&&r.left>=-1&&r.top>=-1&&r.right<=innerWidth+1&&r.bottom<=innerHeight+1;};
            const squares=[...document.querySelectorAll('.square,.shogi-square,.xiangqi-point')];
            return {game:root?.classList.contains('shell-xiangqi')?'xiangqi':root?.classList.contains('shell-shogi')?'shogi':'chess',viewport:{width:innerWidth,height:innerHeight},
                themeIdentity:root?.dataset.theme,
                themes:Object.fromEntries(['.game-shell','.shell-header','.shell-card','.shell-section-title','.shell-file-actions .primary','.shell-file-actions .quiet','.inspector-switch button[aria-pressed="true"]'].map(s=>[s,theme(s)])),
                header:rect('.shell-header'),document:rect('.shell-document'),actions:rect('.shell-file-actions'),
                heading:rect('.shell-live-heading'),boardPanel:rect('.shell-board-panel'),sidebar:rect('.shell-sidebar'),
                gameArea:rect('.shell-game-area'),
                help:rect('.shell-game-footer .shell-board-help,.shell-game-footer .board-help,.shell-game-footer .shogi-board-help,.shell-game-footer .xiangqi-help'),
                helpVisible:visible(document.querySelector('.shell-game-footer .shell-board-help,.shell-game-footer .board-help,.shell-game-footer .shogi-board-help,.shell-game-footer .xiangqi-help')),
                footnoteVisible:!!document.querySelector('[data-position-status]')&&visible(document.querySelector('[data-position-status]')),
                positionStatus:document.querySelector('[data-position-status]')?.textContent,
                documentState:document.querySelector('[data-document-state]')?.dataset.documentState,
                playLimits:document.querySelector('[data-play-limits]')?.textContent,
                slots:[...document.querySelectorAll('.shell-game-area>[data-game-slot]')].map(e=>({name:e.dataset.gameSlot,rect:rectElement(e)})),
                cards:[...document.querySelectorAll('[data-shell-section]')].map(e=>({name:e.dataset.shellSection,rect:rectElement(e)})),
                squareCount:squares.length,allSquaresVisible:squares.every(visible),pageWidth:document.documentElement.scrollWidth,
                sections:[...document.querySelectorAll('[data-shell-section]')].map(e=>e.dataset.shellSection),
                inspectorButtons:[...document.querySelectorAll('.inspector-switch button')].map(e=>({label:e.textContent.trim(),pressed:e.getAttribute('aria-pressed')})),
                documentInHeader:document.querySelector('.shell-header')?.contains(document.querySelector('.shell-document')),
                flipInHeading:document.querySelector('.shell-live-heading')?.contains(document.querySelector('.flip-button')),
                headerButtons:[...document.querySelectorAll('.shell-file-actions button')].map(e=>e.textContent.trim()),
                brand:document.querySelector('.brand-mark')?.textContent,gameLabel:document.querySelector('.product-brand p')?.textContent,
                supportOutsideSidebar:!!document.querySelector('[data-support-trigger]')&&!document.querySelector('.shell-sidebar')?.contains(document.querySelector('[data-support-trigger]')),
                rawDiagnosticsAbsent:!document.querySelector('[data-native-diagnostics],.shell-technical')};
        })()""")
        assert layout['game'] == game and layout['sections'] == ['analysis','actions','history'], layout
        assert layout['themeIdentity'] == 'mshogi-dark' and all(layout['themes'].values()), layout
        assert layout['squareCount']=={'chess': 64, 'shogi': 81, 'xiangqi': 90}[game] and layout['allSquaresVisible'] and layout['pageWidth']<=layout['viewport']['width']+1, layout
        assert layout['inspectorButtons'] == [{'label':'Play details','pressed':'true'}, {'label':'Analyze position','pressed':'false'}], layout
        assert [slot['name'] for slot in layout['slots']] == ['upper-player','board','lower-player','footer'], layout
        assert layout['help'] and layout['help']['height'] > 0 and layout['helpVisible'] and layout['footnoteVisible'], ('live help belongs visibly in the shared footer',layout)
        assert layout['documentState'] == 'unsaved', layout
        assert layout['positionStatus'] == 'Your turnLive position · 0 plies', layout
        expected_depth, expected_nodes = {'chess': (2, 20000), 'shogi': (2, 5000), 'xiangqi': (1, 128)}[game]
        assert play_limit_matches(layout['playLimits'], expected_depth, expected_nodes), layout
        assert layout['documentInHeader'] and layout['flipInHeading'] and layout['supportOutsideSidebar'] and layout['rawDiagnosticsAbsent'], layout
        assert layout['brand'] == '歩' and layout['gameLabel'] == game.upper(), layout
        self.support_access(game)
        assert layout['headerButtons'] == ['New game','Save','Save as…','Open…'], layout
        assert layout['boardPanel']['left'] < layout['sidebar']['left'], layout
        for slot in layout['slots']:
            box=slot['rect']
            assert box['width']>0 and box['height']>0 and box['left']>=-1 and box['top']>=-1 and box['left']+box['width']<=layout['viewport']['width']+1 and box['top']+box['height']<=layout['viewport']['height']+1, slot
        for upper,lower in zip(layout['slots'],layout['slots'][1:]):
            assert upper['rect']['top']+upper['rect']['height']<=lower['rect']['top']+1,(upper,lower)
        key=(layout['viewport']['width'],layout['viewport']['height'])
        previous=self.shell_layouts.get(key)
        if previous and previous['game'] != game:
            for part in ['header','actions','heading','boardPanel','sidebar']:
                for axis in ['left','top','width']:
                    assert abs(layout[part][axis]-previous[part][axis])<=3,(part,axis,previous,layout)
            assert abs((layout['document']['top']+layout['document']['height']/2)-
                       (previous['document']['top']+previous['document']['height']/2))<=3,(previous,layout)
            assert layout['themes']==previous['themes'],('shared chrome/panel/control theme differs',previous,layout)
            assert abs(layout['help']['top']-previous['help']['top'])<=3, ('live help moved across games',previous,layout)
            for collection in ['slots','cards']:
                for current,prior in zip(layout[collection],previous[collection]):
                    assert current['name']==prior['name'],(previous,layout)
                    for axis in ['left','top','width','height']:
                        assert abs(current['rect'][axis]-prior['rect'][axis])<=3,(collection,current['name'],axis,prior,current)
        self.shell_layouts[key]=layout
        self.record('Observe one dark application shell, shared physical slots and initial cards',layout)

    def assert_historical_notice(self, event):
        observed=self.web.read("""(() => {
            const root=document.querySelector('.game-shell'),notice=document.querySelector('.historical-notice');
            const back=[...(notice?.querySelectorAll('button')??[])].find(e=>e.textContent==='Return to live');
            return {heading:notice?.querySelector('strong')?.textContent,meaning:notice?.querySelector('p')?.textContent,
                historical:root?.dataset.historical,event:root?.dataset.currentEvent,returnLiveEnabled:!!back&&!back.disabled};
        })()""")
        assert historical_notice_matches(observed,event),observed
        self.record('Historical notice preserves exact native prefix and explicit Return to live',observed)

    def inspection_state(self):
        return self.web.read("""(() => {
            const text=s=>document.querySelector(s)?.textContent??null;
            const squareLabels=[...document.querySelectorAll('.square,.shogi-square,.xiangqi-point')].map(e=>e.getAttribute('aria-label')?.split(',').slice(0,2).join(','));
            const root=document.querySelector('.game-shell');
            return {pieces:squareLabels,history:text('.shogi-history'),
                status:text('[data-game-status]'),dirty:text('.document-dirty')?.toLowerCase(),
                documentTitle:text('.shell-document h2'),runtime:text('.shell-runtime'),
                analysis:text('.shell-analysis'),
                event:root?.dataset.currentEvent,liveEvents:root?.dataset.liveEvents,historical:root?.dataset.historical,paused:root?.dataset.paused};
        })()""")

    def assert_inspection_unchanged(self, before, label):
        after = self.inspection_state()
        self.record(label, {"before": before, "after": after, "unchanged": before == after})
        assert after == before, (label, {"before": before, "after": after})

    def inspect_piece(self, game, origin, destination, label):
        """Real native query through normal UI; completing a route stays read-only."""
        w=self.web; native_before=self.native_diagnostics(); before=self.inspection_state()
        square=self.square if game=='chess' else self.xiangqi_point if game=='xiangqi' else self.shogi_square
        w.click(square(origin))
        wait(lambda: f'Inspecting {label}' in w.text('.movement-inspection-status'), 'native opponent movement inspection')
        # Shogi/Xiangqi have a separate keyboard-hint span in this row; Chess
        # places the status directly in the paragraph. Observe the status itself.
        status = w.read("(() => { const e=document.querySelector('.movement-inspection-status'); return e?.querySelector('span')?.textContent ?? e?.textContent; })()")
        assert inspection_status_matches(status, label), ('inspection status', status, label)
        assert w.read(f"document.querySelector({json.dumps(square(destination))})?.getAttribute('aria-label')?.includes('read-only inspection destination')")
        assert w.read(f"document.querySelector({json.dumps(square(destination))})?.querySelector('.inspection-marker')!==null")
        assert not w.has('.square.target,.shogi-square.target,.xiangqi-destination')
        self.capture(f'inspect-{game}-{origin}')
        w.click(square(destination))
        time.sleep(0.35)  # Observe across native snapshot polls, not only local selection state.
        self.assert_inspection_unchanged(before, 'Board inspection complete route preserves authoritative context')
        assert not w.has('.promotion-dialog,.shogi-promotion')
        w.key('\ue00c')
        wait(lambda:not w.has('.inspection-marker,.inspection-origin-ring'), 'Escape clears read-only inspection')
        self.assert_inspection_unchanged(before, 'Board inspection Escape preserves authoritative context')
        native_after=self.native_diagnostics()
        for key in ('token','context','document','observations'):
            assert native_before[key] == native_after[key], (key,native_before[key],native_after[key])
        self.record('Native movement inspection and complete read-only route preserve context',{'game':game,'origin':origin,'destination':destination,'label':label,'native_token':native_before['token'],'history':before['history']})

    def file(self, kind, path, cancel=False, button_label=None):
        self._file_receipt_floor = self.native_diagnostics()["io"]["id"]
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
        self.wait_saved(path)
        wait(path.exists, "native shogi Save As created exact path")
        archive = json.loads(path.read_text(encoding="utf-8"))
        body = desktop_document_payload(archive, "shogi")
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
        self.shogi_analysis_tab()
        self.web.set('.shogi-analysis input[name="depth"]', depth)
        self.web.set('.shogi-analysis input[name="nodes"]', nodes)
        self.web.click('[data-analysis-start="shogi"]')

    def shogi_analysis_tab(self):
        if not self.web.has('[data-analysis-start="shogi"]'):
            self.web.button('Analyze position')
        wait(lambda:self.web.has('[data-analysis-start="shogi"]'),'Shogi analysis tab exposes real bounded analysis controls')

    def shogi_complete_analysis(self):
        wait(lambda: "Analysis result available" in self.web.text(".shogi-analysis-state"), "actual read-only shogi analysis result", 45)
        assert self.web.has('[data-search-purpose="move_analysis"]')
        assert "No move or declaration was played" in self.web.text(".shogi-analysis")
        assert "perspective" in self.web.text(".analysis-perspective")

    def shogi_flip(self):
        previous = self.web.read("document.querySelector('.shogi-application')?.dataset.orientation")
        self.web.click(".flip-button")
        wait(lambda: self.web.read("document.querySelector('.shogi-application')?.dataset.orientation") != previous,
             "native shogi orientation changes")

    def save(self, path, observation_count=None):
        self.file("save", path)
        self.wait_saved(path)
        wait(path.exists, "native save created file")
        archive = json.loads(path.read_text(encoding="utf-8"))
        body = desktop_document_payload(archive, "chess")
        if observation_count is not None:
            assert len(body["observations"]) == observation_count
        self.record("Inspect actual saved canonical archive", {
            "path": str(path), "sha256": digest(path), "bytes": path.stat().st_size,
            "observations": len(body["observations"]), "controls": len(body["controls"]),
            "game_events": len(json.loads(body["game_record_json"])["events"])})
        return archive

    def document_file_state(self):
        state = self.web.read("""(() => ({
                path:document.querySelector('.shell-document h2')?.getAttribute('title')??null,
                dirty:document.querySelector('.document-dirty')?.textContent??'',
                buttons:[...document.querySelectorAll('.shell-file-actions button')].map(e=>({label:e.textContent.trim(),enabled:!e.disabled})),
                statuses:[...document.querySelectorAll('.shell-document [role="status"]')].map(e=>e.textContent)
            }))()""")
        state["io"] = self.native_diagnostics()["io"]
        state["after_io_id"] = getattr(self, "_file_receipt_floor", 0)
        return state

    def wait_saved(self, path):
        def accepted():
            observation = self.document_file_state()
            return observation if saved_document_accepted(observation, path) else None
        observation = wait(accepted, f"native document accepts exact saved path with file work settled: {path}")
        self.record('Observe native accepted save path and settled file controls', observation)
        return observation

    def wait_opened(self, path):
        def accepted():
            observation = self.document_file_state()
            return observation if opened_document_accepted(observation, path) else None
        observation = wait(accepted, f"native Open accepts exact path with work stopped and file controls settled: {path}")
        self.record('Observe native accepted Open receipt and stopped document', observation)
        return observation

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
        self.viewport(1180,820)
        self.shared_shell('chess')
        self.plies(0)
        self.capture('initial-chess-normal-size')
        self.viewport(760,650)
        self.shared_shell('chess')
        self.capture('initial-chess-minimum-size')
        self.inspect_piece('chess','e7','e5','Black pawn')
        self.viewport(1180,820)
        w.click(self.square('e2'))
        assert w.read(f"document.querySelector({json.dumps(self.square('e4'))})?.classList.contains('target')")
        w.click(self.square('b8'))
        wait(lambda:'Inspecting Black knight' in w.text('.movement-inspection-status'),'friendly selection switches to enemy inspection')
        assert not w.has('.square.target')
        w.click(self.square('e2'))
        assert not w.has('.inspection-marker')
        assert w.read(f"document.querySelector({json.dumps(self.square('e4'))})?.classList.contains('target')")
        w.key('\ue00c');self.plies(0)
        w.click(self.square("e2")); w.click(self.square("e4"))
        self.plies(2)
        wait(lambda: w.has('[data-search-purpose="engine_turn"]'), "actual engine reply")
        assert not w.has(".outcome-card")
        self.capture("play")
        self.record("Play e2e4 against actual bundled engine", w.text('[data-search-purpose="engine_turn"]'))
        self.moves_navigation("chess", "e2e4", "e4", 2, "e2", "e4")

        w.focused_button("Take back")
        self.plies(0)
        assert self.completed_cue() is None, "Take back to root retires completed cue"
        wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "true", "Chess Take back is stopped")
        self.capture("takeback-stopped")
        w.button("Resume play")
        wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "false", "explicit Chess Resume after Take back")
        w.click(self.square("e2")); w.click(self.square("e4"))
        self.plies(2)
        self.record("Shared Take back / stopped Chess native decision / explicit Resume / replay", self.inspection_state())

        w.button("New game")
        wait(lambda: w.has(".document-guard"), "unsaved replacement guard")
        w.button("Cancel")
        wait(lambda: not w.has(".document-guard"), "cancel replacement")
        self.plies(2)
        w.button("Analyze position")
        w.set('.shogi-analysis input[name="depth"]', 8)
        w.set('.shogi-analysis input[name="nodes"]', 5_000_000)
        w.click('[data-analysis-start="chess"]')
        wait(lambda: w.has('.shogi-analysis[data-phase="analyzing"]'), "real active analysis")
        self.capture("active-analysis")
        w.button("Cancel analysis")
        wait(lambda: "cancelled" in w.text('[aria-label="Analysis status"]').lower(), "analysis cancellation")
        self.plies(2)
        self.record("Cancel demonstrably active analysis", w.text('[aria-label="Analysis status"]'))
        w.click('[data-analysis-start="chess"]')
        wait(lambda: w.has('.shogi-analysis[data-phase="analyzing"]'), "replacement begins from active work")
        w.set('.shogi-analysis input[name="depth"]', 1)
        w.set('.shogi-analysis input[name="nodes"]', 1_000)
        w.click('[data-analysis-start="chess"]')
        wait(lambda: "not played" in w.text(".analysis-result"), "completed read-only replacement")
        self.plies(2)
        self.capture("replacement-complete")
        self.record("Replace with bounded read-only suggestion", w.text(".analysis-result"))

        path = self.args.output / "normal-complete.mshogi"
        archive = self.save(path)
        assert len(desktop_document_payload(archive, "chess")["observations"]) >= 2
        self.inspect_piece('chess','b8','c6','Black knight')
        before_write=path.stat().st_mtime_ns
        w.button('Save')
        wait(lambda:path.stat().st_mtime_ns!=before_write,'native Save commits a fresh archive after inspection')
        self.wait_saved(path)
        assert json.loads(path.read_text(encoding='utf-8'))==archive,'inspection changed canonical chess archive'
        self.file("open", path)
        self.wait_opened(path)
        self.plies(2)
        assert w.has(".play-paused")
        w.button("Play details")
        assert "No engine search has completed" in w.text(".sidebar")
        self.record("Reopen complete document without resuming work", w.text(".document-card"))
        w.click('.shogi-history [data-event="2"]')
        self.plies(1, historical=True)
        self.assert_historical_notice(2)
        self.inspect_piece('chess','b8','c6','Black knight')
        w.button("Analyze position")
        w.set('.shogi-analysis input[name="depth"]', 1)
        w.set('.shogi-analysis input[name="nodes"]', 1_000)
        w.click('[data-analysis-start="chess"]')
        wait(lambda: "not played" in w.text(".analysis-result"), "historical analysis")
        assert "Black" in w.text(".analysis-result")
        self.plies(1, historical=True)
        self.capture("historical-analysis")
        w.button("Return to live")
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
        self.wait_saved(path)
        archive = json.loads(path.read_text(encoding="utf-8"))
        body = desktop_document_payload(archive, "chess")
        observation = body["observations"][-1]["observation"]["context"]
        assert observation["purpose"]["kind"] == "move_analysis"
        assert len(observation["game"]["payload"]["events"]) == 1
        self.record("History prefix provenance / Return live / Resume", {
            "historical_request": observation, "live_game_events":
                len(json.loads(body["game_record_json"])["events"])})
        w.click(".flip-button")
        wait(lambda: w.text(".document-dirty").casefold() == "unsaved changes", "orientation change is protected")
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
        self.choose_game('shogi')
        w.click(f'[data-side="{side}"]')
        w.click("[data-start-game]")
        wait(lambda: w.has(".shogi-application") and not w.has('[aria-label="New game"]'), "new native shogi product game")
        wait(lambda: w.read("document.querySelector('.shogi-application')?.dataset.orientation") == side,
             f"human {side} at bottom")
        self.record("Select Shogi and human side in normal New Game", {"side": side, "board_name":
            w.read("document.querySelector('.shogi-board')?.getAttribute('aria-label')")})

    def xiangqi_point(self, point):
        return f'.xiangqi-point[data-intersection="{point}"]'

    def xiangqi_board_fits(self):
        geometry = self.web.read("""(() => {const points=[...document.querySelectorAll('.xiangqi-point')];
            const visible=e=>{const r=e.getBoundingClientRect();return r.width>0&&r.height>0&&r.left>=-1&&r.top>=-1&&r.right<=innerWidth+1&&r.bottom<=innerHeight+1;};
            return {viewport:{width:innerWidth,height:innerHeight},points:points.length,allVisible:points.every(visible),
                palaceSides:[...new Set([...document.querySelectorAll('[data-palace]')].map(e=>e.dataset.palace))],
                river:document.querySelector('.xiangqi-river')?.textContent};})()""")
        assert geometry["points"] == 90 and geometry["allVisible"], geometry
        assert set(geometry["palaceSides"]) == {"mshogi.xiangqi.wxf2018.v1:red", "mshogi.xiangqi.wxf2018.v1:black"}, geometry
        assert "楚河" in geometry["river"] and "漢界" in geometry["river"], geometry
        self.record("Complete native Xiangqi intersections, river and palaces without scrolling", geometry)
        return geometry

    def native_diagnostics(self):
        """Open the public support surface with real input; no hidden DOM/IPC."""
        w=self.web
        wait(lambda:w.read("!!document.querySelector('[data-support-trigger]:not(:disabled)')"), "support available after native work settles")
        w.click('[data-support-trigger]')
        wait(lambda:w.has('.support-dialog'), "public About dialog")
        try:
            w.click('.support-dialog details summary')
            observed=w.read("[JSON.parse(document.querySelector('[data-native-diagnostics]').textContent)]")[0]
            assert isinstance(observed,dict) and observed.get('token') and observed.get('context'), observed
            return observed
        finally:
            w.click('.support-dialog [data-cancel]')
            wait(lambda:not w.has('.support-dialog'), "Close public About dialog")

    def xiangqi_diagnostics(self):
        observed=self.native_diagnostics()
        assert observed['profile']=='mshogi.xiangqi.wxf2018.v1',observed
        return observed

    def xiangqi_events(self, count, selected=None, historical=False):
        expected = {"live": str(count), "historical": str(historical).lower()}
        if selected is not None:
            expected["selected"] = str(selected)
        def observed():
            actual = self.web.read("""(() => {const e=document.querySelector('.shell-xiangqi');
                return e ? {live:e.dataset.liveEvents,selected:e.dataset.currentEvent,
                    historical:e.dataset.historical}:null;})()""")
            return actual if actual and all(actual[key] == value for key, value in expected.items()) else None
        return wait(observed, f"native Xiangqi event/history boundary {expected}")

    def xiangqi_analysis(self, depth=1, nodes=128):
        if not self.web.has('[data-analysis-start="xiangqi"]'):
            self.web.button("Analyze position")
        wait(lambda: self.web.has('[data-analysis-start="xiangqi"]'), "native Xiangqi analysis controls")
        self.web.set('.shogi-analysis input[name="depth"]', depth)
        self.web.set('.shogi-analysis input[name="nodes"]', nodes)
        self.web.click('[data-analysis-start="xiangqi"]')

    def xiangqi_complete_analysis(self, event_count, depth=1, nodes=128):
        def completed():
            native = self.xiangqi_diagnostics()
            if not native:
                return None
            for item in reversed(native["observations"]):
                context = item["observation"]["context"]
                if (not item["historical"] and context["purpose"]["kind"] == "move_analysis"
                        and context["limits"] == {"depth": depth, "nodes": nodes}
                        and len(context["game"]["payload"]["events"]) == event_count):
                    return item
            return None
        observation = wait(completed, f"actual Xiangqi analysis observation for event{event_count}, depth{depth}/{nodes}visits", 45)
        wait(lambda: "Analysis result available" in self.web.text('.shogi-analysis-state'), "native Xiangqi result presentation")
        assert "No move or claim was played" in self.web.text('.shogi-analysis')
        return observation

    def xiangqi_save(self, path):
        self.file("save_as", path, button_label="Save as…")
        self.wait_saved(path)
        wait(path.exists, "native Xiangqi Save As created exact path")
        archive = json.loads(path.read_text(encoding="utf-8"))
        body = desktop_document_payload(archive, "xiangqi")
        assert body["current"]["game_kind"] == "xiangqi", body
        self.record("Inspect complete native Xiangqi current document", {
            "path": str(path), "sha256": digest(path), "bytes": path.stat().st_size,
            "observations": len(body["observations"]), "exact_observations": len(body["exact_observations"]),
            "controls": len(body["controls"]), "native_events": len(body["current"]["payload"]["events"]),
            "root": body["current"]["payload"]["root"]})
        return archive

    def normal_xiangqi(self):
        """Current runtime ownership/file/close journey in the normal package.

        Existing native fixtures own WXF correctness. This phase observes actual
        command admission, engine work and native document identity; it does not
        infer an outcome or score from an incomplete search.
        """
        self.start(product="xiangqi")
        w = self.web
        w.button("New game")
        wait(lambda: w.has('.document-guard') or w.has('[aria-label="New game"]'), "New Game guard or chooser")
        if w.has(".document-guard"):
            w.button("Discard & continue")
        wait(lambda: w.has('[aria-label="New game"]'), "normal game selection dialog")
        self.choose_game('xiangqi'); w.click('[data-side="red"]'); w.click('[data-start-game]')
        wait(lambda: w.has(".shell-xiangqi") and not w.has('[aria-label="New game"]'), "native Xiangqi/Red document")
        self.viewport(1180, 820)
        self.xiangqi_events(0, selected=0)
        assert "Red at bottom" in w.read("document.querySelector('.xiangqi-board').getAttribute('aria-label')")
        self.xiangqi_board_fits()
        self.shared_shell("xiangqi")
        self.capture("initial-red-normal-size")
        self.viewport(760, 650)
        self.xiangqi_board_fits()
        self.shared_shell("xiangqi")
        self.capture("initial-red-minimum-size")
        self.inspect_piece("xiangqi", "a6", "a5", "Black soldier")
        self.viewport(1180, 820)

        source = '.xiangqi-point[data-intersection="a3"]'
        target = '.xiangqi-point[data-intersection="a4"]'
        w.click(source)
        assert w.read(f"document.querySelector({json.dumps(target)})?.getAttribute('aria-label').includes('legal destination')")
        w.key("\ue00c")
        self.xiangqi_events(0, selected=0)
        assert w.read("document.activeElement?.dataset.intersection") == "a3"
        for key in ("\ue007", "\ue013", "\ue007"):
            w.key(key)
        self.xiangqi_events(2, selected=2)
        wait(lambda: w.has('[data-search-purpose="engine_turn"]'), "actual bundled Xiangqi engine observation")
        played = self.xiangqi_diagnostics()
        assert played["context"]["payload"]["events"][0]["event"]["action"] == "a3a4", played
        assert played["context"]["payload"]["conclusion"] is None, played
        self.capture("play")
        self.record("Pointer selection / Escape / keyboard a3a4 / actual bounded engine reply", played)
        self.moves_navigation("xiangqi", "a3a4", "P9+1", 1, "a3", "a4")

        w.focused_button("Take back")
        self.xiangqi_events(0, selected=0)
        assert self.completed_cue() is None, "Xiangqi Take back to root retires cue"
        wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "true", "Xiangqi Take back is stopped")
        self.capture("takeback-stopped")
        w.button("Resume play")
        wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "false", "explicit Xiangqi Resume after Take back")
        w.click(source); w.click(target)
        self.xiangqi_events(2, selected=2)
        self.record("Shared Take back / stopped Xiangqi native decision / explicit Resume / replay", self.xiangqi_diagnostics())

        # The measured Xiangqi maximum is deliberately small. Completion may win
        # the real input race; only report cancellation when native state proves
        # it. Chess/Shogi phases and native tests establish forced cancellation.
        self.xiangqi_analysis(depth=3, nodes=512)
        wait(lambda: w.has('.shogi-analysis[data-phase="analyzing"]')
             or "Analysis result available" in w.text('.shogi-analysis-state'), "native Xiangqi analysis accepts request")
        cancel_dispatched = False
        if w.has('.shogi-analysis[data-phase="analyzing"]') and w.has('.shogi-analysis button[type="button"]'):
            try:
                w.button("Cancel analysis")
                cancel_dispatched = True
            except RuntimeError as error:
                # Finding the transient button can race genuine completion. Do
                # not retry input or swallow transport/session failures.
                reply = json.loads(str(error))
                value = reply.get("value", reply)
                if value.get("error") not in ("no such element", "stale element reference") or "Analysis result available" not in w.text('.shogi-analysis-state'):
                    raise
                self.record("Xiangqi completion removed cancellation control before input", {"driver_error": str(error), "visible_status": w.text('.shogi-analysis-state')})
        wait(lambda: "Search stopped" in w.text('.shogi-analysis-state')
             or "Analysis result available" in w.text('.shogi-analysis-state'), "bounded Xiangqi analysis settles")
        self.xiangqi_events(2, selected=2)
        self.record("Actual bounded Xiangqi analysis completion/cancellation observation", {
            "cancel_dispatched": cancel_dispatched, "visible_status": w.text('.shogi-analysis-state'),
            "native": self.xiangqi_diagnostics(),
            "qualification": "Completion may beat cancellation at the native depth3/512-visit limit; no cancellation is inferred."})
        self.xiangqi_analysis()
        self.xiangqi_complete_analysis(2)
        self.xiangqi_events(2, selected=2)
        self.capture("analysis")
        path = self.args.output / "normal-xiangqi-complete.mshogi"
        archive = self.xiangqi_save(path)
        body = archive["payload"]
        assert len(body["current"]["payload"]["events"]) == 2 and len(body["observations"]) >= 2, body
        old = self.xiangqi_diagnostics()
        self.file("open", path)
        wait(lambda: (now := self.xiangqi_diagnostics()) and now["token"]["document"] > old["token"]["document"], "Xiangqi Open installs fresh document identity")
        self.xiangqi_events(2, selected=2)
        assert w.has(".play-paused") and w.read("document.querySelector('.shell-xiangqi').dataset.paused") == "true"
        assert not w.has('.shogi-analysis[data-phase="analyzing"]')
        reopened = self.xiangqi_diagnostics()
        assert reopened["context"] == old["context"], (old, reopened)
        assert reopened["token"]["selection"] > old["token"]["selection"], (old, reopened)
        self.record("Xiangqi current document Open preserves native history and retires runtime work", {"before": old, "after": reopened})
        self.capture("reopened-stopped")

        w.click('.shogi-history [data-event="1"]')
        self.xiangqi_events(2, selected=1, historical=True)
        selected = self.xiangqi_diagnostics()
        assert len(selected["context"]["payload"]["events"]) == 1, selected
        assert selected["context"]["payload"]["events"][0]["event"]["action"] == "a3a4", selected
        self.xiangqi_analysis()
        self.xiangqi_complete_analysis(1)
        self.xiangqi_events(2, selected=1, historical=True)
        history_path = self.args.output / "normal-xiangqi-history.mshogi"
        historical = self.xiangqi_save(history_path)["payload"]
        context = historical["observations"][-1]["observation"]["context"]
        assert context["purpose"]["kind"] == "move_analysis", context
        assert len(context["game"]["payload"]["events"]) == 1, context
        assert len(historical["current"]["payload"]["events"]) == 2, historical
        self.capture("historical-analysis")
        old = self.xiangqi_diagnostics()
        self.file("open", history_path)
        wait(lambda: (now := self.xiangqi_diagnostics()) and now["token"]["document"] > old["token"]["document"], "historical Xiangqi Open establishes fresh authority")
        self.xiangqi_events(2, selected=1, historical=True)
        assert self.xiangqi_diagnostics()["context"] == old["context"]
        self.capture("reopened-history")
        w.button("Return to live")
        self.xiangqi_events(2, selected=2)
        assert w.has(".play-paused")
        w.button("Resume play")
        wait(lambda: w.read("document.querySelector('.shell-xiangqi').dataset.paused") == "false", "explicit Xiangqi Resume")
        self.xiangqi_events(2, selected=2)
        self.capture("return-live-resumed")
        w.button("Save")
        self.wait_saved(history_path)

        corrupt = self.args.output / "normal-xiangqi-corrupt.mshogi"
        corrupt.write_text('{"format":"mshogi.desktop.document.v1",', encoding="utf-8")
        before = self.xiangqi_diagnostics()
        self.file("open", corrupt)
        wait(lambda: (now := self.xiangqi_diagnostics()) and now["io"]["id"] > before["io"]["id"] and now["io"]["phase"] == "failed", "corrupt Xiangqi Open reports native failure")
        after = self.xiangqi_diagnostics()
        assert xiangqi_retained_state(after) == xiangqi_retained_state(before), {"before": before, "after": after}
        assert "current game" in w.text('.shell-notices').lower()
        self.record("Corrupt Open preserves complete accepted Xiangqi document and authority", {"before": before, "after": after})
        self.capture("corrupt-open-preserved")
        w.click(".flip-button")
        wait(lambda: w.text('.document-dirty').casefold() == "unsaved changes", "Xiangqi orientation is protected document state")
        self.record("Native Xiangqi window-close request", self.native.close_window())
        wait(lambda: w.has(".document-guard"), "native window-close protects Xiangqi unsaved changes")
        self.capture("native-close-guard")
        w.button("Cancel")
        wait(lambda: not w.has(".document-guard"), "cancel native Xiangqi close")
        assert self.native.alive()
        self.xiangqi_events(2, selected=2)
        self.quit()

    def normal_shogi(self):
        self.start(product="shogi")
        w = self.web
        self.new_shogi("sente")
        self.viewport(1180, 820); self.shogi_board_fits()
        self.shared_shell('shogi')
        self.shogi_events(0)
        self.inspect_piece('shogi','7c','7d','Gote pawn')
        self.capture("initial-sente")
        self.viewport(760, 650); self.shogi_board_fits()
        self.shared_shell('shogi')
        self.capture("minimum-size")
        self.inspect_piece('shogi','7c','7d','Gote pawn')
        self.viewport(1180, 820)
        self.new_shogi("gote")
        self.shogi_events(1)
        self.shogi_board_fits()
        self.inspect_piece('shogi','5g','5f','Sente pawn')
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
        self.moves_navigation("shogi", "7g7f", "７６歩", 1, "7g", "7f")

        w.focused_button("Take back")
        self.shogi_events(0)
        assert self.completed_cue() is None, "Shogi Take back to root retires cue"
        wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "true", "Shogi Take back is stopped")
        self.capture("takeback-stopped")
        w.button("Resume play")
        wait(lambda: w.read("document.querySelector('.game-shell').dataset.paused") == "false", "explicit Shogi Resume after Take back")
        w.click(self.shogi_square("7g")); w.click(self.shogi_square("7f"))
        self.shogi_events(2)
        self.record("Shared Take back / stopped Shogi native decision / explicit Resume / replay", self.inspection_state())

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
        self.shogi_analysis_tab()
        w.click(".recorded-searches summary")
        assert "Saved observation" in w.text(".recorded-searches")
        w.click(".recorded-searches summary")
        self.capture("reopened-stopped")
        w.click('.shogi-history [data-event="1"]')
        self.shogi_events(2, selected=1, historical=True)
        assert "live game is unchanged" in w.text(".shell-history-notice")
        self.inspect_piece('shogi','7c','7d','Gote pawn')
        self.shogi_analysis()
        self.shogi_complete_analysis()
        self.shogi_events(2, selected=1, historical=True)
        self.capture("historical-analysis")
        archive = self.shogi_save(self.args.output / "normal-shogi-history.mshogi")
        body = archive["payload"]
        context = body["observations"][-1]["observation"]["context"]
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
        assert not w.has(".shell-history-notice")
        self.capture("return-live-resumed")
        # The current native writer reconstructs this reduced-material fixture
        # from its recorded root and complete actions; historical MSH-74 evidence
        # remains unchanged. No fixture entry point is enabled in this package.
        captures=Path(__file__).with_name('fixtures')/'shogi-captures.mshogi'
        assert captures.is_file(),captures
        self.file('open',captures)
        self.shogi_events(2)
        wait(lambda:w.has('.shogi-player[data-player="gote"] .hand-piece[data-hand-piece="pawn"]'),'normal reopened opponent hand resource')
        before_archive=self.shogi_save(self.args.output/'normal-shogi-inspection-before.mshogi')
        before=self.inspection_state()
        hand='.shogi-player[data-player="gote"] .hand-piece[data-hand-piece="pawn"]'
        w.click(hand)
        wait(lambda:'Inspecting Gote pawn in hand' in w.text('.movement-inspection-status'),'native opponent hand/drop inspection')
        assert w.read(f"document.querySelector({json.dumps(self.shogi_square('5e'))})?.querySelector('.inspection-marker')!==null")
        assert not w.has('.shogi-square.target')
        self.capture('opponent-hand-inspection')
        w.key('\ue00c')
        w.key('\ue007')  # Enter on the returned hand button starts a new query.
        wait(lambda:'Inspecting Gote pawn in hand' in w.text('.movement-inspection-status'),'keyboard opponent hand inspection')
        for key in ('\ue015','\ue013','\ue013','\ue007'):  # Down to board, Up twice, Enter at5e.
            w.key(key)
        time.sleep(0.35)
        self.assert_inspection_unchanged(before, 'Opponent hand inspection complete route preserves authoritative context')
        assert not w.has('.shogi-promotion')
        w.key('\ue00c')
        wait(lambda:not w.has('.inspection-marker'),'Escape clears opponent drop inspection')
        self.assert_inspection_unchanged(before, 'Opponent hand inspection Escape preserves authoritative context')
        after_archive=self.shogi_save(self.args.output/'normal-shogi-inspection-after.mshogi')
        assert after_archive==before_archive,'read-only hand inspection changed canonical native archive'
        self.record('Normal-package opponent hand inspection preserves complete canonical archive',{
            'source_archive':str(captures),'source_sha256':digest(captures),
            'root':before_archive['payload']['current']['payload']['root'],
            'native_events':len(before_archive['payload']['current']['payload']['events']),
            'before_sha256':digest(self.args.output/'normal-shogi-inspection-before.mshogi'),
            'after_sha256':digest(self.args.output/'normal-shogi-inspection-after.mshogi')})
        self.shogi_flip()  # Keep the native unsaved-close guard journey meaningful.
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
                self.assert_completed_cue("P*5e", 3, [], ["5e"])
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
            assert w.read("[...document.querySelectorAll('.shogi-game-actions button')].map(e=>({label:e.textContent.trim(),enabled:!e.disabled}))") == [{"label": "Take back", "enabled": True}], "A concluded game exposes only native Take back"
            w.focused_button("Take back")
            self.shogi_events(0)
            wait(lambda: "Sente to move" in w.text("[data-game-status]"), "declaration Take back restores native human decision")
            assert w.read("document.querySelector('.shogi-application').dataset.paused") == "true"
            assert w.read("document.querySelector('.take-back-button').disabled")
            assert "Position restored" in w.text(".play-paused")
            self.capture("takeback-declaration")
            path = self.args.output / "shogi-takeback-declaration.mshogi"
            archive = self.shogi_save(path)
            body = desktop_document_payload(archive, "shogi")
            commands = [entry["command"]["kind"] for entry in body["controls"] if entry["command"] is not None]
            assert "declare" in commands and "take_back" in commands, commands
            assert commands.index("declare") < commands.index("take_back"), commands
            assert not body["current"]["payload"]["events"], body["current"]
            old_token = self.native_diagnostics()["token"]
            assert old_token is not None
            self.file("open", path)
            wait(lambda: (token := self.native_diagnostics()["token"]) and token["document"] > old_token["document"],
                 "Take back archive Open installs fresh native document identity")
            self.shogi_events(0, selected=0, historical=False)
            wait(lambda: "Sente to move" in w.text("[data-game-status]"), "reopened Take back retains the native ongoing root")
            assert "wins" not in w.text("[data-game-status]")
            assert w.read("document.querySelector('.shogi-application').dataset.paused") == "true"
            assert not w.has('.shogi-analysis[data-phase="analyzing"]')
            self.capture("takeback-reopened-stopped")
            w.button("Resume play")
            wait(lambda: w.read("document.querySelector('.shogi-application').dataset.paused") == "false", "explicit Resume after Take back archive Open")
            self.shogi_events(0, selected=0, historical=False)
            self.record("Declare / Take back / native Save Open / stopped identities / explicit Resume", {
                "path": str(path), "sha256": digest(path), "retained_commands": commands,
                "saved_native_events": len(body["current"]["payload"]["events"]),
                "old_token": old_token, "fresh_token": self.native_diagnostics()["token"],
                "status": w.text("[data-game-status]")})
            self.capture("takeback-reopened-resumed")
        elif name == "shogi-concluded":
            self.shogi_events(1)
            assert "Sente resigned" in w.text("[data-game-status]")
            assert w.read("[...document.querySelectorAll('.shogi-game-actions button')].map(e=>({label:e.textContent.trim(),enabled:!e.disabled}))") == [{"label": "Take back", "enabled": True}], "A concluded game exposes only native Take back"
            self.capture("concluded")
            w.click('.shogi-history [data-event="0"]')
            self.shogi_events(1, selected=0, historical=True)
            self.capture("before-terminal-event")
            w.click(".moves-events summary")
            w.click('.shogi-history [data-event="1"]')
            self.shogi_events(1, selected=1, historical=True)
            w.button("Return to live")
            self.shogi_events(1, selected=1)
            assert "Sente resigned" in w.text("[data-game-status]")
            self.record("Select distinct native terminal event at unchanged board ply", w.text(".shogi-history"))
            w.focused_button("Take back")
            self.shogi_events(0)
            wait(lambda: "Sente to move" in w.text("[data-game-status]"), "resignation Take back restores native human decision")
            assert w.read("document.querySelector('.shogi-application').dataset.paused") == "true"
            assert w.read("document.querySelector('.take-back-button').disabled")
            assert "Position restored" in w.text(".play-paused")
            self.capture("takeback-resignation")
        elif name == "shogi-capacity":
            self.shogi_analysis_tab()
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
                wait(lambda: "Engine connection failed" in w.text(".shell-notices"), "genuine missing engine failure")
                self.plies(3)
                assert not w.has(".outcome-card")
                self.capture("engine-failure")
                self.record("Genuine missing-engine startup failure preserves three plies", w.text(".shell-notices"))
            finally:
                if held.exists():
                    held.rename(engine)
            assert digest(engine) == original
            w.button("Retry engine turn")
            self.plies(4)
            assert not w.has(".outcome-card")
            self.record("Restore identical sidecar and Retry without losing history", {
                "engine_sha256": digest(engine), "result": w.text('[data-search-purpose="engine_turn"]')})
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
            w.button("Analyze position")
            assert saved_chess_evidence_count(w.text(".recorded-chess-evidence summary"),255)
            w.click('[data-analysis-start="chess"]')
            wait(lambda:saved_chess_evidence_count(w.text(".recorded-chess-evidence summary"),256), "accept observation 256")
            w.click('[data-analysis-start="chess"]')
            wait(lambda: "Application limit reached" in w.text(".capacity-notice"), "reject operation before observation 257")
            wait(lambda: w.read("""(() => {
                const notice = document.querySelector('.capacity-notice');
                const sidebar = document.querySelector('.sidebar');
                if (!notice || !sidebar || document.activeElement !== notice) return false;
                const rect = notice.getBoundingClientRect(), box = sidebar.getBoundingClientRect();
                return rect.top >= Math.max(0, box.top) && rect.bottom <= Math.min(innerHeight, box.bottom);
            })()"""), "new capacity pause is focused and visible in the actual inspector")
            assert saved_chess_evidence_count(w.text(".recorded-chess-evidence summary"),256)
            assert not w.has(".outcome-card")
            self.capture("capacity-pause")
            self.save(self.args.output / "capacity-complete.mshogi", 256)
            # A preference change keeps this temporary saved document dirty for
            # the native lifecycle guard; it does not remove any observation.
            w.click(".flip-button")
            wait(lambda: w.text(".document-dirty").casefold() == "unsaved changes", "protected preference change completes before Quit")
        elif name == "synthetic":
            assert "not a real shogi variant" in w.text()
            assert not w.has(".document-card")
            self.viewport(1180, 820)
            self.synthetic_board_fits("white")
            self.capture("full-board-white-normal-size")
            self.viewport(760, 650)
            self.synthetic_board_fits("white")
            self.capture("full-board-white-minimum-size")
            w.button("↕ Flip board")
            self.synthetic_board_fits("black")
            self.capture("full-board-black-minimum-size")
            self.viewport(1180, 820)
            self.synthetic_board_fits("black")
            self.capture("full-board-black-normal-size")
            self.synthetic_record([])

            w.key("\ue004")  # Actual Tab from Flip enters the board's single stop.
            wait(lambda: str(w.read("document.activeElement?.getAttribute('aria-label')")).startswith("i9,"),
                 "keyboard focus enters the fully visible native synthetic board")
            # Black orientation: left/down goes i9 -> j10. The entire board is
            # already visible; keyboard navigation is not a scrolling mechanism.
            for key in ("\ue007", "\ue012", "\ue015", "\ue007"):
                w.key(key)
            assert "No action submitted" in w.text(".route-selection")
            self.synthetic_record([])
            self.synthetic_board_fits("black")
            self.capture("partial-prefix-no-commit")
            w.key("\ue004"); w.key("\ue00c")  # Tab to route control, Escape cancels.
            wait(lambda: not w.has(".route-selection"), "Escape cancels staged selection")
            self.synthetic_record([])

            # Pointer selection and a real Cancel selection button also preserve
            # the untouched native record; no route prefix counts as an action.
            w.click(self.square("i9")); w.click(self.square("j10"))
            self.synthetic_record([])
            w.focused_button("Cancel selection")
            wait(lambda: not w.has(".route-selection"), "Cancel selection abandons native prefix")
            self.synthetic_record([])

            w.click(self.square("i9")); w.click(self.square("j10"))
            self.synthetic_record([])
            w.focused_button("Complete action here")
            terminal = self.synthetic_record(["136>153"])
            self.assert_completed_cue("136>153", 1, ["i9"], ["j10"])
            assert "strider" in w.read(f"document.querySelector({json.dumps(self.square('j10'))}).getAttribute('aria-label')")
            assert "empty" in w.read(f"document.querySelector({json.dumps(self.square('i9'))}).getAttribute('aria-label')")
            self.synthetic_board_fits("black")
            self.capture("terminal-prefix-complete-here")

            w.button("Reset synthetic fixture")
            self.synthetic_record([])
            # Establish the origin with pointer + Escape before keyboard input;
            # reset need not erase an otherwise valid roving-focus coordinate.
            w.click(self.square("i9")); w.key("\ue00c")
            self.synthetic_record([])
            for key in ("\ue007", "\ue012", "\ue015", "\ue007", "\ue014", "\ue013", "\ue007"):
                w.key(key)
            captured = self.synthetic_record(["136>153>136"])
            self.assert_completed_cue("136>153>136", 1, ["i9"], ["i9"], "i9>j10>i9")
            assert "empty" in w.read(f"document.querySelector({json.dumps(self.square('j10'))}).getAttribute('aria-label')")
            assert "strider" in w.read(f"document.querySelector({json.dumps(self.square('i9'))}).getAttribute('aria-label')")
            w.click(".synthetic-inspector details summary")
            self.capture("capture-route")

            w.button("Reset synthetic fixture")
            self.synthetic_record([])
            w.button("↕ Flip board")
            self.synthetic_board_fits("white")
            w.click(self.square("i9")); w.key("\ue00c")
            for key in ("\ue007", "\ue012", "\ue007", "\ue014", "\ue007"):
                w.key(key)
            alternate = self.synthetic_record(["136>135>136"])
            self.assert_completed_cue("136>135>136", 1, ["i9"], ["i9"], "i9>h9>i9")
            assert "stone" in w.read(f"document.querySelector({json.dumps(self.square('j10'))}).getAttribute('aria-label')")
            assert "strider" in w.read(f"document.querySelector({json.dumps(self.square('i9'))}).getAttribute('aria-label')")
            self.capture("alternate-route")
            self.record("Terminal prefix and distinct equal-endpoint routes retain their exact native effects", {
                "terminal_prefix": terminal, "capture_and_return": captured, "alternate_return": alternate,
                "native_expectation": "136>153 ends on j10; both return routes end on i9; only the j10 route removes its stone."})
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
    parser.add_argument("--phase", choices=["normal", "normal-shogi", "normal-xiangqi", "verification", "verification-shogi", "all"], default="all")
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
            "sha256": digest(shutil.which(args.driver) or args.driver),
            "version": subprocess.run([args.driver, "--version"], capture_output=True,
                text=True, timeout=5, check=False).stdout.strip()},
        "phases": []}
    journey = None
    try:
        journey = Journey(args, report)
        phases = selected_gui_phases(args.phase)
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
                elif fixture == "normal-xiangqi":
                    journey.normal_xiangqi()
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
