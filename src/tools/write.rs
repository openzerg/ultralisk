use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub struct WriteTool;

#[async_trait]
impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write"
    }

    fn description(&self) -> &str {
        "Write a file to the local filesystem. This tool will overwrite the existing file if there is one at the provided path. NOTE: You must read the file first before overwriting an existing file."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file to write (must be absolute, not relative)"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["file_path", "content"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let content = args.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if file_path.is_empty() {
            return error_result("filePath is required");
        }

        let resolved_path = if PathBuf::from(file_path).is_absolute() {
            PathBuf::from(file_path)
        } else {
            PathBuf::from(&context.working_directory).join(file_path)
        };

        let exists = resolved_path.exists();
        let mut old_content = String::new();

        if exists {
            old_content = match fs::read_to_string(&resolved_path) {
                Ok(c) => c,
                Err(e) => return error_result_with_path(format!("Failed to read existing file: {}", e), resolved_path.to_string_lossy().to_string()),
            };
        }

        if let Some(parent) = resolved_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return error_result_with_path(format!("Failed to create directory: {}", e), resolved_path.to_string_lossy().to_string());
            }
        }

        if let Err(e) = fs::write(&resolved_path, content) {
            return error_result_with_path(format!("Failed to write file: {}", e), resolved_path.to_string_lossy().to_string());
        }

        let lines = content.lines().count();
        let diff = create_diff(&resolved_path.to_string_lossy(), &old_content, content);

        let mut metadata = HashMap::new();
        metadata.insert("path".to_string(), serde_json::json!(resolved_path.to_string_lossy().to_string()));
        metadata.insert("lines".to_string(), serde_json::json!(lines));
        metadata.insert("bytes".to_string(), serde_json::json!(content.len()));
        metadata.insert("exists".to_string(), serde_json::json!(exists));
        metadata.insert("success".to_string(), serde_json::json!(true));

        ToolResult {
            title: format!("Wrote: {}", resolved_path.display()),
            output: diff,
            metadata,
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }
}

fn create_diff(filename: &str, old_content: &str, new_content: &str) -> String {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let mut diff_lines: Vec<String> = Vec::new();
    diff_lines.push(format!("--- {}", filename));
    diff_lines.push(format!("+++ {}", filename));

    let max_len = old_lines.len().max(new_lines.len());

    for i in 0..max_len {
        let old_line = old_lines.get(i).map(|s| s.to_string()).unwrap_or_default();
        let new_line = new_lines.get(i).map(|s| s.to_string()).unwrap_or_default();

        if i >= old_lines.len() {
            diff_lines.push(format!("+{}", new_line));
        } else if i >= new_lines.len() {
            diff_lines.push(format!("-{}", old_line));
        } else if old_line != new_line {
            diff_lines.push(format!("-{}", old_line));
            diff_lines.push(format!("+{}", new_line));
        } else {
            diff_lines.push(format!(" {}", old_line));
        }
    }

    diff_lines.join("\n")
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

fn error_result_with_path(message: impl Into<String>, path: String) -> ToolResult {
    let mut metadata = HashMap::new();
    metadata.insert("error".to_string(), serde_json::json!(true));
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