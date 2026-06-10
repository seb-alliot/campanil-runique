use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("devis_traiteur"))
                    .add_column(
                        ColumnDef::new(Alias::new("remise_appliquee"))
                            .decimal_len(5, 2)
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("devis_traiteur"))
                    .drop_column(Alias::new("remise_appliquee"))
                    .to_owned(),
            )
            .await
    }
}
