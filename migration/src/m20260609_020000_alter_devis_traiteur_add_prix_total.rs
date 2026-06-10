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
                        ColumnDef::new(Alias::new("prix_total"))
                            .decimal_len(10, 2)
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
                    .drop_column(Alias::new("prix_total"))
                    .to_owned(),
            )
            .await
    }
}
