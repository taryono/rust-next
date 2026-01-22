// ============================================================================
// backend/src/modules/user_profiles/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateUserProfileRequest, UpdateUserProfileRequest, UserProfileResponse};
use super::repository::UserProfileRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::user_profiles;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct UserProfileService {
    repository: UserProfileRepository,
}

impl UserProfileService {
    pub fn new(repository: UserProfileRepository) -> Self {
        Self { repository }
    }

    /// Create new user_profile with validation
    pub async fn create(
        &self,
        request: CreateUserProfileRequest,
    ) -> Result<UserProfileResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_user(&request.user_id.to_string(), request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "UserProfile with this user already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = user_profiles::ActiveModel {
            foundation_id: Set(request.foundation_id),
            user_id: Set(request.user_id),
            phone: Set(request.phone),
            dob: Set(request.dob),
            pob: Set(request.pob),
            bio: Set(request.bio),
            avatar: Set(request.avatar),
            gender: Set(request.gender),
            address: Set(request.address),
            city: Set(request.city),
            province: Set(request.province),
            country: Set(request.country),
            postal_code: Set(request.postal_code),
            latitude: Set(request.latitude),
            longitude: Set(request.longitude),
            timezone: Set(request.timezone),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(UserProfileResponse::from(created))
    }

    /// Get user_profile by ID
    pub async fn get_by_id(&self, id: i64) -> Result<UserProfileResponse, AppError> {
        let user_profile = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("UserProfile not found".to_string()))?;

        Ok(UserProfileResponse::from(user_profile))
    }

    /// Get all user_profiles with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<UserProfileResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<UserProfileResponse> =
            items.into_iter().map(UserProfileResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update user_profile
    pub async fn update(
        &self,
        id: i64,
        request: UpdateUserProfileRequest,
    ) -> Result<UserProfileResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;
        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("UserProfile not found".to_string()))?;
        if !existing.user_id.eq(&request.user_id) {
            // Check duplicate user_id
            if let Some(_) = self
                .repository
                .find_by_user(&request.user_id.to_string(), existing.foundation_id)
                .await?
            {
                return Err(AppError::ConflictError(
                    "UserProfile with this user already exists".to_string(),
                ));
            }
        }
        // Additional business validations
        if let Some(ref postal_code) = request.postal_code {
            if postal_code.len() != 6 {
                return Err(AppError::ValidationError(
                    "Postal code must be exactly 6 characters long".to_string(),
                ));
            }
        }
        if let Some(ref phone) = request.phone {
            if phone.len() < 10 || phone.len() > 15 {
                return Err(AppError::ValidationError(
                    "Phone number must be between 10 and 15 characters long".to_string(),
                ));
            }
        }

        // Build update model
        let mut active_model = user_profiles::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        active_model.bio = Set(request.bio.clone());
        active_model.phone = Set(request.phone.clone());
        active_model.dob = Set(request.dob.clone());
        active_model.pob = Set(request.pob.clone());
        active_model.avatar = Set(request.avatar.clone());
        active_model.gender = Set(request.gender);
        active_model.address = Set(request.address.clone());
        active_model.city = Set(request.city.clone());
        active_model.province = Set(request.province.clone());
        active_model.country = Set(request.country.clone());
        active_model.postal_code = Set(request.postal_code.clone());
        active_model.latitude = Set(request.latitude.clone());
        active_model.longitude = Set(request.longitude.clone());
        active_model.timezone = Set(request.timezone.clone());

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(UserProfileResponse::from(updated))
    }

    /// Delete user_profile
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFoundError("UserProfile not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related user_profiles
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
}
