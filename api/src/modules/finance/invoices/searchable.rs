// src/modules/invoices/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::invoices; // ← import entity dari crate entity

impl Searchable for invoices::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            // ✅ Sesuaikan dengan field yang ada di Model kamu
            SearchColumn::new("invoices", "name"),
            SearchColumn::new("invoices", "student_number"),
            SearchColumn::new("invoices", "parent_name"),
            SearchColumn::new("invoices", "parent_phone"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("classes", "invoices.class_id = classes.id"),
            SearchRelation::new("foundations", "invoices.foundation_id = foundations.id"),
        ]
    }
}
