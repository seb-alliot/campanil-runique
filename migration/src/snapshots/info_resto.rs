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
                    .table(Alias::new("info_resto"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("nom")).string_len(150).not_null())
                    .col(ColumnDef::new(Alias::new("adresse")).string_len(200).not_null())
                    .col(ColumnDef::new(Alias::new("telephone")).string_len(20).not_null())
                    .col(ColumnDef::new(Alias::new("email")).string_len(254).null())
                    .col(ColumnDef::new(Alias::new("periode_ouverture")).string_len(100).null())
                    .col(ColumnDef::new(Alias::new("facebook")).string().null())
                    .col(ColumnDef::new(Alias::new("instagram")).string().null())
                    .col(ColumnDef::new(Alias::new("tripadvisor")).string().null())
                    .col(ColumnDef::new(Alias::new("google_maps")).string().null())
                    .col(ColumnDef::new(Alias::new("description")).string().null())
                    .col(ColumnDef::new(Alias::new("ville")).string_len(100).null())
                    .col(ColumnDef::new(Alias::new("prix_km_livraison")).decimal().not_null().default(0.59))
                    .col(ColumnDef::new(Alias::new("prix_livraison_minimal")).decimal().not_null().default(5.00))
                    .col(ColumnDef::new(Alias::new("penalite_materiel")).decimal().not_null().default(600.00))
                    .col(ColumnDef::new(Alias::new("latitude")).decimal().null())
                    .col(ColumnDef::new(Alias::new("longitude")).decimal().null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("info_resto"))
                .to_owned())
            .await?;
        Ok(())
}
}
