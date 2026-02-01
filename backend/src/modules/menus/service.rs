// ============================================================================
// backend/src/modules/menus/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreateMenuRequest, MenuResponse, UpdateMenuRequest};
use super::repository::MenuRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::menus;
use sea_orm::Set;
use validator::Validate;

#[derive(Clone)]
pub struct MenuService {
    repository: MenuRepository,
}

impl MenuService {
    pub fn new(repository: MenuRepository) -> Self {
        Self { repository }
    }

    /// Create new menu with validation
    pub async fn create(&self, request: CreateMenuRequest) -> Result<MenuResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Menu with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = menus::ActiveModel {
            foundation_id: Set(request.foundation_id),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(MenuResponse::from(created))
    }

    /// Get menu by ID
    pub async fn get_by_id(&self, id: i64) -> Result<MenuResponse, AppError> {
        let menu = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Menu not found".to_string()))?;

        Ok(MenuResponse::from(menu))
    }

    /// Get all menus with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<MenuResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<MenuResponse> = items.into_iter().map(MenuResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update menu
    pub async fn update(
        &self,
        id: i64,
        request: UpdateMenuRequest,
    ) -> Result<MenuResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Menu not found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Menu with this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = menus::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(MenuResponse::from(updated))
    }

    /// Delete menu
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Menu not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }

    pub async fn get_menus_for_user(&self, user: &User) -> Result<Vec<MenuTree>> {
        // 1. Ambil semua role_id user
        let user_role_ids = get_user_role_ids(self.repository.conn(), user.id).await?;

        // 2. Ambil semua permission_id dari role-role tersebut
        let user_permission_ids =
            get_permissions_from_roles(self.repository.conn(), &user_role_ids).await?;

        // 3. Query menus:
        //    - menu yang TIDAK punya entry di menu_roles (visible to all)
        //      OR menu yang menu_roles-nya OVERLAP dengan user_role_ids
        //    - AND: menu yang TIDAK punya entry di menu_permissions
        //      OR menu yang menu_permissions-nya OVERLAP dengan user_permission_ids
        //    - AND: is_active = true, deleted_at IS NULL
        //    - ORDER BY parent_id, sort_order
        let flat_menus =
            get_filtered_flat_menus(self.repository.conn(), &user_role_ids, &user_permission_ids)
                .await?;

        // 4. Build tree dari flat list
        let tree = build_menu_tree(&flat_menus, None); // parent_id = None = root

        Ok(tree)
    }

    // ============================================================
    // Entry point utama
    // ============================================================
    pub async fn get_menus_for_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<MenuTree>, Box<dyn std::error::Error>> {
        // 1. Ambil role_id yang dimiliki user
        let user_role_ids = Self::get_user_role_ids(self, user_id).await?;

        // 2. Ambil permission_id dari semua role user
        let user_permission_ids = Self::get_permissions_from_roles(self, &user_role_ids).await?;

        // 3. Query menus yang berhak diakses user
        let flat_menus =
            Self::get_filtered_flat_menus(self, &user_role_ids, &user_permission_ids).await?;

        // 4. Build tree dari flat list
        let tree = Self::build_menu_tree(&flat_menus, None);

        Ok(tree)
    }

    // ============================================================
    // Ambil role_id dari user
    // Asumsi: kamu punya tabel user_roles atau relasi users -> roles
    // Adjust sesuai entity kamu
    // ============================================================
    async fn get_user_role_ids(
        &self,
        user_id: i64,
    ) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
        use crate::entities::user_roles;

        let role_ids: Vec<i64> = user_roles::Entity::find()
            .filter(user_roles::Column::UserId.eq(user_id))
            .all(self.repository.conn())
            .await?
            .iter()
            .map(|row| row.role_id)
            .collect();

        Ok(role_ids)
    }

    // ============================================================
    // Ambil permission_id dari sekumpulan role
    // Asumsi: kamu punya tabel role_permissions
    // ============================================================
    async fn get_permissions_from_roles(
        &self,
        role_ids: &[i64],
    ) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
        use crate::entities::role_permissions;

        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        let permission_ids: Vec<i64> = role_permissions::Entity::find()
            .filter(role_permissions::Column::RoleId.is_in(role_ids.to_vec()))
            .all(self.repository.conn())
            .await?
            .iter()
            .map(|row| row.permission_id)
            .collect();

        Ok(permission_ids)
    }

    // ============================================================
    // CORE: Query menus yang boleh dilihat user
    //
    // Logic filtering:
    //   Menu VISIBLE kalau:
    //     (tidak ada entry di menu_roles)            <- visible to all
    //     OR (menu_roles overlap dengan user roles)  <- user punya role yang match
    //   AND:
    //     (tidak ada entry di menu_permissions)              <- tidak butuh permission
    //     OR (menu_permissions overlap dengan user perms)    <- user punya permission
    //   AND:
    //     is_active = true
    //     deleted_at IS NULL
    // ============================================================
    async fn get_filtered_flat_menus(
        &self,
        user_role_ids: &[i64],
        user_permission_ids: &[i64],
    ) -> Result<Vec<FlatMenu>, Box<dyn std::error::Error>> {
        // --- Step A: Load semua menu_roles entries ---
        let all_menu_role_entries: Vec<(i64, i64)> = menu_roles::Entity::find()
            .all(self.repository.conn())
            .await?
            .iter()
            .map(|row| (row.menu_id, row.role_id))
            .collect();

        // Set of menu_id yang PUNYA role restriction
        let menus_with_roles: HashSet<i64> = all_menu_role_entries
            .iter()
            .map(|(menu_id, _)| *menu_id)
            .collect();

        // Set of menu_id yang role-nya MATCH dengan user
        let menus_role_matched: HashSet<i64> = all_menu_role_entries
            .iter()
            .filter(|(_, role_id)| user_role_ids.contains(role_id))
            .map(|(menu_id, _)| *menu_id)
            .collect();

        // --- Step B: Load semua menu_permissions entries ---
        let all_menu_perm_entries: Vec<(i64, i64)> = menu_permissions::Entity::find()
            .all(self.repository.conn())
            .await?
            .iter()
            .map(|row| (row.menu_id, row.permission_id))
            .collect();

        // Set of menu_id yang PUNYA permission restriction
        let menus_with_permissions: HashSet<i64> = all_menu_perm_entries
            .iter()
            .map(|(menu_id, _)| *menu_id)
            .collect();

        // Set of menu_id yang permission-nya MATCH dengan user
        let menus_perm_matched: HashSet<i64> = all_menu_perm_entries
            .iter()
            .filter(|(_, perm_id)| user_permission_ids.contains(perm_id))
            .map(|(menu_id, _)| *menu_id)
            .collect();

        // --- Step C: Query semua menu aktif dari DB ---
        let all_menus: Vec<menus::Model> = menus::Entity::find()
            .filter(menus::Column::IsActive.eq(true))
            .filter(menus::Column::DeletedAt.is_null())
            .order_by_asc(menus::Column::ParentId)
            .order_by_asc(menus::Column::SortOrder)
            .all(self.repository.conn())
            .await?;

        // --- Step D: Filter berdasarkan role & permission ---
        let filtered: Vec<FlatMenu> = all_menus
            .into_iter()
            .filter(|menu| {
                // Role check:
                //   - Kalau menu tidak punya role restriction -> PASS
                //   - Kalau punya restriction, user harus punya role yang match
                let role_ok =
                    !menus_with_roles.contains(&menu.id) || menus_role_matched.contains(&menu.id);

                // Permission check:
                //   - Kalau menu tidak punya permission restriction -> PASS
                //   - Kalau punya restriction, user harus punya permission yang match
                let perm_ok = !menus_with_permissions.contains(&menu.id)
                    || menus_perm_matched.contains(&menu.id);

                role_ok && perm_ok
            })
            .map(|m| FlatMenu {
                id: m.id,
                key: m.key,
                label: m.label,
                href: m.href,
                icon: m.icon,
                parent_id: m.parent_id,
                sort_order: m.sort_order,
                menu_context: m.menu_context,
            })
            .collect();

        Ok(filtered)
    }

    // ============================================================
    // CORE: Build tree dari flat list (recursive)
    //
    // Cara kerja:
    //   1. Cari semua menu yang parent_id == current_parent_id
    //   2. Untuk setiap menu, recursively cari children-nya
    //   3. Kalau menu adalah "group" (href = None) dan
    //      tidak punya children setelah filtering
    //      -> buang, jangan tampilkan parent kosong
    // ============================================================
    fn build_menu_tree(flat_menus: &[FlatMenu], parent_id: Option<i64>) -> Vec<MenuTree> {
        let mut tree: Vec<MenuTree> = Vec::new();

        for menu in flat_menus {
            // Cek apakah parent_id menu ini cocok dengan yang kita cari
            let is_match = match (menu.parent_id, parent_id) {
                (None, None) => true,         // keduanya root
                (Some(a), Some(b)) => a == b, // sama parent
                _ => false,                   // tidak cocok
            };

            if !is_match {
                continue;
            }

            // Recursively cari children dari menu ini
            let children = Self::build_menu_tree(flat_menus, Some(menu.id));

            // Kalau ini group (tidak clickable) tapi tidak ada children
            // setelah filtering -> jangan tampilkan
            if menu.href.is_none() && children.is_empty() {
                continue;
            }

            tree.push(MenuTree {
                key: menu.key.clone(),
                label: menu.label.clone(),
                href: menu.href.clone(),
                icon: menu.icon.clone(),
                menu_context: menu.menu_context.clone(),
                children,
            });
        }

        tree
    }
}
