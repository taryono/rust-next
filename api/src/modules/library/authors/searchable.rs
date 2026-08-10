// src/modules/author/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::author; // ← import entity dari crate entity

impl Searchable for author::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("author", "name"),
            SearchColumn::new("author", "nis"),
            SearchColumn::new("author", "nib"),
            SearchColumn::new("author", "author"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "author.class_id = classes.id"),
            SearchRelation::new("foundations", "author.foundation_id = foundations.id"),
        ]
    }
}
