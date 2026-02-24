// backend/src/modules/menus/dto.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================================
// RESPONSE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuResponse {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: i16,
    pub menu_context: Option<String>,
}

// ✅ Fix stack overflow — gunakan Box<MenuTree> untuk children
// agar utoipa tidak generate schema rekursif tanpa batas
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)] // ✅ derive biasa
pub struct MenuTree {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i16,
    pub menu_context: Option<String>,
    // ✅ ToSchema tidak perlu tahu struktur children untuk swagger
    #[schema(value_type = Vec<Object>)]
    pub children: Vec<Box<MenuTree>>,
}
// ============================================================
// INTERNAL — flat struct sebelum di-build jadi tree
// ============================================================

#[derive(Debug, Clone)]
pub struct FlatMenu {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: i16,
    pub menu_context: Option<String>,
}

// ============================================================
// REQUEST
// ============================================================

#[derive(Debug, Clone, Deserialize, ToSchema, Validate)]
pub struct CreateMenuRequest {
    #[validate(length(min = 1, message = "Key cannot be empty"))]
    pub key: String,

    #[validate(length(min = 1, message = "Label cannot be empty"))]
    pub label: String,

    pub href: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,  // ✅ tambah untuk nested menu
    pub sort_order: Option<i16>, // ✅ tambah untuk ordering
    pub menu_context: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema, Validate)]
pub struct UpdateMenuRequest {
    #[validate(length(min = 1, message = "Key cannot be empty"))]
    pub key: Option<String>, // ✅ Optional untuk partial update

    #[validate(length(min = 1, message = "Label cannot be empty"))]
    pub label: Option<String>, // ✅ Optional untuk partial update

    pub href: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: Option<i16>,
    pub menu_context: Option<String>,
}

// ============================================================
// CONVERSIONS
// ============================================================

impl From<entity::menus::Model> for MenuResponse {
    fn from(model: entity::menus::Model) -> Self {
        Self {
            id: model.id,
            key: model.key,
            label: model.label,
            href: model.href,
            icon: model.icon,
            parent_id: model.parent_id,
            sort_order: model.sort_order,
            menu_context: model.menu_context,
        }
    }
}

impl From<entity::menus::Model> for FlatMenu {
    fn from(model: entity::menus::Model) -> Self {
        Self {
            id: model.id,
            key: model.key,
            label: model.label,
            href: model.href,
            icon: model.icon,
            parent_id: model.parent_id,
            sort_order: model.sort_order,
            menu_context: model.menu_context,
        }
    }
}
