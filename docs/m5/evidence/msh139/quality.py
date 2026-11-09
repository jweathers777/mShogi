"""Small frozen independent quality diagnostic. No product changes or rating inference."""
import gzip,io,json,subprocess,time,logging,os,signal
from pathlib import Path
import chess,chess.pgn,chess.engine
import assessment as a
B=a.HERE


def fixtures():
    rows=[]
    for item in json.loads(gzip.decompress((B/'tactical-source.json.gz').read_bytes()))['puzzles']:
        game=chess.pgn.read_game(io.StringIO(item['game']['pgn']))
        if game.errors:raise ValueError(game.errors)
        board=game.end().board(); moves=[m.uci() for m in game.mainline_moves()]
        assert len(moves)==item['puzzle']['initialPly']+1  # API uses zero-based setup ply index
        line=item['puzzle']['solution']; check=board.copy()
        for move in line:check.push_uci(move)
        rows.append({'kind':'puzzle','id':item['puzzle']['id'],'fen':board.fen(),
                     'history':moves,'expected_line':line,'themes':item['puzzle']['themes'],
                     'published_puzzle_rating':item['puzzle']['rating']})
    for i,item in enumerate(a.read(B/'endgame-source.json')):
        rows.append({'kind':'tablebase','id':'tb-'+str(i),'fen':item['fen'],
                     'history':[],'expected_category':item['response']['category'],
                     'tb_moves':{m['uci']:m['category'] for m in item['response']['moves']}})
    return rows


def native_admit(binary,row,line):
    if row['history']:
        commands='position startpos\nmove '+' '.join(row['history'])+'\n'
    else:commands='position fen '+row['fen']+'\n'
    if line:commands+='move '+' '.join(line)+'\n'
    commands+='fen\nquit\n'
    result=subprocess.run([binary,'shell'],input=commands,text=True,capture_output=True,timeout=5)
    if result.returncode or result.stderr:raise ValueError(result.stderr)
    return {'command_sha256':__import__('hashlib').sha256(commands.encode()).hexdigest(),'returncode':result.returncode,'final_native_fen':result.stdout.split('fen ')[-1].strip()}


def run():
    ledger=a.read(B/'verification.json')
    if ledger.get('inflight'):raise ValueError('Do not compete with a timed match')
    binary=ledger['build']['candidate']['path']
    if a.digest(binary)!=ledger['build']['candidate']['sha256']:raise ValueError('candidate changed')
    rows=fixtures(); input_path=B/'quality-inputs.json'
    if input_path.exists() and a.read(input_path)!=rows:raise ValueError('frozen quality inputs changed')
    if not input_path.exists():a.save(input_path,rows)
    if (B/'quality-results.json').exists():raise ValueError('Preserve prior diagnostic; do not automatically rerun')
    started=time.monotonic();results=[];error=None;current=None;engine=None;pid=None;settled=False
    ledger['inflight']={'kind':'quality','launcher_pid':os.getpid(),'started_utc':a.utc(),'process_group':None}
    a.save(B/'verification.json',ledger)
    logging.basicConfig(filename='/private/tmp/msh139/quality-uci.log',level=logging.DEBUG)
    try:
        engine=chess.engine.SimpleEngine.popen_uci([binary,'uci'],timeout=3,start_new_session=True)
        pid=engine.transport.get_pid();ledger['inflight']['process_group']=pid
        a.save(B/'verification.json',ledger)
        if engine:
            for row in rows:
                current=row['id']
                board=chess.Board() if row['history'] else chess.Board(row['fen'])
                for move in row['history']:board.push_uci(move)
                assert board.fen()==row['fen']
                expected=row.get('expected_line',[])
                admitted=native_admit(binary,row,expected)
                begin=time.monotonic()
                info=engine.analyse(board,chess.engine.Limit(time=1),game=object())
                if not info.get('pv'):raise ValueError('No completed PV for '+current)
                elapsed=time.monotonic()-begin
                move=info['pv'][0].uci()
                row_result={'id':row['id'],'kind':row['kind'],'move':move,'seconds':elapsed,
                            'depth':info.get('depth'),'seldepth':info.get('seldepth'),'nodes':info.get('nodes'),
                            'score_cp_side_to_move':info['score'].pov(board.turn).score(mate_score=32000),
                            'score':str(info['score']), 'pv':[m.uci() for m in info.get('pv',[])],
                            'expected_native_admission':admitted,'chosen_native_admission':native_admit(binary,row,[move])}
                if not expected:
                    legal=board.legal_moves
                    if set(row['tb_moves'])!={m.uci() for m in legal}:raise ValueError('Incomplete tablebase alternatives')
                if expected:row_result['published_first_move_agreement']=move==expected[0]
                else:
                    child=row['tb_moves'][move];inverse={'win':'loss','loss':'win','draw':'draw','cursed-win':'blessed-loss','blessed-loss':'cursed-win'}
                    row_result.update(child_tablebase_category=child,wdl_preserved=inverse[child]==row['expected_category'])
                results.append(row_result)
                a.save(B/'quality-progress.json',{'started_utc':a.utc(),'results':results,'seconds':time.monotonic()-started})
    except BaseException as failure:
        error={'position':current,'exception':repr(failure)}
        raise
    finally:
        deadline=time.monotonic()+5
        if engine is not None:
            try: engine.quit()
            except BaseException:
                if pid is not None:
                    try:os.killpg(pid,signal.SIGKILL)
                    except ProcessLookupError:pass
            finally:
                engine.close()
                if pid is not None and a.group_alive(pid):
                    try:os.killpg(pid,signal.SIGKILL)
                    except ProcessLookupError:pass
                    while a.group_alive(pid) and time.monotonic()<deadline:time.sleep(.02)
                settled=pid is not None and not a.group_alive(pid)
        else:settled=False  # startup may have spawned a child before raising; retain uncertainty
        elapsed=time.monotonic()-started
        a.save(B/'quality-results.json',{'source':ledger['build']['candidate']['source'],'candidate_sha256':a.digest(binary),
                    'inputs_sha256':a.digest(input_path),'code_sha256':a.digest(__file__),'seconds':elapsed,'results':results,'error':error,'process_group_settled':settled,
                    'scope':'Small independent diagnostic; published puzzle move disagreement needs alternative analysis, not automatic error. Tablebasecategories from child side. Not a tactical rating or strength estimate.'})
        ledger=a.read(B/'verification.json');ledger['inflight']=None if settled else ledger['inflight'];ledger.setdefault('diagnostics',[]).append({'kind':'quality','seconds':elapsed,'results':'quality-results.json','complete':len(results)==len(rows) and error is None and settled})
        a.save(B/'verification.json',ledger)
        log=Path('/private/tmp/msh139/quality-uci.log')
        if log.exists():(B/'quality-uci.log.gz').write_bytes(gzip.compress(log.read_bytes(),mtime=0))
    (B/'quality-progress.json').unlink(missing_ok=True)
    print(json.dumps(results,indent=2))


if __name__=='__main__':
    with a.lock(): run()
