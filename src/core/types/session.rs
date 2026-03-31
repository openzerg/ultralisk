use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SessionState {
    Idle,
    Running,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Plan,
    Build,
}

impl Default for AgentMode {
    fn default() -> Self {
        Self::Build
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingEventData {
    pub job_id: Option<String>,
    pub exit_code: Option<i32>,
    pub timer_id: Option<String>,
    pub timer_name: Option<String>,
    pub message: Option<String>,
    pub run_count: Option<i32>,
    pub next_run: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PendingEventType {
    JobCompleted,
    JobFailed,
    JobTimeout,
    JobKilled,
    TimerNotify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: PendingEventType,
    pub timestamp: String,
    pub session_id: String,
    pub data: PendingEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub state: SessionState,
    pub agent: AgentMode,
    pub provider_name: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub system_prompt: String,
    pub parent_id: Option<String>,
    pub child_ids: Vec<String>,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub has_compacted_history: bool,
    pub compacted_message_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionData {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub agent: Option<AgentMode>,
    #[serde(default)]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSessionData {
    pub name: Option<String>,
    pub state: Option<SessionState>,
    pub agent: Option<AgentMode>,
    pub provider_name: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub has_compacted_history: Option<bool>,
    pub compacted_message_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionFilter {
    pub state: Option<SessionState>,
    pub purpose: Option<String>,
    pub parent_id: Option<String>,
}
