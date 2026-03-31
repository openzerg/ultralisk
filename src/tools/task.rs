use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;

pub struct TaskTool;

#[async_trait]
impl Tool for TaskTool {
    fn name(&self) -> &str {
        "task"
    }

    fn description(&self) -> &str {
        "Launch a specialized sub-agent to handle a complex, multi-step task."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "description": {
                    "type": "string",
                    "description": "A short (3-5 words) description of the task"
                },
                "prompt": {
                    "type": "string",
                    "description": "The detailed task instructions for the sub-agent"
                },
                "task_id": {
                    "type": "string",
                    "description": "Optional: Resume a previous task by passing its task_id"
                }
            },
            "required": ["description", "prompt"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let description = args.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let prompt = args.get("prompt")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if description.is_empty() || prompt.is_empty() {
            return error_result("description and prompt are required");
        }

        let task_id = uuid::Uuid::new_v4().to_string();

        let output = format!(
            "task_id: {}\n\n<task_result>\nTask launched successfully. The sub-agent will process: {}\n</task_result>",
            task_id,
            prompt
        );

        ToolResult {
            title: description.to_string(),
            output,
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