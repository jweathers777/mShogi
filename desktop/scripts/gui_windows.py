"""External Windows GUI adapter for installed-package verification only.

WebDriver drives the webview. This module drives real native file pickers and
window/menu close requests; it never calls application IPC or changes game state.
"""
from __future__ import annotations

import ctypes
from ctypes import wintypes
from datetime import datetime, timezone
import importlib.metadata
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys
import threading
import time

_TIMEOUT = 20.0
_RUNTIME_KEY = r"Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"


def _windows():
    if sys.platform != "win32":
        raise RuntimeError("The native Windows GUI helper requires Windows")
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    user = ctypes.WinDLL("user32", use_last_error=True)
    kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    kernel.OpenProcess.restype = wintypes.HANDLE
    kernel.QueryFullProcessImageNameW.argtypes = [wintypes.HANDLE, wintypes.DWORD, wintypes.LPWSTR, ctypes.POINTER(wintypes.DWORD)]
    kernel.QueryFullProcessImageNameW.restype = wintypes.BOOL
    kernel.CloseHandle.argtypes = [wintypes.HANDLE]
    kernel.CloseHandle.restype = wintypes.BOOL
    kernel.GetUserDefaultUILanguage.argtypes = []
    kernel.GetUserDefaultUILanguage.restype = wintypes.WORD  # LANGID
    kernel.ProcessIdToSessionId.argtypes = [wintypes.DWORD, ctypes.POINTER(wintypes.DWORD)]
    kernel.ProcessIdToSessionId.restype = wintypes.BOOL
    user.GetProcessWindowStation.argtypes = []
    user.GetProcessWindowStation.restype = wintypes.HANDLE
    user.OpenInputDesktop.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    user.OpenInputDesktop.restype = wintypes.HANDLE
    user.CloseDesktop.argtypes = [wintypes.HANDLE]
    user.CloseDesktop.restype = wintypes.BOOL
    user.GetUserObjectInformationW.argtypes = [wintypes.HANDLE, ctypes.c_int, wintypes.LPVOID, wintypes.DWORD, ctypes.POINTER(wintypes.DWORD)]
    user.GetUserObjectInformationW.restype = wintypes.BOOL
    return kernel, user


def _canonical(path: Path | str) -> str:
    return os.path.normcase(os.path.realpath(str(path)))


def _image(pid: int) -> str | None:
    kernel, _ = _windows()
    process = kernel.OpenProcess(0x1000, False, pid)  # QUERY_LIMITED_INFORMATION
    if not process:
        return None
    try:
        size = wintypes.DWORD(32768)
        buffer = ctypes.create_unicode_buffer(size.value)
        if kernel.QueryFullProcessImageNameW(process, 0, buffer, ctypes.byref(size)):
            return buffer.value
        return None
    finally:
        kernel.CloseHandle(process)


def _process_rows() -> list[dict]:
    kernel, _ = _windows()

    class Entry(ctypes.Structure):
        _fields_ = [
            ("dwSize", wintypes.DWORD), ("cntUsage", wintypes.DWORD),
            ("pid", wintypes.DWORD), ("heap", ctypes.c_size_t),
            ("module", wintypes.DWORD), ("threads", wintypes.DWORD),
            ("parent", wintypes.DWORD), ("priority", wintypes.LONG),
            ("flags", wintypes.DWORD), ("exe", wintypes.WCHAR * 260),
        ]

    kernel.CreateToolhelp32Snapshot.argtypes = [wintypes.DWORD, wintypes.DWORD]
    kernel.CreateToolhelp32Snapshot.restype = wintypes.HANDLE
    kernel.Process32FirstW.argtypes = [wintypes.HANDLE, ctypes.POINTER(Entry)]
    kernel.Process32FirstW.restype = wintypes.BOOL
    kernel.Process32NextW.argtypes = [wintypes.HANDLE, ctypes.POINTER(Entry)]
    kernel.Process32NextW.restype = wintypes.BOOL
    snapshot = kernel.CreateToolhelp32Snapshot(2, 0)
    if snapshot in (None, ctypes.c_void_p(-1).value):
        raise ctypes.WinError(ctypes.get_last_error())
    rows = []
    try:
        entry = Entry()
        entry.dwSize = ctypes.sizeof(entry)
        found = kernel.Process32FirstW(snapshot, ctypes.byref(entry))
        while found:
            rows.append({"pid": entry.pid, "parent_pid": entry.parent, "name": entry.exe})
            found = kernel.Process32NextW(snapshot, ctypes.byref(entry))
    finally:
        kernel.CloseHandle(snapshot)
    return rows


def _runtime_registry() -> list[dict]:
    import winreg
    records = []
    locations = [
        ("HKLM", winreg.HKEY_LOCAL_MACHINE, "SOFTWARE\\WOW6432Node\\" + _RUNTIME_KEY),
        ("HKLM", winreg.HKEY_LOCAL_MACHINE, "SOFTWARE\\" + _RUNTIME_KEY),
        ("HKCU", winreg.HKEY_CURRENT_USER, "Software\\" + _RUNTIME_KEY),
    ]
    for hive_name, hive, path in locations:
        try:
            with winreg.OpenKey(hive, path, 0, winreg.KEY_READ | winreg.KEY_WOW64_64KEY) as key:
                version, _ = winreg.QueryValueEx(key, "pv")
            if re.fullmatch(r"\d+\.\d+\.\d+\.\d+", str(version)) and version != "0.0.0.0":
                records.append({"key": hive_name + "\\" + path, "version": version})
        except FileNotFoundError:
            pass
    return records


def _initialize_com():
    # pywinauto's shared UIA objects must be created in MTA, including when a
    # file-picker worker is the first thread to import the COM dependencies.
    sys.coinit_flags = 0
    import pythoncom
    try:
        pythoncom.CoInitializeEx(pythoncom.COINIT_MULTITHREADED)
        return pythoncom, True
    except pythoncom.com_error as error:
        # pythoncom or another CI library may already have initialized STA on
        # this thread. Keep that apartment and never unbalance its ownership.
        if error.hresult != -2147417850:  # RPC_E_CHANGED_MODE
            raise
        return pythoncom, False


class Native:
    """Only external native GUI interactions; paths/PIDs are scoped to this app."""

    def __init__(self, output: Path):
        self.output = Path(output)
        self.output.mkdir(parents=True, exist_ok=True)
        self.host: str | None = None
        self.engine: str | None = None
        self.pid: int | None = None
        self.hwnd: int | None = None
        self.events: list[dict] = []
        self._started = time.monotonic()
        self._serial = 0
        self._com_owner = None
        self._com_owner_thread = None

    def _keep_com_active(self) -> None:
        if self._com_owner is None:
            com, initialized = _initialize_com()
            if not initialized:
                raise RuntimeError("Native Windows GUI verification requires an MTA owner thread; another library initialized STA first")
            self._com_owner = com
            self._com_owner_thread = threading.get_ident()

    def _event(self, event: str, **fields) -> dict:
        value = {"event": event, "utc": datetime.now(timezone.utc).isoformat(),
                 "elapsed_ms": round((time.monotonic() - self._started) * 1000), **fields}
        self.events.append(value)
        return value

    def environment(self) -> dict:
        kernel, user = _windows()
        self._keep_com_active()
        session = wintypes.DWORD()
        if not kernel.ProcessIdToSessionId(os.getpid(), ctypes.byref(session)):
            raise ctypes.WinError(ctypes.get_last_error())

        class Flags(ctypes.Structure):
            _fields_ = [("inherit", wintypes.BOOL), ("reserved", wintypes.BOOL), ("flags", wintypes.DWORD)]

        flags, needed = Flags(), wintypes.DWORD()
        station = user.GetProcessWindowStation()
        visible = bool(user.GetUserObjectInformationW(station, 1, ctypes.byref(flags), ctypes.sizeof(flags), ctypes.byref(needed)) and flags.flags & 1)
        desktop = user.OpenInputDesktop(0, False, 1)  # DESKTOP_READOBJECTS
        name = ctypes.create_unicode_buffer(256)
        accessible = bool(desktop)
        if desktop:
            try:
                if not user.GetUserObjectInformationW(desktop, 2, name, ctypes.sizeof(name), ctypes.byref(needed)):
                    raise ctypes.WinError(ctypes.get_last_error())
            finally:
                user.CloseDesktop(desktop)
        versions = {}
        for package in ("pywinauto", "pywin32", "comtypes", "Pillow"):
            try:
                versions[package] = importlib.metadata.version(package)
            except importlib.metadata.PackageNotFoundError:
                versions[package] = None
        value = self._event("environment", os=platform.platform(), architecture=platform.machine(),
                            python=sys.version, ui_language_id=kernel.GetUserDefaultUILanguage(),
                            session_id=session.value, window_station_visible=visible,
                            input_desktop=name.value, input_desktop_accessible=accessible,
                            tools=versions, webview2_runtime_registry=_runtime_registry())
        if session.value == 0 or not visible or not accessible or name.value.casefold() != "default":
            self._diagnostics("interactive-desktop-preflight")
            raise RuntimeError("Native GUI verification requires an accessible interactive Default desktop, not session 0/a locked desktop")
        if versions["pywinauto"] != "0.6.9":
            self._diagnostics("native-tool-version")
            raise RuntimeError("CI native Windows GUI helper requires pywinauto==0.6.9")
        return value

    def _windows_owned(self, include_dialogs: bool = True) -> list[dict]:
        import win32gui
        import win32process
        values = []

        def collect(hwnd, _):
            if not win32gui.IsWindowVisible(hwnd):
                return
            _, pid = win32process.GetWindowThreadProcessId(hwnd)
            if self.pid is not None and pid != self.pid:
                return
            image = _image(pid)
            if self.host is not None and (image is None or _canonical(image) != self.host):
                return
            owner = win32gui.GetWindow(hwnd, 4)  # GW_OWNER
            if not include_dialogs and owner:
                return
            values.append({"hwnd": hwnd, "pid": pid, "title": win32gui.GetWindowText(hwnd),
                           "class": win32gui.GetClassName(hwnd), "owner": owner,
                           "image": image, "rectangle": list(win32gui.GetWindowRect(hwnd))})
        win32gui.EnumWindows(collect, None)
        return values

    def attach(self, host: Path, engine: Path | None = None) -> dict:
        self._keep_com_active()
        self.host = _canonical(Path(host).resolve(strict=True))
        self.engine = _canonical(engine) if engine is not None else None
        self.pid, self.hwnd = None, None
        deadline = time.monotonic() + _TIMEOUT
        while time.monotonic() < deadline:
            windows = [window for window in self._windows_owned(False)
                       if window["title"] == "mShogi" or window["title"].startswith("mShogi Verification")]
            if len(windows) == 1:
                selected = windows[0]
                self.pid, self.hwnd = selected["pid"], selected["hwnd"]
                return self._event("attach", **selected, host=self.host, engine=self.engine,
                                   webview2_processes=self._webview_processes())
            if len(windows) > 1:
                self._diagnostics("attach-ambiguous")
                raise RuntimeError(f"Ambiguous installed mShogi main windows: {windows}")
            time.sleep(0.1)
        self._diagnostics("attach-timeout")
        raise TimeoutError(f"No visible main window from exact installed executable {host}")

    def _webview_processes(self) -> list[dict]:
        import win32api
        rows = _process_rows()
        descendants = {self.pid}
        while True:
            enlarged = descendants | {row["pid"] for row in rows if row["parent_pid"] in descendants}
            if enlarged == descendants:
                break
            descendants = enlarged
        values = []
        for row in rows:
            if row["pid"] not in descendants or row["name"].casefold() != "msedgewebview2.exe":
                continue
            image = _image(row["pid"])
            if image:
                info = win32api.GetFileVersionInfo(image, "\\")
                ms, ls = info["FileVersionMS"], info["FileVersionLS"]
                values.append({**row, "path": image,
                               "version": f"{ms >> 16}.{ms & 65535}.{ls >> 16}.{ls & 65535}"})
        return values

    def processes(self) -> list[dict]:
        wanted = {path for path in (self.host, self.engine) if path is not None}
        values = []
        names = {Path(path).name.casefold() for path in wanted}
        for row in _process_rows():
            if row["name"].casefold() not in names:
                continue
            image = _image(row["pid"])
            if image is not None and _canonical(image) in wanted:
                values.append({**row, "path": image,
                               "role": "host" if _canonical(image) == self.host else "engine"})
        return values

    def alive(self) -> bool:
        image = _image(self.pid) if self.pid is not None else None
        return image is not None and _canonical(image) == self.host

    def screenshot(self, path: Path) -> None:
        from PIL import ImageGrab
        import win32gui
        import win32process

        def describe(hwnd):
            if not hwnd or not win32gui.IsWindow(hwnd):
                return None
            _, pid = win32process.GetWindowThreadProcessId(hwnd)
            return {"hwnd": hwnd, "pid": pid, "image": _image(pid),
                    "title": win32gui.GetWindowText(hwnd),
                    "class": win32gui.GetClassName(hwnd)}

        before = describe(win32gui.GetForegroundWindow())
        target = None
        action = "capture_diagnostic_without_live_bound_window"
        focus_error = None
        try:
            if self.pid is not None and self.hwnd is not None and self.alive():
                windows = self._windows_owned()
                main = next((window for window in windows if window["hwnd"] == self.hwnd), None)
                if main is None:
                    raise RuntimeError("Bound host main HWND is no longer visible")

                def owned_depth(window):
                    owner, seen = window["owner"], {window["hwnd"]}
                    while owner and owner not in seen:
                        if owner == self.hwnd:
                            return len(seen)
                        seen.add(owner)
                        owner = win32gui.GetWindow(owner, 4)  # GW_OWNER
                    return 0

                # The actual Tauri file picker can have GW_OWNER=0. Its exact
                # PID/executable, native class and application-defined title still
                # bind it to this host; do not raise the main window above it.
                picker_titles = {"Save mShogi game", "Open mShogi game", "Confirm Save As"}
                modals = [(max(owned_depth(window), int(window["title"] in picker_titles)), window)
                          for window in windows if window["class"] == "#32770"]
                modals = [(depth, window) for depth, window in modals if depth]
                active = [window for _, window in modals
                          if before and window["hwnd"] == before["hwnd"]]
                if active:
                    target = active[0]
                elif modals:
                    deepest = max(depth for depth, _ in modals)
                    choices = [window for depth, window in modals if depth == deepest]
                    if len(choices) != 1:
                        raise RuntimeError("Ambiguous owned native modal windows during capture")
                    target = choices[0]
                else:
                    target = main
                current = describe(target["hwnd"])
                if (current is None or current["pid"] != self.pid or current["image"] is None
                        or _canonical(current["image"]) != self.host):
                    raise RuntimeError("Screenshot target no longer belongs to the bound executable/PID")
                from pywinauto import Application
                Application(backend="win32").connect(handle=target["hwnd"], timeout=2).window(
                    handle=target["hwnd"]).set_focus()
                action = "focus_owned_modal" if target["hwnd"] != self.hwnd else "focus_bound_host"
                deadline = time.monotonic() + 2
                while win32gui.GetForegroundWindow() != target["hwnd"] and time.monotonic() < deadline:
                    time.sleep(0.05)
                foreground = describe(win32gui.GetForegroundWindow())
                if (foreground is None or foreground["hwnd"] != target["hwnd"]
                        or foreground["pid"] != self.pid or foreground["image"] is None
                        or _canonical(foreground["image"]) != self.host):
                    raise RuntimeError("Bound screenshot target did not become the foreground window")
        except Exception as error:
            focus_error = repr(error)
        # Always retain the complete, unaltered desktop, including on focus failure.
        Path(path).parent.mkdir(parents=True, exist_ok=True)
        ImageGrab.grab(all_screens=True).save(path)
        after = describe(win32gui.GetForegroundWindow())
        if target is not None and (after is None or after["hwnd"] != target["hwnd"]
                                   or after["pid"] != self.pid or after["image"] is None
                                   or _canonical(after["image"]) != self.host):
            focus_error = focus_error or "Foreground changed away from the bound target during capture"
        engine_consoles = []

        def collect_console(hwnd, _):
            if self.engine is None or not win32gui.IsWindowVisible(hwnd):
                return
            if win32gui.GetClassName(hwnd) != "ConsoleWindowClass":
                return
            window = describe(hwnd)
            if window and window["image"] and _canonical(window["image"]) == self.engine:
                engine_consoles.append(window)

        win32gui.EnumWindows(collect_console, None)
        if engine_consoles:
            focus_error = focus_error or "The bound bundled engine opened a visible native console"
        self._event("native_screenshot", path=str(path), action=action, target=target,
                    foreground_before=before, foreground_after=after,
                    engine_consoles=engine_consoles, focus_error=focus_error)
        if focus_error is not None:
            raise RuntimeError(f"Native screenshot foreground validation failed: {focus_error}")

    def _diagnostics(self, reason: str) -> None:
        self._serial += 1
        stem = self.output / f"native-{self._serial:03d}-{reason}"
        data = {"reason": reason, "windows": [], "errors": []}
        try:
            data["windows"] = self._windows_owned()
            from pywinauto import Desktop
            for window in data["windows"]:
                wrapper = Desktop(backend="uia").window(handle=window["hwnd"]).wrapper_object()
                window["controls"] = [
                    {"name": item.element_info.name[:300], "type": item.element_info.control_type,
                     "automation_id": item.element_info.automation_id}
                    for item in wrapper.descendants()[:400]
                ]
        except Exception as error:
            data["errors"].append(repr(error))
        try:
            self.screenshot(stem.with_suffix(".png"))
        except Exception as error:
            data["errors"].append(repr(error))
        stem.with_suffix(".json").write_text(json.dumps(data, indent=2), encoding="utf-8")
        self._event("failure_diagnostics", reason=reason, path=str(stem))

    @staticmethod
    def _button(dialog, title: str, auto_id: str):
        buttons = [button for button in dialog.descendants(control_type="Button")
                   if button.is_visible() and button.is_enabled()]
        named = [button for button in buttons if button.window_text().replace("&", "").strip().casefold() == title.casefold()]
        identified = [button for button in buttons if button.element_info.automation_id == auto_id]
        matched = [button for button in named if button.element_info.automation_id == auto_id]
        # The real Open picker exposes multiple Open dropdown buttons alongside
        # its ID 1 action. Prefer both native identifiers within this bound dialog.
        for choices in (matched, identified, named):
            if len(choices) == 1:
                return choices[0]
            if choices:
                raise RuntimeError(f"Expected one native {title} button, found {len(choices)}")
        raise RuntimeError(f"Expected one native {title} button, found 0")

    @staticmethod
    def _filename(dialog):
        candidates = []
        for edit in dialog.descendants(control_type="Edit"):
            if not edit.is_visible() or not edit.is_enabled():
                continue
            name = edit.element_info.name.casefold()
            auto_id = edit.element_info.automation_id
            score = 100 if "file name" in name or "filename" in name else 0
            if auto_id in ("1001", "1148", "FileNameControlHost"):
                score += 50
            candidates.append((score, edit))
        candidates.sort(key=lambda item: item[0], reverse=True)
        if not candidates or (len(candidates) > 1 and candidates[0][0] == candidates[1][0]):
            raise RuntimeError("Cannot uniquely identify the native file-name Edit; capture its UIA tree")
        if candidates[0][0] == 0 and len(candidates) != 1:
            raise RuntimeError("Native file-name control has no recognized label/automation ID")
        return candidates[0][1]

    def choose_file(self, kind: str, path: Path | None) -> dict:
        if self.pid is None:
            raise RuntimeError("Attach the installed host before arming its native picker")
        if kind not in ("save", "save_as", "open"):
            raise ValueError(f"Unknown native picker kind: {kind}")
        save = kind != "open"
        title = "Save mShogi game" if save else "Open mShogi game"
        deadline = time.monotonic() + _TIMEOUT
        com, initialized = None, False
        try:
            com, initialized = _initialize_com()
            from pywinauto import Desktop
            import win32gui
            observed = None
            while time.monotonic() < deadline:
                matches = [window for window in self._windows_owned() if window["title"] == title]
                if len(matches) == 1:
                    observed = matches[0]
                    break
                if len(matches) > 1:
                    raise RuntimeError(f"Ambiguous native pickers: {matches}")
                time.sleep(0.1)
            if observed is None:
                raise TimeoutError(f"Native picker did not appear: {title}")
            dialog = Desktop(backend="uia").window(handle=observed["hwnd"]).wrapper_object()
            self._serial += 1
            screenshot = self.output / f"native-{self._serial:03d}-{kind}-picker.png"
            self.screenshot(screenshot)
            control = None
            if path is None:
                self._button(dialog, "Cancel", "2").invoke()
            else:
                absolute = Path(path).resolve()
                control = self._filename(dialog)
                control_info = {"name": control.element_info.name,
                                "automation_id": control.element_info.automation_id,
                                "value_before": control.get_value()}
                # Real keyboard replacement commits through the common dialog's
                # own input path. ValuePattern alone left its default filename in
                # the observed Windows run, despite the native Save button closing.
                control.set_focus()
                if win32gui.GetForegroundWindow() != observed["hwnd"]:
                    raise RuntimeError("Native filename input did not retain its bound picker foreground")
                control.type_keys("^a{BACKSPACE}", set_foreground=False)
                escaped = "".join("{" + char + "}" if char in "+^%~{}()" else char
                                  for char in str(absolute))
                control.type_keys(escaped, with_spaces=True, set_foreground=False, pause=0.01)
                control_info["value_entered"] = control.get_value()
                if control_info["value_entered"] != str(absolute):
                    self._event("native_filename_rejected", dialog=observed,
                                expected=str(absolute), filename_control=control_info)
                    raise RuntimeError("Native filename readback differs from the requested absolute path")
                control.type_keys("{TAB}", set_foreground=False)
                control_info["value_after_blur"] = control.get_value()
                if control_info["value_after_blur"] != str(absolute):
                    self._event("native_filename_rejected", dialog=observed,
                                expected=str(absolute), filename_control=control_info)
                    raise RuntimeError("Native filename changed before accepting the picker")
                entered_screenshot = screenshot.with_name(screenshot.stem + "-entered.png")
                self.screenshot(entered_screenshot)
                self._event("native_filename_entered", dialog=observed, expected=str(absolute),
                            filename_control=control_info, screenshot=str(entered_screenshot))
                self._button(dialog, "Save" if save else "Open", "1").invoke()
                control = control_info
            while time.monotonic() < deadline:
                # Existing target files can raise the real Windows overwrite
                # confirmation. Only approve this scoped Save-as confirmation.
                if save and path is not None and Path(path).exists():
                    confirmations = [window for window in self._windows_owned()
                                     if window["title"] == "Confirm Save As"]
                    if len(confirmations) == 1:
                        confirmation = Desktop(backend="uia").window(handle=confirmations[0]["hwnd"]).wrapper_object()
                        self._button(confirmation, "Yes", "6").invoke()
                if not win32gui.IsWindow(observed["hwnd"]) or not win32gui.IsWindowVisible(observed["hwnd"]):
                    return self._event("native_file_picker", kind=kind, path=str(Path(path).resolve()) if path is not None else None,
                                       action="cancel" if path is None else "accept", dialog=observed,
                                       filename_control=control, screenshot=str(screenshot))
                time.sleep(0.1)
            raise TimeoutError(f"Native picker did not close after {'Cancel' if path is None else 'selection'}")
        except Exception:
            self._diagnostics(f"{kind}-picker-failure")
            raise
        finally:
            if initialized:
                com.CoUninitialize()

    def close_window(self) -> dict:
        import win32con
        import win32gui
        if not self.alive() or self.hwnd is None or not win32gui.IsWindow(self.hwnd):
            self._diagnostics("native-close-unavailable")
            raise RuntimeError("The attached installed host has no live native main window")
        win32gui.PostMessage(self.hwnd, win32con.WM_CLOSE, 0, 0)
        return self._event("native_wm_close", pid=self.pid, hwnd=self.hwnd)

    def quit_app(self) -> dict:
        if not self.alive() or self.hwnd is None:
            raise RuntimeError("The installed host is not alive")
        com, initialized = None, False
        try:
            com, initialized = _initialize_com()
            from pywinauto import Application
            from PIL import ImageGrab
            import win32gui
            import win32process
            if not win32gui.IsWindow(self.hwnd):
                raise RuntimeError("The bound native Quit window no longer exists")
            _, pid = win32process.GetWindowThreadProcessId(self.hwnd)
            if pid != self.pid or _canonical(_image(pid) or "") != self.host:
                raise RuntimeError("Native Quit HWND no longer belongs to the bound executable/PID")
            window = Application(backend="win32").connect(handle=self.hwnd, timeout=2).window(
                handle=self.hwnd).wrapper_object()
            window.set_focus()
            if win32gui.GetForegroundWindow() != self.hwnd:
                raise RuntimeError("Native Quit target did not become foreground")

            def label(item):
                return item.text().split("\t", 1)[0].replace("&", "").strip()

            menu = window.menu()
            if menu is None:
                raise RuntimeError("The bound application has no native menu")
            applications = [item for item in menu.items() if label(item) == "mShogi"]
            if len(applications) != 1:
                raise RuntimeError(f"Expected one native mShogi menu, found {len(applications)}")
            application = applications[0]
            submenu = application.sub_menu()
            quits = [] if submenu is None else [item for item in submenu.items()
                                                if label(item) == "Quit mShogi"]
            if len(quits) != 1 or not quits[0].is_enabled():
                raise RuntimeError("Expected one enabled native Quit mShogi menu item")
            quit_item = quits[0]
            # Use actual mouse input on observed native menu items. menu_select()
            # would send WM_COMMAND directly; Ctrl-Q did not reach the guard in
            # the actual Windows run, and its cause has not been established.
            application.click_input()
            rectangle = quit_item.rectangle()
            if rectangle.right <= rectangle.left or rectangle.bottom <= rectangle.top:
                raise RuntimeError("Native Quit menu item did not become visible")
            if win32gui.GetForegroundWindow() != self.hwnd:
                raise RuntimeError("Native Quit menu lost its bound foreground window")
            self._serial += 1
            screenshot = self.output / f"native-{self._serial:03d}-quit-menu.png"
            ImageGrab.grab(all_screens=True).save(screenshot)
            target = {"application_text": application.text(), "quit_text": quit_item.text(),
                      "quit_item_id": quit_item.item_id(),
                      "rectangle": [rectangle.left, rectangle.top, rectangle.right, rectangle.bottom]}
            self._event("native_menu_quit_target", pid=self.pid, hwnd=self.hwnd,
                        method="pywinauto.MenuItem.click_input", target=target,
                        foreground=win32gui.GetForegroundWindow(), screenshot=str(screenshot))
            if win32gui.GetForegroundWindow() != self.hwnd or not self.alive():
                raise RuntimeError("Native Quit target changed before the menu click")
            quit_item.click_input()
            return self._event("native_menu_quit", pid=self.pid, hwnd=self.hwnd,
                               method="pywinauto.MenuItem.click_input", target=target)
        except Exception:
            self._diagnostics("native-quit-failure")
            raise
        finally:
            if initialized:
                com.CoUninitialize()

    def cleanup(self) -> None:
        # This is failure containment only. A successful native-quit assertion
        # must observe processes() empty before entering this cleanup path.
        for process in self.processes():
            current = _image(process["pid"])
            if current is None or _canonical(current) != _canonical(process["path"]):
                continue
            self._event("failure_cleanup_not_quit_evidence", process=process)
            try:
                result = subprocess.run(["taskkill", "/PID", str(process["pid"]), "/T", "/F"],
                                        capture_output=True, text=True, timeout=5, check=False)
                self._event("cleanup_result", pid=process["pid"], returncode=result.returncode,
                            stdout=result.stdout[-2000:], stderr=result.stderr[-2000:])
            except Exception as error:
                self._event("cleanup_error", pid=process["pid"], error=repr(error))
        if self._com_owner is not None and self._com_owner_thread == threading.get_ident():
            self._com_owner.CoUninitialize()
            self._com_owner = None
            self._com_owner_thread = None
