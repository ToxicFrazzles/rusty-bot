//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub blacklisted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::member::Entity")]
    Member,
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef{
        super::member::Relation::Guild.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::member::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}