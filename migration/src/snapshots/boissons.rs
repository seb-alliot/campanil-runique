use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("boissons"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_boisson"), ColumnType::Enum { name: Alias::new("TypeBoisson").into_iden(), variants: vec![Alias::new("vin_rouge").into_iden(), Alias::new("vin_rose").into_iden(), Alias::new("vin_blanc").into_iden(), Alias::new("soft").into_iden(), Alias::new("eau_bouteille").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("image")).string().null())
                    .col(ColumnDef::new(Alias::new("disponible")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("boissons"))
                .to_owned())
            .await?;
        Ok(())
}
}
