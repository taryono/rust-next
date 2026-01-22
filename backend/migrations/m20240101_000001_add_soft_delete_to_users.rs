// ============================================
// 5. Migration untuk tambah kolom deleted_at
// ============================================
// migrations/m20240101_000001_add_soft_delete_to_users.rs

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::DeletedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    DeletedAt,
}
