use async_trait::async_trait;
use crate::core::{ToolDefinition, ToolResult, JSONSchema, ExternalToolDefinition, ToolContext, SkillInfo};
use crate::core::interfaces::ToolRegistry as ToolRegistryTrait;
use crate::core::interfaces::Tool;
use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
    external_tools: HashMap<String, ExternalToolDefinition>,
    variables: HashMap<String, HashMap<String, String>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            external_tools: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolRegistryTrait for ToolRegistry {
    fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    fn unregister(&mut self, name: &str) {
        self.tools.remove(name);
    }

    fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|b| b.as_ref())
    }

    async fn get_builtin_definitions(&self) -> Vec<ToolDefinition> {
        Vec::new()
    }

    async fn get_definitions(&self) -> Vec<ToolDefinition> {
        Vec::new()
    }

    async fn execute(&self, _name: &str, _args: HashMap<String, serde_json::Value>, _context: ToolContext) -> ToolResult {
        ToolResult {
            title: "Not implemented".to_string(),
            output: "".to_string(),
            metadata: HashMap::new(),
            attachments: Vec::new(),
            truncated: false,
            success: Some(false),
        }
    }

    fn has(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    fn list(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    fn register_external(&mut self, tool: ExternalToolDefinition) {
        self.external_tools.insert(tool.name.clone(), tool);
    }

    fn unregister_external(&mut self, name: &str) {
        self.external_tools.remove(name);
    }

    fn sync_external_tools(&mut self, tools: Vec<ExternalToolDefinition>) {
        self.external_tools.clear();
        for tool in tools {
            self.external_tools.insert(tool.name.clone(), tool);
        }
    }

    fn set_tool_variable(&mut self, tool_name: &str, var_name: &str, value: &str) {
        self.variables
            .entry(tool_name.to_string())
            .or_insert_with(HashMap::new)
            .insert(var_name.to_string(), value.to_string());
    }

    fn get_tool_variables(&self, tool_name: &str) -> HashMap<String, String> {
        self.variables.get(tool_name).cloned().unwrap_or_default()
    }

    fn set_skills_provider(&mut self, _provider: Box<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<SkillInfo>> + Send>> + Send + Sync>) {
    }
}