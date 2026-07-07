//! `SeaORM` Entity
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, ConnectionTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "authors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub full_name: String,
    pub biography: Option<String>,
    pub nationality: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
}

impl Model {
    /// Buat instance Author baru dengan nilai bawaan yang benar
    pub fn new(full_name: String) -> Self {
        let now: DateTime<Utc> = Utc::now();
        let now_sea: DateTimeUtc = now.into();

        Self {
            id: 0, // Akan diisi otomatis oleh database
            full_name,
            biography: None,
            nationality: None,
            email: None,
            is_active: true,
            created_at: now_sea,
            updated_at: now_sea,
            deleted_at: None,
        }
    }

    /// Cari berdasarkan nama lengkap
    pub fn find_by_name(name: &str) -> Select<Entity> {
        Entity::find().filter(Column::FullName.eq(name)) // ✅ Sesuai nama kolom
    }

    /// Cari berdasarkan ID
    pub fn find_by_id(id: i64) -> Select<Entity> {
        Entity::find().filter(Column::Id.eq(id))
    }

    /// Khusus Admin BO: ambil SEMUA data dari semua foundation
    pub fn all_for_admin() -> Select<Entity> {
        Entity::find()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::books::Entity")]
    Books,
}

impl Related<super::books::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
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

// ✅ Implementasi trait SoftDelete
impl crate::traits::soft_delete::SoftDelete for Entity {
    fn deleted_at_col() -> Column {
        Column::DeletedAt
    }
}

// Buat data baru
// let penulis = Model::new(
//     1, // foundation_id
//     "Andrea Hirata".to_string()
// );

// // Ambil data milik foundation tertentu
// let daftar_penulis = Model::for_foundation(1).all(&db).await?;

// // Cari nama spesifik di foundation itu
// let penulis = Model::find_by_foundation_and_name(1, "Andrea Hirata").one(&db).await?;

// // Khusus Admin: ambil semua penulis dari seluruh foundation
// let semua_penulis = Model::all_for_admin().all(&db).await?;
