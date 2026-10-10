use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .rename_column(Alias::new("prix_livraison"), Alias::new("prix_km_livraison"))
                    .to_owned(),
            )
            .await?;

        // WARNING: SQLite cannot ALTER a column's type/nullable/unique constraint
        // (sea-query panics on any `modify_column` there) — this change only applies on
        // Postgres/MySQL; on SQLite the column keeps its current definition unchanged.
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("info_resto"))
                        .modify_column(ColumnDef::new(Alias::new("penalite_materiel")).decimal().not_null())
                        .to_owned(),
                )
                .await?;
        }

        // WARNING: SQLite cannot ALTER a column's type/nullable/unique constraint
        // (sea-query panics on any `modify_column` there) — this change only applies on
        // Postgres/MySQL; on SQLite the column keeps its current definition unchanged.
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("info_resto"))
                        .modify_column(ColumnDef::new(Alias::new("prix_livraison_minimal")).decimal().not_null())
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
                        .table(Alias::new("info_resto"))
                        .modify_column(ColumnDef::new(Alias::new("penalite_materiel")).decimal().null())
                        .to_owned(),
                )
                .await?;
        }

        // WARNING: SQLite cannot ALTER a column's type/nullable/unique constraint
        // (sea-query panics on any `modify_column` there) — this change only applies on
        // Postgres/MySQL; on SQLite the column keeps its current definition unchanged.
        if manager.get_connection().get_database_backend() != sea_orm::DbBackend::Sqlite {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("info_resto"))
                        .modify_column(ColumnDef::new(Alias::new("prix_livraison_minimal")).decimal().null())
                        .to_owned(),
                )
                .await?;
        }

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .rename_column(Alias::new("prix_km_livraison"), Alias::new("prix_livraison"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
