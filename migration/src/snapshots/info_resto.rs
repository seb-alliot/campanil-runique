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
                    .col(ColumnDef::new(Alias::new("nom")).string().not_null())
                    .col(ColumnDef::new(Alias::new("adresse")).string().not_null())
                    .col(ColumnDef::new(Alias::new("telephone")).string().not_null())
                    .col(ColumnDef::new(Alias::new("email")).string().null())
                    .col(ColumnDef::new(Alias::new("periode_ouverture")).string().null())
                    .col(ColumnDef::new(Alias::new("facebook")).string().null())
                    .col(ColumnDef::new(Alias::new("instagram")).string().null())
                    .col(ColumnDef::new(Alias::new("tripadvisor")).string().null())
                    .col(ColumnDef::new(Alias::new("google_maps")).string().null())
                    .col(ColumnDef::new(Alias::new("description")).string().null())
                    .col(ColumnDef::new(Alias::new("prix_livraison")).decimal().null())
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
