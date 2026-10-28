use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::authors;

impl Searchable for authors::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("authors", "full_name"),
            SearchColumn::new("authors", "email"),
            SearchColumn::new("authors", "nationality"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![]
    }
}
