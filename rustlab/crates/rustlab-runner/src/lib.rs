use std::collections::VecDeque;
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use rustlab_core::{CommandOutput, CommandSpec, ProcessRunner};
use wait_timeout::ChildExt;

pub struct LocalRunner;

impl LocalRunner {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocalRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessRunner for LocalRunner {
    fn run(&self, cmd: &CommandSpec) -> Result<CommandOutput> {
        let started = Instant::now();
        let mut command = Command::new(&cmd.program);
        command
            .args(&cmd.args)
            .current_dir(&cmd.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(if cmd.stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            });

        for (key, _) in std::env::vars_os() {
            if key.to_string_lossy().starts_with("RUSTLAB_") {
                command.env_remove(key);
            }
        }
        command.envs(&cmd.env);

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }

        #[cfg(windows)]
        let job = create_job();

        let mut child = command
            .spawn()
            .with_context(|| format!("failed to spawn {}", cmd.program))?;

        #[cfg(windows)]
        let job = match job {
            Some(job) if assign_job(&job, &child) => Some(job),
            _ => None,
        };

        let stdin_handle = cmd.stdin.clone().and_then(|input| {
            child.stdin.take().map(|mut stdin| {
                std::thread::spawn(move || {
                    let _ = stdin.write_all(input.as_bytes());
                })
            })
        });

        let cap = if cmd.output_cap_bytes == 0 {
            64 * 1024
        } else {
            cmd.output_cap_bytes
        };
        let overflow = Arc::new(AtomicBool::new(false));
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let out_handle = spawn_reader(stdout, cap, overflow.clone());
        let err_handle = spawn_reader(stderr, cap, overflow.clone());

        let timeout = Duration::from_secs(cmd.timeout_secs.max(1));
        let status = child.wait_timeout(timeout)?;
        let timed_out = status.is_none();

        #[cfg(windows)]
        drop(job);

        #[cfg(unix)]
        unsafe {
            libc::kill(-(child.id() as i32), libc::SIGKILL);
        }

        if timed_out {
            kill_tree(&mut child);
            let _ = child.wait();
        }

        if let Some(handle) = stdin_handle {
            let _ = handle.join();
        }

        let stdout = out_handle.join().unwrap_or_default();
        let stderr = err_handle.join().unwrap_or_default();

        if overflow.load(Ordering::SeqCst) {
            anyhow::bail!("output exceeded {} bytes", cap);
        }

        Ok(CommandOutput {
            status: child
                .try_wait()?
                .and_then(|status| status.code())
                .or_else(|| status.and_then(|status| status.code())),
            stdout,
            stderr,
            timed_out,
            duration_ms: started.elapsed().as_millis(),
        })
    }
}

fn spawn_reader(
    stream: Option<impl Read + Send + 'static>,
    cap: usize,
    overflow: Arc<AtomicBool>,
) -> std::thread::JoinHandle<String> {
    std::thread::spawn(move || {
        let Some(mut stream) = stream else {
            return String::new();
        };
        let total = Arc::new(AtomicUsize::new(0));
        let mut output = Vec::new();
        let mut buffer = [0_u8; 8192];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => {
                    let current = total.fetch_add(read, Ordering::SeqCst) + read;
                    if current > cap {
                        overflow.store(true, Ordering::SeqCst);
                        break;
                    }
                    output.extend_from_slice(&buffer[..read]);
                }
                Err(_) => break,
            }
        }
        String::from_utf8_lossy(&output).into_owned()
    })
}

#[cfg(windows)]
struct JobHandle(windows_sys::Win32::Foundation::HANDLE);

#[cfg(windows)]
impl Drop for JobHandle {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

#[cfg(windows)]
fn create_job() -> Option<JobHandle> {
    use std::mem::size_of;

    use windows_sys::Win32::System::JobObjects::{
        CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JobObjectExtendedLimitInformation, SetInformationJobObject,
    };

    unsafe {
        let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job.is_null() {
            return None;
        }
        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        let ok = SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &info as *const _ as *const _,
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        );
        if ok == 0 {
            windows_sys::Win32::Foundation::CloseHandle(job);
            return None;
        }
        Some(JobHandle(job))
    }
}

#[cfg(windows)]
fn assign_job(job: &JobHandle, child: &Child) -> bool {
    use std::os::windows::io::AsRawHandle;

    use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;

    let ok = unsafe { AssignProcessToJobObject(job.0, child.as_raw_handle() as _) };
    if ok == 0 {
        eprintln!("warning: could not assign the child process to the job object");
        false
    } else {
        true
    }
}

fn kill_tree(child: &mut Child) {
    #[cfg(windows)]
    {
        let pid = child.id().to_string();
        let _ = Command::new("taskkill")
            .args(["/PID", pid.as_str(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(-(child.id() as i32), libc::SIGKILL);
        }
    }
    let _ = child.kill();
}

#[derive(Default)]
pub struct MockRunner {
    outputs: Mutex<VecDeque<CommandOutput>>,
    commands: Mutex<Vec<CommandSpec>>,
}

impl MockRunner {
    pub fn new(outputs: Vec<CommandOutput>) -> Self {
        Self {
            outputs: Mutex::new(outputs.into()),
            commands: Mutex::new(Vec::new()),
        }
    }

    pub fn push(&self, output: CommandOutput) {
        self.outputs.lock().expect("mock outputs").push_back(output);
    }

    pub fn commands(&self) -> Vec<CommandSpec> {
        self.commands.lock().expect("mock commands").clone()
    }
}

impl ProcessRunner for MockRunner {
    fn run(&self, cmd: &CommandSpec) -> Result<CommandOutput> {
        self.commands
            .lock()
            .expect("mock commands")
            .push(cmd.clone());
        self.outputs
            .lock()
            .expect("mock outputs")
            .pop_front()
            .with_context(|| format!("MockRunner has no output for {}", cmd.program))
    }
}

#[cfg(test)]
mod tests;
