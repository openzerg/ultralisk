use crate::core::{LLMConfig, LLMMessage, ChatCompletionRequest, ChatCompletionResponse, StreamChunk, TokenUsage, ChatCompletionUsage};
use crate::core::types::tool::ToolDefinition;
use crate::core::interfaces::{LLMClient as LLMClientTrait, LLMStreamError, PartialLLMConfig, Storage};
use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use std::pin::Pin;
use std::sync::Arc;

pub struct AutoConfigLLMClient {
    client: Arc<dyn LLMClientTrait>,
    storage: Arc<dyn Storage>,
}

impl AutoConfigLLMClient {
    pub fn new(client: Arc<dyn LLMClientTrait>, storage: Arc<dyn Storage>) -> Self {
        Self { client, storage }
    }

    async fn ensure_config(&self) -> anyhow::Result<()> {
        if let Some(provider) = self.storage.get_default_provider().await? {
            let extra_params = provider.extra_params
                .and_then(|s| serde_json::from_str(&s).ok());
            
            let new_config = LLMConfig {
                base_url: provider.base_url,
                api_key: provider.api_key,
                model: provider.model,
                max_tokens: provider.max_tokens,
                temperature: provider.temperature,
                top_p: provider.top_p,
                top_k: provider.top_k,
                extra_params,
            };
            
            let current = self.client.get_config();
            if serde_json::to_string(&new_config)? != serde_json::to_string(&current)? {
                self.client.update_config(PartialLLMConfig {
                    base_url: Some(new_config.base_url),
                    api_key: Some(new_config.api_key),
                    model: Some(new_config.model),
                    max_tokens: new_config.max_tokens,
                    temperature: new_config.temperature,
                    top_p: new_config.top_p,
                    top_k: new_config.top_k,
                    extra_params: new_config.extra_params,
                });
            }
        }
        Ok(())
    }
}

#[async_trait]
impl LLMClientTrait for AutoConfigLLMClient {
    async fn complete(
        &self,
        messages: Vec<LLMMessage>,
        tools: Option<Vec<ToolDefinition>>,
        options: Option<ChatCompletionRequest>,
    ) -> anyhow::Result<ChatCompletionResponse> {
        self.ensure_config().await?;
        self.client.complete(messages, tools, options).await
    }

    fn stream(
        &self,
        messages: Vec<LLMMessage>,
        tools: Option<Vec<ToolDefinition>>,
        options: Option<ChatCompletionRequest>,
    ) -> Pin<Box<dyn Stream<Item = Result<StreamChunk, LLMStreamError>> + Send>> {
        let client = Arc::clone(&self.client);
        let storage = Arc::clone(&self.storage);
        
        async_stream::stream! {
            if let Ok(Some(_provider)) = storage.get_default_provider().await {
                // Config would be updated here
            }
            let mut stream = client.stream(messages, tools, options);
            while let Some(item) = futures::StreamExt::next(&mut stream).await {
                yield item;
            }
        }.boxed()
    }

    fn update_config(&self, config: PartialLLMConfig) {
        self.client.update_config(config);
    }

    fn get_config(&self) -> LLMConfig {
        self.client.get_config()
    }

    fn get_token_usage(&self, usage: ChatCompletionUsage) -> TokenUsage {
        self.client.get_token_usage(usage)
    }
}