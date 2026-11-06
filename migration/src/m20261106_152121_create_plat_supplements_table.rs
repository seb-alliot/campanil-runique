use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("plat_supplements"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("supplement_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("plat_supplements_plat_id_plats_fkey")
                            .from(Alias::new("plat_supplements"), Alias::new("plat_id"))
                            .to(Alias::new("plats"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("plat_supplements_supplement_id_supplements_fkey")
                            .from(Alias::new("plat_supplements"), Alias::new("supplement_id"))
                            .to(Alias::new("supplements"), Alias::new("id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("plat_supplements"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("plat_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_plat_supplements_plat_id")
                            .table(Alias::new("plat_supplements"))
                            .col(Alias::new("plat_id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("plat_supplements"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("supplement_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_plat_supplements_supplement_id")
                            .table(Alias::new("plat_supplements"))
                            .col(Alias::new("supplement_id"))
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
                .table(Alias::new("plat_supplements"))
                .to_owned())
            .await?;
        Ok(())
}
}
