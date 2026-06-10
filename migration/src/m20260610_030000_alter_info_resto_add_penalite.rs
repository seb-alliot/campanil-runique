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
                    .add_column(
                        ColumnDef::new(Alias::new("penalite_materiel"))
                            .decimal()
                            .not_null()
                            .default(600.00),
                    )
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
                    .drop_column(Alias::new("penalite_materiel"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
