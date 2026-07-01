// src/modules/payments/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::payments; // ← import entity dari crate entity

impl Searchable for payments::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("payments", "name"),
            SearchColumn::new("payments", "student_number"),
            SearchColumn::new("payments", "parent_name"),
            SearchColumn::new("payments", "parent_phone"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "payments.class_id = classes.id"),
            SearchRelation::new("foundations", "payments.foundation_id = foundations.id"),
        ]
    }
}
