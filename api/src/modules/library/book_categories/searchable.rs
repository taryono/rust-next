use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::book_categories;

impl Searchable for book_categories::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("book_categories", "name"),
            SearchColumn::new("book_categories", "code"),
            SearchColumn::new("book_categories", "description"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("foundations", "book_categories.foundation_id = foundations.id"),
        ]
    }
}
