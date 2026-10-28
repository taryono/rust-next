use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::borrowings;

impl Searchable for borrowings::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("borrowings", "borrower_type"),
            SearchColumn::new("borrowings", "condition_on_borrow"),
            SearchColumn::new("borrowings", "status"),
            SearchColumn::new("borrowings", "notes"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("foundations", "borrowings.foundation_id = foundations.id"),
        ]
    }
}
