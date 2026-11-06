use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_ligne_garnitures"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("ligne_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("garniture_id")).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("commande_ligne_garnitures_ligne_id_commande_lignes_fkey")
                            .from(Alias::new("commande_ligne_garnitures"), Alias::new("ligne_id"))
                            .to(Alias::new("commande_lignes"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("commande_ligne_garnitures_garniture_id_garnitures_fkey")
                            .from(Alias::new("commande_ligne_garnitures"), Alias::new("garniture_id"))
                            .to(Alias::new("garnitures"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict)
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("commande_ligne_garnitures"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("ligne_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_commande_ligne_garnitures_ligne_id")
                            .table(Alias::new("commande_ligne_garnitures"))
                            .col(Alias::new("ligne_id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("commande_ligne_garnitures"))
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
                            .name("idx_commande_ligne_garnitures_garniture_id")
                            .table(Alias::new("commande_ligne_garnitures"))
                            .col(Alias::new("garniture_id"))
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
                .table(Alias::new("commande_ligne_garnitures"))
                .to_owned())
            .await?;
        Ok(())
}
}
