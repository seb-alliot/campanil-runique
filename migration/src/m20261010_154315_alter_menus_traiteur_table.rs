use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // WARNING: SQLite cannot ALTER a column's type/nullable/unique constraint
        // (sea-query panics on any `modify_column` there) — this change only applies on
        // Postgres/MySQL; on SQLite the column keeps its current definition unchanged.
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("menus_traiteur"))
                        .modify_column(ColumnDef::new(Alias::new("remise_groupe_min")).integer().not_null())
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // WARNING: SQLite cannot ALTER a column's type/nullable/unique constraint
        // (sea-query panics on any `modify_column` there) — this change only applies on
        // Postgres/MySQL; on SQLite the column keeps its current definition unchanged.
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("menus_traiteur"))
                        .modify_column(ColumnDef::new(Alias::new("remise_groupe_min")).integer().null())
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}
