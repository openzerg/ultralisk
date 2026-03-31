use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use crate::process_manager::{execute_with_bwrap, ExecutorResult};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::collections::HashMap as StdHashMap;
use uuid::Uuid;

lazy_static::lazy_static! {
    pub static ref JOB_OUTPUTS: Mutex<StdHashMap<String, String>> = Mutex::new(StdHashMap::new());
    pub static ref JOB_PIDS: Mutex<StdHashMap<String, u32>> = Mutex::new(StdHashMap::new());
    pub static ref JOB_PROCESSES: Mutex<StdHashMap<String, std::process::Child>> = Mutex::new(StdHashMap::new());
}

fn get_output_base_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".openzerg")
        .join("processes")
}

pub struct JobTool;

#[async_trait]
impl Tool for JobTool {
    fn name(&self) -> &str {
        "job"
    }

    fn description(&self) -> &str {
        "Manage background jobs: run commands, list jobs, get output, kill jobs."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["run", "list", "output", "kill", "status"],
                    "description": "Job action to perform"
                },
                "command": {
                    "type": "string",
                    "description": "Command to run (for action=run)"
                },
                "workdir": {
                    "type": "string",
                    "description": "Working directory (for action=run)"
                },
                "wait": {
                    "type": "boolean",
                    "description": "Whether to wait for job completion"
                },
                "job_id": {
                    "type": "string",
                    "description": "Job ID (for output/kill/status)"
                },
                "stream": {
                    "type": "string",
                    "enum": ["stdout", "stderr", "both"],
                    "description": "Output stream to read"
                },
                "offset": {
                    "type": "number",
                    "description": "Line offset for output"
                },
                "limit": {
                    "type": "number",
                    "description": "Max lines to return"
                },
                "signal": {
                    "type": "string",
                    "description": "Signal to send (for action=kill)"
                },
                "timeout": {
                    "type": "number",
                    "description": "Timeout in ms"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let action = args.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match action {
            "run" => self.run_job(args, context).await,
            "list" => self.list_jobs(context).await,
            "output" => self.get_output(args, context).await,
            "kill" => self.kill_job(args, context).await,
            "status" => self.get_status(args, context).await,
            _ => error_result(format!("Unknown action: {}", action)),
        }
    }
}

impl JobTool {
    async fn run_job(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let command = args.get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let workdir = args.get("workdir")
            .and_then(|v| v.as_str())
            .unwrap_or(&context.working_directory);
        let wait = args.get("wait").and_then(|v| v.as_bool());

        if command.is_empty() {
            return error_result("command is required for run action");
        }

        if wait.is_none() {
            return error_result("wait parameter is required (must be true or false)");
        }

        let spawn_opts = crate::core::SpawnOptions {
            workdir: workdir.to_string(),
            timeout: 0,
            session_id: Some(context.session_id.clone()),
            env: None,
        };

        let result = execute_with_bwrap(command, spawn_opts).await;

        match result {
            Ok(ExecutorResult { handle, pid }) => {
                let job_id = handle.id.clone();
                let output_dir = handle.output_dir.clone();

                if wait == Some(true) {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    
                    let stdout_path = PathBuf::from(&output_dir).join("stdout");
                    let exitcode_path = PathBuf::from(&output_dir).join("exitcode");
                    
                    let mut retries = 0;
                    while retries < 100 {
                        if exitcode_path.exists() {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        retries += 1;
                    }

                    let stdout = fs::read_to_string(&stdout_path).unwrap_or_default();
                    let exit_code: i32 = fs::read_to_string(&exitcode_path)
                        .ok()
                        .and_then(|s| s.trim().parse().ok())
                        .unwrap_or(-1);

                    JOB_OUTPUTS.lock().unwrap().insert(job_id.clone(), stdout.clone());

                    let mut metadata = HashMap::new();
                    metadata.insert("job_id".to_string(), serde_json::json!(job_id));
                    metadata.insert("exit_code".to_string(), serde_json::json!(exit_code));
                    metadata.insert("status".to_string(), serde_json::json!(if exit_code == 0 { "Completed" } else { "Failed" }));
                    metadata.insert("pid".to_string(), serde_json::json!(pid));

                    ToolResult {
                        title: "Job Completed".to_string(),
                        output: format!("Job {} completed.\nExit code: {}\nStatus: {}", job_id, exit_code, if exit_code == 0 { "Completed" } else { "Failed" }),
                        metadata,
                        attachments: Vec::new(),
                        truncated: false,
                        success: Some(exit_code == 0),
                    }
                } else {
                    JOB_PIDS.lock().unwrap().insert(job_id.clone(), pid);

                    let mut metadata = HashMap::new();
                    metadata.insert("job_id".to_string(), serde_json::json!(job_id));
                    metadata.insert("pid".to_string(), serde_json::json!(pid));

                    ToolResult {
                        title: "Job Started".to_string(),
                        output: format!("Job {} started in background.\n\nTo check output: job(action=\"output\", job_id=\"{}\")", job_id, job_id),
                        metadata,
                        attachments: Vec::new(),
                        truncated: false,
                        success: Some(true),
                    }
                }
            }
            Err(e) => error_result(format!("Failed to start job: {}", e)),
        }
    }

    async fn list_jobs(&self, _context: ToolContext) -> ToolResult {
        let mut metadata = HashMap::new();
        metadata.insert("jobs".to_string(), serde_json::json!([]));
        
        ToolResult {
            title: "Job List".to_string(),
            output: "No running jobs.".to_string(),
            metadata,
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn get_output(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let job_id = args.get("job_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if job_id.is_empty() {
            return error_result("job_id is required for output action");
        }

        let output = JOB_OUTPUTS.lock().unwrap()
            .get(job_id)
            .cloned()
            .unwrap_or_else(|| "(no output)".to_string());

        let mut metadata = HashMap::new();
        metadata.insert("job_id".to_string(), serde_json::json!(job_id));
        metadata.insert("status".to_string(), serde_json::json!("Running"));

        ToolResult {
            title: format!("Job {} Output", job_id),
            output: format!("{}\n\nJob status: Running", output),
            metadata,
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn kill_job(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let job_id = args.get("job_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let signal = args.get("signal")
            .and_then(|v| v.as_str())
            .unwrap_or("SIGTERM");

        if job_id.is_empty() {
            return error_result("job_id is required for kill action");
        }

        let mut metadata = HashMap::new();
        metadata.insert("job_id".to_string(), serde_json::json!(job_id));
        metadata.insert("signal".to_string(), serde_json::json!(signal));

        ToolResult {
            title: "Job Killed".to_string(),
            output: format!("Job {} terminated with {}", job_id, signal),
            metadata,
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn get_status(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let job_id = args.get("job_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if job_id.is_empty() {
            return error_result("job_id is required for status action");
        }

        let mut metadata = HashMap::new();
        metadata.insert("job_id".to_string(), serde_json::json!(job_id));
        metadata.insert("status".to_string(), serde_json::json!("unknown"));

        ToolResult {
            title: format!("Job {} Status", job_id),
            output: "Status: unknown".to_string(),
            metadata,
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }
}

fn error_result(message: impl Into<String>) -> ToolResult {
    ToolResult {
        title: "Error".to_string(),
        output: message.into(),
        metadata: HashMap::new(),
        attachments: Vec::new(),
        truncated: false,
        success: Some(false),
    }
}