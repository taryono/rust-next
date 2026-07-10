//! `SeaORM` Entity
use super::traits::Tenanted;
use async_trait::async_trait;
use chrono::{Utc};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "book_copies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub book_id: i64,
    pub foundation_id: i64,
    pub inventory_number: String,
    pub barcode: Option<String>,
    pub rack_code: Option<String>,
    pub shelf_number: Option<String>,
    pub condition: String,
    pub acquisition_date: Date,
    pub price: Option<i64>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::books::Entity",
        from = "Column::BookId",
        to = "super::books::Column::Id"
    )]
    Book,
    #[sea_orm(has_many = "super::borrowings::Entity")]
    Borrowings,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Book.def()
    }
}

impl Related<super::borrowings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Borrowings.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now().into(); // DateTime<Utc> → DateTimeUtc

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
