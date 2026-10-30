"""External driver boundary tests; these do not establish actual packaged GUI results."""
import ast
import copy
import http.client
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import Mock, patch
import urllib.error

spec = importlib.util.spec_from_file_location('gui_smoke', Path(__file__).with_name('gui-smoke.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class SharedPresentationTests(unittest.TestCase):
    def test_inspection_status_uses_exact_shared_copy_and_native_labels(self):
        for label in ('Black pawn', 'Gote pawn', 'Gote pawn in hand', 'Black cannon'):
            with self.subTest(label=label):
                self.assertTrue(module.inspection_status_matches(f'Inspecting {label} · Esc clears', label))
                for text in (f'Inspecting {label} · Read-only · Esc clears',
                             f'Inspecting {label} · Hollow rings · Esc clears',
                             f'Inspecting {label}', 'Inspecting Wrong piece · Esc clears', None):
                    self.assertFalse(module.inspection_status_matches(text, label))

    def test_actual_numeric_budget_is_independent_of_locale_grouping(self):
        for value in ('5000', '5,000', '5.000', '5\u00a0000', '5\u202f000'):
            with self.subTest(value=value):
                self.assertTrue(module.play_limit_matches(f'Play limit: depth 2 · {value} nodes', 2, 5000))
        self.assertTrue(module.play_limit_matches('Play limit: depth 2 · 20000 nodes', 2, 20000))

    def test_wrong_or_missing_budget_is_not_accepted(self):
        for value in ('Untimed · bounded native search', 'Play limit: depth 3 · 5,000 nodes',
                      'Play limit: depth 2 · 5,001 nodes', 'Play limit: depth 2 · 20,000 nodes', None):
            with self.subTest(value=value):
                self.assertFalse(module.play_limit_matches(value, 2, 5000))

    def test_native_footer_accepts_singular_and_plural_in_live_and_history(self):
        journey = object.__new__(module.Journey)
        journey.web = Mock()
        for count, historical, actual in (
                (0, False, 'Your turn Live position · 0 plies'),
                (1, True, 'Read-only history · Selected position · 1 ply'),
                (1, False, 'Game concluded Live position · 1 ply'),
                (2, False, 'Play paused Live position · 2 plies')):
            with self.subTest(count=count, historical=historical):
                journey.web.text.return_value = actual
                with patch.object(module, 'wait', side_effect=lambda check, label: self.assertTrue(check(), label)):
                    journey.plies(count, historical)


class CurrentDocumentInspectionTests(unittest.TestCase):
    def document(self, game="chess"):
        profile, edition = {
            "chess": ("mshogi.chess.match.v1", "FIDE-2023-01-01+mshogi.chess.match.v1"),
            "shogi": ("mshogi.shogi.csa36.v1", "JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026"),
            "xiangqi": ("mshogi.xiangqi.wxf2018.v1", "WXF-2018+XIANGQI-P1-2026-10-27"),
        }[game]
        return {"envelope_version": 1, "format": "mshogi.desktop.document.v1",
                "game_kind": game, "rules_profile": profile, "profile_edition": edition,
                "native_schema_version": 1, "payload": {"observations": []}}

    def test_current_envelopes_expose_native_payload_without_reconstruction(self):
        for game in ("chess", "shogi", "xiangqi"):
            with self.subTest(game=game):
                document = self.document(game)
                self.assertIs(module.desktop_document_payload(document, game), document["payload"])

    def test_old_or_conflicting_envelopes_are_not_accepted_as_gui_evidence(self):
        for game in ("chess", "shogi", "xiangqi"):
            for field, value in (("envelope_version", 2), ("format", "mshogi.desktop.archive.v2"),
                                 ("game_kind", "unknown"), ("rules_profile", "unapproved"),
                                 ("profile_edition", "unapproved"), ("native_schema_version", 2),
                                 ("payload", []), ("schema_version", 2), ("profile", "legacy")):
                with self.subTest(game=game, field=field):
                    document = self.document(game)
                    document[field] = value
                    with self.assertRaises(AssertionError):
                        module.desktop_document_payload(document, game)
            for field in self.document(game):
                with self.subTest(game=game, missing=field):
                    document = self.document(game)
                    del document[field]
                    with self.assertRaises(AssertionError):
                        module.desktop_document_payload(document, game)


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


class BoardPointerReadinessTests(unittest.TestCase):
    def setUp(self):
        self.web = object.__new__(module.Webview)
        self.web.record = Mock()
        self.web.read = Mock()
        self.web.command = Mock()
        self.selector = '.square[aria-label^="e2,"]'
        self.ready = {"present": True, "enabled": True, "visible": True,
                      "containsHit": True, "x": 120, "y": 230,
                      "label": "e2, White pawn", "runtime": "Local engine"}

    def test_pending_board_is_observed_before_exactly_one_pointer_dispatch(self):
        for pending in ({"present": False, "enabled": False},
                        {**self.ready, "enabled": False},
                        {**self.ready, "visible": False},
                        {**self.ready, "containsHit": False}):
            with self.subTest(pending=pending):
                self.web.read = Mock(side_effect=[pending, self.ready])
                self.web.command.reset_mock()
                self.web.record.reset_mock()
                # Each observation precedes any input, including the ready one.
                def observe(*args):
                    self.web.command.assert_not_called()
                    return next(states)
                states = iter([pending, self.ready])
                self.web.read.side_effect = observe
                with patch.object(module.time, 'sleep'):
                    self.web.click(self.selector)
                self.assertEqual(self.web.read.call_count, 2)
                self.web.command.assert_called_once()
                method, path, payload = self.web.command.call_args.args
                self.assertEqual((method, path), ('POST', '/actions'))
                actions = payload['actions'][0]['actions']
                self.assertEqual([a['type'] for a in actions], ['pointerMove', 'pointerDown', 'pointerUp'])
                self.assertEqual((actions[0]['x'], actions[0]['y']), (120, 230))
                observations = [c.args[1] for c in self.web.record.call_args_list]
                self.assertEqual([o['ready'] for o in observations], [False, True])
                self.assertEqual(observations[0]['target'], pending)

    def test_unready_target_times_out_with_reason_and_without_any_input(self):
        for pending in ({"present": False, "enabled": False},
                        {**self.ready, "enabled": False},
                        {**self.ready, "visible": False},
                        {**self.ready, "containsHit": False}):
            with self.subTest(pending=pending):
                self.web.read.return_value = pending
                with patch.object(module.time, 'monotonic', side_effect=[0, 0, 26]), \
                        patch.object(module.time, 'sleep'):
                    with self.assertRaisesRegex(AssertionError, 'Timed out: board pointer target ready') as failure:
                        self.web.click(self.selector)
                last = ast.literal_eval(str(failure.exception).split('; last observation: ', 1)[1])
                self.assertEqual(ast.literal_eval(last)['target'], pending)
                self.web.command.assert_not_called()

    def test_exhausted_observation_transport_is_fatal_not_another_wait_budget(self):
        self.web.read.side_effect = ConnectionResetError('exhausted observation')
        with self.assertRaises(ConnectionResetError):
            self.web.click(self.selector)
        self.web.read.assert_called_once()
        self.web.command.assert_not_called()

    def test_pointer_delivery_failure_is_never_replayed(self):
        self.web.read.return_value = self.ready
        self.web.command.side_effect = http.client.RemoteDisconnected('after receipt')
        with self.assertRaises(http.client.RemoteDisconnected):
            self.web.click(self.selector)
        self.web.read.assert_called_once()
        self.web.command.assert_called_once()


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


class ObservationRetryTests(unittest.TestCase):
    def setUp(self):
        self.web = object.__new__(module.Webview)
        self.web.session = 'active-session'
        self.web.record = Mock()
        self.web._request_once = Mock()

    def observe(self, kind):
        if kind == 'dom':
            return self.web.read('document.body.innerText')
        output = Mock()
        self.web.screenshot(output)
        output.write_bytes.assert_called_once_with(b'image')

    def test_wrappers_recover_once_and_record_transport_failure(self):
        for kind, value in (('dom', 'visible text'), ('screenshot', 'aW1hZ2U=')):
            for error in (http.client.RemoteDisconnected('closed'), ConnectionResetError('reset'),
                          urllib.error.URLError(ConnectionResetError('wrapped reset'))):
                with self.subTest(kind=kind, error=type(error).__name__):
                    self.web._request_once = Mock(side_effect=[error, value])
                    self.web.record.reset_mock()
                    result = self.observe(kind)
                    if kind == 'dom':
                        self.assertEqual(result, value)
                    self.assertEqual(self.web._request_once.call_count, 2)
                    self.assertEqual(self.web._request_once.call_args_list[0], self.web._request_once.call_args_list[1])
                    first = self.web.record.call_args_list[0].args[1]
                    self.assertEqual((first['attempt'], first['retry']), (1, True))
                    self.assertIn('error', first)
                    self.assertEqual(self.web.record.call_args_list[-1].args[1]['attempt'], 2)

    def test_wrappers_exhaust_after_one_retry(self):
        for kind in ('dom', 'screenshot'):
            with self.subTest(kind=kind):
                self.web._request_once = Mock(side_effect=[ConnectionResetError('first'), ConnectionResetError('second')])
                self.web.record.reset_mock()
                with self.assertRaisesRegex(ConnectionResetError, 'second'):
                    self.observe(kind)
                self.assertEqual(self.web._request_once.call_count, 2)
                self.assertFalse(self.web.record.call_args_list[-1].args[1]['retry'])

    def test_semantic_errors_remain_fatal_without_retry(self):
        for kind in ('dom', 'screenshot'):
            with self.subTest(kind=kind):
                self.web._request_once = Mock(side_effect=RuntimeError('driver semantic error'))
                self.web.record.reset_mock()
                with self.assertRaisesRegex(RuntimeError, 'driver semantic error'):
                    self.observe(kind)
                self.web._request_once.assert_called_once()
                self.web.record.assert_not_called()

    def test_observer_rejects_other_commands_before_transport(self):
        for method, path in (('POST', '/actions'), ('DELETE', ''), ('POST', '/element/node/click')):
            with self.subTest(method=method, path=path):
                with self.assertRaises(ValueError):
                    self.web._observe(method, path, {})
        self.web._request_once.assert_not_called()


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


class FatalWaitBoundaryTests(unittest.TestCase):
    def test_exhausted_observations_are_fatal_inside_wait(self):
        for reason in (ConnectionResetError('reset'), http.client.RemoteDisconnected('closed')):
            for kind in ('dom', 'lookup'):
                with self.subTest(reason=type(reason).__name__, kind=kind):
                    web = object.__new__(module.Webview)
                    web.session, web.record = 'boundary', Mock()
                    # Wrapped resets were previously swallowed by wait(),
                    # silently opening another two-attempt observation budget.
                    web._request_once = Mock(side_effect=urllib.error.URLError(reason))
                    predicate = (lambda: web.read('document.body.innerText')) if kind == 'dom' else (
                        lambda: web.find('.board'))
                    with self.assertRaises(urllib.error.URLError):
                        module.wait(predicate, 'exhausted')
                    self.assertEqual(web._request_once.call_count, 2)

    def test_semantic_errors_are_fatal_inside_wait(self):
        predicate = Mock(side_effect=RuntimeError('no such element'))
        with self.assertRaisesRegex(RuntimeError, 'no such element'):
            module.wait(predicate, 'semantic')
        predicate.assert_called_once()

    def test_false_observations_still_poll_without_replaying_input(self):
        predicate = Mock(side_effect=[False, False, 'ready'])
        with patch.object(module.time, 'sleep'):
            self.assertEqual(module.wait(predicate, 'ordinary observation'), 'ready')
        self.assertEqual(predicate.call_count, 3)

    def test_high_level_mutations_do_not_replay_after_delivery_failure(self):
        for action in ('click', 'button', 'key', 'set', 'close'):
            with self.subTest(action=action):
                web = object.__new__(module.Webview)
                web.session, web.record = 'boundary', Mock()
                web.find = Mock(return_value='node')
                web._request_once = Mock(side_effect=http.client.RemoteDisconnected('after delivery'))
                with self.assertRaises(http.client.RemoteDisconnected):
                    if action == 'click': web.click('#save')
                    elif action == 'button': web.button('Save')
                    elif action == 'key': web.key('a')
                    elif action == 'set': web.set('#name', 'x')
                    else: web.close()
                web._request_once.assert_called_once()
                web.record.assert_not_called()


if __name__ == '__main__':
    unittest.main(verbosity=2)
