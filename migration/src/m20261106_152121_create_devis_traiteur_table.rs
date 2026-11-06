use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            let exists = manager
                .get_connection()
                .query_one(
                    &Query::select()
                        .expr(Expr::val(1))
                        .from((Alias::new("pg_catalog"), Alias::new("pg_type")))
                        .and_where(Expr::col(Alias::new("typname")).eq("statutdevis"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("statutdevis"))
                            .values([Alias::new("en_attente"), Alias::new("en_cours"), Alias::new("accepte"), Alias::new("refuse")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("devis_traiteur"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("nom")).string_len(150).not_null())
                    .col(ColumnDef::new(Alias::new("email")).string_len(255).not_null())
                    .col(ColumnDef::new(Alias::new("telephone")).string_len(30).null())
                    .col(ColumnDef::new(Alias::new("date_evenement")).date().not_null())
                    .col(ColumnDef::new(Alias::new("nb_personnes")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("message")).text().not_null())
                    .col(ColumnDef::new(Alias::new("prix_total")).decimal_len(10, 2).null())
                    .col(ColumnDef::new(Alias::new("remise_appliquee")).decimal_len(5, 2).null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("StatutDevis").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("en_cours").into_iden(), Alias::new("accepte").into_iden(), Alias::new("refuse").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null().default("en_attente"))
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("devis_traiteur_menu_id_menus_traiteur_fkey")
                            .from(Alias::new("devis_traiteur"), Alias::new("menu_id"))
                            .to(Alias::new("menus_traiteur"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::NoAction)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("devis_traiteur_user_id_eihwaz_users_fkey")
                            .from(Alias::new("devis_traiteur"), Alias::new("user_id"))
                            .to(Alias::new("eihwaz_users"), Alias::new("id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("devis_traiteur"))
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
                            .name("idx_devis_traiteur_menu_id")
                            .table(Alias::new("devis_traiteur"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("devis_traiteur"))
                            .and_where(Expr::col(Alias::new("COLUMN_NAME")).eq("user_id"))
                            .and_where(Expr::col(Alias::new("REFERENCED_TABLE_NAME")).is_not_null())
                            .to_owned(),
                    )
                    .await?
                    .is_some();
            if !backs_fk {
                manager
                    .create_index(
                        Index::create()
                            .name("idx_devis_traiteur_user_id")
                            .table(Alias::new("devis_traiteur"))
                            .col(Alias::new("user_id"))
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
                .table(Alias::new("devis_traiteur"))
                .to_owned())
            .await?;
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("statutdevis"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
}
}
