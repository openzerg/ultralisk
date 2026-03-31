use crate::core::{LLMConfig, LLMMessage, ChatCompletionRequest, ChatCompletionResponse, StreamChunk, TokenUsage, ChatCompletionUsage};
use crate::core::types::tool::ToolDefinition;
use crate::core::interfaces::{LLMClient as LLMClientTrait, LLMStreamError, PartialLLMConfig};
use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use std::pin::Pin;
use std::sync::RwLock;

const DEFAULT_BASE_URL: &str = "https://coding.dashscope.aliyuncs.com/v1";
const DEFAULT_MODEL: &str = "qwen3.5-plus";

pub struct LLMClient {
    config: RwLock<LLMConfig>,
    client: reqwest::Client,
}

impl LLMClient {
    pub fn new(config: Option<PartialLLMConfig>) -> Self {
        let base_url = config.as_ref().and_then(|c| c.base_url.clone()).unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let api_key = config.as_ref().and_then(|c| c.api_key.clone()).unwrap_or_default();
        let model = config.as_ref().and_then(|c| c.model.clone()).unwrap_or_else(|| DEFAULT_MODEL.to_string());
        let max_tokens = config.as_ref().and_then(|c| c.max_tokens);
        let temperature = config.as_ref().and_then(|c| c.temperature);
        let top_p = config.as_ref().and_then(|c| c.top_p);
        let top_k = config.as_ref().and_then(|c| c.top_k);
        let extra_params = config.and_then(|c| c.extra_params);
        
        Self {
            config: RwLock::new(LLMConfig {
                base_url: base_url.trim_end_matches('/').to_string(),
                api_key,
                model,
                max_tokens,
                temperature,
                top_p,
                top_k,
                extra_params,
            }),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLMClientTrait for LLMClient {
    async fn complete(
        &self,
        _messages: Vec<LLMMessage>,
        _tools: Option<Vec<ToolDefinition>>,
        _options: Option<ChatCompletionRequest>,
    ) -> anyhow::Result<ChatCompletionResponse> {
        anyhow::bail!("Not implemented")
    }

    fn stream(
        &self,
        _messages: Vec<LLMMessage>,
        _tools: Option<Vec<ToolDefinition>>,
        _options: Option<ChatCompletionRequest>,
    ) -> Pin<Box<dyn Stream<Item = Result<StreamChunk, LLMStreamError>> + Send>> {
        futures::stream::empty().boxed()
    }

    fn update_config(&self, config: PartialLLMConfig) {
        if let Ok(mut c) = self.config.write() {
            if let Some(base_url) = config.base_url {
                c.base_url = base_url.trim_end_matches('/').to_string();
            }
            if let Some(api_key) = config.api_key {
                c.api_key = api_key;
            }
            if let Some(model) = config.model {
                c.model = model;
            }
            if let Some(max_tokens) = config.max_tokens {
                c.max_tokens = Some(max_tokens);
            }
            if let Some(temperature) = config.temperature {
                c.temperature = Some(temperature);
            }
            if let Some(top_p) = config.top_p {
                c.top_p = Some(top_p);
            }
            if let Some(top_k) = config.top_k {
                c.top_k = Some(top_k);
            }
        }
    }

    fn get_config(&self) -> LLMConfig {
        self.config.read().unwrap().clone()
    }

    fn get_token_usage(&self, usage: ChatCompletionUsage) -> TokenUsage {
        TokenUsage {
            input: usage.prompt_tokens,
            output: usage.completion_tokens,
            total: usage.total_tokens,
            reasoning: usage.reasoning_tokens.unwrap_or(0),
            cache: crate::core::TokenUsageCache {
                read: usage.cached_input_tokens.unwrap_or(0),
                write: usage.cache_creation_input_tokens.unwrap_or(0),
            },
        }
    }
}