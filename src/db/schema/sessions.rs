use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub state: String,
    pub agent: String,
    pub provider_name: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub system_prompt: String,
    pub parent_id: Option<String>,
    pub metadata: Option<String>,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub has_compacted_history: bool,
    pub compacted_message_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
    #[sea_orm(has_many = "super::processes::Entity")]
    Processes,
    #[sea_orm(has_many = "super::todos::Entity")]
    Todos,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::processes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Processes.def()
    }
}

impl Related<super::todos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
