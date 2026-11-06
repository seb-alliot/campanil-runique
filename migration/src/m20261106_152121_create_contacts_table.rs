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
                        .and_where(Expr::col(Alias::new("typname")).eq("raisoncontact"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("raisoncontact"))
                            .values([Alias::new("reservation"), Alias::new("traiteur"), Alias::new("commande"), Alias::new("autre")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("contacts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new_with_type(Alias::new("raison"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("RaisonContact").into_iden(), variants: vec![Alias::new("reservation").into_iden(), Alias::new("traiteur").into_iden(), Alias::new("commande").into_iden(), Alias::new("autre").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null())
                    .col(ColumnDef::new(Alias::new("titre")).string_len(255).not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().not_null())
                    .col(ColumnDef::new(Alias::new("email")).string_len(254).not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("contacts"))
                .to_owned())
            .await?;
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("raisoncontact"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
}
}
