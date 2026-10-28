use crate::filters::global_search::{SearchColumn, SearchRelation, Searchable};
use entity::publishers;

impl Searchable for publishers::Entity {
    fn searchable_columns() -> Vec<SearchColumn> {
        vec![
            SearchColumn::new("publishers", "name"),
            SearchColumn::new("publishers", "email"),
            SearchColumn::new("publishers", "phone"),
            SearchColumn::new("publishers", "city"),
        ]
    }

    fn searchable_relations() -> Vec<SearchRelation> {
        vec![]
    }
}
