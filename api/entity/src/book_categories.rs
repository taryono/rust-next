//! `SeaORM` Entity
use super::traits::Tenanted;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "book_categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub foundation_id: i64,
    pub name: String,
    pub code: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i64>,
    pub school_id: i64,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::schools::Entity",
        from = "Column::SchoolId",
        to = "super::schools::Column::Id"
    )]
    School,
    #[sea_orm(has_many = "super::books::Entity")]
    Books,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
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

impl Model {
    pub fn new(name: String) -> Self {
        let now: DateTime<Utc> = Utc::now();
        // Konversi DateTime<Utc> ke tipe DateTimeUtc milik SeaORM
        let now_sea: DateTimeUtc = now.into();

        Self {
            id: 0,             // Nilai sementara, akan diganti auto-increment DB
            foundation_id: 0,  // ID foundation
            name,              // Nama kategori buku
            code: None,        // Kode kategori buku
            description: None, // Deskripsi kategori buku
            parent_id: None,   // ID kategori induk
            school_id: 0,
            is_active: true,
            created_at: now_sea,
            updated_at: now_sea,
            deleted_at: None,
        }
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
