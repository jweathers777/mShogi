"""Sequential reviewed submit and event/retrieval workflow, wrapped by notify_exit."""
import argparse
from pathlib import Path
import adjudication_control as control

def main():
    p=argparse.ArgumentParser()
    p.add_argument('--directory',required=True)
    p.add_argument('--job-name',required=True)
    p.add_argument('--subscription',required=True)
    p.add_argument('--cache',required=True)
    p.add_argument('--spending-approved',action='store_true',required=True)
    a=p.parse_args()
    directory=Path(a.directory).resolve()
    plan=control.read(directory/'plan.json')
    if control.core.file_hash(control.__file__)!=plan['controller_sha256']:
        raise ValueError('Controller changed after plan freeze')
    control.submit(argparse.Namespace(directory=str(directory),job_name=a.job_name,spending_approved=True))
    control.wait_verify(argparse.Namespace(plan=str(directory/'plan.json'),job_name=a.job_name,
        subscription=a.subscription,cache=a.cache,output_directory=str(directory/'results')))

if __name__=='__main__':main()
