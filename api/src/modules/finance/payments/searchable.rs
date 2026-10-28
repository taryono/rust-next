// src/modules/finance/payments/searchable.rs
use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::payments;

impl Searchable for payments::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("payments", "name"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("foundations", "payments.foundation_id = foundations.id"),
        ]
    }
}
