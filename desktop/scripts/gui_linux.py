"""External Linux native UI adapter; never included in application packages.

Run the caller under dbus-run-session and xvfb-run with GDK_BACKEND=x11.
Required tools: xdotool, scrot, openbox. WebKitWebDriver is managed by the
caller's tauri-driver process. Native dialogs use the locked GTK3 backend.
"""
from __future__ import annotations

import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import threading
import time


class Native:
    """Operate only the bound application's native X11 windows."""

    timeout = 20.0

    def __init__(self, output: Path):
        self.output = Path(output).resolve()
        self.output.mkdir(parents=True, exist_ok=True)
        self.events: list[dict] = []
        self.host: Path | None = None
        self.engine: Path | None = None
        self.pid: int | None = None
        self.window: int | None = None
        self._start: str | None = None
        self._wm: subprocess.Popen | None = None
        self._wm_log = None
        self._counter = 0
        self._event_lock = threading.Lock()
        if not os.environ.get("DISPLAY"):
            raise RuntimeError("Linux GUI verification requires an X11 DISPLAY (use xvfb-run)")
        for tool in ("xdotool", "scrot", "openbox"):
            if shutil.which(tool) is None:
                raise RuntimeError(f"Required external native UI tool missing: {tool}")
        desktop = self._run(["xdotool", "get_desktop"], check=False)
        if desktop.returncode != 0 or not desktop.stdout.strip().isdigit():
            self._wm_log = (self.output / "openbox.log").open("w")
            self._wm = subprocess.Popen(
                ["openbox", "--sm-disable"], stdout=self._wm_log,
                stderr=subprocess.STDOUT,
            )
            try:
                self._wait(self._desktop_ready, "Openbox did not establish an X11 desktop")
            except Exception:
                self.cleanup()
                raise
            self._event("desktop_started", process_id=self._wm.pid, tool="openbox")
        else:
            self._event("desktop_reused", desktop=desktop.stdout.strip())

    @staticmethod
    def _run(args: list[str], *, check: bool = True, timeout: float = 5.0):
        result = subprocess.run(args, capture_output=True, text=True, timeout=timeout)
        if check and result.returncode != 0:
            raise RuntimeError(
                f"Native command failed ({result.returncode}): {args!r}; "
                f"stdout={result.stdout.strip()!r}; stderr={result.stderr.strip()!r}"
            )
        return result

    def _event(self, kind: str, **values) -> dict:
        event = {
            "at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "kind": kind, **values,
        }
        with self._event_lock:
            self.events.append(event)
        return event

    def _desktop_ready(self):
        if self._wm is not None and self._wm.poll() is not None:
            raise RuntimeError(f"Openbox exited with {self._wm.returncode}; see openbox.log")
        result = self._run(["xdotool", "get_desktop"], check=False)
        return result.returncode == 0 and result.stdout.strip().isdigit()

    def _wait(self, predicate, message: str, timeout: float | None = None):
        deadline = time.monotonic() + (self.timeout if timeout is None else timeout)
        while True:
            result = predicate()
            if result:
                return result
            if time.monotonic() >= deadline:
                self._diagnostics("timeout")
                raise TimeoutError(message)
            time.sleep(0.1)

    @staticmethod
    def _identity(pid: int) -> dict | None:
        try:
            directory = Path("/proc") / str(pid)
            stat = (directory / "stat").read_text()
            # The comm field can contain spaces and parentheses; fields after
            # its final ')' begin at field 3. starttime is field 22.
            fields = stat.rsplit(")", 1)[1].split()
            executable = Path(os.readlink(directory / "exe")).resolve()
            return {
                "pid": pid, "parent_pid": int(fields[1]),
                "start_ticks": fields[19], "state": fields[0],
                "executable": str(executable),
                "command": (directory / "cmdline").read_bytes()
                    .replace(b"\0", b" ").decode("utf-8", "replace").strip(),
            }
        except (FileNotFoundError, ProcessLookupError, PermissionError, OSError, IndexError):
            return None

    @classmethod
    def _process_table(cls) -> list[dict]:
        return [identity for item in Path("/proc").iterdir()
                if item.name.isdigit()
                if (identity := cls._identity(int(item.name))) is not None]

    def processes(self) -> list[dict]:
        """Report the exact host, sibling sidecar, and observed descendants."""
        if self.host is None:
            return []
        table = self._process_table()
        wanted = {self.pid} if self.alive() else set()
        while True:
            expanded = wanted | {item["pid"] for item in table if item["parent_pid"] in wanted}
            if expanded == wanted:
                break
            wanted = expanded
        paths = {str(self.host), str(self.engine)}
        return [item for item in table if item["pid"] in wanted or item["executable"] in paths]

    @staticmethod
    def _observation(args: list[str]) -> dict:
        try:
            result = Native._run(args, check=False, timeout=5)
            return {"command": args, "exit_code": result.returncode,
                    "stdout": result.stdout.strip(), "stderr": result.stderr.strip()}
        except (OSError, subprocess.TimeoutExpired) as error:
            return {"command": args, "unavailable": str(error)}

    def environment(self) -> dict:
        """Capture actual tool/package versions; do not infer them from a label."""
        tools = {}
        for executable in ("xdotool", "scrot", "openbox", "WebKitWebDriver"):
            location = shutil.which(executable)
            tools[executable] = {"path": location}
            if location is not None:
                tools[executable]["sha256"] = hashlib.sha256(Path(location).read_bytes()).hexdigest()
                if executable != "WebKitWebDriver":
                    tools[executable]["version"] = self._observation([executable, "--version"])
        return {
            "os_release": Path("/etc/os-release").read_text(),
            "display": os.environ["DISPLAY"],
            "gdk_backend": os.environ.get("GDK_BACKEND"),
            "lang": os.environ.get("LANG"),
            "desktop_geometry": self._observation(["xdotool", "getdisplaygeometry"]),
            "tools": tools,
            "packages": self._observation([
                "dpkg-query", "-W", "-f=${binary:Package}=${Version}\n",
                "webkit2gtk-driver", "libwebkit2gtk-4.1-0", "libgtk-3-0*",
                "xdotool", "xvfb", "openbox", "scrot",
            ]),
            "native_observation": "X11 window title, geometry and owner PID; optional xprop properties. No accessibility-tree claim.",
        }

    def _windows(self) -> list[dict]:
        if self.pid is None:
            return []
        result = self._run([
            "xdotool", "search", "--onlyvisible", "--pid", str(self.pid), "--name", ".*",
        ], check=False)
        if result.returncode not in (0, 1):
            raise RuntimeError(f"Cannot enumerate bound native windows: {result.stderr}")
        windows = []
        for value in sorted(set(result.stdout.split())):
            if not value.isdigit():
                continue
            title = self._run(["xdotool", "getwindowname", value], check=False)
            owner = self._run(["xdotool", "getwindowpid", value], check=False)
            if title.returncode != 0 or owner.stdout.strip() != str(self.pid):
                continue
            geometry = self._run(["xdotool", "getwindowgeometry", "--shell", value], check=False)
            item = {"id": int(value), "pid": self.pid, "title": title.stdout.strip(),
                    "geometry": geometry.stdout.strip()}
            if shutil.which("xprop") is not None:
                item["properties"] = self._run([
                    "xprop", "-id", value, "WM_CLASS", "WM_TRANSIENT_FOR", "_NET_WM_PID",
                ], check=False).stdout.strip()
            windows.append(item)
        return windows

    def attach(self, host: Path, engine: Path | None = None) -> dict:
        """Bind one real process/window, failing instead of guessing among apps."""
        self.host = Path(host).resolve(strict=True)
        sibling = (self.host.parent / "mshogi").resolve(strict=True)
        self.engine = sibling if engine is None else Path(engine).resolve(strict=True)
        if self.engine != sibling:
            raise ValueError(f"Engine must be the host's actual sibling mshogi: {self.engine} != {sibling}")

        def locate():
            matches = [item for item in self._process_table() if item["executable"] == str(self.host)]
            if len(matches) > 1:
                raise RuntimeError(f"Ambiguous exact native host processes: {matches!r}")
            if not matches:
                return None
            identity = matches[0]
            self.pid, self._start = identity["pid"], identity["start_ticks"]
            windows = [window for window in self._windows() if window["title"].startswith("mShogi")]
            if len(windows) > 1:
                raise RuntimeError(f"Ambiguous main windows for exact host PID: {windows!r}")
            if not windows:
                return None
            self.window = windows[0]["id"]
            return {"process": identity, "window": windows[0], "host": str(self.host), "engine": str(self.engine)}

        try:
            bound = self._wait(locate, f"No visible main window for exact executable {self.host}")
            self._event("attached", **bound)
            (self.output / "linux-bound.json").write_text(json.dumps(bound, indent=2) + "\n")
            return bound
        except Exception:
            self._diagnostics("attach-failed")
            raise

    def screenshot(self, path: Path) -> None:
        path = Path(path).resolve()
        path.parent.mkdir(parents=True, exist_ok=True)
        self._run(["scrot", "--overwrite", str(path)])

    def _activate(self, window: int) -> None:
        if not self.alive() or window not in {item["id"] for item in self._windows()}:
            raise RuntimeError(f"Refusing native input to an unbound or missing window: {window}")
        self._run(["xdotool", "windowactivate", "--sync", str(window)])
        self._wait(lambda: self._run(["xdotool", "getactivewindow"], check=False).stdout.strip() == str(window),
                   f"Bound window {window} did not become active", timeout=5)

    def _key(self, *keys: str) -> None:
        # No --window: xdotool then uses real XTEST input to the focused window,
        # rather than XSendEvent events which GTK applications can reject.
        self._run(["xdotool", "key", "--clearmodifiers", *keys])

    def choose_file(self, kind: str, path: Path | None) -> dict:
        """Wait for, screenshot, and operate the bound process's real picker."""
        kind = kind.lower()
        titles = {"save": "Save mShogi game", "save_as": "Save mShogi game", "open": "Open mShogi game"}
        if kind not in titles:
            raise ValueError(f"Unknown native picker kind: {kind}")
        with self._event_lock:
            self._counter += 1
            sequence = self._counter
        stem = f"picker-{sequence:02d}-{kind}"

        def locate():
            if not self.alive():
                raise RuntimeError("Bound application exited before its native picker appeared")
            matches = [window for window in self._windows() if window["title"] == titles[kind]]
            if len(matches) > 1:
                raise RuntimeError(f"Ambiguous native pickers for the bound process: {matches!r}")
            return matches[0] if matches else None

        try:
            picker = self._wait(locate, f"Native {kind} picker did not appear for PID {self.pid}")
            self._activate(picker["id"])
            before = self.output / f"{stem}-native.png"
            self.screenshot(before)
            selected = None if path is None else str(Path(path).resolve())
            entered = None
            filesystem = None
            acceptance_fallback = None
            if selected is None:
                self._key("Escape")
            else:
                self._key("ctrl+l")
                # GTK's Save name entry can keep the extension outside its
                # initial basename selection. Replace the entire field so a
                # supplied .mshogi path does not acquire a second extension.
                self._key("ctrl+a")
                self._run(["xdotool", "type", "--clearmodifiers", "--delay", "1", "--", selected])
                entered = self.output / f"{stem}-entered-path.png"
                self.screenshot(entered)
                self._key("Return")
                # GTK can first resolve the location or its completion rather
                # than accept it. If Return leaves the picker open, activate
                # its actual Open/Save mnemonic and preserve that native state.
                deadline = time.monotonic() + 1.0
                while time.monotonic() < deadline:
                    if picker["id"] not in {item["id"] for item in self._windows()}:
                        break
                    time.sleep(0.1)
                else:
                    self._activate(picker["id"])
                    before_accept = self.output / f"{stem}-before-accept-fallback.png"
                    self.screenshot(before_accept)
                    mnemonic = "alt+o" if kind == "open" else "alt+s"
                    acceptance_fallback = {"key": mnemonic,
                                           "before_screenshot": str(before_accept)}
                    self._key(mnemonic)
            self._wait(lambda: picker["id"] not in {item["id"] for item in self._windows()},
                       f"Native {kind} picker did not close after {'Cancel' if selected is None else 'path selection'}")
            if not self.alive():
                raise RuntimeError("Application exited while operating its native picker")
            if selected is not None:
                selected_file = Path(selected)
                # The host writes asynchronously after GTK accepts Save. The
                # exact requested pathname must exist; an alternate filename
                # or appended extension never counts as a successful choice.
                if kind != "open":
                    self._wait(selected_file.is_file,
                               f"Native Save did not create the exact requested path: {selected}",
                               timeout=5)
                filesystem = {"path": selected, "is_file": selected_file.is_file()}
                if filesystem["is_file"]:
                    filesystem["bytes"] = selected_file.stat().st_size
            after = self.output / f"{stem}-after.png"
            self.screenshot(after)
            return self._event("native_picker", picker_kind=kind, window=picker,
                               selected_path=selected, cancelled=selected is None,
                               before_screenshot=str(before), after_screenshot=str(after),
                               entered_path_screenshot=None if entered is None else str(entered),
                               filesystem_observation=filesystem,
                               acceptance_fallback=acceptance_fallback,
                               picker_closed=True, method="Focused GTK picker through XTEST keyboard input")
        except Exception:
            self._diagnostics(f"{stem}-failed")
            raise

    def close_window(self) -> dict:
        if self.window is None:
            raise RuntimeError("Attach the native application before requesting close")
        self._activate(self.window)
        self._key("alt+F4")
        return self._event("native_close_requested", window=self.window,
                           method="Focused Alt+F4 through the X11 window manager", proof_of_exit=False)

    def resize(self, width: int, height: int) -> dict:
        """Resize only the bound main window using the real window manager."""
        if self.window is None or not self.alive():
            raise RuntimeError("Attach the native application before resizing")
        self._activate(self.window)
        self._run(["xdotool", "windowsize", "--sync", str(self.window), str(width), str(height)])
        return self._event("native_resize_requested", window=self.window,
                           requested={"width": width, "height": height},
                           observation=self._run(["xdotool", "getwindowgeometry", "--shell", str(self.window)]).stdout.strip())

    def quit_app(self) -> dict:
        if self.window is None:
            raise RuntimeError("Attach the native application before requesting Quit")
        self._activate(self.window)
        self._key("ctrl+q")
        return self._event("native_quit_requested", window=self.window,
                           method="Focused Ctrl+Q native Quit menu accelerator", proof_of_exit=False)

    def alive(self) -> bool:
        if self.pid is None or self.host is None:
            return False
        identity = self._identity(self.pid)
        return identity is not None and identity["executable"] == str(self.host) and identity["start_ticks"] == self._start

    def _diagnostics(self, stem: str) -> None:
        try:
            observation = {"windows": self._windows(), "processes": self.processes()}
            (self.output / f"{stem}-native.json").write_text(json.dumps(observation, indent=2) + "\n")
            self.screenshot(self.output / f"{stem}-desktop.png")
            self._event("native_diagnostics", name=stem, **observation)
        except Exception as error:
            self._event("native_diagnostics_failed", name=stem, error=str(error))

    def cleanup(self) -> None:
        """Failure teardown only; signals here are never successful Quit proof."""
        if self.alive():
            self._diagnostics("failure-cleanup")
            self._event("failure_cleanup", process_id=self.pid, proof_of_exit=False,
                        method="SIGTERM followed by SIGKILL if necessary; not a GUI close/Quit")
            try:
                os.kill(self.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            deadline = time.monotonic() + 2.0
            while self.alive() and time.monotonic() < deadline:
                time.sleep(0.1)
            if self.alive():
                try:
                    os.kill(self.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
        if self._wm is not None and self._wm.poll() is None:
            self._event("desktop_cleanup", process_id=self._wm.pid,
                        method="Stop only the Openbox instance started by this adapter")
            self._wm.terminate()
            try:
                self._wm.wait(timeout=2)
            except subprocess.TimeoutExpired:
                self._wm.kill()
                self._wm.wait(timeout=2)
        if self._wm_log is not None:
            self._wm_log.close()
