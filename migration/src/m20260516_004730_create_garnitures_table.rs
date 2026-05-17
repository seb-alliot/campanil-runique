use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE TypeGarniture AS ENUM ('feculent', 'legumes'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("garnitures"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("libelle")).string().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_garniture"), ColumnType::Enum { name: Alias::new("TypeGarniture").into_iden(), variants: vec![Alias::new("feculent").into_iden(), Alias::new("legumes").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("disponible")).boolean().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("garnitures"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS TypeGarniture"
        ).await?;

        Ok(())
}
}
