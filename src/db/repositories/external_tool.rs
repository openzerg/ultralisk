use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::external_tools;
use crate::core::types::tool::ExternalToolData;
use crate::core::interfaces::storage::{NewExternalTool, PartialExternalTool};
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub async fn get_external_tool(
    db: &sea_orm::DatabaseConnection,
    name: &str,
) -> Result<Option<ExternalToolData>> {
    let result = external_tools::Entity::find()
        .filter(external_tools::Column::Name.eq(name))
        .one(db)
        .await?;

    Ok(result.map(|r| ExternalToolData {
        id: r.id,
        name: r.name,
        description: r.description,
        parameters_json: r.parameters_json,
        config_json: r.config_json,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn list_external_tools(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<ExternalToolData>> {
    let results = external_tools::Entity::find()
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| ExternalToolData {
        id: r.id,
        name: r.name,
        description: r.description,
        parameters_json: r.parameters_json,
        config_json: r.config_json,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn save_external_tool(
    db: &sea_orm::DatabaseConnection,
    data: NewExternalTool,
) -> Result<ExternalToolData> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let tool = external_tools::ActiveModel {
        id: Set(id.clone()),
        name: Set(data.name.clone()),
        description: Set(data.description.clone()),
        parameters_json: Set(data.parameters_json.clone()),
        config_json: Set(data.config_json.clone()),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
    };

    Insert::one(tool).exec(db).await?;

    Ok(ExternalToolData {
        id,
        name: data.name,
        description: data.description,
        parameters_json: data.parameters_json,
        config_json: data.config_json,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn update_external_tool(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    data: PartialExternalTool,
) -> Result<()> {
    let tool = external_tools::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("External tool not found"))?;

    let mut active: external_tools::ActiveModel = tool.into();

    if let Some(description) = data.description {
        active.description = Set(description);
    }
    if let Some(parameters_json) = data.parameters_json {
        active.parameters_json = Set(parameters_json);
    }
    if let Some(config_json) = data.config_json {
        active.config_json = Set(config_json);
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    active.update(db).await?;
    Ok(())
}

pub async fn delete_external_tool(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    external_tools::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

pub async fn delete_all_external_tools(
    db: &sea_orm::DatabaseConnection,
) -> Result<()> {
    external_tools::Entity::delete_many()
        .exec(db)
        .await?;
    Ok(())
}