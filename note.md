<!-- jalankan ini untuk generate entity dari database -->
### 1. generate entity
sea-orm-cli generate entity -u mysql://taryono:password@localhost/actix_db -o entity_temp --with-serde both --date-time-crate chrono


cd /home/taryono/rust/silsilah/backend

### 2.  struktur folder
mkdir -p src/{config,controllers,errors,middleware,models,routes,services,utils}

### 3. Buat file mod.rs
touch src/config/mod.rs
touch src/controllers/mod.rs
touch src/errors/mod.rs
touch src/middleware/mod.rs
touch src/models/mod.rs
touch src/routes/mod.rs
touch src/services/mod.rs
touch src/utils/mod.rs
Lihat struktur project Mode direktori saja (tanpa file)
tree -d -I "vendor|node_modules|target|entity_tmp|migrations"

# /etc/nginx/sites-available/your-api

server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
C. Add SSL Certificate dengan Let's Encrypt
# Install certbot
sudo apt install certbot python3-certbot-nginx

# Get SSL certificate
sudo certbot --nginx -d api.yourdomain.com

# Auto-renew
sudo certbot renew --dry-run

Opsi 2: Deploy ke Platform Cloud
A. Railway.app (Paling Mudah)

# Install Railway CLI
npm i -g @railway/cli

# Login
railway login

# Deploy
railway up

B. Fly.io
Buat file fly.toml:

app = "your-api-name"

[build]
  builder = "paketobuildpacks/builder:base"

[[services]]
  http_checks = []
  internal_port = 8080
  protocol = "tcp"

  [[services.ports]]
    handlers = ["http"]
    port = 80

  [[services.ports]]
    handlers = ["tls", "http"]
    port = 443

    fly launch
fly deploy


Sama-sama! Senang sudah berhasil! 🎉

**Ya, Rust dengan SeaORM mendukung soft delete!** Bahkan lebih powerful dari Laravel karena bisa dikustomisasi via trait. Mari saya jelaskan beberapa pendekatan:

## Opsi 1: Menggunakan `ActiveModelBehavior` Trait (Seperti Laravel Trait)

SeaORM punya trait `ActiveModelBehavior` yang mirip konsepnya dengan trait di Laravel.## Opsi 2: Global Scope dengan Query Filter (Lebih Otomatis)

Kalau mau lebih otomatis seperti global scope Laravel:

```rust
// src/traits/soft_delete_scope.rs
use sea_orm::{sea_query::IntoCondition, Condition, EntityTrait, QueryFilter, Select};

pub trait SoftDeleteScope: EntityTrait {
    fn deleted_at_col() -> Self::Column;
    
    /// Override find() untuk auto-exclude soft deleted
    fn find_active() -> Select<Self> {
        Self::find().filter(
            Condition::all()
                .add(Self::deleted_at_col().is_null())
        )
    }
}

// Implementasi di entity
impl SoftDeleteScope for users::Entity {
    fn deleted_at_col() -> users::Column {
        users::Column::DeletedAt
    }
}

// Penggunaan
let users = users::Entity::find_active().all(db).await?;
```

## Opsi 3: Menggunakan Macro (Paling Praktis)

Buat macro untuk auto-implement trait:

```rust
// src/macros/soft_delete.rs
#[macro_export]
macro_rules! impl_soft_delete {
    ($entity:ty, $column:expr) => {
        impl $crate::traits::soft_delete::SoftDelete for $entity {
            fn deleted_at_col() -> <Self as sea_orm::EntityTrait>::Column {
                $column
            }
        }
    };
}

// Penggunaan di entity
impl_soft_delete!(users::Entity, users::Column::DeletedAt);
impl_soft_delete!(posts::Entity, posts::Column::DeletedAt);
impl_soft_delete!(comments::Entity, comments::Column::DeletedAt);
```

## Perbandingan dengan Laravel

| Feature | Laravel | SeaORM + Trait |
|---------|---------|----------------|
| **Soft Delete** | `use SoftDeletes;` | `impl SoftDelete for Entity` |
| **Auto Scope** | Otomatis | Manual via `find_not_deleted()` |
| **Restore** | `$model->restore()` | `Entity::restore(db, id)` |
| **Force Delete** | `$model->forceDelete()` | `Entity::force_delete(db, id)` |
| **Only Trashed** | `Model::onlyTrashed()` | `Entity::find_only_deleted()` |
| **With Trashed** | `Model::withTrashed()` | `Entity::find_with_deleted()` |
| **Custom Column** | `const DELETED_AT` | `fn deleted_at_col()` |

## SQL Migration untuk Soft Delete

```sql
-- Tambah kolom deleted_at ke tabel yang perlu soft delete
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMP NULL DEFAULT NULL;
ALTER TABLE posts ADD COLUMN deleted_at TIMESTAMP NULL DEFAULT NULL;

-- Buat index untuk performa
CREATE INDEX idx_users_deleted_at ON users(deleted_at);
CREATE INDEX idx_posts_deleted_at ON posts(deleted_at);
```

## Dependency yang Perlu Ditambahkan

```toml
[dependencies]
async-trait = "0.1"  # Untuk async trait
chrono = { workspace = true }
```

## Struktur File yang Perlu Dibuat

```
src/
├── traits/
│   ├── mod.rs
│   └── soft_delete.rs        # ← Trait SoftDelete
├── macros/
│   ├── mod.rs
│   └── soft_delete.rs        # ← Macro helper (optional)
└── main.rs
```

**Di `src/main.rs`**, tambahkan:
```rust
mod traits;
// mod macros;  // Jika pakai macro
```

## Tips & Best Practices

1. **Buat index** di kolom `deleted_at` untuk performa query
2. **Konsisten** gunakan `find_not_deleted()` di semua query
3. **Logging** saat soft delete untuk audit trail
4. **Cron job** untuk cleanup permanent delete setelah X hari
5. **Permission** berbeda untuk soft delete vs force delete

Mau saya buatkan implementasi lengkap dengan migration dan contoh CRUD yang lebih detail?

├── entity
│   └── src
│       └── traits
├── scripts
└── src
    ├── config
    ├── controllers
    ├── docs
    ├── errors
    ├── macros
    ├── middleware
    ├── models
    ├── routes
    ├── services
    └── utils

├── entity
│   └── src
│       └── traits
├── scripts
└── src
    ├── config
    ├── modules
    |       |
    |       |__users
    |          |___service.rs
    |          |___docs.rs
    |          |___models.rs
    |          |___routes.rs
    |          roles
    |          |___service.rs
    |          |___docs.rs
    |          |___models.rs
    |          |___routes.rs
    ├── errors
    ├── macros
    ├── middleware
    └── utils

🛠️ Opsi Implementasi Generator

Aku sarankan script Rust (binary)
📍 Lokasi: scripts/gen_module.rs
Cara Pakai
cargo run --bin gen_module users
cargo run --bin gen_module employees
cargo run --bin gen_module positions 

Di Rust ada beberapa cara untuk "dump and die" seperti dd() di Laravel:🎯 Opsi Debug di Rust:1. dbg!() Macro (Paling Mirip dd())

let user = User::find_by_id(1).one(db).await?;

// ✅ Print value dan continue
dbg!(&user);

// ✅ Print value dan assign ke variable
let user = dbg!(user);

// ✅ Print multiple values
dbg!(&user, &roles, &permissions);

// Output:
// [src/main.rs:42] &user = Some(
//     Model {
//         id: 1,
//         name: "John",
//         email: "john@example.com",
//     }
// )

 // Load roles via many-to-many relation
let roles = user.find_related(Roles).all(self.repository.conn()).await?;


find_related ini akan muncul error 

no method named find_related found for struct entity::users::Model in the current scope --> src/modules/users/service.rs:352:26 | 352 | let roles = user.find_related(Roles).all(self.repository.conn()).await?;

selama belum di import 

use tracing::{debug, info, warn, error};

let user = User::find_by_id(1).one(db).await?;

// ✅ Log dengan level berbeda
debug!("User: {:?}", user);
info!("User found: {:#?}", user);
warn!("Checking user: {:?}", user);
error!("User error: {:?}", user);

// src/handlers/teacher_handler.rs
use actix_web::{web, HttpResponse, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Related};
use serde::{Deserialize, Serialize};

use crate::entities::{teacher, teacher_assignment, teacher_salary, prelude::*};

#[derive(Deserialize)]
pub struct GetTeachersQuery {
    pub school_id: Option<i64>,
    pub employment_status: Option<String>,
    pub is_active: Option<bool>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Serialize)]
pub struct TeacherResponse {
    pub id: i64,
    pub nip: Option<String>,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_active: bool,
    pub assignments: Vec<AssignmentResponse>,
}

#[derive(Serialize)]
pub struct AssignmentResponse {
    pub school_name: String,
    pub employment_status: String,
    pub position: String,
    pub teaching_hours: i32,
    pub subjects: Vec<SubjectResponse>,
}

// GET /api/teachers
pub async fn get_teachers(
    db: web::Data<DatabaseConnection>,
    query: web::Query<GetTeachersQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    
    let mut query_builder = Teacher::find();
    
    if let Some(is_active) = query.is_active {
        query_builder = query_builder.filter(teacher::Column::IsActive.eq(is_active));
    }
    
    // Join with assignments if school_id filter is present
    if let Some(school_id) = query.school_id {
        query_builder = query_builder
            .find_also_related(TeacherAssignment)
            .filter(teacher_assignment::Column::SchoolId.eq(school_id));
    }
    
    let teachers = query_builder
        .paginate(&**db, per_page)
        .fetch_page(page - 1)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(teachers))
}

// GET /api/teachers/:id
pub async fn get_teacher_detail(
    db: web::Data<DatabaseConnection>,
    teacher_id: web::Path<i64>,
) -> Result<HttpResponse> {
    let teacher = Teacher::find_by_id(*teacher_id)
        .one(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Teacher not found"))?;
    
    // Load assignments with related data
    let assignments = teacher
        .find_related(TeacherAssignment)
        .find_with_related(School)
        .all(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(teacher))
}

// POST /api/teachers
#[derive(Deserialize)]
pub struct CreateTeacherRequest {
    pub foundation_id: i64,
    pub nip: Option<String>,
    pub nik: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub join_date: String,
}

pub async fn create_teacher(
    db: web::Data<DatabaseConnection>,
    req: web::Json<CreateTeacherRequest>,
) -> Result<HttpResponse> {
    let teacher = teacher::ActiveModel {
        foundation_id: Set(req.foundation_id),
        nip: Set(req.nip.clone()),
        nik: Set(req.nik.clone()),
        name: Set(req.name.clone()),
        email: Set(req.email.clone()),
        phone: Set(req.phone.clone()),
        is_active: Set(true),
        join_date: Set(req.join_date.parse().unwrap()),
        ..Default::default()
    };
    
    let result = teacher
        .insert(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Created().json(result))
}

// POST /api/teachers/:id/assignments
#[derive(Deserialize)]
pub struct CreateAssignmentRequest {
    pub school_id: i64,
    pub academic_year_id: i64,
    pub employment_status: String,
    pub employment_type: String,
    pub position: String,
    pub department_id: Option<i64>,
    pub assignment_start_date: String,
}

pub async fn create_teacher_assignment(
    db: web::Data<DatabaseConnection>,
    teacher_id: web::Path<i64>,
    req: web::Json<CreateAssignmentRequest>,
) -> Result<HttpResponse> {
    let assignment = teacher_assignment::ActiveModel {
        teacher_id: Set(*teacher_id),
        school_id: Set(req.school_id),
        academic_year_id: Set(req.academic_year_id),
        employment_status: Set(req.employment_status.clone()),
        employment_type: Set(req.employment_type.clone()),
        position: Set(req.position.clone()),
        department_id: Set(req.department_id),
        assignment_start_date: Set(req.assignment_start_date.parse().unwrap()),
        is_active: Set(true),
        teaching_hours_per_week: Set(0),
        max_teaching_hours: Set(24),
        ..Default::default()
    };
    
    let result = assignment
        .insert(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Created().json(result))
}

// GET /api/teachers/:id/salary
pub async fn get_teacher_salary(
    db: web::Data<DatabaseConnection>,
    teacher_id: web::Path<i64>,
) -> Result<HttpResponse> {
    // Get active assignment
    let assignment = TeacherAssignment::find()
        .filter(teacher_assignment::Column::TeacherId.eq(*teacher_id))
        .filter(teacher_assignment::Column::IsActive.eq(true))
        .one(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Active assignment not found"))?;
    
    // Get salary info
    let salary = assignment
        .find_related(TeacherSalary)
        .one(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(salary))
}

// POST /api/salary-payments
#[derive(Deserialize)]
pub struct CreateSalaryPaymentRequest {
    pub teacher_salary_id: i64,
    pub payment_period_month: i32,
    pub payment_period_year: i32,
    pub working_days: i32,
    pub present_days: i32,
    pub bonus: Option<Decimal>,
}

pub async fn create_salary_payment(
    db: web::Data<DatabaseConnection>,
    req: web::Json<CreateSalaryPaymentRequest>,
) -> Result<HttpResponse> {
    // Get salary configuration
    let salary = TeacherSalary::find_by_id(req.teacher_salary_id)
        .one(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Salary not found"))?;
    
    // Calculate attendance-based salary
    let attendance_ratio = req.present_days as f64 / req.working_days as f64;
    let calculated_base = salary.base_salary * Decimal::from_f64(attendance_ratio).unwrap();
    
    let payment = salary_payment::ActiveModel {
        teacher_salary_id: Set(req.teacher_salary_id),
        payment_period_month: Set(req.payment_period_month),
        payment_period_year: Set(req.payment_period_year),
        base_salary: Set(calculated_base),
        total_allowances: Set(salary.gross_salary - salary.base_salary),
        total_deductions: Set(salary.tax_deduction + salary.insurance_deduction),
        gross_salary: Set(salary.gross_salary),
        net_salary: Set(salary.net_salary + req.bonus.unwrap_or(Decimal::ZERO)),
        working_days: Set(req.working_days),
        present_days: Set(req.present_days),
        bonus: Set(req.bonus.unwrap_or(Decimal::ZERO)),
        payment_status: Set("PENDING".to_string()),
        ..Default::default()
    };
    
    let result = payment
        .insert(&**db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Created().json(result))
}

// Configure routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/teachers")
            .route("", web::get().to(get_teachers))
            .route("", web::post().to(create_teacher))
            .route("/{id}", web::get().to(get_teacher_detail))
            .route("/{id}/assignments", web::post().to(create_teacher_assignment))
            .route("/{id}/salary", web::get().to(get_teacher_salary))
    )
    .service(
        web::scope("/api/salary-payments")
            .route("", web::post().to(create_salary_payment))
    );
}