// src/modules/books/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::books; // ← import entity dari crate entity

impl Searchable for books::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("books", "name"),
            SearchColumn::new("books", "nis"),
            SearchColumn::new("books", "nib"),
            SearchColumn::new("books", "author"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "books.class_id = classes.id"),
            SearchRelation::new("foundations", "books.foundation_id = foundations.id"),
        ]
    }
}
