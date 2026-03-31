use async_trait::async_trait;
use crate::core::types::tool::{ToolDefinition, ToolResult, JSONSchema, ExternalToolDefinition};
use std::collections::HashMap;

pub struct ToolContext {
    pub session_id: String,
    pub session_name: String,
    pub working_directory: String,
    pub process_manager: Option<Box<dyn super::process_manager::ProcessManager>>,
    pub event_bus: Option<Box<dyn super::event_bus::EventBus>>,
    pub storage: Option<Box<dyn super::storage::Storage>>,
    pub llm_client: Option<Box<dyn super::llm_client::LLMClient>>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> JSONSchema;
    async fn execute(&self, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult;
}

#[async_trait]
pub trait ToolRegistry: Send + Sync {
    fn register(&mut self, tool: Box<dyn Tool>);
    fn unregister(&mut self, name: &str);
    fn get(&self, name: &str) -> Option<&dyn Tool>;
    async fn get_builtin_definitions(&self) -> Vec<ToolDefinition>;
    async fn get_definitions(&self) -> Vec<ToolDefinition>;
    async fn execute(&self, name: &str, args: HashMap<String, serde_json::Value>, context: ToolContext) -> ToolResult;
    fn has(&self, name: &str) -> bool;
    fn list(&self) -> Vec<String>;

    fn register_external(&mut self, tool: ExternalToolDefinition);
    fn unregister_external(&mut self, name: &str);
    fn sync_external_tools(&mut self, tools: Vec<ExternalToolDefinition>);
    fn set_tool_variable(&mut self, tool_name: &str, var_name: &str, value: &str);
    fn get_tool_variables(&self, tool_name: &str) -> HashMap<String, String>;

    fn set_skills_provider(&mut self, provider: Box<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<SkillInfo>> + Send>> + Send + Sync>);
}

pub struct SkillInfo {
    pub full_name: String,
    pub description: String,
}