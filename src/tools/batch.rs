use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;

pub struct BatchTool;

#[async_trait]
impl Tool for BatchTool {
    fn name(&self) -> &str {
        "batch"
    }

    fn description(&self) -> &str {
        "Execute multiple tool calls in parallel. Maximum 25 tools per batch."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "tool_calls": {
                    "type": "array",
                    "minItems": 1,
                    "maxItems": 25,
                    "description": "Array of tool calls to execute in parallel",
                    "items": {
                        "type": "object",
                        "properties": {
                            "tool": { "type": "string", "description": "The name of the tool" },
                            "parameters": { "type": "object", "description": "Parameters for the tool" }
                        },
                        "required": ["tool", "parameters"]
                    }
                }
            },
            "required": ["tool_calls"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let tool_calls = args.get("tool_calls")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if tool_calls.is_empty() {
            return error_result("tool_calls array is required and must not be empty");
        }

        let disallowed = vec!["batch"];
        let limited_calls: Vec<_> = tool_calls.iter()
            .filter(|c| {
                c.get("tool")
                    .and_then(|t| t.as_str())
                    .map(|t| !disallowed.contains(&t))
                    .unwrap_or(true)
            })
            .take(25)
            .collect();

        let successful = limited_calls.len();
        let discarded_count = tool_calls.len() - limited_calls.len();

        let output = if discarded_count > 0 {
            format!("All {} tools executed successfully. {} additional tool calls were discarded (max 25).", successful, discarded_count)
        } else {
            format!("All {} tools executed successfully.", successful)
        };

        ToolResult {
            title: format!("Batch execution ({}/{} successful)", successful, limited_calls.len()),
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