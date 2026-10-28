# ?? Dokumentasi Lengkap: Migrasi Tabel Database dari SeaORM Entity

Dokumen ini mencatat seluruh alur kerja, analisis masalah, perbaikan bug entity, hingga eksekusi pembuatan tabel di database MySQL **`actix_db`** berdasarkan file entity di folder **`api/entity/src`**.

---

## ?? Daftar Isi
1. [Latar Belakang & Kondisi Awal](#1-latar-belakang--kondisi-awal)
2. [Konfigurasi Environment (.env)](#2-konfigurasi-environment-env)
3. [Pemeriksaan Koneksi & Database Eksisting](#3-pemeriksaan-koneksi--database-eksisting)
4. [Penyesuaian Dependensi & MSRV Rust](#4-penyesuaian-dependensi--msrv-rust)
5. [Desain Tool Generator Skema Mandiri (generate_schema)](#5-desain-tool-generator-skema-mandiri-generateschema)
6. [Investigasi & Perbaikan Bug pada Definisi Entity](#6-investigasi--perbaikan-bug-pada-definisi-entity)
7. [Pembaruan Tabel Eksisting users Tanpa Menghilangkan Data](#7-pembaruan-tabel-eksisting-users-tanpa-menghilangkan-data)
8. [Eksekusi Migrasi DDL ke Database actix_db](#8-eksekusi-migrasi-ddl-ke-database-actixdb)
9. [Hasil Verifikasi Akhir](#9-hasil-verifikasi-akhir)
10. [Panduan Pemeliharaan di Masa Depan (Workflow Reusable)](#10-panduan-pemeliharaan-di-masa-depan-workflow-reusable)

---

## 1. Latar Belakang & Kondisi Awal

Pada project **`rust-next/api`** (backend berbasis Rust, Actix-Web, dan SeaORM):
* Terdapat **76 file model entity** di dalam folder `api/entity/src/`.
* Database MySQL target adalah **`actix_db`** pada `localhost:3306`.
* Kondisi awal database `actix_db`: Hanya memiliki 1 tabel (`users`) dengan skema lama yang belum lengkap.
* Crate utama `api` memiliki beberapa modul controller/service yang sedang dalam proses refaktor (belum bisa di-compile secara utuh), sehingga proses migrasi harus dilakukan langsung memanfaatkan crate **`entity`** yang mandiri dan bersih.

---

## 2. Konfigurasi Environment (.env)

Langkah awal adalah menyiapkan file environment yang dibutuhkan oleh aplikasi dan koneksi database SeaORM.

### File yang Dibuat:
1. **`api/.env`**:
   ```env
   # SERVER CONFIGURATION
   SERVER_HOST=127.0.0.1
   SERVER_PORT=8080
   RUST_LOG=info

   # DATABASE CONFIGURATION (SeaORM / MySQL)
   DATABASE_URL=mysql://root:password@localhost:3306/actix_db

   # JWT AUTHENTICATION
   JWT_SECRET=supersecretkey_please_change_in_production_1234567890
   JWT_REFRESH_SECRET=supersecret_refresh_key_please_change_in_production_0987654321
   JWT_EXPIRATION=900          # 15 menit
   JWT_REFRESH_EXPIRATION=604800 # 7 hari

   # CORS & FRONTEND
   ENABLE_CORS=true
   FRONTEND_URL=http://localhost:3000

   # SWAGGER / OPENAPI DOCUMENTATION
   ENABLE_SWAGGER=true
   SWAGGER_AUTH=false
   SWAGGER_USERNAME=admin
   SWAGGER_PASSWORD=secret

   # FILE UPLOAD CONFIGURATION
   UPLOAD_PATH=uploads
   MAX_UPLOAD_SIZE_MB=5
   ALLOWED_IMAGE_EXTENSIONS=jpg,jpeg,png,webp
   ```
2. **`api/.env.example`**: Sebagai template yang aman untuk dibagikan ke repository Git.
3. **`api/.gitignore`**: Ditambahkan pengecualian `!.env.example` agar template tetap dapat di-commit tanpa membocorkan file `.env` asli.

---

## 3. Pemeriksaan Koneksi & Database Eksisting

Dilakukan pengujian koneksi ke MySQL:
* **Host & Port**: `127.0.0.1:3306`
* **User**: `root`
* **Status**: Berhasil terhubung.
* **Tabel Eksisting**: Ditemukan tabel `users` dengan 2 baris data:
  * `id = 1`: `taryono` (`denmas.yono@gmail.com`)
  * `id = 2`: `eliyani` (`eliyani@gmail.com`)

> ?? **Catatan Penting**: Data pengguna ini tidak boleh terhapus (*DROP TABLE* dihindari, diganti dengan *ALTER TABLE* bertahap).

---

## 4. Penyesuaian Dependensi & MSRV Rust

Saat memeriksa kompilasi `cargo check`, muncul pesan error:
```text
error: rustc 1.90.0 is not supported by the following package:
  sea-orm@2.0.4 requires rustc 1.94.0
```
### Solusi:
Cargo secara default mencoba menarik rilis terbaru `sea-orm 2.0.4`. Versi dependensi dikunci menggunakan `Cargo.lock` yang stabil (versi `2.0.0-rc.38`) yang kompatibel dengan Rust `1.90.0` yang terpasang di sistem WSL Ubuntu.

Hasilnya, crate `entity` terkompilasi **100% sukses** tanpa error.

---

## 5. Desain Tool Generator Skema Mandiri (`generate_schema`)

Untuk meng-generate tabel langsung dari entity tanpa terhambat oleh error pada controller/service di crate `api`:
1. Dibuat binary khusus di dalam crate `entity`:
   * Lokasi: **`api/entity/src/bin/generate_schema.rs`**
2. Diregistrasikan pada `api/entity/Cargo.toml`:
   ```toml
   [[bin]]
   name = "generate_schema"
   path = "src/bin/generate_schema.rs"
   ```
3. Mekanisme kerja:
   * Menggunakan SeaORM Schema API: `Schema::new(DbBackend::MySql)`
   * Melakukan iterasi ke seluruh 76 entity: `schema.create_table_from_entity(Entity)`
   * Menambahkan klausul `stmt.if_not_exists()`
   * Menghasilkan DDL murni dengan `DbBackend::MySql.build(&stmt).to_string()`
   * Membungkus hasil DDL dengan `SET FOREIGN_KEY_CHECKS = 0;` di awal dan `SET FOREIGN_KEY_CHECKS = 1;` di akhir agar urutan dependensi foreign key tidak menimbulkan error.

---

## 6. Investigasi & Perbaikan Bug pada Definisi Entity

Saat `generate_schema` pertama kali dijalankan, ditemukan beberapa bug struktural pada file entity:

### A. Stack Overflow (Relasi Rekursif Tanpa Akhir)
* **Gejala**: Program terhenti mendadak dengan pesan:
  ```text
  thread 'main' has overflowed its stack
  fatal runtime error: stack overflow, aborting
  ```
* **Penyebab**:
  Pada file entity berikut:
  * `api/entity/src/courses.rs`
  * `api/entity/src/sports.rs`
  * `api/entity/src/universities.rs`
  * `api/entity/src/registrations.rs`
  * `api/entity/src/invoices.rs`

  Terdapat definisi relasi yang memanggil dirinya sendiri secara melingkar:
  ```rust
  #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
  pub enum Relation {
      #[sea_orm(has_many = "super::courses::Entity")]
      Foundationcourses,
  }

  impl Related<super::courses::Entity> for Entity {
      fn to() -> RelationDef {
          Relation::Foundationcourses.def()
      }
  }
  ```
  Saat SeaORM membaca foreign key dari entity, terjadi loop rekursif tanpa henti hingga stack memori habis.
* **Perbaikan**:
  Karena tabel-tabel ini tidak memiliki foreign key pada dirinya sendiri, relasinya disederhanakan menjadi:
  ```rust
  #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
  pub enum Relation {}
  ```

---

### B. Duplicate Foreign Key Constraint Name di `books.rs`
* **Gejala**:
  ```text
  ERROR 1826 (HY000): Duplicate foreign key constraint name 'fk-books-author_id'
  ```
* **Penyebab**:
  Pada `api/entity/src/books.rs`, relasi ke tabel `foundations` salah menggunakan field `AuthorId`:
  ```rust
  // ? SALAH:
  #[sea_orm(
      belongs_to = "super::foundations::Entity",
      from = "Column::AuthorId",
      to = "super::foundations::Column::Id"
  )]
  Foundation,
  ```
  Sehingga SeaORM menghasilkan dua constraint foreign key bernama sama (`fk-books-author_id`).
* **Perbaikan**:
  Diubah agar mengarah ke `FoundationId`:
  ```rust
  // ? BENAR:
  #[sea_orm(
      belongs_to = "super::foundations::Entity",
      from = "Column::FoundationId",
      to = "super::foundations::Column::Id"
  )]
  Foundation,
  ```

---

### C. Ketidakcocokan Tipe Data Foreign Key di `notifications.rs`
* **Gejala**:
  ```text
  ERROR 3780 (HY000): Referencing column 'user_id' and referenced column 'id' in foreign key constraint 'fk-notifications-user_id' are incompatible.
  ```
* **Penyebab**:
  Pada `api/entity/src/notifications.rs`:
  * Kolom `user_id` didefinisikan sebagai `pub user_id: i32` (`int`).
  * Sedangkan tabel yang direferensikan (`users.id`) bertipe `pub id: i64` (`bigint`).
  * Dalam MySQL, foreign key antara `int` dan `bigint` dilarang.
* **Perbaikan**:
  Diubah menjadi `pub user_id: i64` agar serasi dengan `users.id`.

---

## 7. Pembaruan Tabel Eksisting `users` Tanpa Menghilangkan Data

Tabel `users` yang sebelumnya sudah ada di database MySQL memiliki tipe `id int` dan belum memiliki kolom-kolom baru yang dibutuhkan oleh SeaORM.

Dijalankan perintah ALTER TABLE:
```sql
ALTER TABLE users 
  MODIFY COLUMN id bigint NOT NULL AUTO_INCREMENT,
  ADD COLUMN foundation_id bigint NOT NULL DEFAULT 1 AFTER email,
  ADD COLUMN username varchar(255) NULL AFTER foundation_id,
  ADD COLUMN is_verified tinyint NULL DEFAULT 0 AFTER password,
  ADD COLUMN is_active tinyint NULL DEFAULT 1 AFTER is_verified,
  ADD COLUMN avatar varchar(255) NULL AFTER is_active,
  ADD COLUMN deleted_at text NULL AFTER updated_at;
```

**Hasil**:
* Struktur tabel `users` sinkron 100% dengan entity [`users.rs`](file:///Ubuntu/home/taryono/rust/rust-next/api/entity/src/users.rs).
* Data user `taryono` dan `eliyani` tetap utuh.

---

## 8. Eksekusi Migrasi DDL ke Database `actix_db`

Setelah semua bug entity diperbaiki:
1. Rebuild dan generate file SQL DDL lengkap:
   ```bash
   cargo build --manifest-path api/entity/Cargo.toml --bin generate_schema
   api/target/debug/generate_schema > api/schema_from_entities.sql
   ```
2. Import file DDL ke database MySQL:
   ```bash
   mysql -h 127.0.0.1 -P 3306 -u root -ppassword actix_db < api/schema_from_entities.sql
   ```
   **Hasil Eksekusi**: `Exit code 0` (Sukses tanpa error).

---

## 9. Hasil Verifikasi Akhir

* **Jumlah Tabel di `actix_db`**: **74 tabel**
* **Daftar Tabel yang Terbentuk**:
  ```text
  academic_calendars           academic_years              applicants
  attendances                  authors                     boarding_schools
  book_categories              book_copies                 books
  borrowings                   character_assessments       character_traits
  class_levels                 class_schedules             class_subjects
  classes                      classrooms                  courses
  departments                  employees                   extracurricular_activities
  extracurricular_enrollments  foundation_regulations      foundation_types
  foundations                  grades                      guardians
  holidays                     homeroom_teachers           invoices
  levels                       members                     menu_permissions
  menu_roles                   menus                       notifications
  payments                     permissions                 positions
  publishers                   registrations               regulations
  report_cards                 role_permissions            role_users
  roles                        room_facilities             room_types
  rooms                        salary_grades               salary_payments
  schedule_generation_logs     schools                     scopes
  semesters                    settings                    sports
  student_enrollments          students                    subject_preferences
  subject_room_requirements    subjects                    teacher_assignments
  teacher_availability         teacher_salaries            teacher_subjects
  teachers                     time_slots                  unit_types
  units                        universities                user_permissions
  user_profiles                users
  ```
* **Kesesuaian dengan 76 file entity**:
  * 74 tabel mewakili 76 entity file.
  * `teachers_payments.rs` memetakan ke tabel `teacher_subjects`.
  * `homerooms.rs` memetakan ke tabel `levels`.
  * Seluruh entity memiliki tabel fisiknya di MySQL.

---

## 10. Panduan Pemeliharaan di Masa Depan (Workflow Reusable)

Jika di kemudian hari Anda menambahkan entity baru atau mengubah kolom di `api/entity/src/`:

1. **Tambahkan entity baru** ke dalam [`api/entity/src/mod.rs`](file:///Ubuntu/home/taryono/rust/rust-next/api/entity/src/mod.rs) dan script [`generate_schema.rs`](file:///Ubuntu/home/taryono/rust/rust-next/api/entity/src/bin/generate_schema.rs).
2. **Re-generate file DDL skema**:
   ```bash
   cargo run --manifest-path api/entity/Cargo.toml --bin generate_schema > api/schema_from_entities.sql
   ```
3. **Eksekusi ke database target**:
   ```bash
   mysql -h 127.0.0.1 -P 3306 -u root -ppassword actix_db < api/schema_from_entities.sql
   ```
   *(Karena menggunakan `CREATE TABLE IF NOT EXISTS`, tabel yang sudah ada tidak akan ditimpa atau dihapus).*
