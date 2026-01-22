use crate::models::pagination::{PaginatedResponse, PaginationParams};
use sea_orm::*;
use std::collections::HashMap;

/// Generic function to paginate any SeaORM query
pub async fn paginate_query<E, M>(
    query: Select<E>,
    db: &DatabaseConnection,
    params: &PaginationParams,
) -> Result<(Vec<M>, usize), DbErr>
where
    E: EntityTrait<Model = M>,
    M: ModelTrait + Send + Sync,
{
    let page = params.page();
    let per_page = params.per_page();

    // Get total count
    let total = query.clone().count(db).await? as usize;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let data = paginator.fetch_page(page - 1).await?;

    Ok((data, total))
}

/// Batch load related entities by foreign key
pub async fn batch_load_related<E, M, K>(
    db: &DatabaseConnection,
    entity: E,
    foreign_key_column: E::Column,
    ids: Vec<K>,
) -> Result<HashMap<K, Vec<M>>, DbErr>
where
    E: EntityTrait<Model = M>,
    M: ModelTrait + Send + Sync,
    K: Eq + std::hash::Hash + Clone + Into<Value>,
    E::Column: ColumnTrait,
{
    let related = entity
        .find()
        .filter(foreign_key_column.is_in(ids.clone()))
        .all(db)
        .await?;

    let mut map: HashMap<K, Vec<M>> = HashMap::new();

    // Group by foreign key - this is simplified, you'll need to adjust based on your model
    for item in related {
        // You'll need to implement logic to extract the foreign key from the model
        // This is a placeholder
    }

    Ok(map)
}
