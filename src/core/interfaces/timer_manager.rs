use async_trait::async_trait;
use crate::core::types::{Timer, CreateTimerData, TimerFilter};
use anyhow::Result;

#[async_trait]
pub trait TimerManager: Send + Sync {
    async fn create_timer(&self, data: CreateTimerData) -> Result<Timer>;
    async fn cancel_timer(&self, timer_id: &str) -> Result<()>;
    async fn disable_timer(&self, timer_id: &str) -> Result<()>;
    async fn enable_timer(&self, timer_id: &str) -> Result<()>;
    async fn list_timers(&self, filter: Option<TimerFilter>) -> Result<Vec<Timer>>;
    async fn get_timer(&self, timer_id: &str) -> Result<Option<Timer>>;
    async fn load_timers(&self) -> Result<()>;
}