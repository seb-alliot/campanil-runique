use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .add_column(ColumnDef::new(Alias::new("telephone")).string_len(20).null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .add_column(ColumnDef::new(Alias::new("adresse")).string_len(255).null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .add_column(ColumnDef::new(Alias::new("ville")).string_len(100).null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .add_column(ColumnDef::new(Alias::new("code_postal")).string_len(10).null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .add_column(ColumnDef::new(Alias::new("pays")).string_len(100).not_null().default("France"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .drop_column(Alias::new("telephone"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .drop_column(Alias::new("adresse"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .drop_column(Alias::new("ville"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .drop_column(Alias::new("code_postal"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("eihwaz_users"))
                    .drop_column(Alias::new("pays"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
