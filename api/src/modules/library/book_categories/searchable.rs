// src/modules/categories/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::book_categories; // ← import entity dari crate entity

impl Searchable for categories::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("categories", "name"),
            SearchColumn::new("categories", "nis"),
            SearchColumn::new("categories", "nib"),
            SearchColumn::new("categories", "author"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "categories.class_id = classes.id"),
            SearchRelation::new("foundations", "categories.foundation_id = foundations.id"),
        ]
    }
}
