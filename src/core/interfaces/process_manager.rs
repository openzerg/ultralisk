use async_trait::async_trait;
use crate::core::types::{
    ProcessHandle, ProcessResult, ProcessStatus, ProcessOutput,
    OutputStats, SpawnOptions,
};

#[async_trait]
pub trait ProcessManager: Send + Sync {
    async fn spawn(&self, command: &str, options: SpawnOptions) -> anyhow::Result<ProcessHandle>;
    async fn wait(&self, process_id: &str, timeout: Option<i32>) -> anyhow::Result<ProcessResult>;
    async fn kill(&self, process_id: &str, signal: Option<&str>, unit_name: Option<&str>) -> anyhow::Result<()>;
    fn get_status(&self, process_id: &str) -> Option<ProcessStatus>;
    async fn get_output(&self, process_id: &str, request: OutputRequest) -> anyhow::Result<ProcessOutput>;
    async fn get_output_stats(&self, process_id: &str) -> anyhow::Result<OutputStats>;
    async fn list_processes(&self) -> anyhow::Result<Vec<ProcessHandle>>;
}

#[derive(Debug, Clone, Default)]
pub struct OutputRequest {
    pub stream: Option<String>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}