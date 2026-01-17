use crate::sea_orm_active_enums::ScopeLevel;
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user_id: u64,
    pub role_id: u64,
    pub scope_level: ScopeLevel,
    pub foundation_id: Option<u64>,
    pub unit_id: Option<u64>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::foundations::Entity",
        from = "Column::FoundationId",
        to = "super::foundations::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Foundations,

    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    Users,

    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::Id"
    )]
    Roles,
    #[sea_orm(
        belongs_to = "super::units::Entity",
        from = "Column::UnitId",
        to = "super::units::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Units,
}

impl Related<super::foundations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foundations.def()
    }
}

impl Related<super::units::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Units.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
