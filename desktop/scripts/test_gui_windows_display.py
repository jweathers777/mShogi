"""External Windows display admission tests; actual GUI evidence is separate."""
import importlib.util
from pathlib import Path
from types import SimpleNamespace
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('gui_windows', Path(__file__).with_name('gui_windows.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


def mode(width, height, bits=32):
    return SimpleNamespace(PelsWidth=width, PelsHeight=height, BitsPerPel=bits, DisplayFrequency=60)


class Display:
    def __init__(self, current=None, modes=None, test_result=0, apply_result=0, changes=True):
        self.current = current or mode(1024, 768)
        self.modes = modes or []
        self.test_result, self.apply_result, self.changes = test_result, apply_result, changes
        self.calls = []

    def EnumDisplaySettings(self, device, index):
        if index == -1:
            return self.current
        if index >= len(self.modes):
            raise RuntimeError('no more modes')
        return self.modes[index]

    def MonitorFromPoint(self, point, flags):
        return 1

    def GetMonitorInfo(self, monitor):
        return {'Monitor': (0, 0, self.current.PelsWidth, self.current.PelsHeight),
                'Work': (0, 0, self.current.PelsWidth, self.current.PelsHeight - 40)}

    def ChangeDisplaySettings(self, selected, flags):
        self.calls.append((selected, flags))
        if flags == 2:
            return self.test_result
        if self.apply_result == 0 and self.changes:
            self.current = selected
        return self.apply_result


class WindowsDisplayTests(unittest.TestCase):
    constants = SimpleNamespace(ENUM_CURRENT_SETTINGS=-1, MONITOR_DEFAULTTOPRIMARY=1,
                                CDS_TEST=2, DISP_CHANGE_SUCCESSFUL=0)

    def test_sufficient_existing_desktop_needs_no_display_change(self):
        api = Display(current=mode(1920, 1080))
        evidence = module._display_preflight(api, self.constants)
        self.assertEqual(evidence['action'], 'existing_sufficient_desktop')
        self.assertEqual(api.calls, [])

    def test_only_reported_supported_32bit_mode_is_tested_then_applied(self):
        preferred = mode(1920, 1080)
        api = Display(modes=[mode(1280, 960), mode(2560, 1440), preferred, mode(1920, 1080, 16)])
        evidence = module._display_preflight(api, self.constants)
        self.assertEqual(api.calls, [(preferred, 2), (preferred, 0)])
        self.assertEqual(evidence['before']['width'], 1024)
        self.assertEqual(evidence['after']['work_area'], [0, 0, 1920, 1040])
        self.assertEqual(evidence['action'], 'supported_mode_applied')

    def test_no_sufficient_supported_mode_fails_without_arbitrary_mode(self):
        api = Display(modes=[mode(1024, 768), mode(1920, 1080, 16), mode(1440, 900)])
        evidence = module._display_preflight(api, self.constants)
        self.assertEqual(api.calls, [])
        self.assertEqual(evidence['action'], 'insufficient_interactive_display')

    def test_rejected_test_or_apply_is_not_reported_as_changed_display(self):
        for test_result, apply_result in [(-2, 0), (0, -1)]:
            api = Display(modes=[mode(1920, 1080)], test_result=test_result, apply_result=apply_result)
            evidence = module._display_preflight(api, self.constants)
            self.assertEqual(evidence['action'], 'insufficient_interactive_display')
            self.assertEqual(evidence['after']['width'], 1024)
            self.assertEqual(evidence['attempts'][0]['test_result'], test_result)
            self.assertEqual(len(api.calls), 1 if test_result else 2)

    def test_success_return_without_actual_geometry_change_is_not_pass(self):
        api = Display(modes=[mode(1920, 1080)], changes=False)
        with patch.object(module.time, 'monotonic', side_effect=[0, 10]):
            evidence = module._display_preflight(api, self.constants)
        self.assertEqual(evidence['attempts'][0]['apply_result'], 0)
        self.assertEqual(evidence['action'], 'insufficient_interactive_display')

    def test_capture_requires_entire_native_frame_inside_work_area(self):
        self.assertTrue(module._contains([0, 0, 1920, 1040], [20, 20, 1216, 899]))
        self.assertFalse(module._contains([0, 0, 1024, 728], [20, 20, 1216, 899]))
        self.assertFalse(module._contains([0, 0, 1920, 1040], [-1, 20, 760, 650]))
        self.assertFalse(module._contains([0, 0, 1920, 1040], [20, 20, 20, 650]))

    def test_default_windows_cascade_is_placed_inside_without_resizing(self):
        self.assertEqual(module._capture_position([0, 0, 1920, 1040], [234, 221, 1430, 1080]), (20, 20))
        self.assertEqual(module._capture_position([0, 0, 1196, 859], [234, 221, 1430, 1080]), (0, 0))
        self.assertEqual(module._capture_position([-1920, 0, 0, 1040], [-1800, 221, -604, 1080]), (-1900, 20))

    def test_initial_placement_refuses_a_window_that_cannot_fit(self):
        with self.assertRaisesRegex(RuntimeError, 'cannot fit'):
            module._capture_position([0, 0, 1024, 728], [20, 20, 1216, 899])


if __name__ == '__main__':
    unittest.main()
