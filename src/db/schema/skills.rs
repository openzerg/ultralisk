use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "skills")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub registry_id: String,
    pub name: String,
    #[sea_orm(unique)]
    pub full_name: String,
    pub description: String,
    pub folder_path: String,
    pub installed_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::skill_registries::Entity",
        from = "Column::RegistryId",
        to = "super::skill_registries::Column::Id",
        on_delete = "Cascade"
    )]
    Registry,
}

impl Related<super::skill_registries::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Registry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
