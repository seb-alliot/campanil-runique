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
                        ColumnDef::new(Alias::new("prix_livraison_minimal"))
                            .decimal_len(10, 2)
                            .null()
                            .default(5.00),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("info_resto"))
                    .drop_column(Alias::new("prix_livraison_minimal"))
                    .to_owned(),
            )
            .await
    }
}
