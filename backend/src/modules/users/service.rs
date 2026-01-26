// ============================================================================
// backend/src/modules/users/service.rs
// User Service - Business Logic Layer
// ============================================================================
use super::{
    dto::{ChangePasswordRequest, CreateUserRequest, UpdateUserRequest, UserResponse},
    repository::UserRepository,
};
use crate::{
    errors::AppError,
    utils::{
        pagination::{PaginatedResponse, PaginationParams},
        password,
    },
};
use entity::{
    role_users::{self, Entity as RoleUsers},
    roles::Entity as Roles,
    users::{self},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
};
use validator::Validate;

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    /// Create new service instance
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    /// Create new user with validation
    pub async fn create(&self, request: CreateUserRequest) -> Result<UserResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check duplicate email
        if let Some(_) = self.repository.find_by_email(&request.email).await? {
            return Err(AppError::conflict("Email already exists".to_string()));
        }

        // Check duplicate name in foundation
        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::conflict(
                "User with this name already exists in foundation".to_string(),
            ));
        }

        // Hash password
        let hashed_password = password::hash(&request.password)?;

        // Build entity
        let active_model = users::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            email: Set(request.email),
            password: Set(hashed_password),
            is_active: Set(Some(1)),
            is_verified: Set(Some(0)),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        Ok(UserResponse::from(created))
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i64) -> Result<UserResponse, AppError> {
        let user = self
            .repository
            .find_not_deleted(id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        Ok(UserResponse::from(user))
    }

    /// Get all users with pagination and filters
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<UserResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<UserResponse> = items.into_iter().map(UserResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update user information
    pub async fn update(
        &self,
        user_id: i64,
        update_data: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        // Validate request
        update_data
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let user = self
            .repository
            .find_not_deleted(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        let mut active_user: users::ActiveModel = user.into_active_model();

        // Update name if provided
        if let Some(name) = update_data.name {
            active_user.name = Set(name);
        }

        // Update email if provided and check uniqueness
        if let Some(email) = update_data.email {
            if let Some(existing_user) = self.repository.find_by_email(&email).await? {
                if existing_user.id != user_id {
                    return Err(AppError::conflict("Email already in use".to_string()));
                }
            }
            active_user.email = Set(email);
        }

        // Update is_active if provided
        if let Some(is_active) = update_data.is_active {
            active_user.is_active = Set(Some(is_active));
        }

        active_user.updated_at = Set(chrono::Utc::now());

        let updated_user = self.repository.update(user_id, active_user).await?;

        Ok(UserResponse::from(updated_user))
    }

    /// Change user password
    pub async fn change_password(
        &self,
        user_id: i64,
        password_data: ChangePasswordRequest,
    ) -> Result<UserResponse, AppError> {
        // Validate request
        password_data
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let user = self
            .repository
            .find_not_deleted(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // Verify old password
        if !password::verify(&password_data.old_password, &user.password)? {
            return Err(AppError::unauthorized("Incorrect old password".to_string()));
        }

        // Validate: new password must be different
        if password_data.old_password == password_data.new_password {
            return Err(AppError::bad_request(
                "New password must be different from old password".to_string(),
            ));
        }

        // Hash new password
        let hashed_password = password::hash(&password_data.new_password)?;

        let mut active_user: users::ActiveModel = user.into_active_model();
        active_user.password = Set(hashed_password);
        active_user.updated_at = Set(chrono::Utc::now());

        let updated_user = self.repository.update(user_id, active_user).await?;

        Ok(UserResponse::from(updated_user))
    }

    /// Get active user for a foundation
    pub async fn get_active(&self, foundation_id: i64) -> Result<Option<UserResponse>, AppError> {
        let user = self.repository.find_active(foundation_id).await?;
        Ok(user.map(UserResponse::from))
    }

    /// Soft delete user
    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_not_deleted(id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // Business rule: Add constraints here if needed
        // e.g., cannot delete if user has active sessions

        self.repository.soft_delete(id).await
    }

    /// Permanently delete user
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_not_deleted(id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // Business rule: Add constraints here if needed

        self.repository.delete(id).await
    }

    /// Restore soft-deleted user
    pub async fn restore(&self, id: i64) -> Result<UserResponse, AppError> {
        let restored_user = self.repository.restore(id).await?;
        Ok(UserResponse::from(restored_user))
    }

    // ========================================================================
    // Role Management Methods
    // ========================================================================

    /// Assign a role to user
    pub async fn assign_role(&self, user_id: i64, role_id: i64) -> Result<(), AppError> {
        // Validate user exists
        self.repository
            .find_not_deleted(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // Validate role exists
        Roles::find_by_id(role_id)
            .one(self.repository.conn())
            .await?
            .ok_or_else(|| AppError::not_found("Role not found".to_string()))?;

        // Check if already assigned
        let existing = RoleUsers::find()
            .filter(role_users::Column::UserId.eq(user_id))
            .filter(role_users::Column::RoleId.eq(role_id))
            .one(self.repository.conn())
            .await?;

        if existing.is_some() {
            return Err(AppError::conflict("User already has this role".to_string()));
        }

        // Create assignment
        let role_user = role_users::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        role_user.insert(self.repository.conn()).await?;

        Ok(())
    }

    /// Remove a role from user
    pub async fn remove_role(&self, user_id: i64, role_id: i64) -> Result<(), AppError> {
        let result = RoleUsers::delete_many()
            .filter(role_users::Column::UserId.eq(user_id))
            .filter(role_users::Column::RoleId.eq(role_id))
            .exec(self.repository.conn())
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::not_found("Role assignment not found".to_string()));
        }

        Ok(())
    }

    /// Sync user roles (replace all roles)
    pub async fn sync_roles(&self, user_id: i64, role_ids: Vec<i64>) -> Result<(), AppError> {
        // Validate user exists
        self.repository
            .find_not_deleted(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // Validate all roles exist
        for role_id in &role_ids {
            Roles::find_by_id(*role_id)
                .one(self.repository.conn())
                .await?
                .ok_or_else(|| AppError::not_found(format!("Role {} not found", role_id)))?;
        }

        // Delete all existing assignments
        RoleUsers::delete_many()
            .filter(role_users::Column::UserId.eq(user_id))
            .exec(self.repository.conn())
            .await?;

        // Insert new assignments
        if !role_ids.is_empty() {
            let assignments: Vec<role_users::ActiveModel> = role_ids
                .into_iter()
                .map(|role_id| role_users::ActiveModel {
                    user_id: Set(user_id),
                    role_id: Set(role_id),
                    created_at: Set(chrono::Utc::now()),
                    ..Default::default()
                })
                .collect();

            RoleUsers::insert_many(assignments)
                .exec(self.repository.conn())
                .await?;
        }

        Ok(())
    }

    /// Get user with all assigned roles
    pub async fn get_with_roles(&self, user_id: i64) -> Result<UserResponse, AppError> {
        let user = self
            .repository
            .find_not_deleted(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // Load roles via many-to-many relation harus import ModelTrait agar find_related dapat berjalan
        let roles = user.find_related(Roles).all(self.repository.conn()).await?;

        Ok(UserResponse::from_user_with_roles(&user, &roles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add unit tests
    // - test_create_user_success
    // - test_create_user_duplicate_email
    // - test_change_password_success
    // - test_change_password_wrong_old_password
    // - test_assign_role_success
    // - test_sync_roles_success
}
