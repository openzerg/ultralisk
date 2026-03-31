use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Default)]
pub struct LogContext {
    pub session_id: Option<String>,
    pub job_id: Option<String>,
    pub tool_name: Option<String>,
    pub extra: HashMap<String, serde_json::Value>,
}

impl LogContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_session_id(mut self, id: impl Into<String>) -> Self {
        self.session_id = Some(id.into());
        self
    }

    pub fn with_job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn with_tool_name(mut self, name: impl Into<String>) -> Self {
        self.tool_name = Some(name.into());
        self
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.extra.insert(
            key.into(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    }
}

pub trait Logger: Send + Sync {
    fn debug(&self, message: &str, context: Option<&LogContext>);
    fn info(&self, message: &str, context: Option<&LogContext>);
    fn warn(&self, message: &str, context: Option<&LogContext>);
    fn error(
        &self,
        message: &str,
        error: Option<&dyn std::error::Error>,
        context: Option<&LogContext>,
    );
    fn with_context(&self, context: LogContext) -> Box<dyn Logger>;
}
