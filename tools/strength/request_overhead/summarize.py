"""Summarize saved local probe receipts; no builds, tests or cloud actions."""
import hashlib
import json
import platform
from pathlib import Path
import shutil
import statistics
import subprocess

p=Path('docs/m5/evidence/msh189')
def sha(f):return hashlib.sha256(Path(f).read_bytes()).hexdigest()
def save(name,x):(p/name).write_text(json.dumps(x,indent=2)+'\n')
head='b45ab4d43620e3b3cdc0d6db5541086145a2fd59'
# Never relabel an existing measured binary as a later changed native source.
subprocess.run(['git','diff','--exit-code',head,'HEAD','--','crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],check=True,capture_output=True)
subprocess.run(['git','diff','--exit-code',head,'--','crates','Cargo.toml','Cargo.lock','rust-toolchain.toml'],check=True,capture_output=True)
assert not subprocess.check_output(['git','ls-files','--others','--exclude-standard','crates']).strip()
source={'revision':head,'native_crates_tree':subprocess.check_output(['git','rev-parse',head+':crates'],text=True).strip(),'tracked_native_diff_sha256':hashlib.sha256(b'').hexdigest(),'untracked_native_files':[], 'build_time_provenance':'Built before commit: earlier HEAD52e078e6 plus dirty production source; parent confirmed byte-equivalent to committed b45 source. Not a clean-b45 build claim.','measurement_source_sha256':sha('tools/strength/request_overhead/probe.rs'),'files':{f:sha(f) for f in ['crates/mshogi-core/src/native/incremental.rs','crates/mshogi-core/src/shogi/game.rs','crates/mshogi-core/src/shogi/record.rs','crates/mshogi-engine/src/incremental.rs','crates/mshogi-engine/src/bridge.rs','crates/mshogi-engine/src/server.rs','crates/mshogi-engine/src/shared.rs','crates/mshogi-engine/src/shogi.rs','crates/mshogi-search/src/alphabeta/mod.rs','crates/mshogi-search/src/shogi.rs','Cargo.toml','Cargo.lock','rust-toolchain.toml']}}
save('candidate-source.json',source)
b=json.loads((p/'baseline-overhead.json').read_text());c=json.loads((p/'candidate-overhead.json').read_text());rows=[]
for x,y in zip(b['measurements'],c['measurements']):
    assert x['events']==y['events']
    def median(key,z):return statistics.median(z['samples'][key])
    bm=median('combined_nonsearch_cycle_ms',x);cm=statistics.median(y['incremental']['steady_two_action_nonsearch_ms']);bb=median('production_bridge_no_search_roundtrip_ms',x);cb=median('production_bridge_no_search_roundtrip_ms',y)
    rows.append({'events':x['events'],'baseline_combined_ms':bm,'candidate_combined_ms':cm,'combined_ratio':cm/bm,'baseline_bridge_ms':bb,'candidate_bridge_ms':cb,'bridge_ratio':cb/bb,'candidate_cold_prior_reset_ms':statistics.median(y['incremental']['cold_prior_history_full_reset_ms']),'candidate_root_setup_ms':median('root_setup_including_private_history_ms',y),'compact_request_bytes':y['incremental']['request_bytes'],'compact_response_bytes':y['incremental']['response_bytes']})
assert [r['events'] for r in rows]==[50,150,300]
summary={'baseline':'52e078e6d837807e8f02516a518d342a669ee08c','candidate_source_equivalent':head,'environment':{'platform':platform.platform(),'machine':platform.machine(),'cpu':'Apple M1 Pro per prior same-host evidence; direct sysctl query refused by sandbox','rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'profile':'release','concurrency':'idle coordinated window; baseline then candidate sequential'},'repetitions':9,'warmups':2,'measurement_source_sha256':sha('tools/strength/request_overhead/probe.rs'),'fixture_sha256':sha(p/'accepted-history.json'),'baseline_binary_sha256':sha('target/msh189/baseline-source/target/release/examples/request_overhead'),'candidate_binary_sha256':sha('target/release/examples/request_overhead'),'baseline_output_sha256':sha(p/'baseline-overhead.json'),'candidate_output_sha256':sha(p/'candidate-overhead.json'),'results':rows,'acceptance_300_event_le_10percent':rows[-1]['combined_ratio']<=.1 and rows[-1]['bridge_ratio']<=.1,'candidate_50_to_300_combined_growth_ms':rows[-1]['candidate_combined_ms']-rows[0]['candidate_combined_ms'],'candidate_50_to_300_bridge_growth_ms':rows[-1]['candidate_bridge_ms']-rows[0]['candidate_bridge_ms'],'limitations':['One frozen native trajectory; not a playing-strength study','First full reset remains history dependent and is measured separately','Host record creation/private history scans remain; no claim of strict O(1)','Bridge includes50us polling and OS scheduler noise','Preliminary baseline superseded after reviewer required exact successful no-search marker; preserved separately']}
save('overhead-comparison.json',summary)
for src,dest in [('target/msh189/baseline-build-03.log','baseline-build.txt'),('target/msh189/candidate-build.log','candidate-build.txt'),('tools/strength/request_overhead/probe.rs','probe.rs')]:shutil.copyfile(src,p/dest)
shutil.copyfile('tools/strength/request_overhead/probe.rs',p/'baseline-probe.rs')
bs=json.loads((p/'baseline-summary.json').read_text());bs.update(source_probe_sha256=summary['measurement_source_sha256'],binary_sha256=summary['baseline_binary_sha256'],output_sha256=summary['baseline_output_sha256'],results=[{'events':r['events'],'request_bytes':r['request_bytes'],'response_bytes':r['response_bytes'],'median_ms':{k:statistics.median(v) for k,v in r['samples'].items()}} for r in b['measurements']],oracle='Exact deliberate no-search Search failure marker required; all measurements reached admitted callback',supersedes='preliminary-baseline-summary.json after independent measurement success-oracle correction');save('baseline-summary.json',bs)
compat={'candidate_source_equivalent':head,'harness_binary_sha256':summary['candidate_binary_sha256'],'peers':[]}
for label,binary,file in [('legacy','target/strength/engines/1d19a544cc3e/mshogi','legacy-peer-compatibility.json'),('candidate','target/release/mshogi','candidate-peer-compatibility.json')]:
    x=json.loads((p/file).read_text());assert all(r['native_applied'] and r['telemetry']['completed_depth']==1 for r in x['results'])
    stderr=Path('target/msh189')/('legacy-peer.stderr' if label=='legacy' else 'candidate-peer.stderr');assert stderr.stat().st_size==0
    compat['peers'].append({'label':label,'binary':binary,'sha256':sha(binary),'receipt':file,'receipt_sha256':sha(p/file),'events':[r['events'] for r in x['results']],'depths':[r['telemetry']['completed_depth'] for r in x['results']],'shutdown_acknowledged':x['shutdown_acknowledged'],'stderr_bytes':0})
save('peer-compatibility-summary.json',compat)
print(json.dumps({'results':rows,'source':head},indent=2))
