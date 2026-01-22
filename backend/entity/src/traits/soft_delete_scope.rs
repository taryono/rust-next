// src/traits/soft_delete_scope.rs
use sea_orm::{sea_query::IntoCondition, Condition, EntityTrait, QueryFilter, Select};

pub trait SoftDeleteScope: EntityTrait {
    fn deleted_at_col() -> Self::Column;

    /// Override find() untuk auto-exclude soft deleted
    fn find_active() -> Select<Self> {
        Self::find().filter(Condition::all().add(Self::deleted_at_col().is_null()))
    }
}

// Implementasi di entity
// impl SoftDeleteScope for users::Entity {
//     fn deleted_at_col() -> users::Column {
//         users::Column::DeletedAt
//     }
// }

// Penggunaan
// let users = users::Entity::find_active().all(db).await?;

// impl SoftDeleteScope for roles::Entity {
//     fn deleted_at_col() -> roles::Column {
//         roles::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for role_users::Entity {
//     fn deleted_at_col() -> role_users::Column {
//         role_users::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for notifications::Entity {
//     fn deleted_at_col() -> notifications::Column {
//         notifications::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for posts::Entity {
//     fn deleted_at_col() -> posts::Column {
//         posts::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for comments::Entity {
//     fn deleted_at_col() -> comments::Column {
//         comments::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for tags::Entity {
//     fn deleted_at_col() -> tags::Column {
//         tags::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for post_tags::Entity {
//     fn deleted_at_col() -> post_tags::Column {
//         post_tags::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for comment_tags::Entity {
//     fn deleted_at_col() -> comment_tags::Column {
//         comment_tags::Column::DeletedAt
//     }
// }

// impl SoftDeleteScope for post_comments::Entity {
//     fn deleted_at_col() -> post_comments::Column {
//         post_comments::Column::DeletedAt
//     } // Penggunaan
// }
