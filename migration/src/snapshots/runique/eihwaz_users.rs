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
                    .col(ColumnDef::new(Alias::new("telephone")).string().null())
                    .col(ColumnDef::new(Alias::new("adresse")).string().null())
                    .col(ColumnDef::new(Alias::new("ville")).string().null())
                    .col(ColumnDef::new(Alias::new("code_postal")).string().null())
                    .col(ColumnDef::new(Alias::new("pays")).string().null())
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
