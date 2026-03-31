use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait, Insert};
use crate::db::schema::{skill_registries, skills};
use crate::core::interfaces::storage::{SkillRegistry, Skill, CreateRegistryData, CreateSkillData};
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub async fn get_registry(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<Option<SkillRegistry>> {
    let result = skill_registries::Entity::find_by_id(id)
        .one(db)
        .await?;

    Ok(result.map(|r| SkillRegistry {
        id: r.id,
        name: r.name,
        url: r.url,
        api_key: r.api_key,
        created_at: r.created_at,
    }))
}

pub async fn get_registry_by_name(
    db: &sea_orm::DatabaseConnection,
    name: &str,
) -> Result<Option<SkillRegistry>> {
    let result = skill_registries::Entity::find()
        .filter(skill_registries::Column::Name.eq(name))
        .one(db)
        .await?;

    Ok(result.map(|r| SkillRegistry {
        id: r.id,
        name: r.name,
        url: r.url,
        api_key: r.api_key,
        created_at: r.created_at,
    }))
}

pub async fn list_registries(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<SkillRegistry>> {
    let results = skill_registries::Entity::find()
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| SkillRegistry {
        id: r.id,
        name: r.name,
        url: r.url,
        api_key: r.api_key,
        created_at: r.created_at,
    }).collect())
}

pub async fn create_registry(
    db: &sea_orm::DatabaseConnection,
    data: CreateRegistryData,
) -> Result<SkillRegistry> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let name = data.name;
    let url = data.url;
    let api_key = data.api_key;

    let registry = skill_registries::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        url: Set(url.clone()),
        api_key: Set(api_key.clone()),
        created_at: Set(now.clone()),
    };

    Insert::one(registry).exec(db).await?;

    Ok(SkillRegistry {
        id,
        name,
        url,
        api_key,
        created_at: now,
    })
}

pub async fn delete_registry(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    skill_registries::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}

pub async fn get_skill(
    db: &sea_orm::DatabaseConnection,
    full_name: &str,
) -> Result<Option<Skill>> {
    let result = skills::Entity::find()
        .filter(skills::Column::FullName.eq(full_name))
        .one(db)
        .await?;

    Ok(result.map(|r| Skill {
        id: r.id,
        registry_id: r.registry_id,
        name: r.name,
        full_name: r.full_name,
        description: r.description,
        folder_path: r.folder_path,
        installed_at: r.installed_at,
    }))
}

pub async fn list_skills(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<Skill>> {
    let results = skills::Entity::find()
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| Skill {
        id: r.id,
        registry_id: r.registry_id,
        name: r.name,
        full_name: r.full_name,
        description: r.description,
        folder_path: r.folder_path,
        installed_at: r.installed_at,
    }).collect())
}

pub async fn create_skill(
    db: &sea_orm::DatabaseConnection,
    data: CreateSkillData,
) -> Result<Skill> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let registry_id = data.registry_id;
    let name = data.name;
    let full_name = data.full_name;
    let description = data.description;
    let folder_path = data.folder_path;

    let skill = skills::ActiveModel {
        id: Set(id.clone()),
        registry_id: Set(registry_id.clone()),
        name: Set(name.clone()),
        full_name: Set(full_name.clone()),
        description: Set(description.clone()),
        folder_path: Set(folder_path.clone()),
        installed_at: Set(now.clone()),
    };

    Insert::one(skill).exec(db).await?;

    Ok(Skill {
        id,
        registry_id,
        name,
        full_name,
        description,
        folder_path,
        installed_at: now,
    })
}

pub async fn delete_skill(
    db: &sea_orm::DatabaseConnection,
    full_name: &str,
) -> Result<()> {
    skills::Entity::delete_many()
        .filter(skills::Column::FullName.eq(full_name))
        .exec(db)
        .await?;
    Ok(())
}