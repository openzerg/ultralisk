use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::timers;
use crate::core::{Timer, CreateTimerData, UpdateTimerData, TimerFilter, TimerType, TimerStatus};
use anyhow::Result;

pub async fn get_timer(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<Option<Timer>> {
    let result = timers::Entity::find_by_id(id)
        .one(db)
        .await?;

    Ok(result.map(|r| map_timer(r)))
}

pub async fn get_timer_by_name(
    db: &sea_orm::DatabaseConnection,
    name: &str,
) -> Result<Option<Timer>> {
    let result = timers::Entity::find()
        .filter(timers::Column::Name.eq(name))
        .one(db)
        .await?;

    Ok(result.map(|r| map_timer(r)))
}

pub async fn list_timers(
    db: &sea_orm::DatabaseConnection,
    filter: Option<TimerFilter>,
) -> Result<Vec<Timer>> {
    let mut query = timers::Entity::find();

    if let Some(f) = filter {
        if let Some(session_id) = f.session_id {
            query = query.filter(timers::Column::SessionId.eq(session_id));
        }
        if let Some(status) = f.status {
            let status_str = match status {
                TimerStatus::Active => "active",
                TimerStatus::Disabled => "disabled",
                TimerStatus::Completed => "completed",
            };
            query = query.filter(timers::Column::Status.eq(status_str));
        }
    }

    let results = query.all(db).await?;

    Ok(results.into_iter().map(|r| map_timer(r)).collect())
}

pub async fn create_timer(
    db: &sea_orm::DatabaseConnection,
    data: CreateTimerData,
) -> Result<Timer> {
    let id = data.id;
    let name = data.name;
    let description = data.description;
    let message_template = data.message_template;
    let timer_type = data.timer_type;
    let timer_spec = data.timer_spec;
    let session_id = data.session_id;
    let max_runs = data.max_runs.unwrap_or(0);
    let created_at = chrono::Utc::now().to_rfc3339();

    let timer = timers::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        description: Set(description.clone()),
        message_template: Set(message_template.clone()),
        timer_type: Set(timer_type_to_string(&timer_type)),
        timer_spec: Set(timer_spec.clone()),
        status: Set("active".to_string()),
        session_id: Set(session_id.clone()),
        max_runs: Set(max_runs),
        run_count: Set(0),
        last_run_at: Set(None),
        next_run_at: Set(None),
        created_at: Set(created_at.clone()),
    };

    Insert::one(timer).exec(db).await?;

    Ok(Timer {
        id,
        name,
        description,
        message_template,
        timer_type,
        timer_spec,
        status: TimerStatus::Active,
        session_id,
        max_runs,
        run_count: 0,
        last_run_at: None,
        next_run_at: None,
        created_at,
    })
}

pub async fn update_timer(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    data: UpdateTimerData,
) -> Result<()> {
    let timer = timers::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Timer not found"))?;

    let mut active: timers::ActiveModel = timer.into();

    if let Some(status) = data.status {
        active.status = Set(timer_status_to_string(&status));
    }
    if let Some(run_count) = data.run_count {
        active.run_count = Set(run_count);
    }
    if let Some(last_run_at) = data.last_run_at {
        active.last_run_at = Set(Some(last_run_at));
    }
    if let Some(next_run_at) = data.next_run_at {
        active.next_run_at = Set(Some(next_run_at));
    }

    active.update(db).await?;
    Ok(())
}

pub async fn delete_timer(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    timers::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

fn map_timer(r: <timers::Entity as EntityTrait>::Model) -> Timer {
    Timer {
        id: r.id,
        name: r.name,
        description: r.description,
        message_template: r.message_template,
        timer_type: string_to_timer_type(&r.timer_type),
        timer_spec: r.timer_spec,
        status: string_to_timer_status(&r.status),
        session_id: r.session_id,
        max_runs: r.max_runs,
        run_count: r.run_count,
        last_run_at: r.last_run_at,
        next_run_at: r.next_run_at,
        created_at: r.created_at,
    }
}

fn timer_type_to_string(t: &TimerType) -> String {
    match t {
        TimerType::Calendar => "calendar",
        TimerType::Active => "active",
        TimerType::UnitActive => "unit-active",
    }.to_string()
}

fn string_to_timer_type(s: &str) -> TimerType {
    match s {
        "calendar" => TimerType::Calendar,
        "active" => TimerType::Active,
        "unit-active" => TimerType::UnitActive,
        _ => TimerType::Active,
    }
}

fn timer_status_to_string(s: &TimerStatus) -> String {
    match s {
        TimerStatus::Active => "active",
        TimerStatus::Disabled => "disabled",
        TimerStatus::Completed => "completed",
    }.to_string()
}

fn string_to_timer_status(s: &str) -> TimerStatus {
    match s {
        "active" => TimerStatus::Active,
        "disabled" => TimerStatus::Disabled,
        "completed" => TimerStatus::Completed,
        _ => TimerStatus::Active,
    }
}