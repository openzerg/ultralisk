use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "processes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub command: String,
    pub cwd: String,
    pub status: String,
    pub exit_code: Option<i32>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub parent_session_id: Option<String>,
    pub unit_name: String,
    pub timeout_ms: i32,
    pub output_dir: String,
    pub stdout_size: i32,
    pub stderr_size: i32,
    pub stdout_lines: i32,
    pub stderr_lines: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sessions::Entity",
        from = "Column::ParentSessionId",
        to = "super::sessions::Column::Id",
        on_delete = "Cascade"
    )]
    Session,
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
