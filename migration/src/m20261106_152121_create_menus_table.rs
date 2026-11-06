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
                        .and_where(Expr::col(Alias::new("typname")).eq("typemenu"))
                        .to_owned(),
                )
                .await?
                .is_some();
            if !exists {
                manager
                    .create_type(
                        sea_query::extension::postgres::Type::create()
                            .as_enum(Alias::new("typemenu"))
                            .values([Alias::new("menu_resto"), Alias::new("menu_enfant"), Alias::new("formule_jour")])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menus"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new_with_type(Alias::new("type_menu"), if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres { ColumnType::Enum { name: Alias::new("TypeMenu").into_iden(), variants: vec![Alias::new("menu_resto").into_iden(), Alias::new("menu_enfant").into_iden(), Alias::new("formule_jour").into_iden()] } } else { ColumnType::String(Default::default()) }).not_null().default("menu_resto"))
                    .col(ColumnDef::new(Alias::new("nom")).string_len(255).not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("image")).string().null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal_len(10, 2).not_null())
                    .col(ColumnDef::new(Alias::new("ordre")).integer().not_null().default(0))
                    .col(ColumnDef::new(Alias::new("entree_libre")).string_len(500).null())
                    .col(ColumnDef::new(Alias::new("plat_libre")).string_len(500).null())
                    .col(ColumnDef::new(Alias::new("dessert_libre")).string_len(500).null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("menus"))
                .to_owned())
            .await?;
        if manager.get_connection().get_database_backend() == sea_orm::DbBackend::Postgres {
            manager
                .drop_type(
                    sea_query::extension::postgres::Type::drop()
                        .if_exists()
                        .name(Alias::new("typemenu"))
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
}
}
