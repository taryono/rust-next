// src/modules/publisher/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::publisher; // ← import entity dari crate entity

impl Searchable for publisher::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("publisher", "name"),
            SearchColumn::new("publisher", "nis"),
            SearchColumn::new("publisher", "nib"),
            SearchColumn::new("publisher", "author"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "publisher.class_id = classes.id"),
            SearchRelation::new("foundations", "publisher.foundation_id = foundations.id"),
        ]
    }
}
