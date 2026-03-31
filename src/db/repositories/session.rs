use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait};
use crate::db::schema::sessions;
use crate::core::{Session, CreateSessionData, UpdateSessionData, SessionFilter, SessionState, AgentMode};
use anyhow::Result;
use chrono::Utc;

pub async fn create_session(
    db: &sea_orm::DatabaseConnection,
    data: CreateSessionData,
) -> Result<Session> {
    let now = Utc::now().to_rfc3339();
    let id = data.id.clone();
    let name = data.name.clone();
    let purpose = data.purpose.clone().unwrap_or_else(|| "Default".to_string());
    let agent_mode = data.agent.clone().unwrap_or_default();
    let agent = string_to_agent_mode(&agent_mode_to_string(&agent_mode));
    let provider_name = data.provider_name.clone();
    let system_prompt = data.system_prompt.clone().unwrap_or_default();
    let parent_id = data.parent_id.clone();

    let session = sessions::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        purpose: Set(purpose.clone()),
        state: Set("Idle".to_string()),
        agent: Set(agent_mode_to_string(&agent_mode)),
        provider_name: Set(provider_name.clone()),
        created_at: Set(now.clone()),
        started_at: Set(None),
        finished_at: Set(None),
        system_prompt: Set(system_prompt.clone()),
        parent_id: Set(parent_id.clone()),
        metadata: Set(None),
        input_tokens: Set(0),
        output_tokens: Set(0),
        has_compacted_history: Set(false),
        compacted_message_count: Set(0),
    };

    sessions::Entity::insert(session)
        .exec_without_returning(db)
        .await?;

    Ok(Session {
        id,
        name,
        purpose,
        state: SessionState::Idle,
        agent,
        provider_name,
        created_at: now,
        started_at: None,
        finished_at: None,
        system_prompt,
        parent_id,
        child_ids: vec![],
        input_tokens: 0,
        output_tokens: 0,
        has_compacted_history: false,
        compacted_message_count: 0,
    })
}

pub async fn get_session(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<Option<Session>> {
    let result = sessions::Entity::find_by_id(id)
        .one(db)
        .await?;

    Ok(result.map(|r| Session {
        id: r.id,
        name: r.name,
        purpose: r.purpose,
        state: string_to_session_state(&r.state),
        agent: string_to_agent_mode(&r.agent),
        provider_name: r.provider_name,
        created_at: r.created_at,
        started_at: r.started_at,
        finished_at: r.finished_at,
        system_prompt: r.system_prompt,
        parent_id: r.parent_id,
        child_ids: vec![],
        input_tokens: r.input_tokens,
        output_tokens: r.output_tokens,
        has_compacted_history: r.has_compacted_history,
        compacted_message_count: r.compacted_message_count,
    }))
}

pub async fn list_sessions(
    db: &sea_orm::DatabaseConnection,
    filter: Option<SessionFilter>,
) -> Result<Vec<Session>> {
    let mut query = sessions::Entity::find();

    if let Some(f) = filter {
        if let Some(state) = f.state {
            query = query.filter(sessions::Column::State.eq(session_state_to_string(&state)));
        }
        if let Some(purpose) = f.purpose {
            query = query.filter(sessions::Column::Purpose.eq(purpose));
        }
        if let Some(parent_id) = f.parent_id {
            query = query.filter(sessions::Column::ParentId.eq(parent_id));
        }
    }

    let results = query.all(db).await?;

    Ok(results.into_iter().map(|r| Session {
        id: r.id,
        name: r.name,
        purpose: r.purpose,
        state: string_to_session_state(&r.state),
        agent: string_to_agent_mode(&r.agent),
        provider_name: r.provider_name,
        created_at: r.created_at,
        started_at: r.started_at,
        finished_at: r.finished_at,
        system_prompt: r.system_prompt,
        parent_id: r.parent_id,
        child_ids: vec![],
        input_tokens: r.input_tokens,
        output_tokens: r.output_tokens,
        has_compacted_history: r.has_compacted_history,
        compacted_message_count: r.compacted_message_count,
    }).collect())
}

pub async fn update_session(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    data: UpdateSessionData,
) -> Result<()> {
    let session = sessions::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

    let mut active: sessions::ActiveModel = session.into();

    if let Some(name) = data.name {
        active.name = Set(name);
    }
    if let Some(state) = data.state {
        active.state = Set(session_state_to_string(&state));
    }
    if let Some(agent) = data.agent {
        active.agent = Set(agent_mode_to_string(&agent));
    }
    if let Some(provider_name) = data.provider_name {
        active.provider_name = Set(Some(provider_name));
    }
    if let Some(started_at) = data.started_at {
        active.started_at = Set(Some(started_at));
    }
    if let Some(finished_at) = data.finished_at {
        active.finished_at = Set(Some(finished_at));
    }
    if let Some(input_tokens) = data.input_tokens {
        active.input_tokens = Set(input_tokens);
    }
    if let Some(output_tokens) = data.output_tokens {
        active.output_tokens = Set(output_tokens);
    }
    if let Some(has_compacted_history) = data.has_compacted_history {
        active.has_compacted_history = Set(has_compacted_history);
    }
    if let Some(compacted_message_count) = data.compacted_message_count {
        active.compacted_message_count = Set(compacted_message_count);
    }

    active.update(db).await?;
    Ok(())
}

pub async fn delete_session(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    sessions::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

fn session_state_to_string(state: &SessionState) -> String {
    match state {
        SessionState::Idle => "Idle",
        SessionState::Running => "Running",
        SessionState::Done => "Done",
    }.to_string()
}

fn string_to_session_state(s: &str) -> SessionState {
    match s {
        "Idle" => SessionState::Idle,
        "Running" => SessionState::Running,
        "Done" => SessionState::Done,
        _ => SessionState::Idle,
    }
}

fn agent_mode_to_string(mode: &AgentMode) -> String {
    match mode {
        AgentMode::Plan => "plan",
        AgentMode::Build => "build",
    }.to_string()
}

fn string_to_agent_mode(s: &str) -> AgentMode {
    match s {
        "plan" => AgentMode::Plan,
        _ => AgentMode::Build,
    }
}