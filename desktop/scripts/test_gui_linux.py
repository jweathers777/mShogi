"""Focused external-adapter simulation; no app or GUI assertions."""
import importlib.util
from pathlib import Path
from types import SimpleNamespace
import unittest
from unittest.mock import Mock

spec=importlib.util.spec_from_file_location('gui_linux',Path(__file__).with_name('gui_linux.py'))
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
PICKER={'id':123,'pid':456,'title':'Save mShogi game'}

class PickerFallbackTests(unittest.TestCase):
 def setUp(self):
  self.native=object.__new__(module.Native)
  self.native.output=Path('unused-mocked-picker-output')
  self.native.alive=Mock(return_value=True)
  self.native._windows=Mock(return_value=[PICKER.copy()])
  self.native._activate=Mock()
  self.native.screenshot=Mock()
  self.native._run=Mock(return_value=SimpleNamespace(returncode=0,stdout='123\n'))
  self.native._key=Mock()
 def fallback(self,kind='save'):
  return self.native._picker_fallback(PICKER,kind,'test-picker')
 def test_already_closed_sends_nothing(self):
  self.native._windows.return_value=[]
  self.assertIsNone(self.fallback());self.native._activate.assert_not_called();self.native._key.assert_not_called()
 def test_closes_between_enumeration_and_activation(self):
  self.native._windows.side_effect=[[PICKER.copy()],[]]
  self.native._activate.side_effect=RuntimeError('missing window')
  self.assertIsNone(self.fallback());self.native._key.assert_not_called()
 def test_closes_during_focus_wait(self):
  self.native._windows.side_effect=[[PICKER.copy()],[]]
  self.native._activate.side_effect=TimeoutError('focus wait')
  self.assertIsNone(self.fallback());self.native._key.assert_not_called()
 def test_still_open_save_uses_bound_fallback(self):
  self.assertEqual(self.fallback()['key'],'alt+s')
  self.native._activate.assert_called_once_with(123);self.native._key.assert_called_once_with('alt+s')
 def test_still_open_open_uses_bound_fallback(self):
  self.assertEqual(self.fallback('open')['key'],'alt+o');self.native._key.assert_called_once_with('alt+o')
 def test_still_open_activation_error_is_not_ignored(self):
  self.native._activate.side_effect=RuntimeError('activation failed')
  with self.assertRaisesRegex(RuntimeError,'activation failed'):self.fallback()
  self.native._key.assert_not_called()
 def test_host_exit_is_not_tolerated(self):
  self.native.alive.return_value=False
  with self.assertRaisesRegex(RuntimeError,'application exited'):self.fallback()
  self.native._key.assert_not_called()
 def test_changed_picker_identity_is_not_tolerated(self):
  self.native._windows.return_value=[dict(PICKER,title='Other dialog')]
  with self.assertRaisesRegex(RuntimeError,'identity changed'):self.fallback()
  self.native._key.assert_not_called()
 def test_closes_while_screenshot_is_taken(self):
  self.native._windows.side_effect=[[PICKER.copy()],[PICKER.copy()],[]]
  self.assertIsNone(self.fallback());self.native._key.assert_not_called()
 def test_unrelated_focus_never_receives_fallback(self):
  self.native._run.return_value=SimpleNamespace(returncode=0,stdout='999\n')
  with self.assertRaisesRegex(RuntimeError,'lost focus'):self.fallback()
  self.native._key.assert_not_called()

if __name__=='__main__':unittest.main(verbosity=2)
