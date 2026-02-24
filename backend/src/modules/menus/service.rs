// ============================================================================
// backend/src/modules/menus/service.rs
// ============================================================================
use super::dto::{CreateMenuRequest, FlatMenu, MenuResponse, MenuTree, UpdateMenuRequest};
use super::repository::MenuRepository;
use crate::context::service_context::ServiceContext;
use crate::errors::AppError;
use crate::traits::AuditFields;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::menu_permissions;
use entity::menu_roles;
use entity::{menus, role_permissions, role_users};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set};
use std::collections::HashSet;
use validator::Validate;

#[derive(Clone)]
pub struct MenuService {
    repository: MenuRepository,
}

impl MenuService {
    pub fn new(repository: MenuRepository) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        ctx: &ServiceContext,
        request: CreateMenuRequest,
    ) -> Result<MenuResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        if self
            .repository
            .find_by_name(&request.label, ctx.foundation_id)
            .await?
            .is_some()
        {
            return Err(AppError::conflict(
                "Menu with this label already exists".to_string(),
            ));
        }

        let active_model = menus::ActiveModel {
            key: Set(request.key),
            label: Set(request.label),
            href: Set(request.href),
            icon: Set(request.icon),
            parent_id: Set(request.parent_id), // ✅ tambah
            sort_order: Set(request.sort_order.unwrap_or(0)), // ✅ tambah
            menu_context: Set(request.menu_context),
            ..Default::default()
        }
        .set_foundation(ctx)
        .set_created_by(ctx)
        .set_updated_at(ctx);

        Ok(MenuResponse::from(
            self.repository.create(active_model).await?,
        ))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<MenuResponse, AppError> {
        let menu = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Menu not found".to_string()))?;

        Ok(MenuResponse::from(menu))
    }

    pub async fn get_all(
        &self,
        ctx: &ServiceContext,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<MenuResponse>, AppError> {
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, ctx.foundation_id).await?;

        let responses = items.into_iter().map(MenuResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    pub async fn update(
        &self,
        id: i64,
        request: UpdateMenuRequest,
    ) -> Result<MenuResponse, AppError> {
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Menu not found".to_string()))?;

        // Cek duplikat label jika label berubah
        if let Some(ref label) = request.label {
            if *label != existing.label {
                if self
                    .repository
                    .find_by_name(label, existing.foundation_id)
                    .await?
                    .is_some()
                {
                    return Err(AppError::conflict(
                        "Menu with this label already exists".to_string(),
                    ));
                }
            }
        }

        // ✅ Semua field diupdate, bukan hanya label
        let mut active_model = existing.into_active_model();

        if let Some(key) = request.key {
            active_model.key = Set(key);
        }
        if let Some(label) = request.label {
            active_model.label = Set(label);
        }
        if let Some(href) = request.href {
            active_model.href = Set(Some(href));
        }
        if let Some(icon) = request.icon {
            active_model.icon = Set(Some(icon));
        }
        if let Some(parent_id) = request.parent_id {
            active_model.parent_id = Set(Some(parent_id));
        }
        if let Some(sort_order) = request.sort_order {
            active_model.sort_order = Set(sort_order);
        }
        if let Some(menu_context) = request.menu_context {
            active_model.menu_context = Set(Some(menu_context));
        }

        active_model.updated_at = Set(chrono::Utc::now());

        let updated = self.repository.update(id, active_model).await?;
        Ok(MenuResponse::from(updated))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Menu not found".to_string()))?;

        self.repository.delete(id).await
    }

    // ============================================================
    // Menu tree untuk user — return AppError bukan Box<dyn Error>
    // ============================================================
    pub async fn get_menus_for_user(&self, user_id: i64) -> Result<Vec<Box<MenuTree>>, AppError> {
        // ✅ konsisten pakai AppError
        let user_role_ids = self.get_user_role_ids(user_id).await?;
        let user_permission_ids = self.get_permissions_from_roles(&user_role_ids).await?;
        let flat_menus = self
            .get_filtered_flat_menus(&user_role_ids, &user_permission_ids)
            .await?;

        Ok(Self::build_menu_tree(&flat_menus, None))
    }

    async fn get_user_role_ids(&self, user_id: i64) -> Result<Vec<i64>, AppError> {
        let role_ids = role_users::Entity::find()
            .filter(role_users::Column::UserId.eq(user_id))
            .all(self.repository.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(|row| row.role_id)
            .collect();

        Ok(role_ids)
    }

    async fn get_permissions_from_roles(&self, role_ids: &[i64]) -> Result<Vec<i64>, AppError> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        let permission_ids = role_permissions::Entity::find()
            .filter(role_permissions::Column::RoleId.is_in(role_ids.to_vec()))
            .all(self.repository.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(|row| row.permission_id)
            .collect();

        Ok(permission_ids)
    }

    async fn get_filtered_flat_menus(
        &self,
        user_role_ids: &[i64],
        user_permission_ids: &[i64],
    ) -> Result<Vec<FlatMenu>, AppError> {
        // Load menu_roles entries
        let all_menu_role_entries: Vec<(i64, i64)> = menu_roles::Entity::find()
            .all(self.repository.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .into_iter() // ✅ into_iter() bukan iter() agar tidak perlu clone
            .map(|row| (row.menu_id, row.role_id))
            .collect();

        let menus_with_roles: HashSet<i64> = all_menu_role_entries
            .iter()
            .map(|(menu_id, _)| *menu_id)
            .collect();

        let menus_role_matched: HashSet<i64> = all_menu_role_entries
            .iter()
            .filter(|(_, role_id)| user_role_ids.contains(role_id))
            .map(|(menu_id, _)| *menu_id)
            .collect();

        // Load menu_permissions entries
        let all_menu_perm_entries: Vec<(i64, i64)> = menu_permissions::Entity::find()
            .all(self.repository.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .into_iter() // ✅ into_iter()
            .map(|row| (row.menu_id, row.permission_id))
            .collect();

        let menus_with_permissions: HashSet<i64> = all_menu_perm_entries
            .iter()
            .map(|(menu_id, _)| *menu_id)
            .collect();

        let menus_perm_matched: HashSet<i64> = all_menu_perm_entries
            .iter()
            .filter(|(_, perm_id)| user_permission_ids.contains(perm_id))
            .map(|(menu_id, _)| *menu_id)
            .collect();

        // Query semua menu aktif
        let all_menus = menus::Entity::find()
            .filter(menus::Column::IsActive.eq(true))
            .filter(menus::Column::DeletedAt.is_null())
            .order_by_asc(menus::Column::ParentId)
            .order_by_asc(menus::Column::SortOrder)
            .all(self.repository.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let filtered = all_menus
            .into_iter()
            .filter(|menu| {
                let role_ok =
                    !menus_with_roles.contains(&menu.id) || menus_role_matched.contains(&menu.id);
                let perm_ok = !menus_with_permissions.contains(&menu.id)
                    || menus_perm_matched.contains(&menu.id);
                role_ok && perm_ok
            })
            .map(FlatMenu::from) // ✅ pakai From trait yang sudah diimplementasi
            .collect();

        Ok(filtered)
    }

    fn build_menu_tree(flat_menus: &[FlatMenu], parent_id: Option<i64>) -> Vec<Box<MenuTree>> {
        flat_menus
            .iter()
            .filter(|menu| menu.parent_id == parent_id)
            .filter_map(|menu| {
                let children = Self::build_menu_tree(flat_menus, Some(menu.id));

                // Buang group kosong (tidak clickable dan tidak ada children)
                if menu.href.is_none() && children.is_empty() {
                    return None;
                }

                Some(Box::new(MenuTree {
                    id: menu.id, // ✅ tambah id
                    key: menu.key.clone(),
                    label: menu.label.clone(),
                    href: menu.href.clone(),
                    icon: menu.icon.clone(),
                    sort_order: menu.sort_order, // ✅ tambah sort_order
                    menu_context: menu.menu_context.clone(),
                    children,
                }))
            })
            .collect()
    }
}
