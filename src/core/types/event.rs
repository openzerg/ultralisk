use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedEvent {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingEvent {
    pub session_id: String,
    pub content: String,
    pub session_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseEvent {
    pub session_id: String,
    pub content: String,
    pub session_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallEvent {
    pub session_id: String,
    pub tool: String,
    pub args: String,
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultEvent {
    pub session_id: String,
    pub content: String,
    pub call_id: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoneEvent {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub session_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNotificationEventInner {
    #[serde(rename = "type")]
    pub event_type: String,
    pub status: Option<String>,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNotificationEvent {
    pub process_id: String,
    pub event: ProcessNotificationEventInner,
    pub output_preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionActivityType {
    Thinking,
    Response,
    ToolCall,
    ToolResult,
    Error,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionActivityEvent {
    pub session_id: String,
    pub session_name: String,
    pub activity_type: SessionActivityType,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptedEvent {
    pub session_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobCompletedEvent {
    pub session_id: String,
    pub job_id: String,
    pub exit_code: i32,
    pub timeout: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobKilledEvent {
    pub session_id: String,
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoUpdateEvent {
    pub session_id: String,
    pub todos: Vec<TodoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum GlobalEvent {
    Connected(ConnectedEvent),
    Thinking(ThinkingEvent),
    Response(ResponseEvent),
    ToolCall(ToolCallEvent),
    ToolResult(ToolResultEvent),
    Done(DoneEvent),
    Error(ErrorEvent),
    ProcessNotification(ProcessNotificationEvent),
    SessionActivity(SessionActivityEvent),
    Interrupted(InterruptedEvent),
    JobCompleted(JobCompletedEvent),
    JobKilled(JobKilledEvent),
    TodoUpdate(TodoUpdateEvent),
}
