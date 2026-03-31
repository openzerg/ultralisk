use super::types::{LogContext, LogLevel, Logger};
use chrono::Utc;
use std::collections::HashMap;

pub struct ConsoleLogger {
    level: LogLevel,
    base_context: LogContext,
}

impl ConsoleLogger {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            base_context: LogContext::default(),
        }
    }

    pub fn with_base_context(level: LogLevel, base_context: LogContext) -> Self {
        Self {
            level,
            base_context,
        }
    }

    fn should_log(&self, level: LogLevel) -> bool {
        match (self.level, level) {
            (LogLevel::Error, _) => true,
            (LogLevel::Warn, LogLevel::Error) => false,
            (LogLevel::Warn, _) => true,
            (LogLevel::Info, LogLevel::Debug) => false,
            (LogLevel::Info, _) => true,
            (LogLevel::Debug, _) => true,
        }
    }

    fn format(&self, level: LogLevel, message: &str, context: Option<&LogContext>) -> String {
        let timestamp = Utc::now().to_rfc3339();
        let level_str = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };

        let mut ctx_map = HashMap::new();
        if let Some(ref session_id) = self.base_context.session_id {
            ctx_map.insert("sessionId", session_id.clone());
        }
        if let Some(ref job_id) = self.base_context.job_id {
            ctx_map.insert("jobId", job_id.clone());
        }
        if let Some(ref tool_name) = self.base_context.tool_name {
            ctx_map.insert("toolName", tool_name.clone());
        }

        if let Some(ctx) = context {
            if let Some(ref session_id) = ctx.session_id {
                ctx_map.insert("sessionId", session_id.clone());
            }
            if let Some(ref job_id) = ctx.job_id {
                ctx_map.insert("jobId", job_id.clone());
            }
            if let Some(ref tool_name) = ctx.tool_name {
                ctx_map.insert("toolName", tool_name.clone());
            }
        }

        let ctx_str = if ctx_map.is_empty() {
            String::new()
        } else {
            format!(" {}", serde_json::to_string(&ctx_map).unwrap_or_default())
        };

        format!("[{}] [{}]{} {}", timestamp, level_str, ctx_str, message)
    }
}

impl Logger for ConsoleLogger {
    fn debug(&self, message: &str, context: Option<&LogContext>) {
        if self.should_log(LogLevel::Debug) {
            eprintln!("{}", self.format(LogLevel::Debug, message, context));
        }
    }

    fn info(&self, message: &str, context: Option<&LogContext>) {
        if self.should_log(LogLevel::Info) {
            println!("{}", self.format(LogLevel::Info, message, context));
        }
    }

    fn warn(&self, message: &str, context: Option<&LogContext>) {
        if self.should_log(LogLevel::Warn) {
            eprintln!("{}", self.format(LogLevel::Warn, message, context));
        }
    }

    fn error(
        &self,
        message: &str,
        error: Option<&dyn std::error::Error>,
        context: Option<&LogContext>,
    ) {
        if self.should_log(LogLevel::Error) {
            let err_str = error.map(|e| format!(": {}", e)).unwrap_or_default();
            eprintln!(
                "{}",
                self.format(LogLevel::Error, &format!("{}{}", message, err_str), context)
            );
        }
    }

    fn with_context(&self, context: LogContext) -> Box<dyn Logger> {
        let mut merged = self.base_context.clone();
        if context.session_id.is_some() {
            merged.session_id = context.session_id;
        }
        if context.job_id.is_some() {
            merged.job_id = context.job_id;
        }
        if context.tool_name.is_some() {
            merged.tool_name = context.tool_name;
        }
        merged.extra.extend(context.extra);
        Box::new(Self::with_base_context(self.level, merged))
    }
}
