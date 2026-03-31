use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i32>,
    pub extra_params: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default = "default_socket_path")]
    pub socket_path: String,
}

fn default_api_port() -> u16 {
    15317
}
fn default_socket_path() -> String {
    "/var/run/openzerg.sock".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_agent_name")]
    pub agent_name: String,
    #[serde(default = "default_workspace")]
    pub workspace: PathBuf,
    pub llm: Option<PartialLLMConfig>,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub providers: Vec<LLMProviderConfig>,
    pub log_level: Option<String>,
}

fn default_agent_name() -> String {
    "standalone".to_string()
}
fn default_workspace() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("workspace")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialLLMConfig {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub extra_params: Option<serde_json::Value>,
}

pub fn load_config(_config_path: Option<&str>) -> AppConfig {
    AppConfig {
        agent_name: default_agent_name(),
        workspace: default_workspace(),
        llm: None,
        server: ServerConfig {
            api_port: default_api_port(),
            socket_path: default_socket_path(),
        },
        providers: Vec::new(),
        log_level: Some("info".to_string()),
    }
}

pub fn get_active_provider(config: &AppConfig) -> Option<LLMProviderConfig> {
    config
        .providers
        .iter()
        .find(|p| p.is_active.unwrap_or(false))
        .cloned()
}
