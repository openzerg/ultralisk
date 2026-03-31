use crate::core::types::GlobalEvent;

pub trait EventBus: Send + Sync {
    fn emit(&self, event: &str, data: serde_json::Value);
    fn emit_global(&self, event: GlobalEvent);
    fn on(&self, event: &str, handler: Box<dyn Fn(serde_json::Value) + Send + Sync>) -> String;
    fn once(&self, event: &str, handler: Box<dyn Fn(serde_json::Value) + Send + Sync>);
    fn off(&self, event: &str, handler_id: &str);
    fn remove_all_listeners(&self, event: Option<&str>);
}
