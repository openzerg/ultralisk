use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

pub struct MemoryWriteTool;

#[async_trait]
impl Tool for MemoryWriteTool {
    fn name(&self) -> &str {
        "memory_write"
    }

    fn description(&self) -> &str {
        "Write content to memory folder for persistent cross-session memory."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "content": {
                    "type": "string",
                    "description": "Content to write (markdown format recommended)"
                },
                "filename": {
                    "type": "string",
                    "description": "Optional filename (default: auto-generate timestamp-based name)"
                },
                "append": {
                    "type": "boolean",
                    "description": "Append to existing file instead of creating new"
                }
            },
            "required": ["content"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult {
        let content = args.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let filename = args.get("filename").and_then(|v| v.as_str());
        let append = args.get("append").and_then(|v| v.as_bool()).unwrap_or(false);

        if content.is_empty() {
            return error_result("content is required");
        }

        let memory_dir = PathBuf::from(&context.working_directory).join("memory");

        if !memory_dir.exists() {
            if let Err(e) = fs::create_dir_all(&memory_dir) {
                return error_result(format!("Failed to create memory directory: {}", e));
            }
        }

        let target_filename = filename.map(|f| {
            if f.ends_with(".md") { f.to_string() } else { format!("{}.md", f) }
        }).unwrap_or_else(|| {
            let timestamp = chrono::Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();
            format!("{}.md", timestamp)
        });

        let file_path = memory_dir.join(&target_filename);

        if append && file_path.exists() {
            use std::io::Write;
            let mut file = match fs::OpenOptions::new().append(true).open(&file_path) {
                Ok(f) => f,
                Err(e) => return error_result(format!("Failed to append to memory: {}", e)),
            };
            if let Err(e) = writeln!(file, "\n\n{}", content) {
                return error_result(format!("Failed to append to memory: {}", e));
            }
        } else {
            if let Err(e) = fs::write(&file_path, content) {
                return error_result(format!("Failed to write memory: {}", e));
            }
        }

        let rel_path = format!("memory/{}", target_filename);

        ToolResult {
            title: "Memory Written".to_string(),
            output: format!("Content written to {}", rel_path),
            metadata: HashMap::new(),
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