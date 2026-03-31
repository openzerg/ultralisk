use crate::core::interfaces::tool::ToolContext;
use crate::core::types::message::CreateMessageData;
use crate::core::types::tool::ToolResult;
use serde_json::Value;
use std::collections::HashMap;

pub struct ToolCallData {
    pub id: String,
    pub function: ToolCallFunction,
}

pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

pub struct PendingToolCall {
    pub session_id: String,
    pub tool_name: String,
}

pub fn parse_tool_args(arguments: &str) -> HashMap<String, Value> {
    if arguments.is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(arguments).unwrap_or_default()
    }
}

pub fn create_tool_message(
    session_id: &str,
    tool_name: &str,
    tool_call_id: &str,
    result: &ToolResult,
    args_json: &str,
) -> CreateMessageData {
    let tool_success = result.success.unwrap_or(true);

    CreateMessageData {
        id: uuid::Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        role: crate::core::types::message::MessageRole::Tool,
        content: result.output.clone(),
        tool_calls: None,
        tool_calls_json: Some(args_json.to_string()),
        tool_name: Some(tool_name.to_string()),
        tool_call_id: Some(tool_call_id.to_string()),
        tool_success: Some(tool_success),
        metadata: None,
    }
}

pub fn build_tool_context(
    session_id: &str,
    session_name: &str,
    working_directory: &str,
) -> ToolContext {
    ToolContext {
        session_id: session_id.to_string(),
        session_name: session_name.to_string(),
        working_directory: working_directory.to_string(),
        process_manager: None,
        event_bus: None,
        storage: None,
        llm_client: None,
    }
}
