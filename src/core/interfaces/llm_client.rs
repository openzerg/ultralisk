use async_trait::async_trait;
use crate::core::types::{
    LLMMessage, ChatCompletionRequest, ChatCompletionResponse, 
    StreamChunk, LLMConfig, TokenUsage, ChatCompletionUsage,
};
use crate::core::types::tool::ToolDefinition;
use futures::Stream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "PascalCase")]
pub enum LLMStreamError {
    ConnectionError { message: String, status: Option<u16> },
    NetworkError { message: String },
    ParseError { message: String, raw: Option<String> },
    StreamInterrupted { message: String },
}

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn complete(
        &self,
        messages: Vec<LLMMessage>,
        tools: Option<Vec<ToolDefinition>>,
        options: Option<ChatCompletionRequest>,
    ) -> anyhow::Result<ChatCompletionResponse>;

    fn stream(
        &self,
        messages: Vec<LLMMessage>,
        tools: Option<Vec<ToolDefinition>>,
        options: Option<ChatCompletionRequest>,
    ) -> std::pin::Pin<Box<dyn Stream<Item = Result<StreamChunk, LLMStreamError>> + Send>>;

    fn update_config(&self, config: PartialLLMConfig);
    fn get_config(&self) -> LLMConfig;
    fn get_token_usage(&self, usage: ChatCompletionUsage) -> TokenUsage;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PartialLLMConfig {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<i32>,
    pub extra_params: Option<serde_json::Value>,
}