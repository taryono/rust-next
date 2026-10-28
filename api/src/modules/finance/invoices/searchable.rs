// src/modules/finance/invoices/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::invoices;

impl Searchable for invoices::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("invoices", "code"),
            SearchColumn::new("invoices", "name"),
            SearchColumn::new("invoices", "description"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![]
    }
}
