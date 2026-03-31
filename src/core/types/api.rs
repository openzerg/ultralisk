use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<super::session::Session>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageListResponse {
    pub messages: Vec<super::message::Message>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessListResponse {
    pub processes: Vec<super::process::Process>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResponse {
    pub tools: Vec<super::tool::ToolDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptRequest {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptResponse {
    pub interrupted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub session_id: String,
    pub content: String,
    pub status: TodoStatus,
    pub priority: TodoPriority,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoListResponse {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub session_id: Option<String>,
    pub activity_type: String,
    pub description: String,
    pub details: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityListResponse {
    pub activities: Vec<Activity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadInfo {
    pub file_path: String,
    pub mtime_ms: i64,
    pub read_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session: super::session::Session,
    pub messages: Vec<super::message::Message>,
    pub todos: Vec<Todo>,
    pub children: Vec<super::session::Session>,
    pub files: Vec<FileReadInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    pub max_tokens_per_request: i32,
    pub max_requests_per_minute: i32,
    pub max_concurrent_sessions: i32,
    pub default_timeout_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRequest {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemindRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}
