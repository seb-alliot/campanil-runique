use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .add_column(ColumnDef::new(Alias::new("latitude")).decimal().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .add_column(ColumnDef::new(Alias::new("longitude")).decimal().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .drop_column(Alias::new("latitude"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .drop_column(Alias::new("longitude"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
