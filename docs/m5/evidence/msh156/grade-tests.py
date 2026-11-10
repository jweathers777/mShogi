import importlib.util,json
spec=importlib.util.spec_from_file_location('grader','/tmp/msh156-grade.py');g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
def root(e):return {'id':'fixture','expectation':e}
tb=root({'tier':'exact-tablebase-wdl','root_wdl':1,'move_wdl':{'a1a2':1,'a1b1':0,'a1b2':-1}})
assert g.compare(tb,'a1b1','a1a2')['outcome_change']=='exact-improved'
assert g.compare(tb,'a1a2','a1b1')['outcome_change']=='exact-regressed'
assert g.grade(tb,'a1c1')['judgment']=='invalid-or-incomplete-exact-map'
assert g.compare(tb,'a1c1','a1c1')['outcome_change']=='invalid-exact-comparison'
mate=root({'tier':'exact-mate-proof','mate_moves':['a1a2'],'legal_moves':['a1a2','a1b1'],'complete':True})
assert g.compare(mate,'a1b1','a1a2')['outcome_change']=='exact-improved'
mate['expectation']['complete']=False;assert g.grade(mate,'a1b1')['judgment']=='unresolved'
mate1=root({'tier':'exact-mate-in-one','preferred_moves':['a1a2'],'legal_moves':['a1a2','a1b1'],'all_legal_roots_enumerated':2})
assert g.grade(mate1,'a1b1')['judgment']=='exact-goal-missed'
assert g.grade(mate1,'a1a2')['goal']=='immediate mate'
assert g.grade(mate1,'a1c1')['judgment']=='invalid-or-incomplete-exact-map'
assert g.compare(mate1,'a1c1','a1c1')['outcome_change']=='invalid-exact-comparison'
sts=root({'tier':'published-strategic-preference','published_points':{'a1a2':10,'a1b1':3}})
assert g.compare(sts,'a1b1','a1a2')['outcome_change']=='published-preference-improved'
assert g.compare(sts,'a1c1','a1a2')['outcome_change']=='unresolved'
for tier in ['published-puzzle-preference','finite-reference-opinion']:
 r=root({'tier':tier,'preferred_moves':['a1a2','a1b1']})
 assert g.compare(r,'a1a2','a1b1')['outcome_change']=='changed-within-published-preferences'
 assert g.compare(r,'a1c1','a1a2')['outcome_change']=='unresolved'
 assert g.compare(r,'a1c1','a1c1')['outcome_change']=='unchanged-decision'
assert g.compare(tb,'a1a2','a1a2')['outcome_change']=='unchanged-decision'
assert g.grade(tb,None)['judgment']=='unresolved'
assert g.compare(tb,None,None)['outcome_change']=='unresolved'
print(json.dumps({'passed':['exact WDL improvement/regression','complete map missing move invalid','exhaustive vs incomplete mate proof','STS points distinct from exact quality','unlisted STS unresolved','puzzle/opinion preferred alternatives retained','unknown to preferred remains unresolved','same move unchanged regardless external scores','no move unresolved','mate-in-one objective distinct from lost win'],'heldout_access':False,'engine_searches':0},indent=2))
