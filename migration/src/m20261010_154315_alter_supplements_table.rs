use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("supplements"))
                    .name("supplements_garniture_id_garnitures_fkey")
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
                        .table(Alias::new("supplements"))
                        .modify_column(ColumnDef::new(Alias::new("ordre")).integer().not_null())
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("supplements_garniture_id_garnitures_fkey")
                    .from(Alias::new("supplements"), Alias::new("garniture_id"))
                    .to(Alias::new("garnitures"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        {
            let backs_fk = manager.get_connection().get_database_backend() == sea_orm::DbBackend::MySql
                && manager
                    .get_connection()
                    .query_one(
                        &Query::select()
                            .expr(Expr::val(1))
                            .from((Alias::new("information_schema"), Alias::new("KEY_COLUMN_USAGE")))
                            .and_where(Expr::col(Alias::new("TABLE_SCHEMA")).eq(Expr::cust("DATABASE()")))
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("supplements"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("garniture_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_supplements_garniture_id")
                            .table(Alias::new("supplements"))
                            .col(Alias::new("garniture_id"))
                            .to_owned(),
                    )
                    .await?;
            }
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        {
            let backs_fk = manager.get_connection().get_database_backend() == sea_orm::DbBackend::MySql
                && manager
                    .get_connection()
                    .query_one(
                        &Query::select()
                            .expr(Expr::val(1))
                            .from((Alias::new("information_schema"), Alias::new("KEY_COLUMN_USAGE")))
                            .and_where(Expr::col(Alias::new("TABLE_SCHEMA")).eq(Expr::cust("DATABASE()")))
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("supplements"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("garniture_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .drop_index(Index::drop().name("idx_supplements_garniture_id").table(Alias::new("supplements")).to_owned())
                    .await?;
            }
        }

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("supplements"))
                    .name("supplements_garniture_id_garnitures_fkey")
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
                        .table(Alias::new("supplements"))
                        .modify_column(ColumnDef::new(Alias::new("ordre")).integer().null())
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("supplements_garniture_id_garnitures_fkey")
                    .from(Alias::new("supplements"), Alias::new("garniture_id"))
                    .to(Alias::new("garnitures"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
