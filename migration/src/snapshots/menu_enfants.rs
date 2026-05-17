use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menu_enfants"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("plat")).string().not_null())
                    .col(ColumnDef::new(Alias::new("dessert")).string().null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("image")).string().null())
                    .col(ColumnDef::new(Alias::new("actif")).boolean().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("menu_enfants"))
                .to_owned())
            .await?;
        Ok(())
}
}
