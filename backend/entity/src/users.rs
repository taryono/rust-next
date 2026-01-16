use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub is_verified: Option<i8>,
    pub created_at: Option<DateTimeUtc>,
    pub updated_at: Option<DateTimeUtc>,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::notifications::Entity")]
    Notifications,

    #[sea_orm(has_many = "super::role_users::Entity")]
    RoleUsers,
}

impl Related<super::notifications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notifications.def()
    }
}

impl Related<super::role_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Self::Notifications => Entity::has_many(super::notifications::Entity).into(),
//             Self::RoleUsers => Entity::has_many(super::role_users::Entity).into(),
//         }
//     }
// }
