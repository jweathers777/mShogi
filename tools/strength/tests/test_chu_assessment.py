import unittest
from tools.strength.chu_assessment import analyze, classify, probability


def row():
    return dict(game_id='game',ply=0,opening_id='opening',repeated=False,played={'captures':[]},
                before={'score':0},after={'score':1000})


class Assessment(unittest.TestCase):
    def test_score_sign_missing_and_native_draw(self):
        r=row();a=analyze([r]);self.assertEqual(a['clear_blunders'],1)
        r['after']={'terminal_probability':.5};self.assertEqual(analyze([r])['clear_blunders'],0)
        r['after']={'score':None};self.assertEqual(analyze([r])['unjudged_moves'],1)
        self.assertEqual(probability({'score':-999999}),0)
        self.assertEqual(probability({'score':999999}),1)

    def test_priority_and_lion_compound_facts(self):
        r=row();r['after']['best']={'captures_lion':True,'captures':[1,2]}
        self.assertEqual(classify(r),'lion_refutation')
        r['after']['best']['captures_lion']=False
        self.assertEqual(classify(r),'compound_refutation')
        r['before']['best']={'captures':[1]};self.assertEqual(classify(r),'missed_capture')
        r['repeated']=True;self.assertEqual(classify(r),'repetition')
        r['clock_ms']=999;self.assertEqual(classify(r),'time_trouble')

    def test_short_native_win_is_not_assumed_to_be_checkmate(self):
        r=row();r['after']={'score':999993}
        self.assertEqual(classify(r),'allowed_short_win')

    def test_same_opening_split_and_duplicate_refusal(self):
        r=row();s=dict(row(),game_id='other-color')
        a=analyze([r,s]);self.assertEqual(a['errors'][0]['split'],a['errors'][1]['split'])
        with self.assertRaises(ValueError):analyze([r,r])
