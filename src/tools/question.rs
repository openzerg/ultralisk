use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;

pub struct QuestionTool;

#[async_trait]
impl Tool for QuestionTool {
    fn name(&self) -> &str {
        "question"
    }

    fn description(&self) -> &str {
        "Ask the user questions during execution to gather preferences or clarify instructions."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "questions": {
                    "type": "array",
                    "description": "Questions to ask the user",
                    "items": {
                        "type": "object",
                        "properties": {
                            "question": { "type": "string", "description": "The question to ask" },
                            "header": { "type": "string", "description": "Short label for the question" },
                            "options": {
                                "type": "array",
                                "description": "Available choices",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "label": { "type": "string", "description": "Display text" },
                                        "description": { "type": "string", "description": "Explanation of choice" }
                                    },
                                    "required": ["label"]
                                }
                            },
                            "multiple": { "type": "boolean", "description": "Allow multiple choices" }
                        },
                        "required": ["question"]
                    }
                }
            },
            "required": ["questions"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let questions = args.get("questions")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

        if questions.is_empty() {
            return error_result("questions array is required and must not be empty");
        }

        let formatted: Vec<String> = questions.iter()
            .filter_map(|q| q.get("question").and_then(|q| q.as_str()))
            .map(|q| format!("\"{}\"=\"Unanswered\"", q))
            .collect();

        ToolResult {
            title: format!("Asked {} question{}", questions.len(), if questions.len() > 1 { "s" } else { "" }),
            output: format!("User has answered your questions: {}. You can now continue.", formatted.join(", ")),
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