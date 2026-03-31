use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub struct MultiEditTool;

#[derive(Clone)]
struct EditOperation {
    old_string: String,
    new_string: String,
    replace_all: bool,
}

#[async_trait]
impl Tool for MultiEditTool {
    fn name(&self) -> &str {
        "multiedit"
    }

    fn description(&self) -> &str {
        "Perform multiple edit operations on a single file in sequence. Each edit is applied sequentially, and if an edit fails, it continues with the remaining edits. Only the first edit can have an empty oldString (for file creation)."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The absolute path to the file to modify"
                },
                "edits": {
                    "type": "array",
                    "description": "Array of edit operations to perform sequentially on the file",
                    "items": {
                        "type": "object",
                        "properties": {
                            "old_string": { "type": "string", "description": "The text to replace" },
                            "new_string": { "type": "string", "description": "The text to replace it with (must be different from oldString)" },
                            "replace_all": { "type": "boolean", "description": "Replace all occurrences of oldString (default false)" }
                        },
                        "required": ["old_string", "new_string"]
                    }
                }
            },
            "required": ["file_path", "edits"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let edits_raw = args.get("edits")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if file_path.is_empty() {
            return error_result("filePath is required");
        }

        let edits: Vec<EditOperation> = edits_raw.iter()
            .filter_map(|e| {
                let old_string = e.get("old_string").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let new_string = e.get("new_string").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let replace_all = e.get("replace_all").and_then(|v| v.as_bool()).unwrap_or(false);
                Some(EditOperation { old_string, new_string, replace_all })
            })
            .collect();

        if edits.is_empty() {
            return error_result("edits must be a non-empty array");
        }

        for (i, edit) in edits.iter().enumerate() {
            if i > 0 && edit.old_string.is_empty() {
                return error_result("Only the first edit can have empty oldString (for file creation).");
            }
        }

        let resolved_path = if PathBuf::from(file_path).is_absolute() {
            PathBuf::from(file_path)
        } else {
            PathBuf::from(&context.working_directory).join(file_path)
        };

        let is_first_edit_creation = !edits.is_empty() && edits[0].old_string.is_empty();
        let mut current_content: String;
        let mut is_new_file = false;
        let old_content: String;

        if is_first_edit_creation {
            if resolved_path.exists() {
                return error_result_with_path("File already exists", resolved_path.to_string_lossy().to_string());
            }
            is_new_file = true;
            current_content = edits[0].new_string.clone();
            old_content = String::new();
        } else {
            current_content = match fs::read_to_string(&resolved_path) {
                Ok(c) => c,
                Err(e) => return error_result_with_path(format!("Failed to read file: {}", e), resolved_path.to_string_lossy().to_string()),
            };
            old_content = current_content.clone();
        }

        let mut failed_edits: Vec<FailedEdit> = Vec::new();
        let start_index = if is_new_file { 1 } else { 0 };

        for i in start_index..edits.len() {
            let edit = &edits[i];
            match apply_edit(&current_content, edit) {
                Ok(new_content) => current_content = new_content,
                Err(e) => {
                    failed_edits.push(FailedEdit {
                        index: i + 1,
                        error: e,
                    });
                }
            }
        }

        let edits_applied = edits.len() - failed_edits.len();

        if current_content == old_content && failed_edits.is_empty() {
            return ToolResult {
                title: "No changes".to_string(),
                output: "No changes made - all edits resulted in identical content".to_string(),
                metadata: HashMap::new(),
                attachments: Vec::new(),
                truncated: false,
                success: Some(true),
            };
        }

        if is_new_file {
            if let Some(parent) = resolved_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    return error_result_with_path(format!("Failed to create directory: {}", e), resolved_path.to_string_lossy().to_string());
                }
            }
        }

        if let Err(e) = fs::write(&resolved_path, &current_content) {
            return error_result_with_path(format!("Failed to write file: {}", e), resolved_path.to_string_lossy().to_string());
        }

        let diff = create_diff(&resolved_path.to_string_lossy(), &old_content, &current_content);

        let output = if !failed_edits.is_empty() {
            let mut out = diff.clone();
            out.push_str("\n\n---\n\nFailed edits:\n");
            for fe in &failed_edits {
                out.push_str(&format!("  Edit {}: {}\n", fe.index, fe.error));
            }
            out
        } else {
            diff
        };

        let title = if is_new_file {
            format!("Created: {}", resolved_path.display())
        } else {
            format!("Edited: {}", resolved_path.display())
        };

        let mut metadata = HashMap::new();
        metadata.insert("path".to_string(), serde_json::json!(resolved_path.to_string_lossy().to_string()));
        metadata.insert("editsApplied".to_string(), serde_json::json!(edits_applied));
        metadata.insert("editsFailed".to_string(), serde_json::json!(failed_edits.len()));
        metadata.insert("success".to_string(), serde_json::json!(true));

        ToolResult {
            title,
            output,
            metadata,
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }
}

struct FailedEdit {
    index: usize,
    error: String,
}

fn apply_edit(content: &str, edit: &EditOperation) -> Result<String, String> {
    if edit.old_string.is_empty() {
        return Err("oldString cannot be empty for content replacement".to_string());
    }

    if !content.contains(&edit.old_string) {
        return Err("oldString not found in content. Make sure it matches exactly, including whitespace and line breaks".to_string());
    }

    if edit.replace_all {
        return Ok(content.replace(&edit.old_string, &edit.new_string));
    }

    let first_index = content.find(&edit.old_string);
    let last_index = content.rfind(&edit.old_string);

    if first_index != last_index {
        return Err("oldString appears multiple times. Provide more context for a unique match, or set replaceAll to true".to_string());
    }

    Ok(content.replacen(&edit.old_string, &edit.new_string, 1))
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