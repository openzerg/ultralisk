use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait, QueryOrder};
use crate::db::schema::file_reads;
use crate::core::interfaces::storage::{FileReadData, FileReadResult};
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub async fn save_file_read(
    db: &sea_orm::DatabaseConnection,
    data: FileReadData,
) -> Result<()> {
    let read_at = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let file_read = file_reads::ActiveModel {
        id: Set(id),
        session_id: Set(data.session_id),
        file_path: Set(data.file_path),
        mtime_ms: Set(data.mtime_ms),
        read_at: Set(read_at),
    };

    file_read.insert(db).await?;
    Ok(())
}

pub async fn get_file_read(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
    file_path: &str,
) -> Result<Option<FileReadResult>> {
    let result = file_reads::Entity::find()
        .filter(file_reads::Column::SessionId.eq(session_id))
        .filter(file_reads::Column::FilePath.eq(file_path))
        .one(db)
        .await?;

    Ok(result.map(|r| FileReadResult {
        file_path: r.file_path,
        mtime_ms: r.mtime_ms,
        read_at: r.read_at,
    }))
}

pub async fn list_file_reads(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
) -> Result<Vec<FileReadResult>> {
    let results = file_reads::Entity::find()
        .filter(file_reads::Column::SessionId.eq(session_id))
        .order_by_desc(file_reads::Column::ReadAt)
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| FileReadResult {
        file_path: r.file_path,
        mtime_ms: r.mtime_ms,
        read_at: r.read_at,
    }).collect())
}

pub async fn clear_file_reads(
    db: &sea_orm::DatabaseConnection,
    session_id: &str,
) -> Result<()> {
    file_reads::Entity::delete_many()
        .filter(file_reads::Column::SessionId.eq(session_id))
        .exec(db)
        .await?;
    Ok(())
}