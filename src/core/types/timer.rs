use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerType {
    Calendar,
    Active,
    UnitActive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerStatus {
    Active,
    Disabled,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timer {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub message_template: String,
    pub timer_type: TimerType,
    pub timer_spec: String,
    pub status: TimerStatus,
    pub session_id: Option<String>,
    pub max_runs: i32,
    pub run_count: i32,
    pub last_run_at: Option<String>,
    pub next_run_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimerData {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub message_template: String,
    pub timer_type: TimerType,
    pub timer_spec: String,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub max_runs: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTimerData {
    pub status: Option<TimerStatus>,
    pub run_count: Option<i32>,
    pub last_run_at: Option<String>,
    pub next_run_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimerFilter {
    pub session_id: Option<String>,
    pub status: Option<TimerStatus>,
}
