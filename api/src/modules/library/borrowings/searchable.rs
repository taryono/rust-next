// src/modules/borrowings/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::students; // ← import entity dari crate entity

impl Searchable for students::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("students", "name"),
            SearchColumn::new("students", "student_number"),
            SearchColumn::new("students", "parent_name"),
            SearchColumn::new("students", "parent_phone"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "students.class_id = classes.id"),
            SearchRelation::new("foundations", "students.foundation_id = foundations.id"),
        ]
    }
}
