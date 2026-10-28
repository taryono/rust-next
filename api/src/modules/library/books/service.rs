// ============================================================================
// api/src/modules/library/books/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{BookResponse, CreateBookRequest, UpdateBookRequest};
use super::repository::BookRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::books;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct BookService {
    repository: BookRepository,
}

impl BookService {
    pub fn new(repository: BookRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request: CreateBookRequest) -> Result<BookResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if let Some(_) = self
            .repository
            .find_by_title(&request.title, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Book with this title already exists".to_string(),
            ));
        }

        let active_model = books::ActiveModel {
            foundation_id: Set(request.foundation_id),
            school_id: Set(request.school_id),
            title: Set(request.title),
            isbn: Set(request.isbn),
            subtitle: Set(request.subtitle),
            edition: Set(request.edition),
            language: Set(request.language.unwrap_or_else(|| "id".to_string())),
            author_id: Set(request.author_id),
            publisher_id: Set(request.publisher_id),
            publication_year: Set(request.publication_year),
            book_category_id: Set(request.book_category_id),
            classification_code: Set(request.classification_code),
            pages: Set(request.pages),
            synopsis: Set(request.synopsis),
            cover_url: Set(request.cover_url),
            rack_location: Set(request.rack_location),
            total_stock: Set(request.total_stock.unwrap_or(0)),
            available_stock: Set(request.available_stock.unwrap_or(0)),
            price: Set(request.price),
            source: Set(request.source),
            status: Set(request.status.unwrap_or_else(|| "available".to_string())),
            notes: Set(request.notes),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        let created = self.repository.create(active_model).await?;
        Ok(BookResponse::from(created))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<BookResponse, AppError> {
        let book = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Book not found".to_string()))?;

        Ok(BookResponse::from(book))
    }

    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<BookResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<BookResponse> = items.into_iter().map(BookResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    pub async fn update(
        &self,
        id: i64,
        request: UpdateBookRequest,
    ) -> Result<BookResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Book not found".to_string()))?;

        let foundation_id = request.foundation_id.unwrap_or(existing.foundation_id);

        if let Some(ref title) = request.title {
            if title != &existing.title {
                if let Some(_) = self
                    .repository
                    .find_by_title(title, foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Book with this title already exists".to_string(),
                    ));
                }
            }
        }

        let mut active_model = books::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(fid) = request.foundation_id {
            active_model.foundation_id = Set(fid);
        }
        if let Some(sid) = request.school_id {
            active_model.school_id = Set(sid);
        }
        if let Some(title) = request.title {
            active_model.title = Set(title);
        }
        if request.isbn.is_some() {
            active_model.isbn = Set(request.isbn);
        }
        if request.subtitle.is_some() {
            active_model.subtitle = Set(request.subtitle);
        }
        if request.edition.is_some() {
            active_model.edition = Set(request.edition);
        }
        if let Some(lang) = request.language {
            active_model.language = Set(lang);
        }
        if request.author_id.is_some() {
            active_model.author_id = Set(request.author_id);
        }
        if request.publisher_id.is_some() {
            active_model.publisher_id = Set(request.publisher_id);
        }
        if request.publication_year.is_some() {
            active_model.publication_year = Set(request.publication_year);
        }
        if request.book_category_id.is_some() {
            active_model.book_category_id = Set(request.book_category_id);
        }
        if request.classification_code.is_some() {
            active_model.classification_code = Set(request.classification_code);
        }
        if request.pages.is_some() {
            active_model.pages = Set(request.pages);
        }
        if request.synopsis.is_some() {
            active_model.synopsis = Set(request.synopsis);
        }
        if request.cover_url.is_some() {
            active_model.cover_url = Set(request.cover_url);
        }
        if request.rack_location.is_some() {
            active_model.rack_location = Set(request.rack_location);
        }
        if let Some(tot) = request.total_stock {
            active_model.total_stock = Set(tot);
        }
        if let Some(avail) = request.available_stock {
            active_model.available_stock = Set(avail);
        }
        if request.price.is_some() {
            active_model.price = Set(request.price);
        }
        if request.source.is_some() {
            active_model.source = Set(request.source);
        }
        if let Some(status) = request.status {
            active_model.status = Set(status);
        }
        if request.notes.is_some() {
            active_model.notes = Set(request.notes);
        }

        let updated = self.repository.update(id, active_model).await?;
        Ok(BookResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Book not found".to_string()))?;

        self.repository.delete(id).await
    }
}
