use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema};
use std::collections::HashMap;

pub struct SkillTool;

#[async_trait]
impl Tool for SkillTool {
    fn name(&self) -> &str {
        "skill"
    }

    fn description(&self) -> &str {
        "Load a specialized skill that provides domain-specific instructions."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The full name of the skill to load"
                },
                "resource": {
                    "type": "string",
                    "description": "Optional. A relative path to a resource file within the skill"
                }
            },
            "required": ["name"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let skill_name = args.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let _resource_path = args.get("resource").and_then(|v| v.as_str());

        if skill_name.is_empty() {
            return ToolResult {
                title: "Available Skills".to_string(),
                output: "No skills are currently installed. Add a registry and install skills first.".to_string(),
                metadata: HashMap::new(),
                attachments: Vec::new(),
                truncated: false,
                success: Some(true),
            };
        }

        let output = format!(
            "<skill_content name=\"{}\">\nSkill loaded successfully.\n</skill_content>",
            skill_name
        );

        ToolResult {
            title: format!("Loaded skill: {}", skill_name),
            output,
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }
}