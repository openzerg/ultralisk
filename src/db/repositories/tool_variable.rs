use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::tool_variables;
use crate::core::types::tool::ToolVariableData;
use crate::core::interfaces::storage::{NewToolVariable, PartialToolVariable};
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub async fn get_tool_variable(
    db: &sea_orm::DatabaseConnection,
    tool_name: &str,
    variable_name: &str,
) -> Result<Option<ToolVariableData>> {
    let result = tool_variables::Entity::find()
        .filter(tool_variables::Column::ToolName.eq(tool_name))
        .filter(tool_variables::Column::VariableName.eq(variable_name))
        .one(db)
        .await?;

    Ok(result.map(|r| ToolVariableData {
        id: r.id,
        tool_name: r.tool_name,
        variable_name: r.variable_name,
        variable_value: r.variable_value,
        description: r.description,
        required: r.required,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn list_tool_variables(
    db: &sea_orm::DatabaseConnection,
    tool_name: &str,
) -> Result<Vec<ToolVariableData>> {
    let results = tool_variables::Entity::find()
        .filter(tool_variables::Column::ToolName.eq(tool_name))
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| ToolVariableData {
        id: r.id,
        tool_name: r.tool_name,
        variable_name: r.variable_name,
        variable_value: r.variable_value,
        description: r.description,
        required: r.required,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn save_tool_variable(
    db: &sea_orm::DatabaseConnection,
    data: NewToolVariable,
) -> Result<ToolVariableData> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let tool_name = data.tool_name;
    let variable_name = data.variable_name;
    let variable_value = data.variable_value;
    let description = data.description;
    let required = data.required;

    let variable = tool_variables::ActiveModel {
        id: Set(id.clone()),
        tool_name: Set(tool_name.clone()),
        variable_name: Set(variable_name.clone()),
        variable_value: Set(variable_value.clone()),
        description: Set(description.clone()),
        required: Set(required),
        created_at: Set(now.clone()),
        updated_at: Set(now.clone()),
    };

    Insert::one(variable).exec(db).await?;

    Ok(ToolVariableData {
        id,
        tool_name,
        variable_name,
        variable_value,
        description,
        required,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn update_tool_variable(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    data: PartialToolVariable,
) -> Result<()> {
    let variable = tool_variables::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Tool variable not found"))?;

    let mut active: tool_variables::ActiveModel = variable.into();

    if let Some(variable_value) = data.variable_value {
        active.variable_value = Set(Some(variable_value));
    }
    if let Some(description) = data.description {
        active.description = Set(Some(description));
    }
    if let Some(required) = data.required {
        active.required = Set(required);
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    active.update(db).await?;
    Ok(())
}

pub async fn delete_tool_variable(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    tool_variables::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

pub async fn delete_tool_variables_by_tool(
    db: &sea_orm::DatabaseConnection,
    tool_name: &str,
) -> Result<()> {
    tool_variables::Entity::delete_many()
        .filter(tool_variables::Column::ToolName.eq(tool_name))
        .exec(db)
        .await?;
    Ok(())
}