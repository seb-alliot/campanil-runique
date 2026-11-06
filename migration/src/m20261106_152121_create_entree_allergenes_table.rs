use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("entree_allergenes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("entree_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("allergene_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("entree_allergenes_entree_id_entrees_fkey")
                            .from(Alias::new("entree_allergenes"), Alias::new("entree_id"))
                            .to(Alias::new("entrees"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("entree_allergenes_allergene_id_allergenes_fkey")
                            .from(Alias::new("entree_allergenes"), Alias::new("allergene_id"))
                            .to(Alias::new("allergenes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction)
                    )
                    .to_owned()
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("entree_allergenes"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("entree_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_entree_allergenes_entree_id")
                            .table(Alias::new("entree_allergenes"))
                            .col(Alias::new("entree_id"))
                            .to_owned(),
                    )
                    .await?;
            }
        }

        {
            let backs_fk = manager.get_connection().get_database_backend() == sea_orm::DbBackend::MySql
                && manager
                    .get_connection()
                    .query_one(
                        &Query::select()
                            .expr(Expr::val(1))
                            .from((Alias::new("information_schema"), Alias::new("KEY_COLUMN_USAGE")))
                            .and_where(Expr::col(Alias::new("TABLE_SCHEMA")).eq(Expr::cust("DATABASE()")))
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("entree_allergenes"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("allergene_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_entree_allergenes_allergene_id")
                            .table(Alias::new("entree_allergenes"))
                            .col(Alias::new("allergene_id"))
                            .to_owned(),
                    )
                    .await?;
            }
        }

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("entree_allergenes"))
                .to_owned())
            .await?;
        Ok(())
}
}
