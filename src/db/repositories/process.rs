use sea_orm::{EntityTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::processes;
use crate::core::{Process, CreateProcessData, ProcessStatus, OutputStats};
use crate::core::interfaces::storage::ProcessListFilter;
use anyhow::Result;
use chrono::Utc;

pub async fn save_process(
    db: &sea_orm::DatabaseConnection,
    data: CreateProcessData,
) -> Result<Process> {
    let now = Utc::now().to_rfc3339();

    let id = data.id;
    let command = data.command;
    let cwd = data.cwd;
    let parent_session_id = data.parent_session_id;
    let unit_name = data.unit_name;
    let timeout_ms = data.timeout_ms.unwrap_or(120000);
    let output_dir = data.output_dir;

    let process = processes::ActiveModel {
        id: Set(id.clone()),
        command: Set(command.clone()),
        cwd: Set(cwd.clone()),
        status: Set("Running".to_string()),
        exit_code: Set(None),
        started_at: Set(now.clone()),
        finished_at: Set(None),
        parent_session_id: Set(parent_session_id.clone()),
        unit_name: Set(unit_name.clone()),
        timeout_ms: Set(timeout_ms),
        output_dir: Set(output_dir.clone()),
        stdout_size: Set(0),
        stderr_size: Set(0),
        stdout_lines: Set(0),
        stderr_lines: Set(0),
    };

    Insert::one(process).exec(db).await?;

    Ok(Process {
        id,
        command,
        cwd,
        status: ProcessStatus::Running,
        exit_code: None,
        started_at: now,
        finished_at: None,
        parent_session_id,
        unit_name,
        timeout_ms,
        output_dir,
        stdout_size: 0,
        stderr_size: 0,
        stdout_lines: 0,
        stderr_lines: 0,
    })
}

pub async fn get_process(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<Option<Process>> {
    let result = processes::Entity::find_by_id(id)
        .one(db)
        .await?;

    Ok(result.map(|r| Process {
        id: r.id,
        command: r.command,
        cwd: r.cwd,
        status: match r.status.as_str() {
            "Running" => ProcessStatus::Running,
            "Completed" => ProcessStatus::Completed,
            "Failed" => ProcessStatus::Failed,
            "Timeout" => ProcessStatus::Timeout,
            "Killed" => ProcessStatus::Killed,
            _ => ProcessStatus::Running,
        },
        exit_code: r.exit_code,
        started_at: r.started_at,
        finished_at: r.finished_at,
        parent_session_id: r.parent_session_id,
        unit_name: r.unit_name,
        timeout_ms: r.timeout_ms,
        output_dir: r.output_dir,
        stdout_size: r.stdout_size,
        stderr_size: r.stderr_size,
        stdout_lines: r.stdout_lines,
        stderr_lines: r.stderr_lines,
    }))
}

pub async fn list_processes(
    db: &sea_orm::DatabaseConnection,
    filter: Option<ProcessListFilter>,
) -> Result<Vec<Process>> {
    let mut query = processes::Entity::find()
        .order_by_desc(processes::Column::StartedAt);

    if let Some(f) = filter {
        if let Some(session_id) = f.session_id {
            query = query.filter(processes::Column::ParentSessionId.eq(session_id));
        }
        if let Some(status) = f.status {
            let status_str = match status {
                ProcessStatus::Running => "Running",
                ProcessStatus::Completed => "Completed",
                ProcessStatus::Failed => "Failed",
                ProcessStatus::Timeout => "Timeout",
                ProcessStatus::Killed => "Killed",
            };
            query = query.filter(processes::Column::Status.eq(status_str));
        }
    }

    let results = query.all(db).await?;

    Ok(results.into_iter().map(|r| Process {
        id: r.id,
        command: r.command,
        cwd: r.cwd,
        status: match r.status.as_str() {
            "Running" => ProcessStatus::Running,
            "Completed" => ProcessStatus::Completed,
            "Failed" => ProcessStatus::Failed,
            "Timeout" => ProcessStatus::Timeout,
            "Killed" => ProcessStatus::Killed,
            _ => ProcessStatus::Running,
        },
        exit_code: r.exit_code,
        started_at: r.started_at,
        finished_at: r.finished_at,
        parent_session_id: r.parent_session_id,
        unit_name: r.unit_name,
        timeout_ms: r.timeout_ms,
        output_dir: r.output_dir,
        stdout_size: r.stdout_size,
        stderr_size: r.stderr_size,
        stdout_lines: r.stdout_lines,
        stderr_lines: r.stderr_lines,
    }).collect())
}

pub async fn update_process_status(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    status: ProcessStatus,
    exit_code: Option<i32>,
) -> Result<()> {
    let process = processes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?;

    let status_str = match status {
        ProcessStatus::Running => "Running",
        ProcessStatus::Completed => "Completed",
        ProcessStatus::Failed => "Failed",
        ProcessStatus::Timeout => "Timeout",
        ProcessStatus::Killed => "Killed",
    };

    let mut active: processes::ActiveModel = process.into();
    active.status = Set(status_str.to_string());
    active.exit_code = Set(exit_code);
    active.finished_at = Set(Some(Utc::now().to_rfc3339()));

    active.update(db).await?;
    Ok(())
}

pub async fn update_process_output_stats(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    stats: OutputStats,
) -> Result<()> {
    let process = processes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?;

    let mut active: processes::ActiveModel = process.into();
    active.stdout_size = Set(stats.stdout_size);
    active.stderr_size = Set(stats.stderr_size);
    active.stdout_lines = Set(stats.stdout_lines);
    active.stderr_lines = Set(stats.stderr_lines);

    active.update(db).await?;
    Ok(())
}