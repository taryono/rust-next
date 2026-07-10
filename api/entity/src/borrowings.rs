//! `SeaORM` Entity
use super::traits::Tenanted;
use async_trait::async_trait;
use chrono::{Utc};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "borrowings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub foundation_id: i64,
    pub book_copy_id: i64,
    pub borrower_id: i64,
    pub borrower_type: String,
    pub school_id: i64,
    pub borrow_date: Date,
    pub due_date: Date,
    pub return_date: Option<Date>,
    pub condition_on_borrow: String,
    pub condition_on_return: Option<String>,
    pub fine_amount: Option<i64>,
    pub fine_paid: bool,
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
        from = "Column::FoundationId",
        to = "super::foundations::Column::Id"
    )]
    Foundation,
    #[sea_orm(
        belongs_to = "super::book_copies::Entity",
        from = "Column::BookCopyId",
        to = "super::book_copies::Column::Id"
    )]
    BookCopy,
    #[sea_orm(
        belongs_to = "super::schools::Entity",
        from = "Column::SchoolId",
        to = "super::schools::Column::Id"
    )]
    School,
}

impl Related<super::foundations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Foundation.def()
    }
}

impl Related<super::book_copies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookCopy.def()
    }
}

impl Related<super::schools::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::School.def()
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
