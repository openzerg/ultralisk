use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ProcessStatus {
    Running,
    Completed,
    Failed,
    Timeout,
    Killed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub id: String,
    pub command: String,
    pub cwd: String,
    pub status: ProcessStatus,
    pub exit_code: Option<i32>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub parent_session_id: Option<String>,
    pub unit_name: String,
    pub timeout_ms: i32,
    pub output_dir: String,
    pub stdout_size: i32,
    pub stderr_size: i32,
    pub stdout_lines: i32,
    pub stderr_lines: i32,
}

#[derive(Debug, Clone)]
pub struct ProcessHandle {
    pub id: String,
    pub unit_name: String,
    pub output_dir: String,
    pub started_at: DateTime<Utc>,
    pub timeout_ms: i32,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub exit_code: i32,
    pub status: ProcessStatus,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputStats {
    pub stdout_size: i32,
    pub stderr_size: i32,
    pub stdout_lines: i32,
    pub stderr_lines: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOutputLine {
    pub num: i32,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOutput {
    pub process_id: String,
    pub stream: ProcessStream,
    pub lines: Vec<ProcessOutputLine>,
    pub total_lines: i32,
    pub has_more: bool,
    pub offset: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStream {
    Stdout,
    Stderr,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProcessData {
    pub id: String,
    pub command: String,
    pub cwd: String,
    #[serde(default)]
    pub parent_session_id: Option<String>,
    pub unit_name: String,
    pub output_dir: String,
    #[serde(default)]
    pub timeout_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnOptions {
    pub workdir: String,
    pub timeout: i32,
    #[serde(default)]
    pub env: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub session_id: Option<String>,
}
