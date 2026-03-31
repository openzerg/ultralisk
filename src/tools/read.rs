use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

const MAX_LINE_LENGTH: usize = 2000;
const MAX_LINES: usize = 2000;
const MAX_BYTES: usize = 50 * 1024;

pub struct ReadTool;

#[async_trait]
impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read"
    }

    fn description(&self) -> &str {
        "Read a file or directory from the local filesystem. If the path does not exist, an error is returned."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file or directory to read"
                },
                "offset": {
                    "type": "number",
                    "description": "The line number to start reading from (1-indexed)"
                },
                "limit": {
                    "type": "number",
                    "description": "The maximum number of lines to read (defaults to 2000)"
                }
            },
            "required": ["file_path"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let offset = args.get("offset")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;
        let limit = args.get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(MAX_LINES as u64) as usize;

        if file_path.is_empty() {
            return error_result("filePath is required");
        }

        let resolved_path = if PathBuf::from(file_path).is_absolute() {
            PathBuf::from(file_path)
        } else {
            PathBuf::from(&context.working_directory).join(file_path)
        };

        let metadata = match fs::metadata(&resolved_path) {
            Ok(m) => m,
            Err(e) => return error_result_with_path(format!("Failed to read {}: {}", resolved_path.display(), e), resolved_path.to_string_lossy().to_string()),
        };

        if metadata.is_dir() {
            return read_directory(&resolved_path, limit);
        }

        read_file(&resolved_path, offset, limit)
    }
}

fn read_directory(path: &PathBuf, limit: usize) -> ToolResult {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => return error_result_with_path(format!("Failed to read directory: {}", e), path.to_string_lossy().to_string()),
    };

    let mut lines: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let suffix = if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            "/"
        } else {
            ""
        };
        lines.push(format!("{}{}", name, suffix));
    }

    lines.sort();

    let truncated = lines.len() > limit;
    let mut shown_lines: Vec<String> = lines.into_iter().take(limit).collect();

    if truncated {
        shown_lines.push(format!("... ({} more entries truncated)", shown_lines.len().saturating_sub(limit)));
    }

    let mut result_metadata = HashMap::new();
    result_metadata.insert("type".to_string(), serde_json::json!("directory"));
    result_metadata.insert("path".to_string(), serde_json::json!(path.to_string_lossy().to_string()));
    result_metadata.insert("count".to_string(), serde_json::json!(shown_lines.len()));
    result_metadata.insert("truncated".to_string(), serde_json::json!(truncated));

    ToolResult {
        title: format!("Directory: {}", path.display()),
        output: shown_lines.join("\n"),
        metadata: result_metadata,
        attachments: Vec::new(),
        truncated,
        success: Some(true),
    }
}

fn read_file(path: &PathBuf, offset: usize, limit: usize) -> ToolResult {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return error_result_with_path(format!("Failed to read {}: {}", path.display(), e), path.to_string_lossy().to_string()),
    };

    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    let start_line = offset.saturating_sub(1);
    let end_line = (start_line + limit).min(total_lines);
    let selected_lines: Vec<&str> = lines.iter().skip(start_line).take(limit).cloned().collect();

    let truncated_lines: Vec<String> = selected_lines.iter()
        .map(|line| {
            if line.len() > MAX_LINE_LENGTH {
                format!("{}... (line truncated)", &line[..MAX_LINE_LENGTH])
            } else {
                line.to_string()
            }
        })
        .collect();

    let numbered_lines: Vec<String> = truncated_lines.iter()
        .enumerate()
        .map(|(idx, line)| format!("{}: {}", start_line + idx + 1, line))
        .collect();

    let has_more_lines = end_line < total_lines;
    let mut output = numbered_lines.join("\n");
    let mut truncated = has_more_lines;

    if output.len() > MAX_BYTES || has_more_lines {
        let preview_lines: Vec<String> = numbered_lines.iter().take(100).cloned().collect();
        let preview_bytes = preview_lines.join("\n").len();

        if preview_bytes > MAX_BYTES {
            let mut reduced: Vec<String> = Vec::new();
            let mut bytes = 0;
            for ln in &numbered_lines {
                let line_bytes = ln.len() + 1;
                if bytes + line_bytes > MAX_BYTES {
                    break;
                }
                reduced.push(ln.clone());
                bytes += line_bytes;
            }
            output = reduced.join("\n");
        } else {
            output = preview_lines.join("\n");
        }
        output.push_str("\n\n... output truncated. Full content available. Use Read with offset/limit to view specific sections.");
        truncated = true;
    }

    let mut result_metadata = HashMap::new();
    result_metadata.insert("type".to_string(), serde_json::json!("file"));
    result_metadata.insert("path".to_string(), serde_json::json!(path.to_string_lossy().to_string()));
    result_metadata.insert("totalLines".to_string(), serde_json::json!(total_lines));
    result_metadata.insert("shownLines".to_string(), serde_json::json!(selected_lines.len()));
    result_metadata.insert("offset".to_string(), serde_json::json!(start_line + 1));
    result_metadata.insert("truncated".to_string(), serde_json::json!(truncated));

    ToolResult {
        title: format!("File: {}", path.display()),
        output,
        metadata: result_metadata,
        attachments: Vec::new(),
        truncated,
        success: Some(true),
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