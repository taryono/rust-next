// backend/entity/src/users.rs
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub is_verified: Option<i8>,
    pub is_active: Option<i8>,
    pub foundation_id: i64,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(column_type = "Text", nullable)]
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::notifications::Entity")]
    Notifications,

    #[sea_orm(has_many = "super::role_users::Entity")]
    RoleUsers,
    #[sea_orm(has_many = "super::user_permissions::Entity")]
    UserPermissions,
}

impl Related<super::notifications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notifications.def()
    }
}

// users ↔ roles (via role_users)
impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_users::Relation::Roles.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::role_users::Relation::Users.def().rev())
    }
}

// users ↔ permissions (direct)
impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_permissions::Relation::Permissions.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_permissions::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

// ✅ Implementasi trait SoftDelete
impl crate::traits::soft_delete::SoftDelete for Entity {
    fn deleted_at_col() -> Column {
        Column::DeletedAt
    }
}
