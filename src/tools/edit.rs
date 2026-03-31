use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub struct EditTool;

#[async_trait]
impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit"
    }

    fn description(&self) -> &str {
        "Perform exact string replacements in files. The edit will FAIL if oldString is not found in the file or if it is found multiple times and replaceAll is false."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file to modify"
                },
                "old_string": {
                    "type": "string",
                    "description": "The text to replace"
                },
                "new_string": {
                    "type": "string",
                    "description": "The text to replace it with"
                },
                "replace_all": {
                    "type": "boolean",
                    "description": "Replace all occurrences of oldString (default false)"
                }
            },
            "required": ["file_path", "old_string", "new_string"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let old_string = args.get("old_string")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let new_string = args.get("new_string")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let replace_all = args.get("replace_all")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if file_path.is_empty() {
            return error_result("filePath is required");
        }

        let resolved_path = if PathBuf::from(file_path).is_absolute() {
            PathBuf::from(file_path)
        } else {
            PathBuf::from(&context.working_directory).join(file_path)
        };

        if old_string.is_empty() {
            return error_result_with_path("oldString is required", resolved_path.to_string_lossy().to_string());
        }

        let content = match fs::read_to_string(&resolved_path) {
            Ok(c) => c,
            Err(e) => return error_result_with_path(format!("Failed to read file: {}", e), resolved_path.to_string_lossy().to_string()),
        };

        if !content.contains(old_string) {
            return error_result_with_path(format!("oldString not found in {}", resolved_path.display()), resolved_path.to_string_lossy().to_string());
        }

        let occurrences = content.matches(old_string).count();

        if occurrences > 1 && !replace_all {
            return error_result_with_path(
                format!("Found {} matches for oldString. Use replaceAll: true to replace all occurrences.", occurrences),
                resolved_path.to_string_lossy().to_string()
            );
        }

        let new_content = if replace_all {
            content.replace(old_string, new_string)
        } else {
            content.replacen(old_string, new_string, 1)
        };

        if let Err(e) = fs::write(&resolved_path, &new_content) {
            return error_result_with_path(format!("Failed to write file: {}", e), resolved_path.to_string_lossy().to_string());
        }

        let replacements = if replace_all { occurrences } else { 1 };
        let diff = create_diff(&resolved_path.to_string_lossy(), &content, &new_content);

        ToolResult {
            title: format!("Edited: {}", resolved_path.display()),
            output: diff,
            metadata: HashMap::new(),
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