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
                        .and_where(Expr::col(Alias::new("typname")).eq("statutcommande"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("statutcommande"))
                            .values([Alias::new("en_attente"), Alias::new("accepte"), Alias::new("en_preparation"), Alias::new("pret"), Alias::new("en_cours_livraison"), Alias::new("livre"), Alias::new("termine"), Alias::new("annule")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            let exists = manager
                .get_connection()
                .query_one(
                    &Query::select()
                        .expr(Expr::val(1))
                        .from((Alias::new("pg_catalog"), Alias::new("pg_type")))
                        .and_where(Expr::col(Alias::new("typname")).eq("modepaiement"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("modepaiement"))
                            .values([Alias::new("especes"), Alias::new("carte_bancaire"), Alias::new("en_ligne")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            let exists = manager
                .get_connection()
                .query_one(
                    &Query::select()
                        .expr(Expr::val(1))
                        .from((Alias::new("pg_catalog"), Alias::new("pg_type")))
                        .and_where(Expr::col(Alias::new("typname")).eq("typeretrait"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("typeretrait"))
                            .values([Alias::new("sur_place"), Alias::new("livraison")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commandes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("numero")).string_len(20).not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("StatutCommande").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("accepte").into_iden(), Alias::new("en_preparation").into_iden(), Alias::new("pret").into_iden(), Alias::new("en_cours_livraison").into_iden(), Alias::new("livre").into_iden(), Alias::new("termine").into_iden(), Alias::new("annule").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null().default("en_attente"))
                    .col(ColumnDef::new_with_type(Alias::new("mode_paiement"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("ModePaiement").into_iden(), variants: vec![Alias::new("especes").into_iden(), Alias::new("carte_bancaire").into_iden(), Alias::new("en_ligne").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null())
                    .col(ColumnDef::new(Alias::new("prix_total")).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_retrait"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("TypeRetrait").into_iden(), variants: vec![Alias::new("sur_place").into_iden(), Alias::new("livraison").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null())
                    .col(ColumnDef::new(Alias::new("heure_retrait")).date_time().null())
                    .col(ColumnDef::new(Alias::new("adresse_livraison")).string_len(255).null())
                    .col(ColumnDef::new(Alias::new("ville_livraison")).string_len(100).null())
                    .col(ColumnDef::new(Alias::new("cp_livraison")).string_len(10).null())
                    .col(ColumnDef::new(Alias::new("prix_livraison")).decimal_len(10, 2).not_null().default(0))
                    .col(ColumnDef::new(Alias::new("modifiable")).boolean().not_null().default(true))
                    .col(ColumnDef::new(Alias::new("pret_materiel")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("penalite_envoyee")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("stripe_payment_intent_id")).string_len(255).null())
                    .col(ColumnDef::new(Alias::new("motif_annulation")).text().null())
                    .col(ColumnDef::new(Alias::new("mode_contact_annulation")).string_len(100).null())
                    .col(ColumnDef::new(Alias::new("date_annulation")).date_time().null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updated_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("commandes_user_id_eihwaz_users_fkey")
                            .from(Alias::new("commandes"), Alias::new("user_id"))
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
                            .and_where(Expr::col(Alias::new("TABLE_NAME")).eq("commandes"))
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
                            .name("idx_commandes_user_id")
                            .table(Alias::new("commandes"))
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
                .table(Alias::new("commandes"))
                .to_owned())
            .await?;
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("statutcommande"))
                        .to_owned(),
                )
                .await?;
        }

        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("modepaiement"))
                        .to_owned(),
                )
                .await?;
        }

        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("typeretrait"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
}
}
