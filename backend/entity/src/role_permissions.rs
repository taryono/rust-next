// backend/entity/src/role_permissions.rs
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role_permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub role_id: i64,
    pub permission_id: i64,
    pub foundation_id: Option<i64>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Roles,

    #[sea_orm(
        belongs_to = "super::permissions::Entity",
        from = "Column::PermissionId",
        to = "super::permissions::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Permissions,

    #[sea_orm(
        belongs_to = "super::foundations::Entity",
        from = "Column::FoundationId",
        to = "super::foundations::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Foundations,
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permissions.def()
    }
}

impl Related<super::foundations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foundations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
