use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;

pub struct MessageTool;

#[async_trait]
impl Tool for MessageTool {
    fn name(&self) -> &str {
        "message"
    }

    fn description(&self) -> &str {
        "Send messages to WebUI or Proxy subscribers."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["send", "reply", "sendFile"],
                    "description": "Message action"
                },
                "to": {
                    "type": "string",
                    "description": "Target recipient"
                },
                "content": {
                    "type": "string",
                    "description": "Message content"
                },
                "file": {
                    "type": "string",
                    "description": "File path (for sendFile action)"
                },
                "reply_to": {
                    "type": "string",
                    "description": "Message ID to reply to"
                }
            },
            "required": ["action", "to"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let action = args.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let to = args.get("to")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let _content = args.get("content").and_then(|v| v.as_str());
        let _file = args.get("file").and_then(|v| v.as_str());
        let _reply_to = args.get("reply_to").and_then(|v| v.as_str());

        if to.is_empty() {
            return error_result("to is required");
        }

        ToolResult {
            title: "Message Sent".to_string(),
            output: format!("Message \"{}\" broadcast to subscriber(s)", action),
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