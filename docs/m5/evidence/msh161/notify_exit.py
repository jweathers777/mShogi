#!/usr/bin/env python3
"""Run one bounded computation and queue its exit to the owning Codex chat.
Use normal execution escalation for Codex local database access; never change
security settings. Existing computation helpers retain their own process limits.
"""
import argparse, json, os, signal, subprocess, sys, time
from pathlib import Path

def save(path, value):
    temporary=path.with_suffix(path.suffix+'.tmp')
    with temporary.open('w') as f:
        json.dump(value,f,indent=2); f.write('\n'); f.flush(); os.fsync(f.fileno())
    temporary.replace(path)

def group_alive(pid):
    try:os.killpg(pid,0);return True
    except ProcessLookupError:return False

def settle_group(pid):
    if not group_alive(pid):return {'residual_group_detected':False,'process_group_settled':True}
    try:os.killpg(pid,signal.SIGTERM)
    except ProcessLookupError:pass
    deadline=time.monotonic()+2
    while group_alive(pid) and time.monotonic()<deadline:time.sleep(.02)
    if group_alive(pid):
        try:os.killpg(pid,signal.SIGKILL)
        except ProcessLookupError:pass
    deadline=time.monotonic()+2
    while group_alive(pid) and time.monotonic()<deadline:time.sleep(.02)
    return {'residual_group_detected':True,'process_group_settled':not group_alive(pid)}

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--thread',required=True)
    parser.add_argument('--codex',required=True,help='Absolute installed Codex CLI path')
    parser.add_argument('--receipt',required=True)
    parser.add_argument('--label',required=True)
    parser.add_argument('--state',required=True,help='Computation state artifact to inspect on wake')
    parser.add_argument('--message',default='Review the completed batch and continue the authorized study. Do not recreate a scheduled watcher.')
    parser.add_argument('command',nargs=argparse.REMAINDER)
    args=parser.parse_args()
    command=args.command[1:] if args.command[:1]==['--'] else args.command
    if not command:parser.error('A computation command after -- is required')
    receipt=Path(args.receipt).resolve(); receipt.parent.mkdir(parents=True,exist_ok=True)
    state={'label':args.label,'thread':args.thread,'command':command,'state_artifact':str(Path(args.state).resolve()),
           'wrapper_pid':os.getpid(),'started_epoch':time.time(),'status':'starting','queue_attempted':False,
           'cleanup_scope':'Own child process group only; engines detached by existing helpers require their own receipts/reconciliation; not a descendant-tree cleanup guarantee'}
    # Refuse duplicate launches/notifications, including interrupted earlier ones.
    with receipt.open('x') as f:json.dump(state,f)
    child=None; received_signal=None
    def interrupted(signum,frame):
        nonlocal received_signal
        received_signal=signum
        if child is not None:
            # Only the process group created by this wrapper, never unrelated work.
            try:os.killpg(child.pid,signal.SIGTERM)
            except OSError as exc:state['signal_forward_error']=repr(exc)
        raise InterruptedError(f'Wrapper received signal {signum}')
    for sig in (signal.SIGTERM,signal.SIGINT):signal.signal(sig,interrupted)
    result=1
    try:
        child=subprocess.Popen(command,start_new_session=True)
        state.update(status='running',child_pid=child.pid);save(receipt,state)
        result=child.wait()
        state['computation_returncode']=result
        state['status']='success' if result==0 else 'failure'
    except BaseException as exc:
        state.update(status='failure',error=repr(exc))
        if child is not None:
            try:
                try:os.killpg(child.pid,signal.SIGTERM)
                except ProcessLookupError:pass
                try:child.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    try:os.killpg(child.pid,signal.SIGKILL)
                    except ProcessLookupError:pass
                    child.wait(timeout=5)
                state['computation_returncode']=child.returncode
            except Exception as cleanup_error:
                state['cleanup_error']=repr(cleanup_error)
                state['cleanup_review_required']=True
        if received_signal is not None:state['signal']=received_signal
    finally:
        # Do not interrupt the one bounded notification attempt during cleanup.
        for sig in (signal.SIGTERM,signal.SIGINT):signal.signal(sig,signal.SIG_IGN)
        if child is not None:
            try:state.update(settle_group(child.pid))
            except Exception as cleanup_error:
                state.update(process_group_settled=False,residual_group_detected=None,cleanup_error=repr(cleanup_error))
            if state['residual_group_detected'] or not state['process_group_settled']:
                state['status']='failure';state['cleanup_review_required']=True
        state['finished_epoch']=time.time();state['queue_attempted']=True;save(receipt,state)
        message=(f"Computation wrapper exited: {args.label}; status={state['status']}; "
                 f"returncode={state.get('computation_returncode')}. "
                 f"State: {state['state_artifact']}. Notification receipt: {receipt}. {args.message}")
        try:
            with receipt.with_suffix('.queue.stdout.log').open('x') as out, receipt.with_suffix('.queue.stderr.log').open('x') as err:
                queued=subprocess.run([args.codex,'queue','--thread',args.thread,'--message',message],
                                      stdout=out,stderr=err,timeout=30)
            state['queue_returncode']=queued.returncode
            state['queue_accepted']=queued.returncode==0
        except Exception as exc:
            state.update(queue_accepted=False,queue_error=repr(exc))
        save(receipt,state)
    # Notification failure is visible even if computation succeeded. Never retry
    # automatically: an ambiguous timeout could already have queued the message.
    return (0 if result==0 and state['status']=='success' else 1) if state['queue_accepted'] else 2

if __name__=='__main__':sys.exit(main())
