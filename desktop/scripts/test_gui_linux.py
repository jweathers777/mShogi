"""Focused external-adapter simulation; no app or GUI assertions."""
import importlib.util
from pathlib import Path
import threading
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

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

class LoadPickerTests(unittest.TestCase):
 def test_product_load_title_keeps_native_open_semantics_and_escape_cancellation(self):
  native=object.__new__(module.Native)
  native.output=Path('unused-mocked-load-output')
  native.pid=456
  native._event_lock=threading.Lock();native._counter=0
  native.alive=Mock(return_value=True)
  picker=dict(PICKER,title='Load mShogi game')
  native._windows=Mock(side_effect=[[picker],[]])
  native._wait=Mock(side_effect=lambda predicate,*args,**kwargs:predicate())
  native._activate=Mock();native.screenshot=Mock();native._key=Mock()
  native._event=Mock(side_effect=lambda event,**fields:dict(event=event,**fields))
  native._diagnostics=Mock()
  observed=native.choose_file('open',None)
  self.assertEqual(observed['window']['title'],'Load mShogi game')
  self.assertEqual(observed['picker_kind'],'open')
  self.assertTrue(observed['cancelled']);self.assertTrue(observed['picker_closed'])
  native._activate.assert_called_once_with(123)
  native._key.assert_called_once_with('Escape')
  native._diagnostics.assert_not_called()

class FilenameReadbackTests(unittest.TestCase):
 def setUp(self):
  self.native=object.__new__(module.Native)
  self.native.output=Path('unused-mocked-picker-output')
  self.native._key=Mock();self.native._run=Mock();self.native._activate=Mock();self.native.screenshot=Mock()
  self.native._windows=Mock(return_value=[PICKER.copy()])
  self.path='/home/runner/work/mShogi/game.mshogi'
 def test_invalid_requested_entry_is_rejected_before_native_input(self):
  for selected in ('', 'relative.mshogi', '/'+'a'*4096, '/tmp/bad\nname.mshogi', '/tmp/bad\x00name'):
   with self.subTest(selected=repr(selected[:40])):
    with self.assertRaises(ValueError):self.native._enter_picker_path(PICKER,selected,'invalid')
  self.native._key.assert_not_called();self.native._run.assert_not_called()
  self.native._activate.assert_not_called();self.native.screenshot.assert_not_called()
 def test_complete_native_entry_is_verified_without_input_replay(self):
  self.native._copy_picker_text=Mock(return_value=self.path)
  result=self.native._enter_picker_path(PICKER,self.path,'verified')
  self.assertEqual(result['accepted_readback'],self.path)
  self.assertFalse(result['observed_prefix_corrected'])
  self.assertEqual(self.native._run.call_count,1);self.native._activate.assert_not_called()
  self.native._key.assert_any_call('ctrl+l')
  self.assertFalse(any('Return' in call.args for call in self.native._key.call_args_list))
 def test_observed_missing_prefix_gets_one_verified_replacement_not_another_open(self):
  suffix='work/mShogi/game.mshogi';self.native._copy_picker_text=Mock(side_effect=[suffix,self.path])
  result=self.native._enter_picker_path(PICKER,self.path,'prefix')
  self.assertTrue(result['observed_prefix_corrected']);self.assertEqual(result['first_readback'],suffix)
  self.assertEqual(self.native._run.call_count,2);self.native._activate.assert_called_once_with(123)
  self.assertFalse(any('Return' in call.args or 'alt+o' in call.args for call in self.native._key.call_args_list))
 def test_unrelated_or_still_incorrect_entry_is_not_accepted(self):
  for copies in (['unrelated'],[''],['game.mshogi','work/mShogi/game.mshogi']):
   self.native._copy_picker_text=Mock(side_effect=copies)
   with self.assertRaises(RuntimeError):self.native._enter_picker_path(PICKER,self.path,'wrong')
 def test_unavailable_copy_gets_one_bound_input_replacement_and_exact_readback(self):
  self.native._copy_picker_text=Mock(side_effect=[module.FilenameReadbackUnavailable('empty entry'),self.path])
  result=self.native._enter_picker_path(PICKER,self.path,'unavailable')
  self.assertIsNone(result['first_readback']);self.assertEqual(result['accepted_readback'],self.path)
  self.assertEqual(result['unavailable_copy_replaced'],'empty entry');self.assertEqual(result['readback_attempts'],2)
  self.assertFalse(result['observed_prefix_corrected']);self.native._activate.assert_called_once_with(123)
  self.assertEqual(self.native._run.call_count,2)
  self.assertFalse(any('Return' in call.args or 'alt+o' in call.args for call in self.native._key.call_args_list))
 def test_second_unavailable_copy_is_not_retried_or_accepted(self):
  self.native._copy_picker_text=Mock(side_effect=[module.FilenameReadbackUnavailable('first'),module.FilenameReadbackUnavailable('second')])
  with self.assertRaisesRegex(module.FilenameReadbackUnavailable,'second'):
   self.native._enter_picker_path(PICKER,self.path,'twice')
  self.assertEqual(self.native._copy_picker_text.call_count,2);self.assertEqual(self.native._run.call_count,2)
 def test_unavailable_copy_never_redirects_replacement_to_missing_or_changed_picker(self):
  for windows in ([],[dict(PICKER,title='Other dialog')],[dict(PICKER,pid=789)]):
   with self.subTest(windows=windows):
    self.native._windows.return_value=windows
    self.native._copy_picker_text=Mock(side_effect=module.FilenameReadbackUnavailable('empty'))
    self.native._activate.reset_mock();self.native._run.reset_mock()
    with self.assertRaisesRegex(RuntimeError,'identity changed'):
     self.native._enter_picker_path(PICKER,self.path,'changed')
    self.native._activate.assert_not_called();self.assertEqual(self.native._run.call_count,1)
 def test_unrelated_copy_timeout_is_not_a_filename_retry(self):
  self.native._copy_picker_text=Mock(side_effect=TimeoutError('clipboard marker unavailable'))
  with self.assertRaisesRegex(TimeoutError,'marker unavailable'):
   self.native._enter_picker_path(PICKER,self.path,'marker')
  self.native._activate.assert_not_called();self.assertEqual(self.native._run.call_count,1)
 def test_only_entry_copy_timeout_is_classified_for_input_replacement(self):
  for marker_ready in (True,False):
   with self.subTest(marker_ready=marker_ready):
    self.native._wait=Mock(side_effect=[True,TimeoutError('no copied text')] if marker_ready else TimeoutError('marker failed'))
    with patch.object(module.subprocess,'run'):
     try:self.native._copy_picker_text()
     except TimeoutError as error:self.assertEqual(isinstance(error,module.FilenameReadbackUnavailable),marker_ready)
     else:self.fail('missing clipboard evidence must fail')
 def test_clipboard_marker_cannot_masquerade_as_native_entry(self):
  self.native._wait=Mock(side_effect=lambda predicate,*args,**kwargs: predicate() or predicate())
  self.native._run.side_effect=[SimpleNamespace(returncode=0,stdout='mshogi-native-picker-copy-pending'),
   SimpleNamespace(returncode=0,stdout='mshogi-native-picker-copy-pending'),SimpleNamespace(returncode=0,stdout=self.path)]
  with patch.object(module.subprocess,'run') as seed:
   self.assertEqual(self.native._copy_picker_text(),self.path)
  self.assertEqual(seed.call_args.kwargs['input'],'mshogi-native-picker-copy-pending')
  self.native._key.assert_called_once_with('ctrl+a','ctrl+c')

if __name__=='__main__':unittest.main(verbosity=2)
