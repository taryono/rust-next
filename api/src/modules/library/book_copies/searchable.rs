use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::book_copies;

impl Searchable for book_copies::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("book_copies", "inventory_number"),
            SearchColumn::new("book_copies", "barcode"),
            SearchColumn::new("book_copies", "rack_code"),
            SearchColumn::new("book_copies", "status"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("foundations", "book_copies.foundation_id = foundations.id"),
        ]
    }
}
