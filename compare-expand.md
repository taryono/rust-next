# 🏫 PROJECT INSTRUCTIONS — Sekolah Modern (SaaS Billing PPDB)

---

## 🎯 Identitas Project

- **Nama Project:** Sekolah Modern
- **Tipe:** SaaS (Software as a Service) — Multi-foundation
- **Domain:** Sistem Billing & Manajemen Pembayaran PPDB (Penerimaan Peserta Didik Baru) untuk lembaga pendidikan di Indonesia
- **Target User:** Yayasan / Sekolah Swasta sebagai Admin Foundation
- **Bahasa Komunikasi:** Indonesia (Bahasa Indonesia)

---

## 👨‍💻 Profil Developer

- Backend-leaning full-stack developer
- Familiar dengan Laravel (PHP), Rust, Next.js, dan jQuery
- Berlokasi di Brebes, Jawa Tengah, Indonesia
- Konvensi komunikasi: santai tapi teknikal, langsung ke inti masalah

---

## 🛠️ Tech Stack

### Backend — Rust
| Komponen | Library | Versi |
|---|---|---|
| Web Framework | Actix-web | 4 |
| ORM | SeaORM | 2.0.0-rc.30 |
| Database | MySQL via sqlx | - |
| Auth | jsonwebtoken + bcrypt | 9 / 0.15 |
| Validation | validator | 0.18 |
| API Docs | utoipa + swagger-ui | 5 / 8 |
| File Upload | actix-multipart | 0.7.2 |
| Rate Limit | actix-governor | 0.5 |
| UUID | uuid v4 | 1.0 |
| Error Handling | anyhow + thiserror | 1.0 |
| Serialization | serde + serde_json | 1.0 |
| DateTime | chrono | 0.4 |
| Async | tokio (full) | 1 |
| Logging | tracing + tracing-subscriber | 0.1 / 0.3 |

### Frontend — Next.js
- Framework: **Next.js** (React)
- Styling: TBD (Tailwind CSS direkomendasikan)
- State Management: TBD
- HTTP Client: fetch / axios ke Rust REST API

### Infrastructure
- Database: **MySQL**
- Cache: Redis (planned — untuk feature flag caching)
- Deployment: TBD

---

## 🏗️ Arsitektur Utama

### Multi-Tenancy
- **Strategi:** Single Database + `foundation_id` di semua tabel
- Setiap yayasan/sekolah = 1 foundation dengan konfigurasi sendiri
- Foundation diidentifikasi via: **Header `X-Foundation-ID`** atau subdomain
- Yayasan dapat berupa Sekolah Swasta Tanpa boarding class
- Yayasan dapat berupa Sekolah dan ada Boarding class
- Yayasan dapat berupa Pondok Pesantren kombinasi Sekolah reguler SD,SMP,SMA

### Feature Toggle (Policy-Driven)
- Regulasi billing dikontrol dari **database**, bukan hardcode
- Admin yayasan bisa ON/OFF fitur tanpa perlu deploy ulang
- Semua feature key terdefinisi dalam database

### Billing Engine
- Rule engine berbasis database (bukan if-else hardcode)
- Mendukung multi-komponen biaya, multi-diskon, dan multiple skema cicilan
- Setiap kalkulasi menghasilkan `discount_breakdown` untuk audit trail

---

## 📁 Struktur Folder Backend (Rust)
.
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── build.rs
├── entity
│   ├── Cargo.toml
│   └── src
│       ├── academic_calendars.rs
│       ├── academic_years.rs
│       ├── applicants.rs
│       ├── attendances.rs
│       ├── character_assessments.rs
│       ├── character_traits.rs
│       ├── class_levels.rs
│       ├── class_schedules.rs
│       ├── class_subjects.rs
│       ├── classes.rs
│       ├── classrooms.rs
│       ├── departments.rs
│       ├── employees.rs
│       ├── extracurricular_activities.rs
│       ├── extracurricular_enrollments.rs
│       ├── facilities.rs
│       ├── foundation_regulations.rs
│       ├── foundation_types.rs
│       ├── foundations.rs
│       ├── grades.rs
│       ├── homeroom_teachers.rs
│       ├── levels.rs
│       ├── lib.rs
│       ├── members.rs
│       ├── menu_permissions.rs
│       ├── menu_roles.rs
│       ├── menus.rs
│       ├── mod.rs
│       ├── notifications.rs
│       ├── permissions.rs
│       ├── positions.rs
│       ├── prelude.rs
│       ├── regulations.rs
│       ├── report_cards.rs
│       ├── role_permissions.rs
│       ├── role_users.rs
│       ├── roles.rs
│       ├── room_facilities.rs
│       ├── room_types.rs
│       ├── rooms.rs
│       ├── salary_grades.rs
│       ├── salary_payments.rs
│       ├── schedule_generation_logs.rs
│       ├── scopes.rs
│       ├── sea_orm_active_enums.rs
│       ├── semesters.rs
│       ├── settings.rs
│       ├── student_enrollments.rs
│       ├── students.rs
│       ├── subject_preferences.rs
│       ├── subject_room_requirements.rs
│       ├── subjects.rs
│       ├── teacher_assignments.rs
│       ├── teacher_availability.rs
│       ├── teacher_salaries.rs
│       ├── teacher_subjects.rs
│       ├── teachers.rs
│       ├── time_slots.rs
│       ├── traits
│       │   ├── mod.rs
│       │   └── soft_delete.rs
│       ├── unit_types.rs
│       ├── units.rs
│       ├── user_permissions.rs
│       ├── user_profiles.rs
│       └── users.rs
├── entity_temp
│   ├── academic_calendars.rs
│   ├── academic_years.rs
│   ├── assessment_components.rs
│   ├── attendances.rs
│   ├── character_assessments.rs
│   ├── character_traits.rs
│   ├── class_levels.rs
│   ├── class_schedules.rs
│   ├── class_subjects.rs
│   ├── classes.rs
│   ├── classrooms.rs
│   ├── departments.rs
│   ├── employees.rs
│   ├── extracurricular_activities.rs
│   ├── extracurricular_enrollments.rs
│   ├── extracurricular_grades.rs
│   ├── extracurriculars.rs
│   ├── facilities.rs
│   ├── foundation_regulations.rs
│   ├── foundation_types.rs
│   ├── foundations.rs
│   ├── grades.rs
│   ├── homeroom_teachers.rs
│   ├── levels.rs
│   ├── members.rs
│   ├── menu_permissions.rs
│   ├── menu_roles.rs
│   ├── menus.rs
│   ├── mod.rs
│   ├── notifications.rs
│   ├── permissions.rs
│   ├── positions.rs
│   ├── prelude.rs
│   ├── regulations.rs
│   ├── report_card_details.rs
│   ├── report_cards.rs
│   ├── role_permissions.rs
│   ├── role_users.rs
│   ├── roles.rs
│   ├── room_facilities.rs
│   ├── room_types.rs
│   ├── rooms.rs
│   ├── salary_grades.rs
│   ├── salary_payments.rs
│   ├── schedule_generation_logs.rs
│   ├── scopes.rs
│   ├── sea_orm_active_enums.rs
│   ├── semesters.rs
│   ├── settings.rs
│   ├── student_enrollments.rs
│   ├── students.rs
│   ├── subject_preferences.rs
│   ├── subject_room_requirements.rs
│   ├── subjects.rs
│   ├── teacher_assignments.rs
│   ├── teacher_availability.rs
│   ├── teacher_salaries.rs
│   ├── teacher_subjects.rs
│   ├── teachers.rs
│   ├── time_slots.rs
│   ├── unit_types.rs
│   ├── units.rs
│   ├── user_permissions.rs
│   ├── user_profiles.rs
│   └── users.rs
├── scripts
│   ├── gen_module.rs
│   └── generate-entities.sh
├── src
│   ├── app_state.rs
│   ├── config
│   │   ├── database.rs
│   │   └── mod.rs
│   ├── context
│   │   ├── mod.rs
│   │   └── service_context.rs
│   ├── docs
│   │   └── mod.rs
│   ├── errors
│   │   ├── app_error.rs
│   │   └── mod.rs
│   ├── macros
│   │   ├── debug.rs
│   │   ├── mod.rs
│   │   ├── soft_delete.rs
│   │   └── with_context.rs
│   ├── main.rs
│   ├── middleware
│   │   ├── auth.rs
│   │   ├── mod.rs
│   │   └── swagger_auth.rs
│   ├── modules
│   │   ├── academic_years
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── applicants
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── attendances
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── auth
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── class_levels
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── classes
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── departments
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── employees
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── foundation_regulations
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── foundation_types
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── foundations
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── menus
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── mod.rs
│   │   ├── permissions
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── positions
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── regulations
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── roles
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── rooms
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── semesters
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── settings
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── students
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── subjects
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── teachers
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── unit_types
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── units
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   ├── user_profiles
│   │   │   ├── docs.rs
│   │   │   ├── dto.rs
│   │   │   ├── handler.rs
│   │   │   ├── mod.rs
│   │   │   ├── repository.rs
│   │   │   ├── routes.rs
│   │   │   └── service.rs
│   │   └── users
│   │       ├── docs.rs
│   │       ├── dto.rs
│   │       ├── dto_multipart.rs
│   │       ├── handler.rs
│   │       ├── mod.rs
│   │       ├── repository.rs
│   │       ├── routes.rs
│   │       └── service.rs
│   ├── routes
│   │   └── mod.rs
│   ├── seeds
│   │   └── menu_seeder.rs
│   ├── states
│   │   ├── mod.rs
│   │   └── states.rs
│   ├── traits
│   │   ├── audit.rs
│   │   └── mod.rs
│   └── utils
│       ├── date.rs
│       ├── jwt.rs
│       ├── mod.rs
│       ├── pagination.rs
│       ├── pagination_helper.rs
│       ├── password.rs
│       └── response.rs
└── uploads
    └── users
        ├── 06baa840-a445-4a19-b82e-41ba44bb0948_jobstreet.png 

## 🗄️ Skema Database (Rancangan)

### Layer 1 — Multi-Tenancy
```sql
foundations
├── id (UUID)
├── name
├── slug                  -- subdomain
├── plan                  -- free | basic | pro
├── is_active
└── settings (JSON)

foundation_features
├── id (UUID)
├── foundation_id (FK)
├── feature_key           -- "billing.discount_prestasi"
├── is_enabled            -- toggle ON/OFF
├── config (JSON)         -- konfigurasi spesifik
└── enabled_at, enabled_by
```

### Layer 2 — Komponen Biaya
```sql

components
├── id (UUID)
├── foundation_id (FK)
├── code                  -- "SPP" | "UP" | "EKS"
├── name
├── type                  -- recurring | one_time
├── group                 -- "SPP" | "UP" | "EKS"
├── sequence              -- 1,2,3,4,5....
└── is_active

fee_components
├── id (UUID)
├── foundation_id (FK)
├── code                  -- "SPP" | "PANGKAL" | "EKSKUL"
├── name
├── type                  -- recurring | one_time
└── is_active

fee_rates
├── id (UUID)
├── foundation_id (FK)
├── fee_component_id (FK)
├── academic_year         -- "2024/2025"
├── grade_level           -- nullable (null = semua jenjang)
├── amount
└── effective_date
```

### Layer 3 — Policy & Rule Engine
```sql
billing_policies
├── id (UUID)
├── foundation_id (FK)
├── code                  -- "TIDAK_MAMPU" | "PRESTASI" | "ANAK_GURU"
├── name
├── is_active             -- ON/OFF oleh admin
├── priority              -- jika siswa masuk 2 kategori
├── is_combinable         -- boleh gabung dengan policy lain?
└── valid_until

billing_policy_rules
├── id (UUID)
├── billing_policy_id (FK)
├── fee_component_id (FK) -- nullable = berlaku semua komponen
├── discount_type         -- percentage | fixed_amount | free
├── discount_value
├── applies_to            -- all | first_year | recurring
└── conditions (JSON)     -- syarat tambahan (misal min_grade: 85)
```

### Layer 4 — Siswa & Tagihan
```sql
student_billing_assignments
├── id (UUID)
├── foundation_id (FK)
├── student_id (FK)
├── billing_policy_id (FK)
├── academic_year
├── status                -- active | suspended | revoked
├── approved_by, approved_at
└── supporting_docs       -- path dokumen (SKTM, dll)

student_bills
├── id (UUID)
├── foundation_id (FK)
├── student_id (FK)
├── fee_component_id (FK)
├── academic_year
├── billing_period        -- "2024-08"
├── base_amount
├── discount_amount
├── final_amount
├── discount_breakdown (JSON)
├── status                -- unpaid | partial | paid
└── due_date

payment_schedules
├── id (UUID)
├── student_bill_id (FK)
├── installment_number
├── amount
├── due_date
└── status
``` 

---

## 📋 Kategori Regulasi Billing yang Didukung

| Kode | Nama | Keterangan |
|---|---|---|
| `TIDAK_MAMPU` | Siswa Tidak Mampu | Syarat: KIP, PKH, SKTM |
| `PRESTASI` | Beasiswa Akademik | Evaluasi per tahun, syarat nilai minimum |
| `ANAK_GURU` | Anak Pegawai Sekolah | Berlaku selama orang tua aktif |
| `ASN` | Anak PNS/ASN/TNI/Polri | Bisa via MoU dengan instansi |
| `YATIM` | Anak Yatim/Piatu | Terpisah dari kategori tidak mampu |
| `REGULER` | Umum | Bayar penuh, bisa cicil |

---

## 🔄 Alur Billing Engine

```
1. Ambil komponen biaya aktif (berdasarkan jenjang & tahun ajaran)
        ↓
2. Ambil semua policy aktif milik siswa
        ↓
3. Cek feature KOMBINASI_DISKON (ON/OFF per foundation)
        ↓
4. Hitung diskon per komponen → discount_breakdown
        ↓
5. final_amount = max(0, base_amount - total_discount)
        ↓
6. Generate student_bills
        ↓
7. Jika CICILAN aktif → generate payment_schedules
        ↓
8. Jika VA_GENERATION aktif → generate nomor VA unik
```

---

## 📐 Konvensi Koding

| Aspek | Konvensi |
|---|---|
| Nama kolom DB | `snake_case` |
| JSON response | `camelCase` |
| Error handling | `thiserror` untuk define, `anyhow` untuk propagate |
| ID | UUID v4 di semua tabel |
| Tanggal | `chrono::DateTime<Utc>` |
| Response format | Selalu `{ success, data, message }` |
| Foundation context | Selalu inject via middleware, bukan dari body request |

---

## 🗺️ Roadmap

```
Phase 1 — Fondasi ← POSISI SEKARANG
├── [x] Cargo.toml setup
├── [x] Rancangan arsitektur & DB schema
├── [ ] SeaORM migration files
├── [ ] Foundation middleware (X-Foundation-ID)
├── [ ] Auth JWT multi-foundation
└── [ ] Feature flag service

Phase 2 — Billing Core
├── [ ] Fee component & rates CRUD
├── [ ] Billing policy CRUD (toggle ON/OFF)
├── [ ] Student policy assignment
├── [ ] Billing engine (kalkulasi tagihan)
└── [ ] Cicilan / payment schedule generator

Phase 3 — Payment
├── [ ] VA generation
├── [ ] Payment gateway integration (Midtrans / Xendit)
└── [ ] Webhook/callback handler

Phase 4 — Frontend (Next.js)
├── [ ] Admin foundation dashboard
├── [ ] Manajemen regulasi (toggle UI)
├── [ ] Input data siswa & assignment policy
└── [ ] Laporan & rekonsiliasi

Phase 5 — SaaS Layer
├── [ ] Super admin (kelola semua foundation)
├── [ ] Foundation onboarding flow
└── [ ] Subscription plan management
```

---

## ⚠️ Catatan Penting untuk Claude

1. **Selalu gunakan UUID** untuk semua primary key, bukan integer auto-increment
2. **Selalu sertakan `foundation_id`** di setiap query — jangan sampai data cross-foundation
3. **Feature flag harus dicek** sebelum memproses regulasi apapun
4. **Discount breakdown harus disimpan** di JSON — jangan hanya simpan angka akhir
5. **Jangan hardcode** nilai diskon atau kategori — semua dari database
6. **Audit trail wajib** untuk setiap perubahan policy billing
7. Saat memberikan code Rust, gunakan **SeaORM 2.0 API** (bukan versi lama)
8. Saat ada pilihan teknis, **jelaskan trade-off**-nya sebelum memilih satu solusi
9. Jika developer upload file kode, **review dulu** sebelum memberikan saran
10. Gunakan **Bahasa Indonesia** dalam semua penjelasan
11. Sistem ini kedepannya akan mengakomodir semua kebutuhan admin sistem sekolah untuk menajemen Rincian Biaya, Custom Pembayaran angsuran misal angsuran dengan jumlah term dan besaran angsuran ditentukan oleh admin, angsuran dengan jumlah term dan besaran angsuran sesuai pengajuan orang tua murid karena tidak mampu dengan melampirkan surat keteranan tidak mampu,
12. Manajemen KBM
13. Manajemen Ekskul
14. Sistem Penilaian harian PTS,PAS
15. Penilaian dengan multiple kurikulum Misal Cambridge, Merdeka dll
16. Print Raport 
 dan masih banyak lagi feature
---

*Dokumen ini adalah living document — update setiap kali ada keputusan arsitektur baru.*
*Terakhir diupdate: sesi awal perencanaan arsitektur*