// backend/src/modules/menus/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
// ============================================================
// Output structure — ini yang dikembalikan ke frontend
// ============================================================
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MenuTree {
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub menu_context: Option<String>,
    pub children: Vec<MenuTree>,
}

// ============================================================
// Internal flat struct — hasil query sebelum di-build jadi tree
// ============================================================
#[derive(Debug, Clone, Serialize, ToSchema)]
struct FlatMenu {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: i16,
    pub menu_context: Option<String>,
}
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CreateMenuRequest {
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub menu_context: Option<String>,
}
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MenuResponse {
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub menu_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UpdateMenuRequest {
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub menu_context: Option<String>,
}
