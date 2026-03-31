use sea_orm::{EntityTrait, QueryFilter, Set, ActiveModelTrait, ColumnTrait};
use crate::db::schema::providers;
use crate::core::Provider;
use crate::core::interfaces::storage::{NewProvider, ProviderUpdate};
use anyhow::Result;
use uuid::Uuid;
use chrono::Utc;

pub async fn get_provider(
    db: &sea_orm::DatabaseConnection,
    name: &str,
) -> Result<Option<Provider>> {
    let result = providers::Entity::find()
        .filter(providers::Column::Name.eq(name))
        .one(db)
        .await?;

    Ok(result.map(|r| Provider {
        id: r.id,
        name: r.name,
        base_url: r.base_url,
        api_key: r.api_key,
        model: r.model,
        max_tokens: r.max_tokens,
        temperature: r.temperature,
        top_p: r.top_p,
        top_k: r.top_k,
        extra_params: r.extra_params,
        auto_compact_length: r.auto_compact_length,
        is_default: r.is_default,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn get_default_provider(
    db: &sea_orm::DatabaseConnection,
) -> Result<Option<Provider>> {
    let result = providers::Entity::find()
        .filter(providers::Column::IsDefault.eq(true))
        .one(db)
        .await?;

    Ok(result.map(|r| Provider {
        id: r.id,
        name: r.name,
        base_url: r.base_url,
        api_key: r.api_key,
        model: r.model,
        max_tokens: r.max_tokens,
        temperature: r.temperature,
        top_p: r.top_p,
        top_k: r.top_k,
        extra_params: r.extra_params,
        auto_compact_length: r.auto_compact_length,
        is_default: r.is_default,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

pub async fn list_providers(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<Provider>> {
    let results = providers::Entity::find()
        .all(db)
        .await?;

    Ok(results.into_iter().map(|r| Provider {
        id: r.id,
        name: r.name,
        base_url: r.base_url,
        api_key: r.api_key,
        model: r.model,
        max_tokens: r.max_tokens,
        temperature: r.temperature,
        top_p: r.top_p,
        top_k: r.top_k,
        extra_params: r.extra_params,
        auto_compact_length: r.auto_compact_length,
        is_default: r.is_default,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }).collect())
}

pub async fn save_provider(
    db: &sea_orm::DatabaseConnection,
    data: NewProvider,
) -> Result<Provider> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    
    let provider_id = id.clone();
    let provider_name = data.name.clone();
    let provider_base_url = data.base_url.clone();
    let provider_api_key = data.api_key.clone();
    let provider_model = data.model.clone();
    let provider_max_tokens = data.max_tokens;
    let provider_temperature = data.temperature;
    let provider_top_p = data.top_p;
    let provider_top_k = data.top_k;
    let provider_extra_params = data.extra_params.clone();
    let provider_auto_compact_length = data.auto_compact_length;
    let provider_is_default = data.is_default;
    let provider_created_at = now.clone();
    let provider_updated_at = now.clone();

    let provider = providers::ActiveModel {
        id: Set(id),
        name: Set(data.name),
        base_url: Set(data.base_url),
        api_key: Set(data.api_key),
        model: Set(data.model),
        max_tokens: Set(data.max_tokens),
        temperature: Set(data.temperature),
        top_p: Set(data.top_p),
        top_k: Set(data.top_k),
        extra_params: Set(data.extra_params),
        auto_compact_length: Set(data.auto_compact_length),
        is_default: Set(data.is_default),
        created_at: Set(now),
        updated_at: Set(provider_updated_at.clone()),
    };

    providers::Entity::insert(provider)
        .exec_without_returning(db)
        .await?;

    Ok(Provider {
        id: provider_id,
        name: provider_name,
        base_url: provider_base_url,
        api_key: provider_api_key,
        model: provider_model,
        max_tokens: provider_max_tokens,
        temperature: provider_temperature,
        top_p: provider_top_p,
        top_k: provider_top_k,
        extra_params: provider_extra_params,
        auto_compact_length: provider_auto_compact_length,
        is_default: provider_is_default,
        created_at: provider_created_at,
        updated_at: provider_updated_at,
    })
}

pub async fn update_provider(
    db: &sea_orm::DatabaseConnection,
    id: &str,
    data: ProviderUpdate,
) -> Result<()> {
    let provider = providers::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Provider not found"))?;

    let mut active: providers::ActiveModel = provider.into();

    if let Some(base_url) = data.base_url {
        active.base_url = Set(base_url);
    }
    if let Some(api_key) = data.api_key {
        active.api_key = Set(api_key);
    }
    if let Some(model) = data.model {
        active.model = Set(model);
    }
    if let Some(max_tokens) = data.max_tokens {
        active.max_tokens = Set(Some(max_tokens));
    }
    if let Some(temperature) = data.temperature {
        active.temperature = Set(Some(temperature));
    }
    if let Some(top_p) = data.top_p {
        active.top_p = Set(Some(top_p));
    }
    if let Some(top_k) = data.top_k {
        active.top_k = Set(Some(top_k));
    }
    if let Some(extra_params) = data.extra_params {
        active.extra_params = Set(Some(extra_params));
    }
    if let Some(auto_compact_length) = data.auto_compact_length {
        active.auto_compact_length = Set(Some(auto_compact_length));
    }
    if let Some(is_default) = data.is_default {
        active.is_default = Set(is_default);
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    active.update(db).await?;
    Ok(())
}

pub async fn delete_provider(
    db: &sea_orm::DatabaseConnection,
    id: &str,
) -> Result<()> {
    providers::Entity::delete_by_id(id)
        .exec(db)
        .await?;
    Ok(())
}