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
                        .and_where(Expr::col(Alias::new("typname")).eq("thememenu"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("thememenu"))
                            .values([Alias::new("mariage"), Alias::new("bapteme"), Alias::new("anniversaire"), Alias::new("autre")])
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
                        .and_where(Expr::col(Alias::new("typname")).eq("regimemenu"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("regimemenu"))
                            .values([Alias::new("standard"), Alias::new("vegetarien"), Alias::new("sans_gluten"), Alias::new("halal"), Alias::new("casher")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menus_traiteur"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string_len(255).not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().not_null())
                    .col(ColumnDef::new(Alias::new("prix_par_personne")).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Alias::new("nb_personnes_min")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("theme"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("ThemeMenu").into_iden(), variants: vec![Alias::new("mariage").into_iden(), Alias::new("bapteme").into_iden(), Alias::new("anniversaire").into_iden(), Alias::new("autre").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null().default("autre"))
                    .col(ColumnDef::new_with_type(Alias::new("regime"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("RegimeMenu").into_iden(), variants: vec![Alias::new("standard").into_iden(), Alias::new("vegetarien").into_iden(), Alias::new("sans_gluten").into_iden(), Alias::new("halal").into_iden(), Alias::new("casher").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null().default("standard"))
                    .col(ColumnDef::new(Alias::new("conditions")).text().null())
                    .col(ColumnDef::new(Alias::new("remise_groupe")).decimal_len(5, 2).null())
                    .col(ColumnDef::new(Alias::new("remise_groupe_min")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("stock")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("actif")).boolean().not_null().default(true))
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("menus_traiteur"))
                .to_owned())
            .await?;
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("thememenu"))
                        .to_owned(),
                )
                .await?;
        }

        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("regimemenu"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
}
}
