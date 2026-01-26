prefix it with an underscore: `_filters` ini artinya variable tidak digunakan


error[E0599]: no method named `contains` found for enum `entity::roles::Column` in the current scope
   --> src/modules/users/service.rs:235:50
    |
235 |                         .add(roles::Column::Name.contains(search_term))
    |                                                  ^^^^^^^^ method not found in `entity::roles::Column`
    |
    
use sea_orm::ColumnTrait; // ✅ Import ini!

// Kemudian gunakan salah satu method ini:
.add(roles::Column::Name.contains(search_term))  // LIKE '%term%'
.add(roles::Column::Name.starts_with(search_term))  // LIKE 'term%'
.add(roles::Column::Name.ends_with(search_term))  // LIKE '%term'
.add(roles::Column::Name.like(format!("%{}%", search_term)))  // Custom LIKE

/// Delete user function ini akan error 
pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
    let user = User::find_by_id(id).one(self.conn()).await?;
    let mut model = user.into_active_model();
    model.deleted_at = Set(chrono::Utc::now());
    model
        .update(self.conn())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()));
    Ok(())
}

error[E0599]: the method `into_active_model` exists for enum `std::option::Option<entity::users::Model>`, but its trait bounds were not satisfied
   --> src/modules/users/repository.rs:165:30
    |
165 |         let mut model = user.into_active_model();
    |                              ^^^^^^^^^^^^^^^^^ method cannot be called on `std::option::Option<entity::users::Model>` due to unsatisfied trait bounds
    |
    = note: the following trait bounds were not satisfied:
            `std::option::Option<entity::users::Model>: ActiveModelTrait`
            which is required by `std::option::Option<entity::users::Model>: IntoActiveModel<std::option::Option<entity::users::Model>>`
            `&std::option::Option<entity::users::Model>: ActiveModelTrait`
            which is required by `&std::option::Option<entity::users::Model>: IntoActiveModel<&std::option::Option<entity::users::Model>>`
            `&mut std::option::Option<entity::users::Model>: ActiveModelTrait`
            which is required by `&mut std::option::Option<entity::users::Model>: IntoActiveModel<&mut std::option::Option<entity::users::Model>>`
note: the method `into_active_model` exists on the type `entity::users::Model`

Jawaban 

Error ini terjadi karena User::find_by_id(id).one(self.conn()).await? mengembalikan Option<users::Model>, bukan langsung users::Model.

❌ Masalah:
let user = User::find_by_id(id).one(self.conn()).await?;
//         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//         Ini return Option<users::Model>, bukan users::Model

let mut model = user.into_active_model();
//              ^^^^ user adalah Option, bukan Model

✅ Solusi:
Cara 1: Unwrap Option dengan ok_or_else

/// Soft delete user
pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
    let user = User::find_by_id(id)
        .one(self.conn())
        .await?
        .ok_or_else(|| AppError::not_found("User not found".to_string()))?;  // ✅ Handle Option
    
    let mut model = user.into_active_model();
    model.deleted_at = Set(Some(chrono::Utc::now()));
    
    model
        .update(self.conn())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;  // ✅ Tambah ?
    
    Ok(())
}

Cara 2: Lebih Robust dengan Validasi

/// Soft delete user
pub async fn soft_delete(
    &self,
    db: &DatabaseConnection,
    id: i64,
) -> Result<(), AppError> {
    let user = User::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

    // ✅ Validasi: cek apakah sudah di-delete
    if user.deleted_at.is_some() {
        return Err(AppError::bad_request("User already deleted".to_string()));
    }

    let mut active_user = user.into_active_model();
    active_user.deleted_at = Set(Some(chrono::Utc::now()));
    active_user.updated_at = Set(chrono::Utc::now());

    active_user.update(db).await?;

    Ok(())
}

Cara 3: Pakai find_not_deleted Helper (Recommended)
Jika Anda sudah punya trait SoftDelete:

use entity::traits::soft_delete::SoftDelete;

/// Soft delete user
pub async fn soft_delete(
    &self,
    db: &DatabaseConnection,
    id: i64,
) -> Result<(), AppError> {
    // ✅ Gunakan find_not_deleted untuk validasi
    let user = User::find_not_deleted()
        .filter(users::Column::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found or already deleted".to_string()))?;

    let mut active_user = user.into_active_model();
    active_user.deleted_at = Set(Some(chrono::Utc::now()));
    active_user.updated_at = Set(chrono::Utc::now());

    active_user.update(db).await?;

    Ok(())
}

🔧 Fix untuk Method restore Juga:

/// Restore user
pub async fn restore(
    &self,
    db: &DatabaseConnection,
    id: i64,
) -> Result<users::Model, AppError> {
    let user = User::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("User not found".to_string()))?;  // ✅ Handle Option

    // ✅ Validasi: cek apakah memang sudah di-delete
    if user.deleted_at.is_none() {
        return Err(AppError::bad_request("User is not deleted".to_string()));
    }

    let mut active_user = user.into_active_model();
    active_user.deleted_at = Set(None);  // ✅ Set ke None
    active_user.updated_at = Set(chrono::Utc::now());

    let restored = active_user.update(db).await?;

    Ok(restored)
}

📋 Pattern Umum untuk Handle Option:

// Pattern 1: ok_or_else (paling umum)
let user = User::find_by_id(id)
    .one(db)
    .await?
    .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

// Pattern 2: match
let user = match User::find_by_id(id).one(db).await? {
    Some(u) => u,
    None => return Err(AppError::not_found("User not found".to_string())),
};

// Pattern 3: if let
let user = if let Some(u) = User::find_by_id(id).one(db).await? {
    u
} else {
    return Err(AppError::not_found("User not found".to_string()));
};

✅ Complete Fixed Repository Methods:



use entity::{
    users::{self, Entity as User},
    traits::soft_delete::SoftDelete,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, 
    ColumnTrait, Set, ActiveModelTrait, IntoActiveModel,
};

impl UserRepository {
    /// Soft delete user
    pub async fn soft_delete(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError> {
        let user = User::find_not_deleted()
            .filter(users::Column::Id.eq(id))
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        let mut active_user = user.into_active_model();
        active_user.deleted_at = Set(Some(chrono::Utc::now()));
        active_user.updated_at = Set(chrono::Utc::now());

        active_user.update(db).await?;

        Ok(())
    }

    /// Restore soft-deleted user
    pub async fn restore(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<users::Model, AppError> {
        let user = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        if user.deleted_at.is_none() {
            return Err(AppError::bad_request("User is not deleted".to_string()));
        }

        let mut active_user = user.into_active_model();
        active_user.deleted_at = Set(None);
        active_user.updated_at = Set(chrono::Utc::now());

        let restored = active_user.update(db).await?;

        Ok(restored)
    }

    /// Permanently delete user
    pub async fn delete(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<(), AppError> {
        let user = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        user.into_active_model().delete(db).await?;

        Ok(())
    }
}


dari query ini 
/// ini query jika mengambil permissions dari table role permission
 let rows = role_permissions::Entity::find()
            .join(
                sea_orm::JoinType::InnerJoin,
                role_permissions::Relation::Permissions.def(),
            )
            .join_rev(
                sea_orm::JoinType::InnerJoin,
                role_users::Relation::Roles.def(),
            )
            .filter(role_users::Column::UserId.eq(user_id))
            .select_only()
            .column(permissions::Column::Code)
            // Gunakan .into_tuple() atau struct sementara untuk mengambil string saja
            .all(db)
            .await?;

muncul  error ini 
error[E0609]: no field `code` on type `entity::role_permissions::Model`
   --> src/modules/permissions/service.rs:223:39
    |
223 |         Ok(rows.into_iter().map(|p| p.code).collect())
    |                                       ^^^^ unknown field
    |
    = note: available fields are: `id`, `role_id`, `permission_id`, `foundation_id`, `created_at` ... and 2 others


    /// Get permissions via roles
    // Di dalam get_permissions_from_roles
    async fn get_permissions_from_roles(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<String>, AppError> {
        let rows = permissions::Entity::find()
            // Gunakan join_rev untuk menyambung KE role_permissions (sebagai tabel pivot)
            .join_rev(
                sea_orm::JoinType::InnerJoin,
                role_permissions::Relation::Permissions.def(),
            )
            // Join lagi ke role_users menggunakan field RoleId yang ada di role_permissions
            .join_rev(
                sea_orm::JoinType::InnerJoin,
                role_users::Relation::Roles.def(),
            )
            .filter(role_users::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(rows.into_iter().map(|p| p.code).collect())
    }

    muncul error ini 

    "Database error: Query Error: error returned from database: 1054 (42S22): Unknown column 'roles.id' in 'on clause'"