use async_trait::async_trait;
use crate::core::types::{
    Session, CreateSessionData, UpdateSessionData, SessionFilter,
    Message, CreateMessageData, MessageFilter,
    Process, CreateProcessData, ProcessStatus, OutputStats,
    Todo, Provider, Timer, CreateTimerData, UpdateTimerData, TimerFilter,
    TodoStatus,
};
use crate::core::types::tool::{ExternalToolData, ToolVariableData};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRegistry {
    pub id: String,
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub registry_id: String,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub folder_path: String,
    pub installed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRegistryData {
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkillData {
    pub registry_id: String,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub folder_path: String,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn create_session(&self, data: CreateSessionData) -> Result<Session>;
    async fn get_session(&self, id: &str) -> Result<Option<Session>>;
    async fn list_sessions(&self, filter: Option<SessionFilter>) -> Result<Vec<Session>>;
    async fn update_session(&self, id: &str, data: UpdateSessionData) -> Result<()>;
    async fn delete_session(&self, id: &str) -> Result<()>;

    async fn save_message(&self, message: CreateMessageData) -> Result<Message>;
    async fn get_messages(&self, session_id: &str, filter: Option<MessageFilter>) -> Result<Vec<Message>>;
    async fn delete_message(&self, id: &str) -> Result<()>;
    async fn delete_messages(&self, session_id: &str) -> Result<()>;

    async fn save_process(&self, process: CreateProcessData) -> Result<Process>;
    async fn get_process(&self, id: &str) -> Result<Option<Process>>;
    async fn list_processes(&self, filter: Option<ProcessListFilter>) -> Result<Vec<Process>>;
    async fn update_process_status(&self, id: &str, status: ProcessStatus, exit_code: Option<i32>) -> Result<()>;
    async fn update_process_output_stats(&self, id: &str, stats: OutputStats) -> Result<()>;

    async fn create_todo(&self, session_id: &str, content: &str, priority: Option<&str>) -> Result<Todo>;
    async fn get_todos(&self, session_id: &str) -> Result<Vec<Todo>>;
    async fn update_todo(&self, id: &str, data: TodoUpdateData) -> Result<()>;
    async fn delete_todo(&self, id: &str) -> Result<()>;

    async fn get_provider(&self, name: &str) -> Result<Option<Provider>>;
    async fn get_default_provider(&self) -> Result<Option<Provider>>;
    async fn list_providers(&self) -> Result<Vec<Provider>>;
    async fn save_provider(&self, provider: NewProvider) -> Result<Provider>;
    async fn update_provider(&self, id: &str, data: ProviderUpdate) -> Result<()>;
    async fn delete_provider(&self, id: &str) -> Result<()>;

    async fn get_external_tool(&self, name: &str) -> Result<Option<ExternalToolData>>;
    async fn list_external_tools(&self) -> Result<Vec<ExternalToolData>>;
    async fn save_external_tool(&self, tool: NewExternalTool) -> Result<ExternalToolData>;
    async fn update_external_tool(&self, id: &str, data: PartialExternalTool) -> Result<()>;
    async fn delete_external_tool(&self, id: &str) -> Result<()>;
    async fn delete_all_external_tools(&self) -> Result<()>;

    async fn get_tool_variable(&self, tool_name: &str, variable_name: &str) -> Result<Option<ToolVariableData>>;
    async fn list_tool_variables(&self, tool_name: &str) -> Result<Vec<ToolVariableData>>;
    async fn save_tool_variable(&self, variable: NewToolVariable) -> Result<ToolVariableData>;
    async fn update_tool_variable(&self, id: &str, data: PartialToolVariable) -> Result<()>;
    async fn delete_tool_variable(&self, id: &str) -> Result<()>;
    async fn delete_tool_variables_by_tool(&self, tool_name: &str) -> Result<()>;

    async fn save_file_read(&self, data: FileReadData) -> Result<()>;
    async fn get_file_read(&self, session_id: &str, file_path: &str) -> Result<Option<FileReadResult>>;
    async fn list_file_reads(&self, session_id: &str) -> Result<Vec<FileReadResult>>;
    async fn clear_file_reads(&self, session_id: &str) -> Result<()>;

    async fn get_registry(&self, id: &str) -> Result<Option<SkillRegistry>>;
    async fn get_registry_by_name(&self, name: &str) -> Result<Option<SkillRegistry>>;
    async fn list_registries(&self) -> Result<Vec<SkillRegistry>>;
    async fn save_registry(&self, data: CreateRegistryData) -> Result<SkillRegistry>;
    async fn delete_registry(&self, id: &str) -> Result<()>;

    async fn get_skill(&self, full_name: &str) -> Result<Option<Skill>>;
    async fn list_skills(&self) -> Result<Vec<Skill>>;
    async fn save_skill(&self, data: CreateSkillData) -> Result<Skill>;
    async fn delete_skill(&self, full_name: &str) -> Result<()>;

    async fn get_timer(&self, id: &str) -> Result<Option<Timer>>;
    async fn get_timer_by_name(&self, name: &str) -> Result<Option<Timer>>;
    async fn list_timers(&self, filter: Option<TimerFilter>) -> Result<Vec<Timer>>;
    async fn save_timer(&self, data: CreateTimerData) -> Result<Timer>;
    async fn update_timer(&self, id: &str, data: UpdateTimerData) -> Result<()>;
    async fn delete_timer(&self, id: &str) -> Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessListFilter {
    pub session_id: Option<String>,
    pub status: Option<ProcessStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoUpdateData {
    pub status: Option<TodoStatus>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProvider {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i32>,
    pub extra_params: Option<String>,
    pub auto_compact_length: Option<i32>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderUpdate {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i32>,
    pub extra_params: Option<String>,
    pub auto_compact_length: Option<i32>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExternalTool {
    pub name: String,
    pub description: String,
    pub parameters_json: String,
    pub config_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialExternalTool {
    pub description: Option<String>,
    pub parameters_json: Option<String>,
    pub config_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewToolVariable {
    pub tool_name: String,
    pub variable_name: String,
    pub variable_value: Option<String>,
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialToolVariable {
    pub variable_value: Option<String>,
    pub description: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadData {
    pub session_id: String,
    pub file_path: String,
    pub mtime_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadResult {
    pub file_path: String,
    pub mtime_ms: i64,
    pub read_at: String,
}

use serde::{Deserialize, Serialize};