import sys,pathlib,json,subprocess,time,resource,hashlib
out=pathlib.Path('/tmp/msh-adversarial-architecture-inputs');binary=pathlib.Path('/Users/johnweathers/Development/Personal/mShogi/target/release/examples/chess_frontier_profile')
index=int(sys.argv[1]);p=json.load(open(out/'panel-1s.json'))[index];p['time_ms']=5000;p['id']=f'profile-panel{index}'
name=f'profile-panel{index}';(out/(name+'.json')).write_text(json.dumps([p]))
start=time.monotonic();u=resource.getrusage(resource.RUSAGE_CHILDREN)
with open(out/(name+'.out.jsonl'),'w') as f,open(out/(name+'.err'),'w') as e:
 process=subprocess.Popen([str(binary),str(out/(name+'.json')),'64','timed','none'],stdout=f,stderr=e)
 sample=subprocess.run(['/usr/bin/sample',str(process.pid),'3','1','-file',str(out/(name+'.sample.txt'))],capture_output=True,text=True,timeout=8)
 (out/(name+'.sample.stderr')).write_text(sample.stdout+sample.stderr)
 try:status=process.wait(timeout=8)
 except subprocess.TimeoutExpired:process.kill();process.wait();status='watchdog'
v=resource.getrusage(resource.RUSAGE_CHILDREN)
r=dict(input=name,status=status,sampler_status=sample.returncode,wall_seconds=time.monotonic()-start,cpu_seconds=v.ru_utime+v.ru_stime-u.ru_utime-u.ru_stime,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),profile=True)
with open(out/'runs.jsonl','a') as f:f.write(json.dumps(r)+'\n')
print(json.dumps(r));print(sample.stdout+sample.stderr)
