use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_statuts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("commande_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("statut")).string_len(50).not_null())
                    .col(ColumnDef::new(Alias::new("note")).text().null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("commande_statuts_commande_id_commandes_fkey")
                            .from(Alias::new("commande_statuts"), Alias::new("commande_id"))
                            .to(Alias::new("commandes"), Alias::new("id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("commande_statuts"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("commande_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_commande_statuts_commande_id")
                            .table(Alias::new("commande_statuts"))
                            .col(Alias::new("commande_id"))
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
                .table(Alias::new("commande_statuts"))
                .to_owned())
            .await?;
        Ok(())
}
}
