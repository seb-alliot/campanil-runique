use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE TypePlat AS ENUM ('Entree', 'Plat', 'Dessert'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("plats"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_plat"), ColumnType::Enum { name: Alias::new("TypePlat").into_iden(), variants: vec![Alias::new("Entree").into_iden(), Alias::new("Plat").into_iden(), Alias::new("Dessert").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("image")).string().null())
                    .col(ColumnDef::new(Alias::new("disponible")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("est_viande")).boolean().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("plats"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS TypePlat"
        ).await?;

        Ok(())
}
}
