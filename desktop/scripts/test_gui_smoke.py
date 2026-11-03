"""External driver boundary tests; these do not establish actual packaged GUI results."""
import ast
import copy
import http.client
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import Mock, patch
import urllib.error

spec = importlib.util.spec_from_file_location('gui_smoke', Path(__file__).with_name('gui-smoke.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ReplacementAnalysisAuthorityTests(unittest.TestCase):
    def source(self, game='chess'):
        return {'token': {'document': 7, 'version': 8}, 'context': {'native_history': [1, 2]},
                'analysis': {'context': {'purpose': {'kind': 'move_analysis'}, 'request_id': 10,
                             'session_version': 8, 'process_generation': 4}}}

    def test_existing_thinking_state_is_not_replacement_evidence(self):
        before = self.source()
        self.assertFalse(module.replacement_analysis_accepted('chess', before, copy.deepcopy(before), True))
        after = copy.deepcopy(before); after['token']['version'] += 1
        self.assertFalse(module.replacement_analysis_accepted('chess', before, after, True))
        after['analysis']['context']['request_id'] += 1
        self.assertTrue(module.replacement_analysis_accepted('chess', before, after, True))
        self.assertFalse(module.replacement_analysis_accepted('chess', before, after, False))
        after['analysis']['context']['purpose']['kind'] = 'engine_turn'
        self.assertFalse(module.replacement_analysis_accepted('chess', before, after, True))

    def test_changed_game_history_or_document_never_establishes_same_target_replacement(self):
        for game in ('chess', 'shogi', 'xiangqi', 'tori'):
            before = self.source(game); after = copy.deepcopy(before)
            after['token']['version'] += 1; after['analysis']['context']['request_id'] += 1
            self.assertTrue(module.replacement_analysis_accepted(game, before, after, True))
            after['context']['native_history'].append(3)
            self.assertFalse(module.replacement_analysis_accepted(game, before, after, True))
            after = copy.deepcopy(before); after['token']['document'] += 1; after['token']['version'] += 1
            self.assertFalse(module.replacement_analysis_accepted(game, before, after, True))


class PendingResumeAnalysisPreconditionTests(unittest.TestCase):
    def resumed(self, **changed):
        return {'paused': 'false', 'rootEnabled': True, 'pausedBanner': False,
                'analysisEnabled': True, **changed}

    def analysis(self, **changed):
        return {'active': True, 'depthEnabled': True, 'nodesEnabled': True,
                'startEnabled': True, **changed}

    def journey(self, states):
        journey = object.__new__(module.Journey)
        journey.web = Mock()
        journey.web.read.side_effect = states
        journey.record = Mock()
        return journey

    def test_resume_settles_before_one_analysis_click_then_inputs_are_observed(self):
        journey = self.journey([self.resumed(paused='true', rootEnabled=False, pausedBanner=True),
                                self.resumed(), self.analysis(active=False, depthEnabled=False), self.analysis()])
        observed_buttons = []
        def observe(check, label):
            observed_buttons.append([call.args[0] for call in journey.web.button.call_args_list])
            with self.assertRaises(AssertionError): check()
            return check()
        with patch.object(module, 'wait', side_effect=observe):
            self.assertEqual(journey.resume_pending_analysis('tori'), self.analysis())
        self.assertEqual(observed_buttons, [['Resume play'], ['Resume play', 'Analyze position']])
        self.assertEqual(journey.web.button.call_args_list,
                         [unittest.mock.call('Resume play'), unittest.mock.call('Analyze position')])
        self.assertEqual(journey.web.read.call_count, 4)
        for call in journey.web.read.call_args_list:
            for forbidden in ('.click(', '.focus(', 'scrollIntoView(', 'dispatchEvent(', 'invoke('):
                self.assertNotIn(forbidden, call.args[0])
        journey.web.set.assert_not_called()
        journey.web.click.assert_not_called()

    def test_pending_journey_uses_precondition_before_setting_analysis_limits(self):
        source = ast.parse(Path(module.__file__).read_text())
        journey = next(node for node in source.body if isinstance(node, ast.ClassDef) and node.name == 'Journey')
        pending = next(node for node in journey.body if isinstance(node, ast.FunctionDef) and node.name == 'pending')
        calls = [node for node in ast.walk(pending) if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)]
        prepared = [node for node in calls if node.func.attr == 'resume_pending_analysis']
        settings = [node for node in calls if node.func.attr == 'set']
        self.assertEqual(len(prepared), 1)
        self.assertTrue(settings)
        self.assertTrue(all(node.lineno > prepared[0].lineno for node in settings))

    def test_each_resume_precondition_is_required_without_an_analysis_action(self):
        for changed in ({'paused':'true'}, {'rootEnabled':False}, {'pausedBanner':True}, {'analysisEnabled':False}):
            with self.subTest(changed=changed):
                journey = self.journey([self.resumed(**changed)])
                with patch.object(module, 'wait', side_effect=lambda check,label: check()):
                    with self.assertRaises(AssertionError): journey.resume_pending_analysis('tori')
                journey.web.button.assert_called_once_with('Resume play')
                journey.web.set.assert_not_called()

    def test_active_inspector_and_all_inputs_are_required_without_click_retries(self):
        for field in ('active','depthEnabled','nodesEnabled','startEnabled'):
            with self.subTest(field=field):
                journey = self.journey([self.resumed(), self.analysis(**{field:False})])
                with patch.object(module, 'wait', side_effect=lambda check,label: check()):
                    with self.assertRaises(AssertionError): journey.resume_pending_analysis('tori')
                self.assertEqual(journey.web.button.call_args_list,
                                 [unittest.mock.call('Resume play'), unittest.mock.call('Analyze position')])
                journey.web.set.assert_not_called()

    def test_dispatch_and_observer_failures_propagate_without_action_replay(self):
        for stage in ('resume', 'resume_observer', 'analysis', 'analysis_observer'):
            with self.subTest(stage=stage):
                journey = self.journey([self.resumed(), self.analysis()])
                failure = ConnectionResetError('delivery or observation failed')
                if stage == 'resume': journey.web.button.side_effect = failure
                elif stage == 'analysis': journey.web.button.side_effect = [None, failure]
                elif stage == 'resume_observer': journey.web.read.side_effect = failure
                else: journey.web.read.side_effect = [self.resumed(), failure]
                with patch.object(module, 'wait', side_effect=lambda check,label: check()):
                    with self.assertRaises(ConnectionResetError): journey.resume_pending_analysis('tori')
                self.assertEqual(journey.web.button.call_count, 1 if stage in ('resume','resume_observer') else 2)
                journey.web.set.assert_not_called()


class XiangqiImmutableWarningPrefixTests(unittest.TestCase):
    def pair(self):
        events = [{'event': {'kind': 'board_action', 'action': f'independent-{i}'}} for i in range(8)]
        context = {'native_record': 'complete live command history through warning9'}
        earlier = {'token': {'document': 7, 'selection': 8}, 'context': copy.deepcopy(context),
            'xiangqi': {'selected_events': events, 'selected_position': 'same diagram and counters',
                       'canonical_context_scope': 'live_with_immutable_selection',
                       'selected_canonical': None, 'warnings': []}}
        warned = copy.deepcopy(earlier)
        warned['token']['selection'] = 9
        warned['xiangqi'].update(canonical_context_scope='selected_committed', selected_canonical=copy.deepcopy(context),
             selected_events=[*events, {'event': {'kind': 'warning'}}],
             warnings=[{'active': True, 'side': 'red', 'fault': 'perpetual_check'}])
        return earlier, warned

    def test_shared_live_canonical_does_not_erase_native_immutable_selection(self):
        earlier, warned = self.pair()
        self.assertTrue(module.warning_prefix_transition(earlier, warned))
        earlier['xiangqi']['selected_canonical'] = copy.deepcopy(earlier['context'])
        self.assertFalse(module.warning_prefix_transition(earlier, warned), 'cannot fabricate a canonical record for a partial command')

    def test_identical_history_or_wrong_warning_does_not_establish_the_transition(self):
        for mutate in (lambda old,new: new['token'].update(selection=old['token']['selection']),
                       lambda old,new: new['xiangqi'].update(selected_events=old['xiangqi']['selected_events']),
                       lambda old,new: new['xiangqi']['warnings'][0].update(side='black'),
                       lambda old,new: new['xiangqi'].update(selected_position='different board'),
                       lambda old,new: old.update(context={'forged': 'short canonical record'})):
            earlier, warned = self.pair(); mutate(earlier, warned)
            self.assertFalse(module.warning_prefix_transition(earlier, warned))


class NativePendingTakebackArchiveTests(unittest.TestCase):
    def payload(self, game):
        action = {'chess': 'e2e4', 'shogi': '7g7f', 'xiangqi': 'a3a4'}[game]
        play = {'kind': 'play', 'action': action}
        if game == 'xiangqi':
            play = {'kind': 'xiangqi', 'command': play}
        body = {'controls': [{'sequence': 1, 'command': play}, {'sequence': 2, 'command': {'kind': 'take_back'}}]}
        if game == 'chess':
            body.update(game_record_json=json.dumps({'events': []}), takebacks=[{'match_record_json': json.dumps({'events': [{'request': {'command': {'move': action}}}]})}])
        else:
            record = {'game_kind': game, 'payload': {'root': {'native_root': game}, 'events': []}}
            body.update(initial=copy.deepcopy(record), current=copy.deepcopy(record))
        return action, body

    def test_game_native_lineage_is_retained_without_requiring_chess_only_array(self):
        for game in ('chess', 'shogi', 'xiangqi'):
            action, body = self.payload(game)
            self.assertTrue(module.retained_pending_takeback(game, body, action))
            if game != 'chess': self.assertNotIn('takebacks', body)
            body['controls'][0]['sequence'] = 3
            self.assertFalse(module.retained_pending_takeback(game, body, action))

    def test_an_undo_label_alone_does_not_prove_retained_native_actions_or_root(self):
        for game in ('chess', 'shogi', 'xiangqi'):
            action, body = self.payload(game)
            self.assertFalse(module.retained_pending_takeback(game, body, 'different-action'))
            if game == 'chess': body['takebacks'] = []
            else: body['current']['payload']['root'] = {'different': 'root'}
            self.assertFalse(module.retained_pending_takeback(game, body, action))


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


class CompletedCueEvidenceTests(unittest.TestCase):
    def cue(self, action="e2e4", event=2, origins=None, destinations=None, route=None):
        destinations = ["e4"] if destinations is None else destinations
        return {"action": action, "event": str(event), "hidden": "true", "pointerEvents": "none",
                "origins": ["e2"] if origins is None else origins, "destinations": destinations,
                "destinationSignatures": ["double-underline"] * len(destinations), "route": route}

    def test_exact_native_identity_and_nonactionable_signature_are_required(self):
        cue = self.cue()
        self.assertTrue(module.completed_action_matches(cue, "e2e4", 2, ["e2"], ["e4"]))
        for field, value in (("action", "e2e3"), ("event", "3"), ("origins", []),
                             ("destinations", ["e3"]), ("hidden", None), ("pointerEvents", "auto"),
                             ("destinationSignatures", ["solid-corners"]), ("route", "e2>e3>e4")):
            with self.subTest(field=field):
                self.assertFalse(module.completed_action_matches({**cue, field: value}, "e2e4", 2, ["e2"], ["e4"]))
        self.assertFalse(module.completed_action_matches(None, "e2e4", 2, ["e2"], ["e4"]))

    def test_actual_linux_and_windows_tori_action_member_order_is_not_identity(self):
        # Exact expected/observed member orders retained from failed cb1e162
        # Linux and Windows GUI reports. These were native-identical actions.
        expected = ('{"result_face":"swallow","from":"3d","kind":"move","to":"3c"}',
                    '{"from":"3d","kind":"move","result_face":"swallow","to":"3c"}')
        observed = '{"kind":"move","from":"3d","to":"3c","result_face":"swallow"}'
        cue = self.cue(observed, 1, ["3d"], ["3c"])
        for action in expected:
            self.assertTrue(module.completed_action_matches(cue, action, 1, ["3d"], ["3c"]))
            for field, value in (("event", "2"), ("origins", ["4b"]), ("destinations", ["4b"]),
                                 ("hidden", "false"), ("pointerEvents", "auto"),
                                 ("destinationSignatures", ["solid-corners"]), ("route", "3d>4c>3c")):
                self.assertFalse(module.completed_action_matches({**cue,field:value}, action, 1, ["3d"], ["3c"]))
        move = json.loads(observed)
        for field, value in (("kind", "drop"), ("from", "4b"), ("to", "3d"),
                             ("result_face", "goose"), ("invented_victim", "swallow")):
            changed = json.dumps({**move,field:value})
            self.assertFalse(module.completed_action_matches({**cue,"action":changed}, expected[0], 1, ["3d"], ["3c"]))
        for malformed in ('{"kind":"move","from":"3d","to":"3c","result_face":"swallow","to":"3c"}',
                          '{"kind":"move","from":"3d","to":"3c"}', '["3d","3c"]', '{broken'):
            self.assertFalse(module.native_action_identity_matches(malformed, expected[0]))
            self.assertFalse(module.native_action_identity_matches(observed, malformed))

    def test_complete_drop_identity_preserves_face_without_invented_origin(self):
        expected = '{"to":"3d","face":"swallow","kind":"drop"}'
        actual = '{"kind":"drop","face":"swallow","to":"3d"}'
        self.assertTrue(module.completed_action_matches(self.cue(actual,3,[],["3d"]), expected,3,[],["3d"]))
        for changed in ({"kind":"drop","face":"falcon","to":"3d"},
                        {"kind":"drop","face":"swallow","to":"4d"},
                        {"kind":"drop","face":"swallow","to":"3d","from":"3c"}):
            self.assertFalse(module.native_action_identity_matches(json.dumps(changed), expected))
        self.assertFalse(module.native_action_identity_matches('e2e4', 'e2e3'))
        self.assertTrue(module.native_action_identity_matches('e2e4', 'e2e4'))

    def test_drop_requires_no_invented_origin_and_equal_endpoint_routes_remain_distinct(self):
        self.assertTrue(module.completed_action_matches(self.cue("P*5e", 3, [], ["5e"]), "P*5e", 3, [], ["5e"]))
        self.assertFalse(module.completed_action_matches(self.cue("P*5e", 3, ["5d"], ["5e"]), "P*5e", 3, [], ["5e"]))
        cue = self.cue("136>153>136", 1, ["i9"], ["i9"], "i9>j10>i9")
        self.assertTrue(module.completed_action_matches(cue, "136>153>136", 1, ["i9"], ["i9"], "i9>j10>i9"))
        self.assertFalse(module.completed_action_matches(cue, "136>135>136", 1, ["i9"], ["i9"], "i9>h9>i9"))


class MovesKeyboardEvidenceTests(unittest.TestCase):
    def setUp(self):
        self.web = object.__new__(module.Webview)
        self.web.read, self.web.record, self.web.key = Mock(), Mock(), Mock()
        self.hidden = {"matches": 1, "enabled": True, "focused": False, "visible": False}
        self.ready = {**self.hidden, "focused": True, "visible": True}

    def test_actual_tabs_then_one_enter_with_no_dom_input_or_scroll_injection(self):
        self.web.read.side_effect = [self.hidden, self.hidden, self.ready]
        self.web.keyboard_activate('.moves-table [data-event="2"]', max_tabs=3)
        self.assertEqual(self.web.key.call_args_list, [unittest.mock.call("\ue004"), unittest.mock.call("\ue004"), unittest.mock.call("\ue007")])
        expression = self.web.read.call_args.args[0]
        for forbidden in (".focus(", ".click(", "scrollIntoView(", "scrollTop =", "dispatchEvent(", "invoke("):
            self.assertNotIn(forbidden, expression)

    def test_missing_disabled_or_unreachable_target_never_dispatches_enter(self):
        for observed in ({**self.hidden, "matches": 0}, {**self.hidden, "enabled": False}):
            self.web.read.return_value = observed
            with self.assertRaises(AssertionError):
                self.web.keyboard_activate(".moves-root", max_tabs=2)
            self.web.key.assert_not_called()
        self.web.read.return_value = self.hidden
        with self.assertRaisesRegex(AssertionError, "unreachable"):
            self.web.keyboard_activate(".moves-root", max_tabs=2)
        self.assertEqual(self.web.key.call_args_list, [unittest.mock.call("\ue004"), unittest.mock.call("\ue004")])

    def test_enter_delivery_failure_is_not_replayed(self):
        self.web.read.return_value = self.ready
        self.web.key.side_effect = http.client.RemoteDisconnected("Enter delivered or not is unknown")
        with self.assertRaises(http.client.RemoteDisconnected):
            self.web.keyboard_activate(".moves-root")
        self.web.key.assert_called_once_with("\ue007")

    def test_focused_target_waits_for_readonly_paint_without_tabbing_away(self):
        self.web.read.side_effect = [{**self.ready, "visible": False}, self.ready]
        self.web.keyboard_activate(".moves-root")
        self.web.key.assert_called_once_with("\ue007")


class MovesAdmissionReadinessTests(unittest.TestCase):
    def state(self):
        return {"event": "2", "orientation": "Flip board; currently Black at bottom",
                "rootEnabled": True, "flipEnabled": True, "targetEnabled": True, "checkingCapacity": False}

    def test_changed_label_without_settled_admission_does_not_authorize_next_input(self):
        state = self.state()
        self.assertTrue(module.moves_controls_settled(state, 2, changed_from="Flip board; currently White at bottom"))
        for field, value in (("rootEnabled", False), ("flipEnabled", False), ("targetEnabled", False),
                             ("checkingCapacity", True), ("event", "3"), ("orientation", None)):
            with self.subTest(field=field):
                self.assertFalse(module.moves_controls_settled({**state, field: value}, 2))
        self.assertFalse(module.moves_controls_settled(state, 2, changed_from=state["orientation"]))
        self.assertFalse(module.moves_controls_settled(state, 2, orientation="Flip board; currently White at bottom"))

    def test_flip_is_sent_once_then_only_readonly_pending_observations_are_polled(self):
        journey = object.__new__(module.Journey)
        journey.web, journey.record = Mock(), Mock()
        before = {**self.state(), "orientation": "Flip board; currently White at bottom"}
        after = self.state()
        journey.web.read.side_effect = [before, {**after, "rootEnabled": False, "checkingCapacity": True}, after]
        with patch.object(module.time, "sleep"):
            self.assertEqual(journey.flip_moves_board(2, '.moves-table [data-event="2"]'), after)
        journey.web.click.assert_called_once_with(".flip-button")
        journey.web.key.assert_not_called()
        expression = journey.web.read.call_args.args[0]
        for forbidden in (".focus(", ".click(", "scrollIntoView(", "dispatchEvent(", "invoke("):
            self.assertNotIn(forbidden, expression)

    def test_return_flip_requires_restored_label_and_does_not_repeat_failed_input(self):
        journey = object.__new__(module.Journey)
        journey.web, journey.record = Mock(), Mock()
        before = self.state()
        restored = {**before, "orientation": "Flip board; currently White at bottom"}
        journey.web.read.side_effect = [before, before, restored]
        with patch.object(module.time, "sleep"):
            self.assertEqual(journey.flip_moves_board(2, ".moves-root", restored["orientation"]), restored)
        journey.web.click.assert_called_once_with(".flip-button")
        journey.web.read.side_effect = None
        journey.web.read.return_value = before
        journey.web.click.reset_mock()
        journey.web.click.side_effect = http.client.RemoteDisconnected("flip delivery unknown")
        with self.assertRaises(http.client.RemoteDisconnected):
            journey.flip_moves_board(2, ".moves-root")
        journey.web.click.assert_called_once_with(".flip-button")


class XiangqiBoardGeometryTests(unittest.TestCase):
    def geometry(self, points=90, visible=True):
        return {"viewport": {"width": 760, "height": 650}, "points": points, "allVisible": visible,
                "palaceSides": ["mshogi.xiangqi.wxf2018.v1:red", "mshogi.xiangqi.wxf2018.v1:black"],
                "river": "楚河漢界"}

    def journey(self, geometry):
        journey = object.__new__(module.Journey)
        journey.web = Mock()
        journey.web.read.return_value = geometry
        journey.record = Mock()
        return journey

    def test_actual_minimum_xiangqi_geometry_requires90_visible_intersections(self):
        geometry = self.geometry()
        journey = self.journey(geometry)
        self.assertEqual(journey.xiangqi_board_fits(), geometry)
        journey.web.command.assert_not_called()
        self.assertIn(".xiangqi-point", journey.web.read.call_args.args[0])

    def test_square_board_counts_or_clipped_xiangqi_are_not_accepted(self):
        for geometry in (self.geometry(81), self.geometry(64), self.geometry(visible=False)):
            with self.subTest(geometry=geometry):
                with self.assertRaises(AssertionError):
                    self.journey(geometry).xiangqi_board_fits()

    def test_missing_native_palaces_or_river_fail_the_geometry_probe(self):
        for change in (lambda data: data.update(palaceSides=[]),
                       lambda data: data.update(river="ordinary chess board")):
            geometry = self.geometry()
            change(geometry)
            with self.assertRaises(AssertionError):
                self.journey(geometry).xiangqi_board_fits()


class SyntheticFullBoardEvidenceTests(unittest.TestCase):
    def geometry(self, orientation="white", width=1180, height=820):
        return {"viewport": {"width": width, "height": height}, "positionCount": 256,
                "uniquePositions": 256, "positions": [f"{file}{rank}" for file in "abcdefghijklmnop" for rank in range(1, 17)],
                "fullyVisible": True, "clippedPositions": [], "boardVisible": True,
                "orientationLabel": f"Synthetic development strider board, 16 by 16; invented rules, {orientation} at bottom",
                "board": {"left": 20, "top": 160, "right": 460, "bottom": 600, "width": 440, "height": 440},
                "pageWidth": width, "boardAncestors": []}

    def journey(self, observed):
        journey = object.__new__(module.Journey)
        journey.web, journey.record = Mock(), Mock()
        journey.web.read.return_value = observed
        return journey

    def test_all256_positions_must_already_fit_at_both_supported_sizes_and_orientations(self):
        for orientation in ("white", "black"):
            for width, height in ((1180, 820), (760, 650)):
                with self.subTest(orientation=orientation, viewport=(width, height)):
                    geometry = self.geometry(orientation, width, height)
                    journey = self.journey(geometry)
                    self.assertEqual(journey.synthetic_board_fits(orientation), geometry)
                    journey.web.command.assert_not_called()
                    expression = journey.web.read.call_args.args[0]
                    self.assertIn(".synthetic-board .square", expression)
                    self.assertIn("getBoundingClientRect", expression)
                    self.assertIn("getComputedStyle", expression)
                    for mutation in ("scrollIntoView(", "scrollTo(", ".focus(", "dispatchEvent(", "invoke(", "scrollTop ="):
                        self.assertNotIn(mutation, expression)

    def test_incomplete_duplicate_clipped_or_scrolling_board_cannot_pass_the_geometry_probe(self):
        changes = (
            lambda data: data.update(positionCount=255),
            lambda data: data.update(uniquePositions=255),
            lambda data: data["positions"].__setitem__(0, data["positions"][1]),
            lambda data: data["positions"].pop(),
            lambda data: data.update(fullyVisible=False, clippedPositions=["p16"]),
            lambda data: data.update(clippedPositions=["p16"]),
            lambda data: data.update(boardVisible=False),
            lambda data: data.update(pageWidth=data["viewport"]["width"] + 50),
            lambda data: data.update(orientationLabel="Synthetic board, black at bottom"),
        )
        for change in changes:
            geometry = self.geometry()
            change(geometry)
            with self.subTest(geometry=geometry):
                with self.assertRaises(AssertionError):
                    self.journey(geometry).synthetic_board_fits("white")

    def record(self, actions):
        return {"schema_version": 1, "rules_profile": "mshogi.synthetic.strider16.v1",
                "initial_fixture": "strider16-capture-and-return-v1", "actions": actions}

    def test_partial_or_cancelled_selection_requires_exactly_zero_native_actions(self):
        journey = self.journey(self.record([]))
        journey.web.text.return_value = "Version 1 · Accepted actions 0"
        self.assertEqual(journey.synthetic_record([]), self.record([]))
        journey.web.command.assert_not_called()

    def test_terminal_prefix_is_not_confused_with_a_longer_complete_route(self):
        journey = self.journey(None)
        journey.web.read.side_effect = [self.record(["136>153>136"]), self.record(["136>153"])]
        journey.web.text.return_value = "Version 2 · Accepted actions 1"
        with patch.object(module.time, "sleep"):
            self.assertEqual(journey.synthetic_record(["136>153"]), self.record(["136>153"]))
        self.assertEqual(journey.web.read.call_count, 2)
        journey.web.command.assert_not_called()

    def test_equal_endpoint_routes_remain_distinct_in_native_acceptance(self):
        journey = self.journey(None)
        journey.web.read.side_effect = [self.record(["136>153>136"]), self.record(["136>135>136"])]
        journey.web.text.return_value = "Version 4 · Accepted actions 1"
        with patch.object(module.time, "sleep"):
            self.assertEqual(journey.synthetic_record(["136>135>136"]), self.record(["136>135>136"]))
        self.assertEqual(journey.web.read.call_count, 2)

    def test_a_wrong_profile_root_or_schema_is_not_synthetic_acceptance(self):
        for field, value in (("rules_profile", "mshogi.chess.match.v1"),
                             ("initial_fixture", "different-root"), ("schema_version", 2)):
            with self.subTest(field=field):
                record = self.record([])
                record[field] = value
                with self.assertRaisesRegex(ValueError, "Unexpected synthetic native record identity"):
                    self.journey(record).synthetic_record([])

    def test_displayed_count_must_agree_exactly_without_accepting_a_numeric_prefix(self):
        journey = self.journey(self.record(["136>153"]))
        journey.web.text.return_value = "Version 12 · Accepted actions 10"
        with self.assertRaises(AssertionError):
            journey.synthetic_record(["136>153"])


class CurrentDocumentInspectionTests(unittest.TestCase):
    def document(self, game="chess"):
        profile, edition = {
            "chess": ("mshogi.chess.match.v1", "FIDE-2023-01-01+mshogi.chess.match.v1"),
            "shogi": ("mshogi.shogi.csa36.v1", "JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026"),
            "xiangqi": ("mshogi.xiangqi.wxf2018.v1", "WXF-2018+XIANGQI-P1-2026-10-27"),
            "tori": ("mshogi.tori.modern.v1", "MSH81-MODERN-2026-11-01"),
            "chu": ("mshogi.chu.jcsa2019.v1", "JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02"),
        }[game]
        return {"envelope_version": 1, "format": "mshogi.desktop.document.v1",
                "game_kind": game, "rules_profile": profile, "profile_edition": edition,
                "native_schema_version": 1, "payload": {"observations": []}}

    def test_current_envelopes_expose_native_payload_without_reconstruction(self):
        for game in ("chess", "shogi", "xiangqi", "tori", "chu"):
            with self.subTest(game=game):
                document = self.document(game)
                self.assertIs(module.desktop_document_payload(document, game), document["payload"])

    def test_old_or_conflicting_envelopes_are_not_accepted_as_gui_evidence(self):
        for game in ("chess", "shogi", "xiangqi", "tori", "chu"):
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


class XiangqiRuntimeEvidenceTests(unittest.TestCase):
    def diagnostics(self):
        snapshot = json.loads((Path(__file__).parents[1] / 'src' / '__fixtures__' /
                               'shared-xiangqi-played.json').read_text())
        return {key: copy.deepcopy(snapshot[key]) for key in
                ('token', 'profile', 'context', 'xiangqi', 'document', 'io', 'observations')}

    def test_r6_keeps_prior_campaign_and_requires_real_game_native_capabilities(self):
        previous = [None, 'normal-shogi', 'normal-shogi-reopened', 'normal-xiangqi',
                    'standard', 'promotion', 'claim', 'capacity', 'synthetic', 'shogi-hands',
                    'shogi-promotion', 'shogi-forced', 'shogi-declaration', 'shogi-concluded', 'shogi-capacity']
        actual = module.selected_gui_phases('all')
        self.assertEqual(len(actual), len(set(actual)))
        self.assertTrue(set(previous) <= set(actual))
        self.assertTrue({'current-claim', 'concluded', 'full-unknown', 'pending-chess',
                         'pending-shogi', 'pending-xiangqi'} <= set(actual))
        xiangqi = module.selected_gui_phases('verification-xiangqi')
        self.assertEqual(set(xiangqi), {'xiangqi-warning', 'xiangqi-fourth', 'xiangqi-count',
            'xiangqi-offer', 'xiangqi-agreed', 'xiangqi-fast-proof', 'xiangqi-resigned',
            'xiangqi-capacity', 'xiangqi-failure', 'xiangqi-exact-evidence'})
        self.assertTrue(set(xiangqi) <= set(actual))
        self.assertEqual(module.selected_gui_phases('normal'), previous[:4] + ['normal-tori', 'normal-tori-native', 'normal-chu', 'normal-chu-native'])
        self.assertEqual(module.selected_gui_phases('normal-tori'), ['normal-tori', 'normal-tori-native'])
        self.assertEqual(module.selected_gui_phases('verification-tori'), ['pending-tori', 'tori-recovery'])
        self.assertEqual(module.selected_gui_phases('normal-shogi'), previous[1:3])
        self.assertEqual(module.selected_gui_phases('normal-xiangqi'), ['normal-xiangqi'])
        self.assertEqual(set(module.selected_gui_phases('verification')), set(actual) - set(module.selected_gui_phases('normal')))

    def test_rejected_open_may_report_io_failure_without_changing_native_authority(self):
        before = self.diagnostics()
        after = copy.deepcopy(before)
        after['token']['revision'] += 1
        after['io'] = {'id': before['io']['id'] + 1, 'phase': 'failed', 'message': 'Malformed document'}
        self.assertEqual(module.xiangqi_retained_state(before), module.xiangqi_retained_state(after))

    def test_failed_open_comparison_detects_native_history_settings_and_identity_changes(self):
        before = self.diagnostics()
        changes = [
            lambda data: data['context']['payload']['events'].pop(),
            lambda data: data['context']['payload']['root'].update(no_capture_plies=29),
            lambda data: data['xiangqi']['warnings'].append({'side': 'red', 'event_index': 1}),
            lambda data: data['xiangqi']['count_advice'][0].update(eligible=True),
            lambda data: data['document']['settings'].update(orientation='black'),
            lambda data: data['document'].update(dirty=False),
            lambda data: data['document'].update(path='/wrong/game.mshogi'),
            lambda data: data['observations'].append({'historical': True, 'unscored': True}),
        ]
        changes += [lambda data, key=key: data['token'].update({key: data['token'][key] + 1})
                    for key in ('document', 'version', 'selection')]
        for index, change in enumerate(changes):
            with self.subTest(index=index):
                after = copy.deepcopy(before)
                change(after)
                self.assertNotEqual(module.xiangqi_retained_state(before), module.xiangqi_retained_state(after))

    def test_native_comparison_rejects_wrong_game_or_profile(self):
        for field, value in (('profile', 'mshogi.shogi.csa36.v1'), ('game_kind', 'shogi'),
                             ('rules_profile', 'mshogi.shogi.csa36.v1')):
            with self.subTest(field=field):
                data = self.diagnostics()
                if field == 'profile':
                    data[field] = value
                else:
                    data['context'][field] = value
                with self.assertRaises(AssertionError):
                    module.xiangqi_retained_state(data)

    def test_normal_native_error_field_is_wrapped_outside_w3c_error_shape(self):
        journey = object.__new__(module.Journey)
        journey.web = Mock()
        diagnostics = {**self.diagnostics(), 'error': ''}
        journey.web.read.side_effect = [True, {'text':json.dumps(diagnostics),'page':0,'pages':1}]
        journey.web.has.side_effect = [True, True, False]
        self.assertEqual(journey.xiangqi_diagnostics(), diagnostics)
        expression = journey.web.read.call_args.args[0]
        self.assertIn('text:e.textContent',expression)
        self.assertNotIn('JSON.parse(',expression)
        journey.web.command.assert_not_called()

    def test_xiangqi_history_observer_requires_event_prefix_not_only_board_ply(self):
        journey = object.__new__(module.Journey)
        journey.web = Mock()
        journey.web.read.side_effect = [
            {'live': '4', 'selected': '2', 'historical': 'false'},
            {'live': '4', 'selected': '2', 'historical': 'true'}]
        with patch.object(module.time, 'sleep'):
            self.assertEqual(journey.xiangqi_events(4, selected=2, historical=True),
                             {'live': '4', 'selected': '2', 'historical': 'true'})
        self.assertEqual(journey.web.read.call_count, 2)
        journey.web.command.assert_not_called()

    def test_analysis_completion_requires_actual_current_purpose_history_and_limits(self):
        journey = object.__new__(module.Journey)
        journey.web = Mock()
        journey.web.text.side_effect = lambda selector: (
            'Analysis result available' if selector == '.shogi-analysis-state'
            else 'No move or claim was played')
        accepted = {'historical': False, 'observation': {'context': {
            'purpose': {'kind': 'move_analysis'}, 'limits': {'depth': 1, 'nodes': 128},
            'game': copy.deepcopy(self.diagnostics()['context'])}}}
        wrong = []
        for change in (
                lambda item: item.update(historical=True),
                lambda item: item['observation']['context'].update(purpose={'kind': 'engine_turn'}),
                lambda item: item['observation']['context']['limits'].update(nodes=512),
                lambda item: item['observation']['context']['game']['payload']['events'].pop()):
            item = copy.deepcopy(accepted)
            change(item)
            wrong.append({'observations': [item]})
        journey.xiangqi_diagnostics = Mock(side_effect=[*wrong, {'observations': [accepted]}])
        with patch.object(module.time, 'sleep'):
            self.assertEqual(journey.xiangqi_complete_analysis(2), accepted)
        self.assertEqual(journey.xiangqi_diagnostics.call_count, 5)


class SaveAcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.expected = '/games/new.mshogi'
        self.observation = {
            'path': self.expected, 'dirty': 'UNMODIFIED', 'statuses': [],
            'io': {'id': 8, 'phase': 'complete', 'message': 'Saved complete native desktop archive.'},
            'buttons': [{'label': label, 'enabled': True}
                        for label in ('New game', 'Save', 'Load')]}

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

    def test_prior_native_receipt_cannot_accept_a_new_file_operation(self):
        self.observation['after_io_id']=8
        self.assertFalse(module.saved_document_accepted(self.observation,self.expected))
        self.observation['io']['id']=9
        self.assertTrue(module.saved_document_accepted(self.observation,self.expected))
        self.observation['io']['phase']='failed'
        self.assertFalse(module.saved_document_accepted(self.observation,self.expected))

    def test_all_three_current_controls_are_required(self):
        self.observation['buttons'].pop()
        self.assertFalse(module.saved_document_accepted(self.observation, self.expected))

    def test_open_requires_actual_current_receipt_and_settled_correct_document(self):
        # Actual R3 packaged failure screenshot/DOM: Open succeeded with this
        # new native receipt, while the verifier waited for an obsolete phrase.
        self.observation['io'] = {'id':9,'phase':'complete','message':
            'Opened complete native document with fresh runtime identities and work stopped.'}
        self.assertTrue(module.opened_document_accepted(self.observation, self.expected))
        for change in (
                lambda state: state.update(io={'id':9,'phase':'complete','message':'Saved complete native desktop archive.'}),
                lambda state: state.update(io={'id':0,'phase':'complete','message':'Opened complete native document with work stopped.'}),
                lambda state: state.update(io={'id':9,'phase':'failed','message':'Opened complete native document with work stopped.'}),
                lambda state: state.update(statuses=['Opening your game…']),
                lambda state: state.update(io={'id':10,'phase':'failed','message':'Open failed; current game retained.'}),
                lambda state: state.update(path='/games/old.mshogi'),
                lambda state: state.update(dirty='Unsaved changes'),
                lambda state: state['buttons'][2].update(enabled=False)):
            state = copy.deepcopy(self.observation)
            change(state)
            self.assertFalse(module.opened_document_accepted(state, self.expected))


class CurrentFileWorkflowTests(unittest.TestCase):
    def journey(self, path=None):
        journey = object.__new__(module.Journey)
        journey.native_diagnostics = Mock(return_value={'document': {'path': path}, 'io': {'id': 4}})
        journey.file, journey.wait_saved, journey.web, journey.record = Mock(), Mock(), Mock(), Mock()
        return journey

    def test_first_save_uses_picker_then_requires_actual_accepted_path(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'first.mshogi'
            path.write_text('{"complete":"native evidence"}')
            journey = self.journey()
            self.assertEqual(journey.save_current(path), {'complete': 'native evidence'})
            journey.file.assert_called_once_with('save', path)
            journey.wait_saved.assert_called_once_with(path)
            journey.web.button.assert_not_called()

    def test_subsequent_save_uses_current_path_and_labels_external_copy_honestly(self):
        with tempfile.TemporaryDirectory() as directory:
            actual, evidence = Path(directory) / 'current.mshogi', Path(directory) / 'evidence.mshogi'
            actual.write_text('{"complete":"accepted current native archive"}')
            journey = self.journey(str(actual))
            journey.save_current(evidence)
            journey.file.assert_not_called()
            journey.web.button.assert_called_once_with('Save')
            journey.wait_saved.assert_called_once_with(actual)
            self.assertEqual(evidence.read_bytes(), actual.read_bytes())
            self.assertIn('not product Save As', journey.record.call_args.args[1]['qualification'])

    def test_existing_file_is_not_evidence_of_success_until_receipt_is_accepted(self):
        with tempfile.TemporaryDirectory() as directory:
            actual, evidence = Path(directory) / 'old.mshogi', Path(directory) / 'not-written.mshogi'
            actual.write_text('{"old":"valid"}')
            journey = self.journey(str(actual))
            journey.wait_saved.side_effect = AssertionError('native Save failed')
            with self.assertRaisesRegex(AssertionError, 'native Save failed'):
                journey.save_current(evidence)
            self.assertFalse(evidence.exists())

    def test_never_saved_startup_requires_native_close_authority_even_when_internally_clean(self):
        for dirty in (False, True):
            with self.subTest(dirty=dirty):
                native = {'document': {'path': None, 'dirty': dirty}, 'close_requires_save': True}
                self.assertTrue(module.never_saved_close_source(native))
                for wrong in (
                        {**native, 'close_requires_save': False},
                        {'document': native['document']},
                        {**native, 'document': {'path': '/games/saved.mshogi', 'dirty': dirty}},
                        {**native, 'document': {'path': None, 'dirty': None}}):
                    self.assertFalse(module.never_saved_close_source(wrong))

    def test_replacement_comparison_preserves_native_history_and_runtime_authority(self):
        for fixture in ('shared-chess-standard.json', 'shared-shogi-standard.json', 'shared-xiangqi-played.json'):
            path = Path(__file__).parents[1] / 'src' / '__fixtures__' / fixture
            self.assertTrue(path.exists(), path)
            with self.subTest(fixture=fixture):
                before = json.loads(path.read_text())
                after = copy.deepcopy(before)
                after['token']['revision'] += 1
                after['io']['phase'] = 'failed'
                self.assertEqual(module.retained_document_state(before), module.retained_document_state(after))
                for key in ('context', 'document', 'observations'):
                    changed = copy.deepcopy(after)
                    changed[key] = {'lost': key}
                    self.assertNotEqual(module.retained_document_state(before), module.retained_document_state(changed))
                for key in ('document', 'version', 'selection'):
                    changed = copy.deepcopy(after)
                    changed['token'][key] += 1
                    self.assertNotEqual(module.retained_document_state(before), module.retained_document_state(changed))


class ChessMatchResignationEvidenceTests(unittest.TestCase):
    def setUp(self):
        fixture = json.loads((Path(__file__).with_name('fixtures') /
                              'msh125-chess-resignation-actual.json').read_text())
        self.before, self.after = fixture['before'], fixture['after']
        self.conclusion = {'unresolved': {'trigger': 'resignation', 'actor': 'white'}}

    def test_actual_linux_match_terminal_event_preserves_board_record_and_is_unscored(self):
        self.assertEqual(self.before['context'], self.after['context'])
        self.assertTrue(module.chess_resignation_matches(self.before, self.after, 'white', self.conclusion))
        self.assertFalse(module.chess_resignation_matches(self.before, self.after, 'black', self.conclusion))
        self.assertFalse(module.chess_resignation_matches(self.before, self.after, 'white',
                         {'win': {'winner': 'black', 'trigger': 'resignation'}}))

    def test_wrong_actor_stale_rejected_duplicate_or_invented_board_action_cannot_pass(self):
        changes = [
            lambda d: d['chess_match']['events'][-1]['request'].update(actor='black'),
            lambda d: d['chess_match']['events'][-1]['request'].update(version=0),
            lambda d: d['chess_match']['events'][-1]['reply'].update(rejection='stale'),
            lambda d: d['chess_match']['events'][-1]['reply'].update(active='black'),
            lambda d: d['chess_match']['events'].append(copy.deepcopy(d['chess_match']['events'][-1])),
            lambda d: d['context']['payload']['events'].append({'event': {'kind': 'move', 'action': 'e4e5'}, 'ply': 2}),
            lambda d: d['chess'].update(live_conclusion={'win': {'winner': 'black', 'trigger': 'resignation'}}),
            lambda d: d['token'].update(document=d['token']['document']+1),
            lambda d: d['token'].update(version=self.before['token']['version']),
        ]
        for index, change in enumerate(changes):
            with self.subTest(index=index):
                after = copy.deepcopy(self.after)
                change(after)
                self.assertFalse(module.chess_resignation_matches(self.before, after, 'white', self.conclusion))


class KeyboardControlReadinessTests(unittest.TestCase):
    def setUp(self):
        self.web = object.__new__(module.Webview)
        self.web.record, self.web.read = Mock(), Mock()
        self.web.key, self.web.button = Mock(), Mock()
        self.hidden = {"matches": 1, "enabled": True, "focused": False, "visible": False}
        self.ready = {"matches": 1, "enabled": True, "focused": True, "visible": True}

    def test_clipped_control_is_reached_by_bounded_actual_tabs_before_one_click(self):
        self.web.read.side_effect = [self.hidden, self.hidden, self.ready]
        self.web.focused_button("Take back", max_tabs=3)
        self.assertEqual(self.web.key.call_args_list, [unittest.mock.call("\ue004"), unittest.mock.call("\ue004")])
        self.web.button.assert_called_once_with("Take back")
        self.assertEqual(self.web.read.call_count, 3)
        expression = self.web.read.call_args.args[0]
        self.assertIn("document.activeElement===e", expression)
        self.assertIn("getBoundingClientRect", expression)
        self.assertNotIn("scrollIntoView", expression)
        self.assertNotIn(".focus(", expression)
        self.assertNotIn(".click(", expression)
        self.assertNotIn("invoke(", expression)
        self.assertEqual(self.web.record.call_args.args[0], "Keyboard control focused and visible before one click")

    def test_already_focused_visible_control_needs_no_extra_input_before_one_click(self):
        self.web.read.return_value = self.ready
        self.web.focused_button("Take back", max_tabs=0)
        self.web.key.assert_not_called()
        self.web.button.assert_called_once_with("Take back")

    def test_disabled_missing_and_ambiguous_controls_fail_without_any_input(self):
        for state in ({**self.hidden, "enabled": False},
                      {**self.hidden, "matches": 0}, {**self.hidden, "matches": 2}):
            with self.subTest(state=state):
                self.web.read.return_value = state
                with self.assertRaises(AssertionError):
                    self.web.focused_button("Take back", max_tabs=3)
                self.web.key.assert_not_called()
                self.web.button.assert_not_called()

    def test_unreachable_control_exhausts_exact_tab_bound_and_never_clicks(self):
        self.web.read.return_value = self.hidden
        with self.assertRaisesRegex(AssertionError, "unreachable within 3 actual Tab inputs"):
            self.web.focused_button("Take back", max_tabs=3)
        self.assertEqual(self.web.read.call_count, 4)
        self.assertEqual(self.web.key.call_count, 3)
        self.web.button.assert_not_called()

    def test_focused_control_waits_only_for_read_only_paint_observations(self):
        self.web.read.side_effect = [self.hidden, {**self.ready, "visible": False}, self.ready]
        self.web.focused_button("Take back", max_tabs=3)
        self.web.key.assert_called_once_with("\ue004")
        self.web.button.assert_called_once_with("Take back")

    def test_failed_final_click_is_not_repeated_or_recovered_as_a_pass(self):
        self.web.read.side_effect = [self.hidden, self.ready]
        self.web.button.side_effect = http.client.RemoteDisconnected("after delivery")
        with self.assertRaises(http.client.RemoteDisconnected):
            self.web.focused_button("Take back", max_tabs=3)
        self.web.key.assert_called_once_with("\ue004")
        self.web.button.assert_called_once_with("Take back")

    def test_failed_keyboard_delivery_is_not_repeated(self):
        self.web.read.return_value = self.hidden
        self.web.key.side_effect = http.client.RemoteDisconnected("Tab delivery unknown")
        with self.assertRaises(http.client.RemoteDisconnected):
            self.web.focused_button("Take back", max_tabs=3)
        self.web.key.assert_called_once_with("\ue004")
        self.web.button.assert_not_called()


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

    def test_xiangqi_intersections_use_actual_w3c_pointer_not_svg_element_click(self):
        self.web.read.return_value = self.ready
        selector = '.xiangqi-point[data-intersection="a3"]'
        self.web.click(selector)
        self.web.command.assert_called_once()
        method, path, payload = self.web.command.call_args.args
        self.assertEqual((method, path), ('POST', '/actions'))
        self.assertEqual(payload['actions'][0]['type'], 'pointer')
        self.assertEqual(self.web.record.call_args.args[1]['selector'], selector)

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




class ShellFocusEvidenceTests(unittest.TestCase):
    def observation(self):
        return {'focused': True, 'focusVisible': True, 'outlineStyle': 'solid', 'outlineWidth': 2,
                'outline': {'left': 410, 'right': 700, 'top': 580, 'bottom': 646},
                'clip': {'left': 408, 'right': 702, 'top': 120, 'bottom': 648}}

    def test_full_outline_not_only_button_label_must_fit_all_clip_edges(self):
        observed=self.observation()
        self.assertTrue(module.focus_outline_visible(observed))
        for key,value in (('left',407),('right',703),('top',119),('bottom',649)):
            bad=copy.deepcopy(observed);bad['outline'][key]=value
            self.assertFalse(module.focus_outline_visible(bad),key)
        for key,value in (('focused',False),('focusVisible',False),('outlineStyle','none'),('outlineWidth',0)):
            self.assertFalse(module.focus_outline_visible({**observed,key:value}),key)
        self.assertFalse(module.focus_outline_visible(None))

    def test_actual_focus_probe_observes_every_ancestor_without_reveal_injection(self):
        journey=object.__new__(module.Journey);journey.web=Mock();journey.record=Mock()
        journey.web.read.return_value=self.observation()
        journey.assert_focus_outline('.moves-table [data-event="2"]','chess')
        expression=journey.web.read.call_args.args[0]
        for required in ('outlineWidth','outlineOffset','parentElement','overflowX','overflowY','clientWidth','clientHeight','document.activeElement===e',":focus-visible"):
            self.assertIn(required,expression)
        for forbidden in ('.focus(','.click(','scrollIntoView(', 'scrollTop =','dispatchEvent(', 'invoke('):
            self.assertNotIn(forbidden,expression)
        journey.web.click.assert_not_called();journey.web.key.assert_not_called()


class PublicSupportDiagnosticsTests(unittest.TestCase):
    def test_retry_wait_reads_live_move_idle_dom_then_observes_native_once(self):
        journey=object.__new__(module.Journey);journey.web=Mock();journey.record=Mock()
        journey.web.read.side_effect=[False,True]
        native={'moves':{'entries':[{'native_action':'b2b9'}]}}
        journey.xiangqi_diagnostics=Mock(return_value=native)
        self.assertEqual(journey.completed_xiangqi_retry(),native)
        self.assertEqual(journey.web.read.call_count,2)
        journey.xiangqi_diagnostics.assert_called_once()
        expression=journey.web.read.call_args.args[0]
        for required in ('.moves-table [data-event]',"dataset.phase === 'idle'",'[data-support-trigger]:not(:disabled)'):
            self.assertIn(required,expression)
        journey.web.click.assert_not_called()

    def test_retry_dom_cannot_substitute_for_native_completed_move_evidence(self):
        journey=object.__new__(module.Journey);journey.web=Mock();journey.record=Mock()
        journey.web.read.return_value=True
        journey.xiangqi_diagnostics=Mock(return_value={'moves':{'entries':[]}})
        with self.assertRaises(AssertionError):journey.completed_xiangqi_retry()
        journey.xiangqi_diagnostics.assert_called_once()
        journey.record.assert_not_called()

    def test_native_diagnostics_require_public_actual_input_and_always_close(self):
        journey=object.__new__(module.Journey);journey.web=Mock()
        native={'token':{'document':7},'context':{'game_kind':'shogi'}}
        journey.web.read.side_effect=[True,{'text':json.dumps(native),'page':0,'pages':1}]
        journey.web.has.side_effect=[True,True,False]
        self.assertEqual(journey.native_diagnostics(),native)
        self.assertEqual(journey.web.click.call_args_list,[unittest.mock.call('[data-support-trigger]'),
            unittest.mock.call('.support-dialog details summary'),unittest.mock.call('.support-dialog [data-cancel]')])
        for call in journey.web.read.call_args_list:
            for forbidden in ('invoke(', 'dispatchEvent(', '.click(', '.focus('):
                self.assertNotIn(forbidden,call.args[0])

    def test_bad_diagnostic_payload_still_closes_public_dialog(self):
        journey=object.__new__(module.Journey);journey.web=Mock()
        journey.web.read.side_effect=[True,{'text':'null','page':0,'pages':1}];journey.web.has.side_effect=[True,True,False]
        with self.assertRaises(AssertionError):journey.native_diagnostics()
        self.assertEqual(journey.web.click.call_args_list[-1],unittest.mock.call('.support-dialog [data-cancel]'))

    def test_public_diagnostic_pages_reconstruct_exact_context_through_real_buttons(self):
        journey=object.__new__(module.Journey);journey.web=Mock()
        native={'token':{'document':7},'context':{'game_kind':'chu','history':['same board','different history']}}
        text=json.dumps(native);split=len(text)//2
        journey.web.read.side_effect=[True,{'text':text[:split],'page':0,'pages':2},1,
            {'text':text[split:],'page':1,'pages':2}]
        journey.web.has.side_effect=[True,True,False]
        self.assertEqual(journey.native_diagnostics(),native)
        journey.web.button.assert_called_once_with('Next diagnostics page')
        self.assertEqual(journey.web.click.call_args_list[-1],unittest.mock.call('.support-dialog [data-cancel]'))

    def test_public_diagnostic_page_identity_and_size_are_bounded(self):
        for page in ({'text':'{}','page':1,'pages':2},{'text':'x'*65537,'page':0,'pages':1},
                     {'text':'{}','page':0,'pages':513}):
            journey=object.__new__(module.Journey);journey.web=Mock()
            journey.web.read.side_effect=[True,page];journey.web.has.side_effect=[True,True,False]
            with self.assertRaises(AssertionError):journey.native_diagnostics()
            journey.web.button.assert_not_called()
            self.assertEqual(journey.web.click.call_args_list[-1],unittest.mock.call('.support-dialog [data-cancel]'))

    def test_lightweight_inspection_observer_never_opens_support_or_invents_native_token(self):
        journey=object.__new__(module.Journey);journey.web=Mock()
        journey.inspection_state()
        expression=journey.web.read.call_args.args[0]
        self.assertNotIn('nativeToken',expression);self.assertNotIn('data-native-diagnostics',expression)
        journey.web.click.assert_not_called();journey.web.key.assert_not_called()


class CurrentShellCopyEvidenceTests(unittest.TestCase):
    def test_historical_copy_requires_native_selected_prefix_and_return_control(self):
        observed={'heading':'Historical position','meaning':'Your live game is unchanged.',
                  'historical':'true','event':'2','returnLiveEnabled':True}
        self.assertTrue(module.historical_notice_matches(observed,2))
        for field,value in (('heading','History'),('meaning','This game is read only.'),
                            ('historical','false'),('event','3'),('returnLiveEnabled',False)):
            self.assertFalse(module.historical_notice_matches({**observed,field:value},2),field)
        self.assertFalse(module.historical_notice_matches(None,2))

    def test_saved_evidence_summary_preserves_exact_capacity_transition_count(self):
        for count in (255,256):
            self.assertTrue(module.saved_chess_evidence_count(f'Saved analysis evidence ({count})',count))
            self.assertFalse(module.saved_chess_evidence_count(f'Saved analysis evidence ({count+1})',count))
            self.assertFalse(module.saved_chess_evidence_count(f'Recorded native evidence ({count})',count))
        self.assertFalse(module.saved_chess_evidence_count(None,256))



class ChuNormalProductEvidenceTests(unittest.TestCase):
    """Observer checks only; actual packaged GUI evidence must come from each OS."""
    def test_native_action_card_content_can_grow_without_moving_shared_anchors_or_gaps(self):
        def cards(action_height):
            return [{'name':'analysis','rect':{'left':860,'top':116,'width':292,'height':218}},
                    {'name':'actions','rect':{'left':860,'top':348,'width':292,'height':action_height}},
                    {'name':'history','rect':{'left':860,'top':362+action_height,'width':292,'height':362-action_height}}]
        previous=cards(90);current=cards(126)
        module.assert_shared_native_card_flow(current,previous)
        module.assert_shared_native_card_flow(previous,current)
        for index,axis in ((0,'top'),(0,'height'),(1,'top'),(1,'left'),(2,'width'),(2,'top'),(2,'height')):
            altered=copy.deepcopy(current);altered[index]['rect'][axis]+=10
            with self.subTest(index=index,axis=axis):
                with self.assertRaises(AssertionError):module.assert_shared_native_card_flow(altered,previous)
        altered=copy.deepcopy(current);altered[1]['rect']['height']=0
        with self.assertRaises(AssertionError):module.assert_shared_native_card_flow(altered,previous)
        with self.assertRaises(AssertionError):module.assert_shared_native_card_flow(list(reversed(current)),previous)

    def test_chu_reopen_uses_actual_native_saved_path_once_and_checks_evidence_bytes(self):
        with tempfile.TemporaryDirectory() as temp:
            actual=Path(temp)/'ordinary-saved.mshogi';evidence=Path(temp)/'external-evidence.mshogi'
            actual.write_bytes(b'exact native saved archive');evidence.write_bytes(actual.read_bytes())
            before={'document':{'path':str(actual)}};after={'native':'fresh stopped document'}
            journey=object.__new__(module.Journey);journey.native_diagnostics=Mock(side_effect=[before,after])
            journey.file=Mock();journey.wait_opened=Mock();journey.record=Mock()
            self.assertEqual(journey.chu_reopen_saved(evidence),after)
            journey.file.assert_called_once_with('open',actual);journey.wait_opened.assert_called_once_with(actual)
            self.assertEqual(journey.record.call_args.args[1]['evidence_copy'],str(evidence))
            evidence.write_bytes(b'different archive')
            journey.native_diagnostics=Mock(return_value=before);journey.file.reset_mock();journey.wait_opened.reset_mock()
            with self.assertRaises(AssertionError):journey.chu_reopen_saved(evidence)
            journey.file.assert_not_called();journey.wait_opened.assert_not_called()
        source=ast.parse(Path(module.__file__).read_text())
        for name,expected in (('normal_chu',1),('normal_chu_native',3)):
            method=next(node for node in ast.walk(source) if isinstance(node,ast.FunctionDef) and node.name==name)
            reopens=[node for node in ast.walk(method) if isinstance(node,ast.Call) and isinstance(node.func,ast.Attribute) and node.func.attr=='chu_reopen_saved']
            self.assertEqual(len(reopens),expected)

    def test_shared_shell_observes_approved_chu_header_without_renaming_the_product(self):
        rect={'left':0,'top':0,'width':100,'height':10}
        layout={'game':'chu','sections':['analysis','actions','history'],
            'themeIdentity':'mshogi-dark','themes':{'shared':'dark'},'squareCount':144,
            'allSquaresVisible':True,'pageWidth':1180,'viewport':{'width':1180,'height':820},
            'inspectorButtons':[{'label':'Play details','pressed':'true'},{'label':'Analyze position','pressed':'false'}],
            'slots':[{'name':name,'rect':{**rect,'top':index*10}} for index,name in enumerate(('upper-player','board','lower-player','footer'))],
            'help':rect,'helpVisible':True,'footnoteVisible':True,'documentState':'unsaved',
            'positionStatus':'Your turnLive position · 0 plies','playLimits':'Play limit: depth 1 · 512 nodes',
            'documentInHeader':True,'flipInHeading':True,'supportOutsideSidebar':True,
            'rawDiagnosticsAbsent':True,'brand':'歩','gameLabel':'CHU',
            'headerButtons':['New game','Save','Load'],'boardPanel':rect,'sidebar':{**rect,'left':200},
            'cards':[]}
        journey=object.__new__(module.Journey);journey.web=Mock();journey.record=Mock()
        journey.support_access=Mock();journey.shell_layouts={};journey.web.read.return_value=layout
        journey.shared_shell('chu')
        journey.support_access.assert_called_once_with('chu')
        journey.record.assert_called_once();journey.web.click.assert_not_called()
        for label in ('CHU SHOGI','TORI SHOGI','SHOGI',''):
            journey.web.read.return_value={**layout,'gameLabel':label}
            with self.assertRaises(AssertionError):journey.shared_shell('chu')

    def test_chu_royal_survivor_selects_native_no_promotion_before_committing(self):
        journey=object.__new__(module.Journey);journey.web=Mock()
        journey.chu_square=lambda square:square;journey.chu_events=Mock()
        journey.assert_chu_promotion_choices=Mock();journey.assert_completed_cue=Mock()
        action={'from':[12,10],'route':[[12,12]],'promote':False}
        native={'context':{'payload':{'events':[{'command':{'action':action}}]}}}
        journey.native_diagnostics=Mock(return_value=native)
        sequence=[]
        journey.web.button.side_effect=lambda label:sequence.append(('choice',label))
        journey.chu_events.side_effect=lambda count:sequence.append(('committed',count))
        with patch.object(module,'wait',side_effect=lambda check,label:check()):
            self.assertEqual(journey.chu_action('12j',['12l'],1,promote=False),native)
        self.assertEqual(sequence,[('choice','Do not promote'),('committed',1)])
        journey.assert_chu_promotion_choices.assert_called_once()
        self.assertEqual(journey.web.click.call_args_list,[unittest.mock.call('12j'),unittest.mock.call('12l')])
        journey.web.button.assert_called_once_with('Do not promote')
        source=ast.parse(Path(module.__file__).read_text())
        method=next(node for node in ast.walk(source) if isinstance(node,ast.FunctionDef) and node.name=='normal_chu_native')
        call=next(node for node in ast.walk(method) if isinstance(node,ast.Call) and isinstance(node.func,ast.Attribute)
                  and node.func.attr=='chu_action' and node.args and isinstance(node.args[0],ast.Constant) and node.args[0].value=='12j')
        self.assertTrue(any(keyword.arg=='promote' and isinstance(keyword.value,ast.Constant) and keyword.value.value is False for keyword in call.keywords))

    def test_all_default_phases_include_normal_chu_and_actual_pending_recovery(self):
        self.assertEqual(module.selected_gui_phases('normal-chu'), ['normal-chu','normal-chu-native'])
        self.assertEqual(module.selected_gui_phases('verification-chu'), ['pending-chu','chu-recovery'])
        self.assertTrue({'normal-chu','normal-chu-native','pending-chu','chu-recovery'} <= set(module.selected_gui_phases('all')))
        for method,product in ((module.Journey.normal_chu,'chu'),(module.Journey.normal_chu_native,'chu-native')):
            journey=object.__new__(module.Journey);journey.start=Mock(side_effect=RuntimeError('launch boundary'))
            with self.assertRaisesRegex(RuntimeError,'launch boundary'): method(journey)
            journey.start.assert_called_once_with(product=product)

    def test_complete_chu_action_identity_preserves_order_promotion_and_strict_shapes(self):
        expected={'from':[6,6],'route':[[7,5],[6,6]],'promote':False}
        text=json.dumps(expected)
        reordered=json.dumps({'promote':False,'route':[[7,5],[6,6]],'from':[6,6]})
        self.assertTrue(module.native_action_identity_matches(reordered,text))
        for change in ({'route':[[6,6],[7,5]]},{'route':[[7,5]]},{'promote':True},{'promote':0},
                       {'from':[True,6]},{'route':[]},{'route':[[7,5],[6,6],[7,5]]},{'unknown':'victim'}):
            self.assertFalse(module.native_action_identity_matches(json.dumps({**expected,**change}),text),change)
        self.assertFalse(module.native_action_identity_matches('{"from":[6,6],"route":[[7,5],[6,6]],"promote":false,"promote":false}',text))

    def test_chu_optional_promotion_observes_native_labels_despite_capitalized_innertext(self):
        journey=object.__new__(module.Journey);journey.web=Mock();journey.record=Mock()
        journey.web.text.return_value='Promote Do Not Promote Cancel Selection'
        journey.web.read.return_value=['Promote','Do not promote']
        self.assertEqual(journey.assert_chu_promotion_choices(),['Promote','Do not promote'])
        expression=journey.web.read.call_args.args[0]
        self.assertIn('textContent.trim()',expression)
        self.assertIn('.promotion-dialog .promotion-choices button',expression)
        self.assertNotIn('innerText',expression)
        journey.web.text.assert_not_called();journey.web.click.assert_not_called();journey.web.button.assert_not_called()
        for labels in (['Promote'],['Do not promote'],['Promote','Do Not Promote'],['Promote','Do not promote','invented action']):
            journey.web.read.return_value=labels
            with self.assertRaises(AssertionError):journey.assert_chu_promotion_choices()

    def test_capacity_reserved_resignation_requires_native_ending_zero_board_ply_and_all_observations(self):
        payload={'conclusion':None,'events':[],'root':{'native':'Chu root'},
            'current_position':{'accepted_plies':0,'native_board':'unchanged'},'current_protection':{'native':'rights'}}
        observations=[{'historical':True,'observation':{'native_context':i}} for i in range(256)]
        before={'context':{'game_kind':'chu','rules_profile':'mshogi.chu.jcsa2019.v1','payload':payload},'observations':observations}
        after=copy.deepcopy(before);after['context']['payload']['conclusion']={'kind':'resignation','winner':'gote'}
        after['context']['payload']['events']=[{'actor':'sente','command':{'kind':'resign'},'board_ply':0,'effects':None}]
        self.assertTrue(module.chu_capacity_resignation_matches(before,after))
        for mutate in (lambda a:a['observations'].pop(),lambda a:a['observations'][0].update(historical=False),
                       lambda a:a['observations'][0]['observation'].update(native_context='changed'),
                       lambda a:a['context']['payload']['current_position'].update(accepted_plies=1),
                       lambda a:a['context']['payload'].update(conclusion={'kind':'royal_capture','winner':'gote'}),
                       lambda a:a['context']['payload']['events'][0].update(actor='gote'),
                       lambda a:a['context']['payload']['events'][0].update(command={'kind':'stop'}),
                       lambda a:a['context']['payload'].update(root={'different':'root'})):
            changed=copy.deepcopy(after);mutate(changed)
            self.assertFalse(module.chu_capacity_resignation_matches(before,changed))

    def test_chu_archive_retains_exact_complete_route_before_takeback(self):
        action={'from':[6,6],'route':[[7,5],[6,6]],'promote':False}
        record={'game_kind':'chu','payload':{'root':{'native':'Chu root'},'events':[]}}
        body={'initial':copy.deepcopy(record),'current':copy.deepcopy(record),
            'controls':[{'kind':'native','actor':'sente','command':{'kind':'play','action':action}},
                        {'kind':'rewind','event_index':0,'reason':'take_back'}],
            'retired':[{'record':{'payload':{'events':[{'command':{'kind':'play','action':action}}]}}}]}
        self.assertTrue(module.retained_pending_takeback('chu',body,action))
        changed=copy.deepcopy(action);changed['route'].reverse()
        self.assertFalse(module.retained_pending_takeback('chu',body,changed))
        for mutate in (lambda b:b.update(retired=[]),lambda b:b['controls'][1].update(reason='undo'),
                       lambda b:b['current']['payload'].update(root={'different':'root'})):
            altered=copy.deepcopy(body);mutate(altered)
            self.assertFalse(module.retained_pending_takeback('chu',altered,action))

    def test_native_generated_archives_and_manifest_preserve_exact_adjudication_context(self):
        fixtures=Path(__file__).with_name('fixtures')
        manifest=json.loads((fixtures/'chu-native-fixtures.json').read_text())
        self.assertEqual(manifest['profile'],'mshogi.chu.jcsa2019.v1')
        self.assertEqual(len(manifest['cases']),17)
        bodies={case['file']:module.desktop_document_payload(json.loads((fixtures/case['file']).read_text()),'chu') for case in manifest['cases']}
        for name in ('contract','captures','promotion','royal-both','royal-survivor','material-reply'):
            self.assertIsNone(bodies[f'chu-{name}.mshogi']['current']['payload']['conclusion'])
        obligation=bodies['chu-repetition10.mshogi']['current']['payload']
        self.assertEqual(len(obligation['events']),10);self.assertTrue(obligation['obligations']);self.assertIsNone(obligation['conclusion'])
        offered=bodies['chu-agreement.mshogi']
        self.assertEqual(offered['human'],'gote');self.assertEqual(offered['current']['payload']['draw_offer'],'sente')
        self.assertEqual(offered['current']['payload']['current_position']['side'],'sente');self.assertIsNone(offered['current']['payload']['conclusion'])
        for name,conclusion in (('mate',{'kind':'mate','winner':'gote'}),('agreed-draw',{'kind':'agreed_draw'}),
                                ('royal-concluded',{'kind':'royal_capture','winner':'sente'}),('material-concluded',{'kind':'material','winner':'sente'})):
            self.assertEqual(bodies[f'chu-{name}.mshogi']['current']['payload']['conclusion'],conclusion)
        complete=bodies['chu-complete-resignation.mshogi']
        self.assertEqual(len(complete['observations']),12);self.assertEqual(len(complete['current']['payload']['events']),13)
        self.assertEqual(complete['current']['payload']['conclusion'],{'kind':'resignation','winner':'gote'})
        self.assertEqual({item['context']['game']['payload']['current_position']['side'] for item in complete['observations']},{'sente','gote'})
        natural=bodies['chu-complete-royal-engine.mshogi']
        self.assertEqual(natural['current']['payload']['conclusion'],{'kind':'royal_capture','winner':'sente'})
        self.assertEqual([item['result']['result']['kind'] for item in natural['observations']],['choice','finished'])
        capacity=bodies['chu-capacity256.mshogi']
        self.assertEqual(len(capacity['observations']),256);self.assertEqual(capacity['current']['payload']['events'],[])
        self.assertTrue(all(item['result']['result']['kind']=='incomplete' and item['result']['result']['telemetry']['score'] is None for item in capacity['observations']))

    def test_chu_geometry_observer_requires_all144_squares_font_and_no_hands(self):
        journey=object.__new__(module.Journey);journey.web=Mock();journey.record=Mock()
        geometry={'allVisible':True,'boardVisible':True,'hands':0,'minimumSquareWidth':24,
            'uniqueSquares':144,'nativeOrientation':'sente','orientation':'Chu board, Sente at bottom',
            'kanjiTexts':[{'pixels':11,'text':'歩'}],'pageWidth':760,'viewport':{'width':760}}
        for change in ({},{'allVisible':False},{'hands':1},{'uniqueSquares':143},{'orientation':'Chu board, Gote at bottom'},{'minimumSquareWidth':20},{'kanjiTexts':[]},{'kanjiTexts':[{'pixels':8}]}):
            journey.web.read.side_effect=[True,{**geometry,**change}]
            with patch.object(module,'wait',side_effect=lambda check,label:check()):
                if change:
                    with self.assertRaises(AssertionError): journey.chu_board_fits()
                else: self.assertEqual(journey.chu_board_fits(),geometry)
        expressions=[call.args[0] for call in journey.web.read.call_args_list]
        self.assertTrue(any('squares.length===144' in expression for expression in expressions))
        for expression in expressions:
            for forbidden in ('.click(','.focus(','scrollIntoView(','dispatchEvent(','invoke('):self.assertNotIn(forbidden,expression)
        journey.web.click.assert_not_called()


class ToriNormalProductEvidenceTests(unittest.TestCase):
    """Observer tests only; actual packaged journeys are separate CI evidence."""
    def test_normal_tori_phases_use_normal_package_not_qa_activation(self):
        self.assertEqual(module.selected_gui_phases('normal-tori'),
                         ['normal-tori', 'normal-tori-native'])
        for method, product in ((module.Journey.normal_tori, 'tori'),
                                (module.Journey.normal_tori_native, 'tori-native')):
            journey = object.__new__(module.Journey)
            journey.start = Mock(side_effect=RuntimeError('launch boundary'))
            with self.assertRaisesRegex(RuntimeError, 'launch boundary'):
                method(journey)
            journey.start.assert_called_once_with(product=product)
        self.assertTrue({'normal-tori', 'normal-tori-native', 'pending-tori',
                         'tori-recovery'} <= set(module.selected_gui_phases('all')))

    def test_tori_archive_lineage_requires_exact_complete_action_and_native_rewind(self):
        action = {'kind':'move', 'from':'3d', 'to':'3c', 'result_face':'swallow'}
        record = {'game_kind':'tori', 'payload':{'root':{'native':'root'}, 'events':[]}}
        body = {'initial':copy.deepcopy(record), 'current':copy.deepcopy(record),
                'controls':[{'kind':'native','actor':'sente','command':{'kind':'play','action':action}},
                            {'kind':'rewind','event_index':0,'reason':'take_back'}],
                'retired':[{'control_index':1,'record':{'payload':{'events':[
                    {'command':{'kind':'play','action':action}}]}}}]}
        self.assertTrue(module.retained_pending_takeback('tori', body, action))
        for mutate in (lambda b: b['controls'].reverse(),
                       lambda b: b['controls'][1].update(reason='undo'),
                       lambda b: b['current']['payload'].update(events=[{'not':'root'}]),
                       lambda b: b['current']['payload'].update(root={'different':'root'}),
                       lambda b: b.update(retired=[]),
                       lambda b: b['controls'][0].update(command={'kind':'resign'})):
            changed = copy.deepcopy(body); mutate(changed)
            self.assertFalse(module.retained_pending_takeback('tori', changed, action))
        self.assertFalse(module.retained_pending_takeback('tori', body,
                        {**action, 'result_face':'goose'}), 'promotion is part of complete native action identity')

    def test_native_generated_normal_documents_retain_known_prefix_and_independent_outcomes(self):
        fixtures = Path(__file__).with_name('fixtures')
        bodies = {name: module.desktop_document_payload(json.loads(
            (fixtures / f'tori-{name}.mshogi').read_text()), 'tori')
            for name in ('contract','mate-in-one','quiet-loss','forced-prefix7','obligation8')}
        for body in bodies.values():
            self.assertEqual(body['initial'], body['current'])
            self.assertEqual(body['observations'], [])
            self.assertEqual(body['controls'], [])
            self.assertFalse(body['current']['payload']['root']['historical_reachability_proven'])
            self.assertTrue(body['current']['payload']['root']['earlier_history_unknown'])
        forced = bodies['forced-prefix7']['current']['payload']
        self.assertEqual(len(forced['events']), 7)
        self.assertIsNone(forced['conclusion'])
        self.assertEqual(forced['events'][-1]['command']['action'],
                         {'kind':'move','from':'5a','to':'4a','result_face':'phoenix'})
        obligation = bodies['obligation8']['current']['payload']
        self.assertEqual(len(obligation['events']), 8)
        self.assertIsNone(obligation['conclusion'])
        self.assertEqual(obligation['obligation']['side'], 'sente')
        self.assertEqual(obligation['obligation']['occurrences'], [0,4,8])
        self.assertEqual(obligation['obligation']['forbidden_departures'],
                         [{'kind':'move','from':'4g','to':'5g','result_face':'phoenix'}])
        self.assertEqual(bodies['quiet-loss']['current']['payload']['conclusion'],
                         {'kind':'stalemate_loss','loser':'gote'})
        mate = bodies['mate-in-one']['current']['payload']
        self.assertIsNone(mate['conclusion'])
        self.assertIn({'square':'4c','piece':{'side':'sente','face':'falcon'}}, mate['root']['board'])
        self.assertEqual(mate['root']['side_to_move'], 'sente')

    def test_tori_font_and49_square_probe_is_distinct_from_shogi81(self):
        journey = object.__new__(module.Journey)
        journey.web, journey.record = Mock(), Mock()
        geometry = {'allSquaresVisible':True, 'bothPlayerAreasVisible':True,
                    'essentialControlsVisible':True, 'pageWidth':760,
                    'viewport':{'width':760,'height':650}, 'board':{'width':380}}
        journey.web.read.side_effect = [True, geometry]
        self.assertEqual(journey.shogi_board_fits(tori=True), geometry)
        font_expression, geometry_expression = [call.args[0] for call in journey.web.read.call_args_list]
        self.assertIn('mShogi Tori Pieces', font_expression)
        self.assertIn('squares.length===49', geometry_expression)
        self.assertNotIn('squares.length===81', geometry_expression)
        journey.web.command.assert_not_called()


class ToriDirtyCloseAdmissionTests(unittest.TestCase):
    """No native Close may be inferred from an unacknowledged preference input."""
    def controls(self):
        return {'documentState':'modified', 'event':'3', 'orientation':'gote',
                'flipEnabled':True, 'supportEnabled':True, 'fileControlsEnabled':True,
                'checkingCapacity':False}

    def native(self):
        before = {'document':{'path':'current.mshogi','dirty':False,'revision':14,
                   'settings':{'game_kind':'tori','settings':{'orientation':'sente'}}},
                  'context':{'payload':{'events':[{'exact':i} for i in range(3)],
                                       'conclusion':{'kind':'resignation','winner':'gote'}}},
                  'token':{'document':4,'version':40},'close_requires_save':False}
        after = copy.deepcopy(before)
        after['document'].update(dirty=True, revision=15)
        after['document']['settings']['settings']['orientation']='gote'
        after['token']['version']=41
        after['close_requires_save']=True
        return before, after

    def journey(self, observations, native=None):
        journey = object.__new__(module.Journey)
        journey.web, journey.record, journey.native = Mock(), Mock(), Mock()
        journey.web.read.side_effect=observations
        journey.native_diagnostics=Mock(side_effect=self.native() if native is None else native)
        return journey

    def test_clean_and_busy_snapshots_cannot_establish_accepted_dirty_close_source(self):
        ready = self.controls()
        self.assertTrue(module.dirty_close_controls_settled(ready, 'sente', 3))
        for field, value in (('documentState','unmodified'),('orientation','sente'),
                             ('orientation',None),('event','2'),('flipEnabled',False),
                             ('supportEnabled',False),('fileControlsEnabled',False),('checkingCapacity',True)):
            with self.subTest(field=field,value=value):
                self.assertFalse(module.dirty_close_controls_settled({**ready,field:value},'sente',3))

    def test_flip_dispatched_once_then_clean_busy_dirty_pending_and_dirty_settled_are_observed(self):
        ready = self.controls()
        clean = {**ready,'documentState':'unmodified','orientation':'sente'}
        busy = {**clean,'flipEnabled':False,'supportEnabled':False,'checkingCapacity':True}
        pending = {**ready,'supportEnabled':False}
        journey = self.journey([clean,busy,pending,ready])
        with patch.object(module.time,'sleep'):
            self.assertEqual(journey.dirty_tori_close_source(),self.native()[1])
        journey.web.click.assert_called_once_with('.flip-button')
        self.assertEqual(journey.web.read.call_count,4)
        self.assertEqual(journey.native_diagnostics.call_count,2)
        journey.native.close_window.assert_not_called()
        expression=journey.web.read.call_args.args[0]
        for mutation in ('invoke(', 'dispatchEvent(', '.click(', '.focus(', 'scrollIntoView('):
            self.assertNotIn(mutation,expression)

    def test_visible_dirty_without_authoritative_native_admission_cannot_pass(self):
        before,after=self.native()
        for mutate in (lambda a:a['document'].update(dirty=False),
                       lambda a:a.update(close_requires_save=False),
                       lambda a:a['token'].update(version=40),
                       lambda a:a['token'].update(document=5),
                       lambda a:a['document'].update(revision=14),
                       lambda a:a['document']['settings']['settings'].update(orientation='sente'),
                       lambda a:a['context']['payload']['events'].append({'unexpected':'move'})):
            changed=copy.deepcopy(after);mutate(changed)
            journey=self.journey([self.controls()],[before,changed])
            with self.assertRaises(AssertionError):
                journey.dirty_tori_close_source()
            journey.native.close_window.assert_not_called()

    def test_unknown_input_delivery_is_never_replayed(self):
        journey=self.journey([self.controls()])
        journey.web.click.side_effect=ConnectionResetError('delivery unknown')
        with self.assertRaises(ConnectionResetError):
            journey.dirty_tori_close_source()
        journey.web.click.assert_called_once_with('.flip-button')
        journey.web.read.assert_not_called()
        journey.native.close_window.assert_not_called()


if __name__ == '__main__':
    unittest.main(verbosity=2)
