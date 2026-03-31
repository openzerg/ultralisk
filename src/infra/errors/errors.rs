use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ErrorCode {
    #[error("SESSION_NOT_FOUND")]
    SessionNotFound,
    #[error("SESSION_INVALID_STATE")]
    SessionInvalidState,
    #[error("PROVIDER_NOT_FOUND")]
    ProviderNotFound,
    #[error("TOOL_NOT_FOUND")]
    ToolNotFound,
    #[error("TOOL_EXECUTION_FAILED")]
    ToolExecutionFailed,
    #[error("LLM_REQUEST_FAILED")]
    LlmRequestFailed,
    #[error("INVALID_ARGUMENT")]
    InvalidArgument,
    #[error("INTERNAL_ERROR")]
    InternalError,
}

#[derive(Debug, Error)]
#[error("{code}: {message}")]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    #[source]
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    pub context: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            cause: None,
            context: None,
        }
    }

    pub fn with_cause(mut self, cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    pub fn with_context(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.context
            .get_or_insert_with(Default::default)
            .insert(key.into(), value);
        self
    }
}

pub struct Errors;

impl Errors {
    pub fn session_not_found(id: impl Into<String>) -> AppError {
        let id = id.into();
        AppError::new(
            ErrorCode::SessionNotFound,
            format!("Session {} not found", id),
        )
        .with_context("sessionId", serde_json::json!(id))
    }

    pub fn provider_not_found(name: impl Into<String>) -> AppError {
        let name = name.into();
        AppError::new(
            ErrorCode::ProviderNotFound,
            format!("Provider \"{}\" not found", name),
        )
        .with_context("providerName", serde_json::json!(name))
    }

    pub fn tool_not_found(name: impl Into<String>) -> AppError {
        let name = name.into();
        AppError::new(
            ErrorCode::ToolNotFound,
            format!("Tool \"{}\" not found", name),
        )
        .with_context("toolName", serde_json::json!(name))
    }

    pub fn tool_execution_failed(
        tool_name: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> AppError {
        let tool_name = tool_name.into();
        let message = format!("Tool \"{}\" failed: {}", tool_name, cause);
        AppError::new(ErrorCode::ToolExecutionFailed, message)
            .with_cause(cause)
            .with_context("toolName", serde_json::json!(tool_name))
    }

    pub fn llm_request_failed(cause: impl std::error::Error + Send + Sync + 'static) -> AppError {
        let message = format!("LLM request failed: {}", cause);
        AppError::new(ErrorCode::LlmRequestFailed, message).with_cause(cause)
    }

    pub fn invalid_argument(message: impl Into<String>) -> AppError {
        AppError::new(ErrorCode::InvalidArgument, message)
    }

    pub fn internal(message: impl Into<String>) -> AppError {
        AppError::new(ErrorCode::InternalError, message)
    }

    pub fn internal_with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> AppError {
        AppError::new(ErrorCode::InternalError, message).with_cause(cause)
    }
}
