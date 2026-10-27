"""External driver boundary tests; these do not establish actual packaged GUI results."""
import copy
import http.client
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import Mock
import urllib.error

spec = importlib.util.spec_from_file_location('gui_smoke', Path(__file__).with_name('gui-smoke.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class SaveAcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.expected = '/games/new.mshogi'
        self.observation = {
            'path': self.expected, 'dirty': 'UNMODIFIED', 'statuses': [],
            'buttons': [{'label': label, 'enabled': True}
                        for label in ('New game', 'Save', 'Save as…', 'Open…')]}

    def test_already_saved_old_path_is_not_acceptance(self):
        self.observation['path'] = '/games/old.mshogi'
        self.assertFalse(module.saved_document_accepted(self.observation, self.expected))

    def test_new_path_busy_is_not_acceptance(self):
        self.observation['buttons'][2]['enabled'] = False
        self.assertFalse(module.saved_document_accepted(self.observation, self.expected))

    def test_new_path_dirty_is_not_acceptance(self):
        self.observation['dirty'] = 'Unsaved changes'
        self.assertFalse(module.saved_document_accepted(self.observation, self.expected))

    def test_new_path_settled_saved_is_acceptance(self):
        self.assertTrue(module.saved_document_accepted(self.observation, self.expected))

    def test_saving_or_opening_status_is_not_acceptance(self):
        for status in ('Saving your game…', 'Opening your game…'):
            with self.subTest(status=status):
                self.observation['statuses'] = [status]
                self.assertFalse(module.saved_document_accepted(self.observation, self.expected))

    def test_all_four_controls_are_required(self):
        self.observation['buttons'].pop()
        self.assertFalse(module.saved_document_accepted(self.observation, self.expected))


class LookupRetryTests(unittest.TestCase):
    def setUp(self):
        self.web = object.__new__(module.Webview)
        self.web.session = 'active-session'
        self.web.record = Mock()
        self.web._request_once = Mock()

    def test_each_read_only_lookup_retries_transport_once(self):
        for suffix in ('element', 'elements'):
            for error in (http.client.RemoteDisconnected('closed'), ConnectionResetError('reset'),
                          urllib.error.URLError(ConnectionResetError('wrapped reset'))):
                with self.subTest(suffix=suffix, error=type(error).__name__):
                    self.web._request_once = Mock(side_effect=[error, {'found': True}])
                    self.web.record.reset_mock()
                    path = f'/session/active-session/{suffix}'
                    self.assertEqual(self.web.request('POST', path, {'using': 'css selector', 'value': '.board'}), {'found': True})
                    self.assertEqual(self.web._request_once.call_count, 2)
                    first = self.web.record.call_args_list[0].args[1]
                    self.assertEqual((first['method'], first['path'], first['attempt'], first['retry']), ('POST', path, 1, True))
                    self.assertIn('error', first)
                    self.assertEqual(self.web.record.call_args_list[-1].args[1]['attempt'], 2)

    def test_repeated_transport_failure_raises_after_one_retry(self):
        self.web._request_once.side_effect = [ConnectionResetError('first'), ConnectionResetError('second')]
        with self.assertRaisesRegex(ConnectionResetError, 'second'):
            self.web.request('POST', '/session/active-session/element', {})
        self.assertEqual(self.web._request_once.call_count, 2)
        self.assertFalse(self.web.record.call_args_list[-1].args[1]['retry'])

    def test_mutations_and_other_commands_are_never_retried(self):
        for method, path in (
            ('POST', '/session'), ('POST', '/session/active-session/element/node/click'),
            ('POST', '/session/active-session/element/node/value'),
            ('POST', '/session/active-session/actions'), ('POST', '/session/active-session/execute/sync'),
            ('DELETE', '/session/active-session'), ('GET', '/session/active-session/screenshot'),
            ('POST', '/session/other-session/element'), ('GET', '/session/active-session/element'),
            ('POST', '/session/active-session/element?extra=1')):
            with self.subTest(method=method, path=path):
                self.web._request_once = Mock(side_effect=http.client.RemoteDisconnected('closed'))
                self.web.record.reset_mock()
                with self.assertRaises(http.client.RemoteDisconnected):
                    self.web.request(method, path, {})
                self.web._request_once.assert_called_once()
                self.web.record.assert_not_called()

    def test_driver_semantic_errors_are_not_retried(self):
        self.web._request_once.side_effect = RuntimeError('no such element')
        with self.assertRaisesRegex(RuntimeError, 'no such element'):
            self.web.request('POST', '/session/active-session/element', {})
        self.web._request_once.assert_called_once()


class InspectionDiagnosticsTests(unittest.TestCase):
    def test_failure_retains_exact_before_and_after(self):
        journey = module.Journey(None, {})
        journey.record = Mock()
        before = {'nativeToken': {'revision': 3}, 'history': 'event 2'}
        after = copy.deepcopy(before)
        after['nativeToken']['revision'] = 4
        journey.inspection_state = Mock(return_value=after)
        with self.assertRaises(AssertionError):
            journey.assert_inspection_unchanged(before, 'hand invariant')
        self.assertEqual(journey.record.call_args.args[1], {
            'before': before, 'after': after, 'unchanged': False})

    def test_unchanged_comparison_retains_both_objects(self):
        journey = module.Journey(None, {})
        journey.record = Mock()
        before = {'nativeToken': {'revision': 3}}
        journey.inspection_state = Mock(return_value=copy.deepcopy(before))
        journey.assert_inspection_unchanged(before, 'board invariant')
        self.assertTrue(journey.record.call_args.args[1]['unchanged'])


if __name__ == '__main__':
    unittest.main(verbosity=2)
