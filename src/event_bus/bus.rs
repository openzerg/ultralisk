use crate::core::GlobalEvent;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

type Handler = Box<dyn Fn(serde_json::Value) + Send + Sync>;

pub struct EventBus {
    handlers: Arc<Mutex<HashMap<String, Vec<(String, Handler)>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn emit(&self, event: &str, data: serde_json::Value) {
        let handlers = self.handlers.lock().unwrap();
        if let Some(entries) = handlers.get(event) {
            for (_, handler) in entries {
                handler(data.clone());
            }
        }
    }

    pub fn emit_global(&self, event: GlobalEvent) {
        let event_type = match &event {
            GlobalEvent::Connected(_) => "connected",
            GlobalEvent::Thinking(_) => "thinking",
            GlobalEvent::Response(_) => "response",
            GlobalEvent::ToolCall(_) => "tool_call",
            GlobalEvent::ToolResult(_) => "tool_result",
            GlobalEvent::Done(_) => "done",
            GlobalEvent::Error(_) => "error",
            GlobalEvent::ProcessNotification(_) => "process_notification",
            GlobalEvent::SessionActivity(_) => "session_activity",
            GlobalEvent::Interrupted(_) => "interrupted",
            GlobalEvent::JobCompleted(_) => "job_completed",
            GlobalEvent::JobKilled(_) => "job_killed",
            GlobalEvent::TodoUpdate(_) => "todo_update",
        };
        let data = serde_json::to_value(&event).unwrap_or(serde_json::Value::Null);
        self.emit(event_type, data);
    }

    pub fn on(&self, event: &str, handler: Handler) -> String {
        let id = Uuid::new_v4().to_string();
        let mut handlers = self.handlers.lock().unwrap();
        handlers
            .entry(event.to_string())
            .or_insert_with(Vec::new)
            .push((id.clone(), handler));
        id
    }

    pub fn once(&self, event: &str, handler: Handler) {
        let id = Uuid::new_v4().to_string();
        let handlers = Arc::clone(&self.handlers);
        let event_str = event.to_string();
        let id_for_closure = id.clone();
        let event_for_closure = event_str.clone();
        let once_handler = Box::new(move |data: serde_json::Value| {
            if let Ok(mut h) = handlers.lock() {
                if let Some(entries) = h.get_mut(&event_for_closure) {
                    entries.retain(|(h_id, _)| h_id != &id_for_closure);
                }
            }
            handler(data);
        });
        let mut handlers = self.handlers.lock().unwrap();
        handlers
            .entry(event_str)
            .or_insert_with(Vec::new)
            .push((id, once_handler));
    }

    pub fn off(&self, event: &str, handler_id: &str) {
        let mut handlers = self.handlers.lock().unwrap();
        if let Some(entries) = handlers.get_mut(event) {
            entries.retain(|(id, _)| id != handler_id);
            if entries.is_empty() {
                handlers.remove(event);
            }
        }
    }

    pub fn remove_all_listeners(&self, event: Option<&str>) {
        let mut handlers = self.handlers.lock().unwrap();
        if let Some(event) = event {
            handlers.remove(event);
        } else {
            handlers.clear();
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
