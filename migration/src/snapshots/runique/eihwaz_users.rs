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
                    .table(Alias::new("eihwaz_users"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("telephone")).string_len(20).null())
                    .col(ColumnDef::new(Alias::new("adresse")).string_len(255).null())
                    .col(ColumnDef::new(Alias::new("ville")).string_len(100).null())
                    .col(ColumnDef::new(Alias::new("code_postal")).string_len(10).null())
                    .col(ColumnDef::new(Alias::new("pays")).string_len(100).not_null().default("France"))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("eihwaz_users"))
                .to_owned())
            .await?;
        Ok(())
}
}
