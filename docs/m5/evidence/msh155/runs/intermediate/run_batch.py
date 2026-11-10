import subprocess,sys,json,pathlib,time
root=pathlib.Path(__file__).parent
end=int(sys.argv[1])
while True:
 rows=[json.loads(l) for l in (root/'runs.jsonl').read_text().splitlines()] if (root/'runs.jsonl').exists() else []
 if len(rows)>=end:break
 if rows and rows[-1]['status']!=0:raise SystemExit('Prior failure requires inspection')
 used=sum(x['charged_active_seconds'] for x in rows)+0.306145542
 assert used<800,'Reserve100s for independent adjudication'
 p=subprocess.run([sys.executable,str(root/'driver.py'),'next'],capture_output=True,text=True)
 if p.returncode:print(p.stdout,p.stderr,flush=True);raise SystemExit(p.returncode)
 x=json.loads(p.stdout);r=json.load(open(x['result_path']));native=r.get('result',{})
 print(json.dumps({'cell':x['index'],'root':x['root_id'],'engine':x['engine'],'budget':x['seconds'],'active':round(x['charged_active_seconds'],3),'move':native.get('move',r.get('bestmove')),'depth':native.get('completed_depth'),'used':round(x['aggregate_charged_seconds']+0.306145542,3)}),flush=True)
