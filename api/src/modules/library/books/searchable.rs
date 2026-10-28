use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::books;

impl Searchable for books::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("books", "title"),
            SearchColumn::new("books", "isbn"),
            SearchColumn::new("books", "subtitle"),
            SearchColumn::new("books", "language"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![
            SearchRelation::new("foundations", "books.foundation_id = foundations.id"),
        ]
    }
}
