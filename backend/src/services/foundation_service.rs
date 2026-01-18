// ============================================
// backend/src/services/foundation_service.rs
// ============================================
use crate::models::foundation::{FoundationResponse, UpdateFoundationRequest};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use chrono::Utc;
use entity::foundations::{self, Entity as Foundations};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

pub async fn get_all_foundations(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<FoundationResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Build base query - hanya yang tidak dihapus
    let mut query = Foundations::find();

    // Apply search filter
    if let Some(search_term) = &params.search {
        if !search_term.is_empty() {
            query = query.filter(
                Condition::any()
                    .add(foundations::Column::Name.contains(search_term))
                    .add(foundations::Column::Description.contains(search_term)),
            );
        }
    }

    // Apply sorting
    query = match params.sort_order.as_deref() {
        Some("asc") => query.order_by_asc(foundations::Column::CreatedAt),
        _ => query.order_by_desc(foundations::Column::CreatedAt),
    };

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let foundations_list = paginator.fetch_page(page - 1).await?;

    // Map to response
    let foundation_responses: Vec<FoundationResponse> = foundations_list
        .iter()
        .map(|foundation| FoundationResponse::from_entity(foundation))
        .collect();

    Ok(PaginatedResponse::new(
        foundation_responses,
        total.try_into().unwrap(),
        page.try_into().unwrap(),
        per_page.try_into().unwrap(),
    ))
}

pub async fn get_foundation_by_id(
    db: &DatabaseConnection,
    foundation_id: i64,
) -> Result<FoundationResponse, DbErr> {
    let foundation = Foundations::find()
        .filter(foundations::Column::Id.eq(foundation_id))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Foundation not found".to_string()))?;

    Ok(FoundationResponse::from_entity(&foundation))
}

pub async fn update_foundation(
    db: &DatabaseConnection,
    foundation_id: i64,
    update_data: UpdateFoundationRequest,
) -> Result<FoundationResponse, DbErr> {
    let foundation = Foundations::find()
        .filter(foundations::Column::Id.eq(foundation_id))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Foundation not found".to_string()))?;

    let mut active_foundation: foundations::ActiveModel = foundation.into_active_model();

    // ✅ Langsung set tanpa if let karena name bukan Option
    active_foundation.name = Set(update_data.name);

    // Description tetap optional
    if let Some(description) = update_data.description {
        // Check if description already exists
        let existing = Foundations::find()
            .filter(foundations::Column::Description.eq(&description))
            .one(db)
            .await?;

        if let Some(existing_foundation) = existing {
            if existing_foundation.id != foundation_id as u64 {
                return Err(DbErr::Custom("Description already in use".to_string()));
            }
        }
        active_foundation.description = Set(Some(description));
    }

    active_foundation.updated_at = Set(Utc::now());

    let updated_foundation = active_foundation.update(db).await?;

    Ok(FoundationResponse::from_entity(&updated_foundation))
}

/// Soft delete foundation
pub async fn soft_delete(db: &DatabaseConnection, foundation_id: i64) -> Result<(), DbErr> {
    let foundation = Foundations::find_by_id(foundation_id as u64)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Foundation not found".to_string()))?;

    let mut active_foundation: foundations::ActiveModel = foundation.into_active_model();
    active_foundation.deleted_at = Set(Some(Utc::now()));
    active_foundation.update(db).await?;

    Ok(())
}

/// Restore deleted foundation
pub async fn restore(db: &DatabaseConnection, foundation_id: i64) -> Result<(), DbErr> {
    let foundation = Foundations::find_by_id(foundation_id as u64)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Foundation not found".to_string()))?;

    let mut active_foundation: foundations::ActiveModel = foundation.into_active_model();
    active_foundation.deleted_at = Set(None);
    active_foundation.update(db).await?;

    Ok(())
}

/// Get deleted foundations (with pagination)
pub async fn get_deleted_foundations(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<FoundationResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Query hanya yang sudah dihapus
    let query = Foundations::find();

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let foundations_list = paginator.fetch_page(page - 1).await?;

    // Map to response
    let foundation_responses: Vec<FoundationResponse> = foundations_list
        .iter()
        .map(|foundation| FoundationResponse::from_entity(foundation))
        .collect();

    Ok(PaginatedResponse::new(
        foundation_responses,
        total.try_into().unwrap(),
        page.try_into().unwrap(),
        per_page.try_into().unwrap(),
    ))
}

/// Get all foundations including deleted
pub async fn get_all_with_deleted(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<FoundationResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Query semua termasuk yang dihapus
    let query = Foundations::find();

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let foundations_list = paginator.fetch_page(page - 1).await?;

    // Map to response
    let foundation_responses: Vec<FoundationResponse> = foundations_list
        .iter()
        .map(|foundation| FoundationResponse::from_entity(foundation))
        .collect();

    Ok(PaginatedResponse::new(
        foundation_responses,
        total.try_into().unwrap(),
        page.try_into().unwrap(),
        per_page.try_into().unwrap(),
    ))
}

/// Force delete (permanent)
pub async fn force_delete(db: &DatabaseConnection, foundation_id: i64) -> Result<(), DbErr> {
    Foundations::delete_by_id(foundation_id as u64)
        .exec(db)
        .await?;
    Ok(())
}
