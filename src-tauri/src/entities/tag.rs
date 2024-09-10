//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::game_tag::Entity")]
    GameTag,
}

impl Related<super::game_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GameTag.def()
    }
}

impl Related<super::game::Entity> for Entity {
    fn to() -> RelationDef {
        super::game_tag::Relation::Game.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::game_tag::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}