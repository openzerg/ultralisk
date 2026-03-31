use async_trait::async_trait;
use crate::core::{ProcessHandle, ProcessResult, ProcessStatus, ProcessOutput, OutputStats, SpawnOptions, OutputRequest};
use crate::core::interfaces::ProcessManager as ProcessManagerTrait;

pub struct ProcessManager;

#[async_trait]
impl ProcessManagerTrait for ProcessManager {
    async fn spawn(&self, _command: &str, _options: SpawnOptions) -> anyhow::Result<ProcessHandle> {
        anyhow::bail!("Not implemented")
    }

    async fn wait(&self, _process_id: &str, _timeout: Option<i32>) -> anyhow::Result<ProcessResult> {
        anyhow::bail!("Not implemented")
    }

    async fn kill(&self, _process_id: &str, _signal: Option<&str>, _unit_name: Option<&str>) -> anyhow::Result<()> {
        anyhow::bail!("Not implemented")
    }

    fn get_status(&self, _process_id: &str) -> Option<ProcessStatus> {
        None
    }

    async fn get_output(&self, _process_id: &str, _request: OutputRequest) -> anyhow::Result<ProcessOutput> {
        anyhow::bail!("Not implemented")
    }

    async fn get_output_stats(&self, _process_id: &str) -> anyhow::Result<OutputStats> {
        anyhow::bail!("Not implemented")
    }

    async fn list_processes(&self) -> anyhow::Result<Vec<ProcessHandle>> {
        Ok(Vec::new())
    }
}