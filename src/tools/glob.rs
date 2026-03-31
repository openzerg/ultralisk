use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

pub struct GlobTool;

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Fast file pattern matching tool. Supports glob patterns like \"**/*.js\" or \"src/**/*.ts\". Returns matching file paths sorted by modification time."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The glob pattern to match files against (e.g., \"**/*.js\", \"src/**/*.ts\")"
                },
                "path": {
                    "type": "string",
                    "description": "The directory to search in (defaults to current working directory)"
                }
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let pattern = args.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let search_path = args.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(&context.working_directory);

        if pattern.is_empty() {
            return error_result("pattern is required");
        }

        let resolved_path = if PathBuf::from(search_path).is_absolute() {
            PathBuf::from(search_path)
        } else {
            PathBuf::from(&context.working_directory).join(search_path)
        };

        let files = match ripgrep_files(&resolved_path, pattern) {
            Ok(f) => f,
            Err(e) => return error_result_with_meta(format!("Glob search failed: {}", e), pattern.to_string(), resolved_path.to_string_lossy().to_string()),
        };

        let mut files_with_mtime: Vec<(String, i64)> = Vec::new();
        for file in &files {
            let full_path = resolved_path.join(file);
            let mtime = fs::metadata(&full_path)
                .map(|m| m.modified().map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64).unwrap_or(0))
                .unwrap_or(0);
            files_with_mtime.push((file.clone(), mtime));
        }

        files_with_mtime.sort_by(|a, b| b.1.cmp(&a.1));

        let limit = 100;
        let truncated = files_with_mtime.len() > limit;
        let result_files: Vec<String> = files_with_mtime.iter().take(limit).map(|(f, _)| f.clone()).collect();

        let output = if result_files.is_empty() {
            "(no matches)".to_string()
        } else {
            result_files.join("\n")
        };

        let final_output = if truncated {
            format!("{}\n\n(Results truncated: showing {} of {} matches. Use a more specific pattern.)", output, limit, files.len())
        } else {
            output
        };

        let mut metadata = HashMap::new();
        metadata.insert("pattern".to_string(), serde_json::json!(pattern));
        metadata.insert("path".to_string(), serde_json::json!(resolved_path.to_string_lossy().to_string()));
        metadata.insert("count".to_string(), serde_json::json!(files.len()));
        metadata.insert("truncated".to_string(), serde_json::json!(truncated));

        ToolResult {
            title: format!("Glob: {}", pattern),
            output: final_output,
            metadata,
            attachments: Vec::new(),
            truncated,
            success: Some(true),
        }
    }
}

fn ripgrep_files(cwd: &PathBuf, pattern: &str) -> Result<Vec<String>, String> {
    let output = Command::new("rg")
        .args(["--files", "--hidden", "--glob=!.git/*", &format!("--glob={}", pattern)])
        .current_dir(cwd)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() && output.status.code() != Some(1) {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect();

    Ok(lines)
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

fn error_result_with_meta(message: impl Into<String>, pattern: String, path: String) -> ToolResult {
    let mut metadata = HashMap::new();
    metadata.insert("error".to_string(), serde_json::json!(true));
    metadata.insert("pattern".to_string(), serde_json::json!(pattern));
    metadata.insert("path".to_string(), serde_json::json!(path));
    ToolResult {
        title: "Error".to_string(),
        output: message.into(),
        metadata,
        attachments: Vec::new(),
        truncated: false,
        success: Some(false),
    }
}