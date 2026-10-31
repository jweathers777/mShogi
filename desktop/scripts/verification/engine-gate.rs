//! External R6 verifier only. Delay startup, then run the unchanged real engine.
//! No protocol parsing, generated responses, telemetry or product limit changes.
use std::{
    env,
    process::{Command, ExitCode},
    time::Duration,
};
fn main() -> ExitCode {
    let result = run();
    match result {
        Ok(code) => ExitCode::from(code as u8),
        Err(error) => {
            eprintln!("R6 external engine gate failed: {error}");
            ExitCode::FAILURE
        }
    }
}
fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let real = env::var_os("MSHOGI_R6_REAL_ENGINE").ok_or("missing real engine")?;
    let delay: u64 = env::var("MSHOGI_R6_DELAY_MS")?.parse()?;
    if delay > 5000 {
        return Err("delay exceeds external verification bound".into());
    }
    std::thread::sleep(Duration::from_millis(delay));
    let mut command = Command::new(real);
    command.args(env::args_os().skip(1));
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        Err(command.exec().into())
    }
    #[cfg(windows)]
    {
        windows::wait(command)
    }
}
#[cfg(windows)]
mod windows {
    use super::*;
    use std::{ffi::c_void, mem::size_of, ptr::null_mut};
    type Handle = *mut c_void;
    #[repr(C)]
    #[derive(Default)]
    struct Basic {
        process_time: i64,
        job_time: i64,
        flags: u32,
        min_working: usize,
        max_working: usize,
        active: u32,
        affinity: usize,
        priority: u32,
        scheduling: u32,
    }
    #[repr(C)]
    #[derive(Default)]
    struct Extended {
        basic: Basic,
        io: [u64; 6],
        process_memory: usize,
        job_memory: usize,
        peak_process: usize,
        peak_job: usize,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn CreateJobObjectW(attributes: *mut c_void, name: *const u16) -> Handle;
        fn SetInformationJobObject(
            job: Handle,
            class: i32,
            info: *const c_void,
            length: u32,
        ) -> i32;
        fn AssignProcessToJobObject(job: Handle, process: Handle) -> i32;
        fn GetCurrentProcess() -> Handle;
    }
    pub fn wait(mut command: Command) -> Result<i32, Box<dyn std::error::Error>> {
        let job = unsafe { CreateJobObjectW(null_mut(), std::ptr::null()) };
        if job.is_null() {
            return Err(std::io::Error::last_os_error().into());
        }
        let mut limits = Extended::default();
        limits.basic.flags = 0x2000; // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        if unsafe {
            SetInformationJobObject(
                job,
                9,
                (&limits as *const Extended).cast(),
                size_of::<Extended>() as u32,
            )
        } == 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        // Join the gate itself before creating the real engine. Children inherit
        // job membership atomically, eliminating a spawn/assign orphan window.
        if unsafe { AssignProcessToJobObject(job, GetCurrentProcess()) } == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        // Intentionally keep this non-inheritable handle until process exit.
        // Closing it in a Drop would terminate this gate before returning its
        // real child's exit status. OS cleanup closes it on ExitProcess, killing
        // any remaining job member on cancellation as well as normal shutdown.
        let mut child = command.spawn()?;
        Ok(child.wait()?.code().unwrap_or(1))
    }
}
