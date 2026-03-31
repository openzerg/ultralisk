use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;
use chrono::Utc;
use crate::core::{ProcessHandle, SpawnOptions};

fn get_output_base_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".openzerg")
        .join("processes")
}

pub fn is_in_systemd_scope() -> bool {
    fs::read_to_string("/proc/self/cgroup")
        .map(|c| c.contains("openzerg-"))
        .unwrap_or(false)
}

pub fn get_systemd_slice_name() -> Option<String> {
    let cgroup = fs::read_to_string("/proc/self/cgroup").ok()?;
    let re = regex::Regex::new(r"openzerg-([^.]+)\.slice").unwrap();
    re.captures(&cgroup)
        .map(|caps| format!("openzerg-{}", &caps[1]))
}

pub struct ExecutorResult {
    pub handle: ProcessHandle,
    pub pid: u32,
}

pub async fn execute_with_systemd(
    command: &str,
    options: SpawnOptions,
    slice_name: &str,
) -> anyhow::Result<ExecutorResult> {
    let process_id = Uuid::new_v4().to_string();
    let unit_name = format!("openzerg-{}.scope", process_id);
    let output_base_dir = get_output_base_dir();
    let output_dir = output_base_dir.join(&process_id);
    let stdout_file = output_dir.join("stdout");
    let stderr_file = output_dir.join("stderr");
    let exitcode_file = output_dir.join("exitcode");

    fs::create_dir_all(&output_dir)?;

    let mut args = vec![
        "--user".to_string(),
        "--scope".to_string(),
        "--slice".to_string(),
        slice_name.to_string(),
        "-p".to_string(),
        "KillMode=control-group".to_string(),
        "--unit".to_string(),
        unit_name.to_string(),
        "--working-directory".to_string(),
        options.workdir.clone(),
        format!("--setenv=OPENZERG_PROCESS_ID={}", process_id),
    ];

    if let Some(env) = &options.env {
        for (key, value) in env {
            args.push(format!("--setenv={}={}", key, value));
        }
    }

    args.push("--".to_string());

    let wrapped_command = format!(
        "({}) >> {} 2>> {}; echo $? > {}",
        command,
        stdout_file.display(),
        stderr_file.display(),
        exitcode_file.display()
    );
    args.push("bash".to_string());
    args.push("-lc".to_string());
    args.push(wrapped_command);

    let mut systemd_run = Command::new("systemd-run")
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let pid = systemd_run.id();

    let handle = ProcessHandle {
        id: process_id,
        unit_name,
        output_dir: output_dir.to_string_lossy().to_string(),
        started_at: Utc::now(),
        timeout_ms: options.timeout,
        session_id: options.session_id,
    };

    Ok(ExecutorResult { handle, pid })
}

pub async fn kill_process(unit_name: &str, signal: &str) -> anyhow::Result<()> {
    let output = Command::new("systemctl")
        .args(&["--user", "kill", "--signal", signal, unit_name])
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "systemctl kill failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}