"""Bounded descriptive reference analysis; no matches, tuning, or outcome adjudication."""
import hashlib,json,time,os,signal
from pathlib import Path
import chess,chess.engine
import assessment as a
import inspect_reports


def inputs():
    first=inspect_reports.inspect()['games'][0]
    rows=[]
    for version in [61,73,77]:
        turn=next(t for t in first['turns'] if t['engine']==0 and t['version']==version)
        rows.append({'id':'first-loss-v'+str(version),'report':first['pair'],'history':turn['history'],
                     'chosen':turn['move'],'candidate_latest':turn.get('latest'),'fen':None})
    row=next(r for r in a.read(a.HERE/'quality-inputs.json') if r['id']=='tb-7')
    rows.append({'id':'unique-rook-win','fen':row['fen'],'history':[],'chosen':'d1c1','proven_unique_win':'d1e1'})
    return rows


def attack_inputs():
    game=next(g for g in inspect_reports.inspect()['games'] if g['pair']=='pair-006-discovery-goldfish113' and g['pairing']==1)
    rows=[]
    for version in [62,66]:
        turn=next(t for t in game['turns'] if t['engine']==0 and t['version']==version)
        rows.append({'id':'goldfish-loss-v'+str(version),'report':game['pair'],'history':turn['history'],
                     'chosen':turn['move'],'candidate_latest':turn.get('latest'),'fen':None})
    return rows


def run(attack=False):
    v=a.read(a.HERE/'verification.json')
    if v['inflight']:raise ValueError('No concurrent match')
    path='/private/tmp/msh49/opponent/stockfish/stockfish-macos-universal'
    expected='8eed61129d1493c5d1f2fd9323f0c54c47ac49319911fbde18c6b9c87e8b13c5'
    if a.digest(path)!=expected:raise ValueError('different reference')
    label='attack' if attack else 'example'
    outpath=a.HERE/(label+'-analysis.json')
    inputpath=a.HERE/(label+'-inputs.json')
    if outpath.exists():raise ValueError('Preserve prior analysis, no automatic repeat')
    rows=attack_inputs() if attack else inputs();a.save(inputpath,rows)
    started=time.monotonic();results=[];engine=None;pid=None;settled=False;error=None
    v['inflight']={'kind':'example_analysis','started_utc':a.utc(),'launcher_pid':os.getpid(),'process_group':None};a.save(a.HERE/'verification.json',v)
    try:
        engine=chess.engine.SimpleEngine.popen_uci(path,timeout=3,start_new_session=True)
        pid=engine.transport.get_pid();v['inflight']['process_group']=pid;a.save(a.HERE/'verification.json',v)
        engine.configure({'Threads':1,'Hash':16,'UCI_LimitStrength':False,'Skill Level':20})
        for row in rows:
            board=chess.Board(row['fen']) if row['fen'] else chess.Board()
            for move in row['history']:board.push_uci(move)
            side=board.turn
            before=engine.analyse(board,chess.engine.Limit(time=3),game=object())
            board.push_uci(row['chosen'])
            after=engine.analyse(board,chess.engine.Limit(time=3),game=object())
            def details(info):return {'depth':info.get('depth'),'nodes':info.get('nodes'),'time':info.get('time'),'score_from_mover':str(info['score'].pov(side)),'cp_from_mover':info['score'].pov(side).score(mate_score=32000),'pv':[m.uci() for m in info.get('pv',[])]}
            results.append({'id':row['id'],'chosen':row['chosen'],'before':details(before),'after':details(after)})
    except BaseException as e:error=repr(e);raise
    finally:
        deadline=time.monotonic()+5
        if engine is not None:
            try:engine.quit()
            except BaseException:
                try:os.killpg(pid,signal.SIGKILL)
                except ProcessLookupError:pass
            engine.close()
            if pid is not None:
                if a.group_alive(pid):
                    try:os.killpg(pid,signal.SIGKILL)
                    except ProcessLookupError:pass
                while a.group_alive(pid) and time.monotonic()<deadline:time.sleep(.02)
                settled=not a.group_alive(pid)
        elapsed=time.monotonic()-started
        a.save(outpath,{'reference_sha256':expected,'reference':'Stockfish19unrestricted,1thread16MBdefaultembeddednetwork,go movetime3000','candidate_source':v['source'],'inputs_sha256':a.digest(inputpath),'code_sha256':a.digest(__file__),'seconds':elapsed,'results':results,'error':error,'process_group_settled':settled,'scope':'Representative three-second reference opinions, not proven game results. Rookwin independently certified by tablebase. No root score adjudication or match study.'})
        v=a.read(a.HERE/'verification.json');v['inflight']=None if settled else v['inflight'];v.setdefault('diagnostics',[]).append({'kind':'representative_reference_analysis','seconds':elapsed,'complete':len(results)==len(rows) and error is None and settled,'path':outpath.name});a.save(a.HERE/'verification.json',v)
    print(json.dumps(results,indent=2))


if __name__=='__main__':
    import sys
    with a.lock():run(attack=sys.argv[1:]==['attack'])
