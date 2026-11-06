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
                        .and_where(Expr::col(Alias::new("typname")).eq("jour"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("jour"))
                            .values([Alias::new("lundi"), Alias::new("mardi"), Alias::new("mercredi"), Alias::new("jeudi"), Alias::new("vendredi"), Alias::new("samedi"), Alias::new("dimanche")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("horaires"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new_with_type(Alias::new("jour"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("Jour").into_iden(), variants: vec![Alias::new("lundi").into_iden(), Alias::new("mardi").into_iden(), Alias::new("mercredi").into_iden(), Alias::new("jeudi").into_iden(), Alias::new("vendredi").into_iden(), Alias::new("samedi").into_iden(), Alias::new("dimanche").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("ouverture_midi")).time().null())
                    .col(ColumnDef::new(Alias::new("fermeture_midi")).time().null())
                    .col(ColumnDef::new(Alias::new("ouverture_soir")).time().null())
                    .col(ColumnDef::new(Alias::new("fermeture_soir")).time().null())
                    .col(ColumnDef::new(Alias::new("ferme")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("note")).string().null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("horaires"))
                .to_owned())
            .await?;
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("jour"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
}
}
