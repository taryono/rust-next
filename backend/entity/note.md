<!-- jalankan ini untuk generate entity dari database -->
### 1. generate entity
sea-orm-cli generate entity -u mysql://taryono:password@localhost/actix_db -o entity/src --with-serde both


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