"""Offline phase-policy regressions; no engine or network."""
import unittest
import chess
from phase_tags import tag_history, START_FEN

def history(san):
 b=chess.Board();moves=[]
 for word in san.split():
  m=b.parse_san(word);moves.append(m.uci());b.push(m)
 return moves

BASE='e4 e5 Nf3 Nc6 Bc4 Bc5 O-O Nf6 d3 O-O Nc3 d6'
class PhaseTags(unittest.TestCase):
 def test_three_boundaries_confirm_first_not_last(self):
  h=history(BASE+' Bg5 h6 Bh4 Be6')
  self.assertIsNone(tag_history(START_FEN,h[:12])['opening_boundary_ply'])
  self.assertIsNone(tag_history(START_FEN,h[:14])['opening_boundary_ply'])
  r=tag_history(START_FEN,h)
  self.assertEqual(r['opening_boundary_ply'],12)
  self.assertEqual(r['positions'][11]['phase'],'opening')
  self.assertEqual(r['positions'][12]['phase'],'middlegame')
  self.assertTrue(r['positions'][12]['fullmove_boundary'])
 def test_returned_original_minor_resets_pending_boundary(self):
  r=tag_history(START_FEN,history(BASE+' Nb1 a6 Nc3 Ba7 Bg5 Be6 Bxe6 fxe6'))
  self.assertEqual(r['positions'][14]['indicators']['sides']['white']['original_minors_home'],2)
  self.assertEqual(r['positions'][14]['indicators']['original_minors']['white_b1']['current'],'b1')
  self.assertEqual(r['opening_boundary_ply'],16)
 def test_capture_is_not_an_undeveloped_survivor(self):
  r=tag_history(START_FEN,history('e4 d5 Bc4 dxc4'))
  p=r['positions'][4]['indicators']
  self.assertTrue(p['original_minors']['white_f1']['captured'])
  self.assertEqual(p['sides']['white']['original_minors_home'],3)
  self.assertEqual(p['sides']['black']['original_minors_home'],4)
 def test_king_rights_disposition_does_not_claim_development(self):
  r=tag_history(START_FEN,history('e4 e5 Ke2 Ke7'))
  p=r['positions'][4]['indicators']
  self.assertTrue(all(s['king_disposed'] for s in p['sides'].values()))
  self.assertFalse(p['development_king_condition']);self.assertIsNone(r['opening_boundary_ply'])
 def test_material_threshold_precedes_unresolved_opening(self):
  fen='4k2q/4p3/8/8/8/8/8/RN1QK3 w - - 0 1'
  r=tag_history(fen,[],history_origin='imported')
  self.assertEqual(r['positions'][0]['indicators']['nonpawn_nonking_points'],26)
  self.assertEqual(r['positions'][0]['phase'],'endgame')
  self.assertTrue(r['opening_boundary_unresolved'])
  above=tag_history('4k2q/4p3/8/8/8/8/8/RNBQK3 w - - 0 1',[],history_origin='imported')
  self.assertEqual(above['positions'][0]['indicators']['nonpawn_nonking_points'],29)
  self.assertEqual(above['positions'][0]['phase'],'opening_or_middlegame_unresolved')
 def test_queenless_disposition_and_unknown_history(self):
  r=tag_history('r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1',[],history_origin='imported')
  p=r['positions'][0]['indicators'];self.assertTrue(p['both_queens_gone'])
  self.assertTrue(all(s['king_disposed'] and s['castling_rights_remaining'] for s in p['sides'].values()))
  unknown=tag_history(START_FEN,[],history_origin='imported')
  self.assertFalse(unknown['positions'][0]['indicators']['original_minor_identities_known'])
 def test_illegal_history_refused_and_input_not_mutated(self):
  h=['e2e4'];before=h.copy();r=tag_history(START_FEN,h);self.assertEqual(h,before)
  self.assertEqual(len(r['positions']),2)
  with self.assertRaises(ValueError):tag_history(START_FEN,['e2e5'])

if __name__=='__main__':unittest.main()
