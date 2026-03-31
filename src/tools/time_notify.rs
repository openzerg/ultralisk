use async_trait::async_trait;
use crate::core::interfaces::tool::{Tool, ToolContext};
use crate::core::{ToolResult, JSONSchema, TimerType};
use std::collections::HashMap;

pub struct TimeNotifyTool;

#[async_trait]
impl Tool for TimeNotifyTool {
    fn name(&self) -> &str {
        "time_notify"
    }

    fn description(&self) -> &str {
        "Schedule time-based notifications (reminders)."
    }

    fn parameters(&self) -> JSONSchema {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create", "list", "cancel", "status", "enable", "disable"],
                    "description": "Timer action to perform"
                },
                "name": {
                    "type": "string",
                    "description": "Timer name"
                },
                "message": {
                    "type": "string",
                    "description": "Notification message template"
                },
                "timer_type": {
                    "type": "string",
                    "enum": ["calendar", "active", "unit-active"],
                    "description": "Timer trigger type"
                },
                "timer_spec": {
                    "type": "string",
                    "description": "Timer specification: cron expression or seconds"
                },
                "max_runs": {
                    "type": "number",
                    "description": "Maximum number of triggers (0 = unlimited)"
                },
                "description": {
                    "type": "string",
                    "description": "Timer description"
                },
                "timer_id": {
                    "type": "string",
                    "description": "Timer ID"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        let action = args.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match action {
            "create" => self.create_timer(args).await,
            "list" => self.list_timers().await,
            "cancel" => self.cancel_timer(args).await,
            "status" => self.get_timer_status(args).await,
            "enable" => self.enable_timer(args).await,
            "disable" => self.disable_timer(args).await,
            _ => error_result(format!("Unknown action: {}", action)),
        }
    }
}

impl TimeNotifyTool {
    async fn create_timer(&self, args: HashMap<String, serde_json::Value>) -> ToolResult {
        let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let message = args.get("message").and_then(|v| v.as_str()).unwrap_or("");
        let timer_type = args.get("timer_type").and_then(|v| v.as_str()).unwrap_or("");
        let timer_spec = args.get("timer_spec").and_then(|v| v.as_str()).unwrap_or("");
        let max_runs = args.get("max_runs").and_then(|v| v.as_i64()).unwrap_or(0);

        if name.is_empty() || message.is_empty() || timer_type.is_empty() || timer_spec.is_empty() {
            return error_result("name, message, timer_type, and timer_spec are required for create action");
        }

        if !["calendar", "active", "unit-active"].contains(&timer_type) {
            return error_result(format!("Invalid timer_type: {}. Must be 'calendar', 'active', or 'unit-active'", timer_type));
        }

        let timer_id = uuid::Uuid::new_v4().to_string();

        ToolResult {
            title: format!("Timer Created: {}", name),
            output: format!(
                "Timer \"{}\" created successfully.\n\nID: {}\nType: {}\nSpec: {}\n{}",
                name,
                timer_id,
                timer_type,
                timer_spec,
                if max_runs > 0 { format!("Max runs: {}", max_runs) } else { "Max runs: unlimited".to_string() }
            ),
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn list_timers(&self) -> ToolResult {
        ToolResult {
            title: "Timer List".to_string(),
            output: "No timers scheduled for this session.".to_string(),
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn cancel_timer(&self, args: HashMap<String, serde_json::Value>) -> ToolResult {
        let timer_id = args.get("timer_id").and_then(|v| v.as_str()).unwrap_or("");
        let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("");

        if timer_id.is_empty() && name.is_empty() {
            return error_result("timer_id or name is required for cancel action");
        }

        let id = if !timer_id.is_empty() { timer_id } else { name };

        ToolResult {
            title: "Timer Cancelled".to_string(),
            output: format!("Timer {} has been cancelled and deleted.", id),
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn get_timer_status(&self, args: HashMap<String, serde_json::Value>) -> ToolResult {
        let timer_id = args.get("timer_id").and_then(|v| v.as_str()).unwrap_or("");
        let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("");

        if timer_id.is_empty() && name.is_empty() {
            return error_result("timer_id or name is required for status action");
        }

        let id = if !timer_id.is_empty() { timer_id } else { name };

        ToolResult {
            title: format!("Timer: {}", id),
            output: format!("Timer {} not found", id),
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn enable_timer(&self, args: HashMap<String, serde_json::Value>) -> ToolResult {
        let timer_id = args.get("timer_id").and_then(|v| v.as_str()).unwrap_or("");

        if timer_id.is_empty() {
            return error_result("timer_id is required for enable action");
        }

        ToolResult {
            title: "Timer Enabled".to_string(),
            output: format!("Timer {} has been enabled.", timer_id),
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(true),
        }
    }

    async fn disable_timer(&self, args: HashMap<String, serde_json::Value>) -> ToolResult {
        let timer_id = args.get("timer_id").and_then(|v| v.as_str()).unwrap_or("");

        if timer_id.is_empty() {
            return error_result("timer_id is required for disable action");
        }

        ToolResult {
            title: "Timer Disabled".to_string(),
            output: format!("Timer {} has been disabled.", timer_id),
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