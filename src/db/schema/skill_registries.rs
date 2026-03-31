use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "skill_registries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    #[sea_orm(unique)]
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::skills::Entity")]
    Skills,
}

impl Related<super::skills::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Skills.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
