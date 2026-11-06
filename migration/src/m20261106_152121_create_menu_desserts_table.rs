use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menu_desserts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("dessert_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("menu_desserts_menu_id_menus_fkey")
                            .from(Alias::new("menu_desserts"), Alias::new("menu_id"))
                            .to(Alias::new("menus"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("menu_desserts_dessert_id_desserts_fkey")
                            .from(Alias::new("menu_desserts"), Alias::new("dessert_id"))
                            .to(Alias::new("desserts"), Alias::new("id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("menu_desserts"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("menu_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_menu_desserts_menu_id")
                            .table(Alias::new("menu_desserts"))
                            .col(Alias::new("menu_id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("menu_desserts"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("dessert_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_menu_desserts_dessert_id")
                            .table(Alias::new("menu_desserts"))
                            .col(Alias::new("dessert_id"))
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
                .table(Alias::new("menu_desserts"))
                .to_owned())
            .await?;
        Ok(())
}
}
