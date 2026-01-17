use crate::{
    models::academic_year::{
        AcademicYearListResponse, AcademicYearResponse, CreateAcademicYearRequest,
        UpdateAcademicYearRequest,
    },
    models::pagination::PaginationParams,
};
use chrono::{NaiveDate, Utc};
use entity::academic_years;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

fn to_response(model: academic_years::Model) -> AcademicYearResponse {
    AcademicYearResponse {
        id: model.id,
        foundation_id: model.foundation_id,
        name: model.name,
        start_date: model.start_date.to_string(),
        end_date: model.end_date.to_string(),
        is_active: model.is_active,
    }
}

pub async fn get_all(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<AcademicYearListResponse, String> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    let mut query =
        academic_years::Entity::find().filter(academic_years::Column::DeletedAt.is_null());

    if let Some(search) = params.search {
        query = query.filter(academic_years::Column::Name.contains(&search));
    }

    let paginator = query.paginate(db, per_page);
    let total = paginator.num_items().await.map_err(|e| e.to_string())?;
    let models = paginator
        .fetch_page(page - 1)
        .await
        .map_err(|e| e.to_string())?;

    Ok(AcademicYearListResponse {
        data: models.into_iter().map(to_response).collect(),
        total,
        page,
        per_page,
        total_pages: (total as f64 / per_page as f64).ceil() as u64,
    })
}

pub async fn get_by_id(db: &DatabaseConnection, id: u64) -> Result<AcademicYearResponse, String> {
    let model = academic_years::Entity::find_by_id(id)
        .filter(academic_years::Column::DeletedAt.is_null())
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Academic year not found")?;

    Ok(to_response(model))
}

pub async fn create(
    db: &DatabaseConnection,
    payload: CreateAcademicYearRequest,
) -> Result<AcademicYearResponse, String> {
    let model = academic_years::ActiveModel {
        foundation_id: Set(payload.foundation_id),
        name: Set(payload.name),
        start_date: Set(NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%d")
            .map_err(|_| "Invalid start_date")?),
        end_date: Set(NaiveDate::parse_from_str(&payload.end_date, "%Y-%m-%d")
            .map_err(|_| "Invalid end_date")?),
        is_active: Set(payload.is_active),
        created_at: Set(Some(Utc::now())),
        ..Default::default()
    };

    let result = model.insert(db).await.map_err(|e| e.to_string())?;
    Ok(to_response(result))
}

pub async fn update(
    db: &DatabaseConnection,
    id: u64,
    payload: UpdateAcademicYearRequest,
) -> Result<AcademicYearResponse, String> {
    let mut model: academic_years::ActiveModel = academic_years::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Academic year not found")?
        .into();

    if let Some(name) = payload.name {
        model.name = Set(name);
    }
    if let Some(start_date) = payload.start_date {
        model.start_date =
            Set(NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
                .map_err(|_| "Invalid start_date")?);
    }
    if let Some(end_date) = payload.end_date {
        model.end_date =
            Set(NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").map_err(|_| "Invalid end_date")?);
    }
    if let Some(is_active) = payload.is_active {
        model.is_active = Set(Some(is_active));
    }

    model.updated_at = Set(Some(Utc::now()));

    let result = model.update(db).await.map_err(|e| e.to_string())?;
    Ok(to_response(result))
}

pub async fn delete(db: &DatabaseConnection, id: u64) -> Result<(), String> {
    let mut model: academic_years::ActiveModel = academic_years::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Academic year not found")?
        .into();

    model.deleted_at = Set(Some(Utc::now().naive_utc()));
    model.update(db).await.map_err(|e| e.to_string())?;
    Ok(())
}
