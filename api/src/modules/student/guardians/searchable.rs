// src/modules/guardians/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::guardians; // ← import entity dari crate entity

impl Searchable for guardians::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("guardians", "name"),
            SearchColumn::new("guardians", "guardian_number"),
            SearchColumn::new("guardians", "parent_name"),
            SearchColumn::new("guardians", "parent_phone"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "guardians.class_id = classes.id"),
            SearchRelation::new("foundations", "guardians.foundation_id = foundations.id"),
        ]
    }
}
