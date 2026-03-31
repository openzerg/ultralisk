use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type JSONSchema = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: JSONSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AttachmentType {
    File,
    Image,
    Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub attachment_type: AttachmentType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub title: String,
    pub output: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub attachments: Vec<Attachment>,
    pub truncated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecuteRequest {
    pub tool_name: String,
    pub args: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum HttpToolConfigMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpToolConfig {
    #[serde(rename = "type")]
    pub config_type: String,
    pub url: String,
    pub method: HttpToolConfigMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandToolConfig {
    #[serde(rename = "type")]
    pub config_type: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExternalToolConfig {
    Http(HttpToolConfig),
    Command(CommandToolConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalToolData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameters_json: String,
    pub config_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolVariableData {
    pub id: String,
    pub tool_name: String,
    pub variable_name: String,
    pub variable_value: Option<String>,
    pub description: Option<String>,
    pub required: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolVariableDefinition {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalToolDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parameters: JSONSchema,
    pub config: ExternalToolConfig,
    pub variables: Vec<ToolVariableDefinition>,
    pub created_at: String,
    pub updated_at: String,
}
