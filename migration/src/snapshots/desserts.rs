// runique: column lengths recorded
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("desserts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string_len(255).not_null())
                    .col(ColumnDef::new(Alias::new("label")).string_len(80).null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("image")).string().null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("disponible")).boolean().not_null().default(true))
                    .col(ColumnDef::new_with_type(Alias::new("usage"), ColumnType::Enum { name: Alias::new("UsageDessert").into_iden(), variants: vec![Alias::new("carte").into_iden(), Alias::new("menu").into_iden(), Alias::new("les_deux").into_iden()] }).not_null().default("les_deux"))
                    .col(ColumnDef::new(Alias::new("ordre")).integer().not_null().default(0))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("desserts"))
                .to_owned())
            .await?;
        Ok(())
}
}
