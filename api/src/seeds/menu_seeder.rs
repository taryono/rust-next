// seeds/menu_seeder.rs
// Jalankan: cargo run --bin seed -- menus

use crate::entities::{menu_permissions, menu_roles, menus};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

struct MenuSeed {
    key: &'static str,
    label: &'static str,
    href: Option<&'static str>,
    icon: Option<&'static str>,
    parent_key: Option<&'static str>, // NULL = root
    sort_order: i16,
    menu_context: Option<&'static str>,
    roles: Vec<&'static str>,       // nama role
    permissions: Vec<&'static str>, // nama permission
}

// Flatten semua menu dari tree config mu jadi array flat
// Urutan penting: parent harus sebelum child
const MENU_SEEDS: &[MenuSeed] = &[
    MenuSeed {
        key: "dashboard",
        label: "Dashboard",
        href: Some("/dashboard"),
        icon: Some("ti ti-dashboard"),
        parent_key: None,
        sort_order: 0,
        menu_context: Some("global"),
        roles: vec![],
        permissions: vec![],
    },
    MenuSeed {
        key: "parent_user",
        label: "Authentications",
        href: None,
        icon: Some("ti ti-users"),
        parent_key: None,
        sort_order: 1,
        menu_context: Some("system_owner"),
        roles: vec!["system_owner", "admin"],
        permissions: vec![],
    },
    MenuSeed {
        key: "user",
        label: "User",
        href: Some("/dashboard/users"),
        icon: Some("ti ti-list"),
        parent_key: Some("parent_user"),
        sort_order: 0,
        menu_context: None,
        roles: vec![],
        permissions: vec!["users.view"],
    },
    MenuSeed {
        key: "role",
        label: "Role",
        href: Some("/dashboard/roles"),
        icon: Some("ti ti-shield"),
        parent_key: Some("parent_user"),
        sort_order: 1,
        menu_context: None,
        roles: vec![],
        permissions: vec!["roles.view"],
    },
    // ... dst untuk semua menu
];
