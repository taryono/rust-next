//! `SeaORM` Entity
use super::traits::Tenanted;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub foundation_id: i64,
    pub isbn: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub edition: Option<String>,
    pub language: String,
    pub author_id: Option<i64>,
    pub publisher_id: Option<i64>,
    pub publication_year: Option<i16>,
    pub book_category_id: Option<i64>,
    pub classification_code: Option<String>,
    pub pages: Option<i32>,
    pub synopsis: Option<String>,
    pub cover_url: Option<String>,
    pub school_id: i64,
    pub rack_location: Option<String>,
    pub total_stock: i32,
    pub available_stock: i32,
    pub price: Option<i64>,
    pub source: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::foundations::Entity",
        from = "Column::AuthorId",
        to = "super::foundations::Column::Id"
    )]
    Foundation,
    #[sea_orm(
        belongs_to = "super::authors::Entity",
        from = "Column::AuthorId",
        to = "super::authors::Column::Id"
    )]
    Author,
    #[sea_orm(
        belongs_to = "super::publishers::Entity",
        from = "Column::PublisherId",
        to = "super::publishers::Column::Id"
    )]
    Publisher,
    #[sea_orm(
        belongs_to = "super::book_categories::Entity",
        from = "Column::BookCategoryId",
        to = "super::book_categories::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::schools::Entity",
        from = "Column::SchoolId",
        to = "super::schools::Column::Id"
    )]
    School,
    #[sea_orm(has_many = "super::book_copies::Entity")]
    BookCopies,
}

impl Related<super::foundations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foundation.def()
    }
}
impl Related<super::authors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl Related<super::publishers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Publisher.def()
    }
}

impl Related<super::book_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::schools::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::School.def()
    }
}

impl Related<super::book_copies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookCopies.def()
    }
}

// ✅ Implementasi yang benar untuk v2.0-rc
#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now().into();
        if insert {
            self.created_at = Set(now);
        }
        self.updated_at = Set(now);
        Ok(self)
    }
}

impl Tenanted for Entity {
    type Col = Column;
    fn foundation_id_column() -> Self::Col {
        Column::FoundationId
    }
}

// ✅ Implementasi trait SoftDelete
impl crate::traits::soft_delete::SoftDelete for Entity {
    fn deleted_at_col() -> Column {
        Column::DeletedAt
    }
}
