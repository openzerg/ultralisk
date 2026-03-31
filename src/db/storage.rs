use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::core::interfaces::storage::*;
use crate::core::types::{
    Session, CreateSessionData, UpdateSessionData, SessionFilter,
    Message, CreateMessageData, MessageFilter,
    Process, CreateProcessData, ProcessStatus, OutputStats, ProcessOutput,
    Todo, Provider, Timer, CreateTimerData, UpdateTimerData, TimerFilter,
};
use crate::core::types::tool::{ExternalToolData, ToolVariableData, JSONSchema};
use crate::db::repositories;
use anyhow::Result;

pub struct DbStorage {
    db: Arc<DatabaseConnection>,
}

impl DbStorage {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Storage for DbStorage {
    async fn create_session(&self, data: CreateSessionData) -> Result<Session> {
        repositories::session::create_session(&self.db, data).await
    }

    async fn get_session(&self, id: &str) -> Result<Option<Session>> {
        repositories::session::get_session(&self.db, id).await
    }

    async fn list_sessions(&self, filter: Option<SessionFilter>) -> Result<Vec<Session>> {
        repositories::session::list_sessions(&self.db, filter).await
    }

    async fn update_session(&self, id: &str, data: UpdateSessionData) -> Result<()> {
        repositories::session::update_session(&self.db, id, data).await
    }

    async fn delete_session(&self, id: &str) -> Result<()> {
        repositories::session::delete_session(&self.db, id).await
    }

    async fn save_message(&self, message: CreateMessageData) -> Result<Message> {
        repositories::message::save_message(&self.db, message).await
    }

    async fn get_messages(&self, session_id: &str, filter: Option<MessageFilter>) -> Result<Vec<Message>> {
        repositories::message::get_messages(&self.db, session_id, filter).await
    }

    async fn delete_message(&self, id: &str) -> Result<()> {
        repositories::message::delete_message(&self.db, id).await
    }

    async fn delete_messages(&self, session_id: &str) -> Result<()> {
        repositories::message::delete_messages(&self.db, session_id).await
    }

    async fn save_process(&self, process: CreateProcessData) -> Result<Process> {
        repositories::process::save_process(&self.db, process).await
    }

    async fn get_process(&self, id: &str) -> Result<Option<Process>> {
        repositories::process::get_process(&self.db, id).await
    }

    async fn list_processes(&self, filter: Option<ProcessListFilter>) -> Result<Vec<Process>> {
        repositories::process::list_processes(&self.db, filter).await
    }

    async fn update_process_status(&self, id: &str, status: ProcessStatus, exit_code: Option<i32>) -> Result<()> {
        repositories::process::update_process_status(&self.db, id, status, exit_code).await
    }

    async fn update_process_output_stats(&self, id: &str, stats: OutputStats) -> Result<()> {
        repositories::process::update_process_output_stats(&self.db, id, stats).await
    }

    async fn create_todo(&self, session_id: &str, content: &str, priority: Option<&str>) -> Result<Todo> {
        repositories::todo::create_todo(&self.db, session_id, content, priority).await
    }

    async fn get_todos(&self, session_id: &str) -> Result<Vec<Todo>> {
        repositories::todo::get_todos(&self.db, session_id).await
    }

    async fn update_todo(&self, id: &str, data: TodoUpdateData) -> Result<()> {
        repositories::todo::update_todo(&self.db, id, data).await
    }

    async fn delete_todo(&self, id: &str) -> Result<()> {
        repositories::todo::delete_todo(&self.db, id).await
    }

    async fn get_provider(&self, name: &str) -> Result<Option<Provider>> {
        repositories::provider::get_provider(&self.db, name).await
    }

    async fn get_default_provider(&self) -> Result<Option<Provider>> {
        repositories::provider::get_default_provider(&self.db).await
    }

    async fn list_providers(&self) -> Result<Vec<Provider>> {
        repositories::provider::list_providers(&self.db).await
    }

    async fn save_provider(&self, provider: NewProvider) -> Result<Provider> {
        repositories::provider::save_provider(&self.db, provider).await
    }

    async fn update_provider(&self, id: &str, data: ProviderUpdate) -> Result<()> {
        repositories::provider::update_provider(&self.db, id, data).await
    }

    async fn delete_provider(&self, id: &str) -> Result<()> {
        repositories::provider::delete_provider(&self.db, id).await
    }

    async fn get_external_tool(&self, name: &str) -> Result<Option<ExternalToolData>> {
        repositories::external_tool::get_external_tool(&self.db, name).await
    }

    async fn list_external_tools(&self) -> Result<Vec<ExternalToolData>> {
        repositories::external_tool::list_external_tools(&self.db).await
    }

    async fn save_external_tool(&self, tool: NewExternalTool) -> Result<ExternalToolData> {
        repositories::external_tool::save_external_tool(&self.db, tool).await
    }

    async fn update_external_tool(&self, id: &str, data: PartialExternalTool) -> Result<()> {
        repositories::external_tool::update_external_tool(&self.db, id, data).await
    }

    async fn delete_external_tool(&self, id: &str) -> Result<()> {
        repositories::external_tool::delete_external_tool(&self.db, id).await
    }

    async fn delete_all_external_tools(&self) -> Result<()> {
        repositories::external_tool::delete_all_external_tools(&self.db).await
    }

    async fn get_tool_variable(&self, tool_name: &str, variable_name: &str) -> Result<Option<ToolVariableData>> {
        repositories::tool_variable::get_tool_variable(&self.db, tool_name, variable_name).await
    }

    async fn list_tool_variables(&self, tool_name: &str) -> Result<Vec<ToolVariableData>> {
        repositories::tool_variable::list_tool_variables(&self.db, tool_name).await
    }

    async fn save_tool_variable(&self, variable: NewToolVariable) -> Result<ToolVariableData> {
        repositories::tool_variable::save_tool_variable(&self.db, variable).await
    }

    async fn update_tool_variable(&self, id: &str, data: PartialToolVariable) -> Result<()> {
        repositories::tool_variable::update_tool_variable(&self.db, id, data).await
    }

    async fn delete_tool_variable(&self, id: &str) -> Result<()> {
        repositories::tool_variable::delete_tool_variable(&self.db, id).await
    }

    async fn delete_tool_variables_by_tool(&self, tool_name: &str) -> Result<()> {
        repositories::tool_variable::delete_tool_variables_by_tool(&self.db, tool_name).await
    }

    async fn save_file_read(&self, data: FileReadData) -> Result<()> {
        repositories::file_read::save_file_read(&self.db, data).await
    }

    async fn get_file_read(&self, session_id: &str, file_path: &str) -> Result<Option<FileReadResult>> {
        repositories::file_read::get_file_read(&self.db, session_id, file_path).await
    }

    async fn list_file_reads(&self, session_id: &str) -> Result<Vec<FileReadResult>> {
        repositories::file_read::list_file_reads(&self.db, session_id).await
    }

    async fn clear_file_reads(&self, session_id: &str) -> Result<()> {
        repositories::file_read::clear_file_reads(&self.db, session_id).await
    }

    async fn get_registry(&self, id: &str) -> Result<Option<SkillRegistry>> {
        repositories::skill::get_registry(&self.db, id).await
    }

    async fn get_registry_by_name(&self, name: &str) -> Result<Option<SkillRegistry>> {
        repositories::skill::get_registry_by_name(&self.db, name).await
    }

    async fn list_registries(&self) -> Result<Vec<SkillRegistry>> {
        repositories::skill::list_registries(&self.db).await
    }

    async fn save_registry(&self, data: CreateRegistryData) -> Result<SkillRegistry> {
        repositories::skill::create_registry(&self.db, data).await
    }

    async fn delete_registry(&self, id: &str) -> Result<()> {
        repositories::skill::delete_registry(&self.db, id).await
    }

    async fn get_skill(&self, full_name: &str) -> Result<Option<Skill>> {
        repositories::skill::get_skill(&self.db, full_name).await
    }

    async fn list_skills(&self) -> Result<Vec<Skill>> {
        repositories::skill::list_skills(&self.db).await
    }

    async fn save_skill(&self, data: CreateSkillData) -> Result<Skill> {
        repositories::skill::create_skill(&self.db, data).await
    }

    async fn delete_skill(&self, full_name: &str) -> Result<()> {
        repositories::skill::delete_skill(&self.db, full_name).await
    }

    async fn get_timer(&self, id: &str) -> Result<Option<Timer>> {
        repositories::timer::get_timer(&self.db, id).await
    }

    async fn get_timer_by_name(&self, name: &str) -> Result<Option<Timer>> {
        repositories::timer::get_timer_by_name(&self.db, name).await
    }

    async fn list_timers(&self, filter: Option<TimerFilter>) -> Result<Vec<Timer>> {
        repositories::timer::list_timers(&self.db, filter).await
    }

    async fn save_timer(&self, data: CreateTimerData) -> Result<Timer> {
        repositories::timer::create_timer(&self.db, data).await
    }

    async fn update_timer(&self, id: &str, data: UpdateTimerData) -> Result<()> {
        repositories::timer::update_timer(&self.db, id, data).await
    }

    async fn delete_timer(&self, id: &str) -> Result<()> {
        repositories::timer::delete_timer(&self.db, id).await
    }
}