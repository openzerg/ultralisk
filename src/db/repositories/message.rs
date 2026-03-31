use sea_orm::{EntityTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::messages;
use crate::core::{Message, CreateMessageData, MessageFilter, MessageRole};
use anyhow::Result;
use chrono::Utc;

pub async fn save_message(
    db: &sea_orm::DatabaseConnection,
    data: CreateMessageData,
) -> Result<Message> {
    let now = Utc::now().to_rfc3339();

    let id = data.id;
    let session_id = data.session_id;
    let role = data.role.clone();
    let content = data.content;
    let tool_calls = data.tool_calls.clone();
    let tool_calls_json = data.tool_calls.map(|t| serde_json::to_string(&t).unwrap_or_default());
    let tool_name = data.tool_name;
    let tool_call_id = data.tool_call_id;
    let tool_success = data.tool_success;
    let metadata = data.metadata.clone();

    let message = messages::ActiveModel {
        id: Set(id.clone()),
        session_id: Set(session_id.clone()),
        role: Set(message_role_to_string(&role)),
        content: Set(content.clone()),
        timestamp: Set(now.clone()),
        tool_calls: Set(tool_calls_json.clone()),
        tool_name: Set(tool_name.clone()),
        tool_call_id: Set(tool_call_id.clone()),
        tool_success: Set(tool_success),
        metadata: Set(metadata.as_ref().map(|m| serde_json::to_string(m).unwrap_or_default())),
    };

    Insert::one(message).exec(db).await?;

    Ok(Message {
        id,
        session_id,
        role,
        content,
        timestamp: now,
        tool_calls,
        tool_calls_json,
        tool_name,
        tool_call_id,
        tool_success,
        metadata,
    })
}

pub async fn get_messages(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
    filter: Option<MessageFilter>,
) -> Result<Vec<Message>> {
    let mut query = messages::Entity::find()
        .filter(messages::Column::SessionId.eq(session_id))
        .order_by_asc(messages::Column::Timestamp);

    if let Some(f) = &filter {
        if let Some(role) = &f.role {
            query = query.filter(messages::Column::Role.eq(message_role_to_string(role)));
        }
    }

    let results = query.all(db).await?;

    Ok(results.into_iter().map(|r| Message {
        id: r.id,
        session_id: r.session_id,
        role: string_to_message_role(&r.role),
        content: r.content,
        timestamp: r.timestamp,
        tool_calls: r.tool_calls.as_ref().and_then(|t| serde_json::from_str(t).ok()),
        tool_calls_json: r.tool_calls,
        tool_name: r.tool_name,
        tool_call_id: r.tool_call_id,
        tool_success: r.tool_success,
        metadata: r.metadata.and_then(|m| serde_json::from_str(&m).ok()),
    }).collect())
}

pub async fn delete_message(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    messages::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

pub async fn delete_messages(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
) -> Result<()> {
    messages::Entity::delete_many()
        .filter(messages::Column::SessionId.eq(session_id))
        .exec(db)
        .await?;
    Ok(())
}

fn message_role_to_string(role: &MessageRole) -> String {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => "tool",
        MessageRole::Thinking => "thinking",
        MessageRole::Error => "error",
        MessageRole::Summary => "summary",
    }.to_string()
}

fn string_to_message_role(s: &str) -> MessageRole {
    match s {
        "system" => MessageRole::System,
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        "tool" => MessageRole::Tool,
        "thinking" => MessageRole::Thinking,
        "error" => MessageRole::Error,
        "summary" => MessageRole::Summary,
        _ => MessageRole::User,
    }
}