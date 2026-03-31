use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tool_variables")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub tool_name: String,
    pub variable_name: String,
    pub variable_value: Option<String>,
    pub description: Option<String>,
    pub required: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
