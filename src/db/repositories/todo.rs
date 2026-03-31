use sea_orm::{EntityTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::todos;
use crate::core::{Todo, TodoStatus, TodoPriority};
use crate::core::interfaces::storage::TodoUpdateData;
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub async fn create_todo(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
    content: &str,
    priority: Option<&str>,
) -> Result<Todo> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let session_id = session_id.to_string();
    let content = content.to_string();
    let priority_str = priority.unwrap_or("medium");

    let todo = todos::ActiveModel {
        id: Set(id.clone()),
        session_id: Set(session_id.clone()),
        content: Set(content.clone()),
        status: Set("pending".to_string()),
        priority: Set(priority_str.to_string()),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
    };

    Insert::one(todo).exec(db).await?;

    Ok(Todo {
        id,
        session_id,
        content,
        status: TodoStatus::Pending,
        priority: string_to_todo_priority(priority_str),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_todos(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
) -> Result<Vec<Todo>> {
    let results = todos::Entity::find()
        .filter(todos::Column::SessionId.eq(session_id))
        .order_by_asc(todos::Column::CreatedAt)
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| Todo {
        id: r.id,
        session_id: r.session_id,
        content: r.content,
        status: string_to_todo_status(&r.status),
        priority: string_to_todo_priority(&r.priority),
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn update_todo(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    data: TodoUpdateData,
) -> Result<()> {
    let todo = todos::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Todo not found"))?;

    let mut active: todos::ActiveModel = todo.into();

    if let Some(status) = data.status {
        active.status = Set(todo_status_to_string(&status));
    }
    if let Some(content) = data.content {
        active.content = Set(content);
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    active.update(db).await?;
    Ok(())
}

pub async fn delete_todo(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    todos::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

fn todo_status_to_string(status: &TodoStatus) -> String {
    match status {
        TodoStatus::Pending => "pending",
        TodoStatus::InProgress => "in_progress",
        TodoStatus::Completed => "completed",
    }.to_string()
}

fn string_to_todo_status(s: &str) -> TodoStatus {
    match s {
        "pending" => TodoStatus::Pending,
        "in_progress" => TodoStatus::InProgress,
        "completed" => TodoStatus::Completed,
        _ => TodoStatus::Pending,
    }
}

fn string_to_todo_priority(s: &str) -> TodoPriority {
    match s {
        "high" => TodoPriority::High,
        "medium" => TodoPriority::Medium,
        "low" => TodoPriority::Low,
        _ => TodoPriority::Medium,
    }
}