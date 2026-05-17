// src/filters/global_search.rs
//
// Equivalent of Laravel's GlobalSearch for Rust + SeaORM + MySQL
//
// Cargo.toml dependencies:
// sea-orm = { version = "1", features = ["runtime-tokio-rustls", "sqlx-mysql", "macros"] }
// tokio = { version = "1", features = ["full"] }

use sea_orm::sea_query::{Alias, Condition, Expr, IntoCondition, SimpleExpr};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, QueryTrait, Select, Statement,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Tipe integer MySQL yang akan di-skip jika keyword bukan angka
// ---------------------------------------------------------------------------
const INTEGER_TYPES: &[&str] = &[
    "int",
    "tinyint",
    "smallint",
    "mediumint",
    "bigint",
    "integer",
];

// ---------------------------------------------------------------------------
// Struct untuk mendefinisikan kolom yang bisa disearch
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct SearchColumn {
    /// Nama tabel, contoh: "users"
    pub table: String,
    /// Nama kolom, contoh: "name"
    pub column: String,
}

impl SearchColumn {
    pub fn new(table: impl Into<String>, column: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            column: column.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Struct untuk join relation (pengganti whereHas di Laravel)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct SearchRelation {
    /// Nama tabel relasi, contoh: "categories"
    pub related_table: String,
    /// Kondisi JOIN ON, contoh: "products.category_id = categories.id"
    pub join_condition: String,
}

impl SearchRelation {
    pub fn new(related_table: impl Into<String>, join_condition: impl Into<String>) -> Self {
        Self {
            related_table: related_table.into(),
            join_condition: join_condition.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: baris dari information_schema
// ---------------------------------------------------------------------------
#[derive(Debug, FromQueryResult)]
struct ColumnSchemaRow {
    column_name: String,
    data_type: String,
}

// ---------------------------------------------------------------------------
// GlobalSearch
// ---------------------------------------------------------------------------
pub struct GlobalSearch;

impl GlobalSearch {
    // -----------------------------------------------------------------------
    // Cek apakah keyword adalah angka (sama seperti isNumericSearch di Laravel)
    // -----------------------------------------------------------------------
    pub fn is_numeric_search(search: &str) -> bool {
        let keyword = search.replace('%', "");
        keyword.parse::<f64>().is_ok()
    }

    // -----------------------------------------------------------------------
    // Cek apakah tipe kolom adalah integer
    // -----------------------------------------------------------------------
    pub fn is_integer_type(data_type: &str) -> bool {
        let lower = data_type.to_lowercase();
        INTEGER_TYPES.iter().any(|t| lower.contains(t))
    }

    // -----------------------------------------------------------------------
    // Ambil tipe kolom dari information_schema MySQL
    // Equivalent: getColumnTypes() di Laravel
    // -----------------------------------------------------------------------
    pub async fn get_column_types(
        db: &DatabaseConnection,
        database: &str,
        table: &str,
    ) -> Result<HashMap<String, String>, DbErr> {
        let sql = format!(
            "SELECT COLUMN_NAME AS column_name, DATA_TYPE AS data_type \
             FROM information_schema.COLUMNS \
             WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'",
            database, table
        );

        let rows = ColumnSchemaRow::find_by_statement(Statement::from_string(
            sea_orm::DatabaseBackend::MySql,
            sql,
        ))
        .all(db)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| (r.column_name, r.data_type))
            .collect())
    }

    // -----------------------------------------------------------------------
    // Core: build Condition dari searchable columns
    // -----------------------------------------------------------------------
    fn build_column_condition(
        columns: &[SearchColumn],
        column_types: &HashMap<String, String>, // column_name -> data_type
        search: &str,
        is_numeric: bool,
    ) -> Option<Condition> {
        let mut condition = Condition::any();
        let mut has_any = false;

        for col in columns {
            let data_type = column_types
                .get(&col.column)
                .map(|s| s.as_str())
                .unwrap_or("");

            // Skip kolom integer jika keyword bukan angka
            // Equivalent: if (!$isNumeric && self::isIntegerColumn($columnType)) continue;
            if !is_numeric && Self::is_integer_type(data_type) {
                continue;
            }

            let expr = Expr::col((Alias::new(&col.table), Alias::new(&col.column))).like(search);

            condition = condition.add(expr);
            has_any = true;
        }

        if has_any {
            Some(condition)
        } else {
            None
        }
    }

    // -----------------------------------------------------------------------
    // Main entry point — apply global search ke SeaORM Select<E>
    //
    // Parameters:
    //   db              - DatabaseConnection
    //   select          - SeaORM Select<E> query builder
    //   database_name   - Nama database MySQL (untuk information_schema)
    //   search_value    - Nilai pencarian dari request, None = tidak ada search
    //   main_table      - Nama tabel utama entity
    //   searchable_cols - Kolom-kolom yang ikut disearch
    //   relations       - Relasi (JOIN) yang ikut disearch
    //
    // Equivalent: GlobalSearch::search($builder, $relations, $searchableFields)
    // -----------------------------------------------------------------------
    pub async fn apply<E>(
        db: &DatabaseConnection,
        select: Select<E>,
        database_name: &str,
        search_value: Option<&str>,
        main_table: &str,
        searchable_cols: &[SearchColumn],
        relations: &[SearchRelation],
    ) -> Result<Select<E>, DbErr>
    where
        E: EntityTrait,
    {
        // Tidak ada search value → kembalikan query apa adanya
        let search_value = match search_value {
            Some(v) if !v.is_empty() => v,
            _ => return Ok(select),
        };

        let search = format!("%{}%", search_value);
        let is_numeric = Self::is_numeric_search(&search);

        // Ambil tipe kolom tabel utama
        let main_col_types = Self::get_column_types(db, database_name, main_table).await?;

        // Build kondisi untuk kolom utama
        let mut outer_condition = Condition::any();
        let mut has_outer = false;

        if let Some(cond) =
            Self::build_column_condition(searchable_cols, &main_col_types, &search, is_numeric)
        {
            outer_condition = outer_condition.add(cond);
            has_outer = true;
        }

        // Build kondisi untuk setiap relasi (via LEFT JOIN)
        // Equivalent: orWhereHas() di Laravel
        let mut joined_select = select;

        for relation in relations {
            // Ambil semua kolom dari tabel relasi via information_schema
            let rel_col_types =
                Self::get_column_types(db, database_name, &relation.related_table).await?;

            // Buat SearchColumn otomatis dari semua kolom tabel relasi
            let rel_cols: Vec<SearchColumn> = rel_col_types
                .keys()
                .map(|col_name| SearchColumn::new(&relation.related_table, col_name))
                .collect();

            if let Some(cond) =
                Self::build_column_condition(&rel_cols, &rel_col_types, &search, is_numeric)
            {
                outer_condition = outer_condition.add(cond);
                has_outer = true;

                QueryTrait::query(&mut joined_select).join(
                    JoinType::LeftJoin,
                    Alias::new(&relation.related_table),
                    Condition::all()
                        .add(SimpleExpr::Custom(relation.join_condition.clone().into())),
                );
            }
        }

        if has_outer {
            Ok(joined_select.filter(outer_condition))
        } else {
            Ok(joined_select)
        }
    }

    /// apply_from_entity: baca searchable_columns & relations langsung dari trait entity
    /// Equivalent: tidak perlu definisi manual di repository
    pub async fn apply_from_entity<E>(
        db: &DatabaseConnection,
        select: Select<E>,
        database_name: &str,
        search_value: Option<&str>,
    ) -> Result<Select<E>, DbErr>
    where
        E: EntityTrait + Searchable, // ← entity harus impl Searchable
    {
        Self::apply(
            db,
            select,
            database_name,
            search_value,
            &E::searchable_columns()
                .first()
                .map(|c| c.table.as_str())
                .unwrap_or(""),
            &E::searchable_columns(),
            &E::searchable_relations(),
        )
        .await
    }
}

// Tambahkan trait ini di global_search.rs
pub trait Searchable {
    /// Kolom-kolom yang bisa disearch pada tabel utama
    fn searchable_columns() -> Vec<SearchColumn>;

    /// Relasi yang ikut disearch (default kosong)
    fn searchable_relations() -> Vec<SearchRelation> {
        vec![]
    }
}

// ---------------------------------------------------------------------------
// CONTOH PENGGUNAAN
// ---------------------------------------------------------------------------
//
// use sea_orm::EntityTrait;
// use crate::entities::products;  // entity SeaORM kamu
// use crate::filters::global_search::{GlobalSearch, SearchColumn, SearchRelation};
//
// async fn list_products(
//     db: &DatabaseConnection,
//     search: Option<String>,
// ) -> Result<Vec<products::Model>, DbErr> {
//
//     // Kolom-kolom yang disearch (hanya dari tabel utama)
//     let searchable_cols = vec![
//         SearchColumn::new("products", "name"),
//         SearchColumn::new("products", "description"),
//         SearchColumn::new("products", "sku"),
//         SearchColumn::new("products", "id"),   // integer, akan di-skip jika keyword bukan angka
//     ];
//
//     // Relasi yang ikut disearch (LEFT JOIN)
//     let relations = vec![
//         SearchRelation::new(
//             "categories",
//             "products.category_id = categories.id",
//         ),
//         SearchRelation::new(
//             "brands",
//             "products.brand_id = brands.id",
//         ),
//     ];
//
//     let query = products::Entity::find();
//
//     let query = GlobalSearch::apply(
//         db,
//         query,
//         "my_database",          // nama database MySQL
//         search.as_deref(),      // Option<&str>
//         "products",             // nama tabel utama
//         &searchable_cols,
//         &relations,
//     )
//     .await?;
//
//     query.all(db).await
// }
