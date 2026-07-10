// src/modules/book_copies/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::book_copies; // ← import entity dari crate entity

impl Searchable for book_copies::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("book_copies", "name"),
            SearchColumn::new("book_copies", "nis"),
            SearchColumn::new("book_copies", "nib"),
            SearchColumn::new("book_copies", "author"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "book_copies.class_id = classes.id"),
            SearchRelation::new("foundations", "book_copies.foundation_id = foundations.id"),
        ]
    }
}
