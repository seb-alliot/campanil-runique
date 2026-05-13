use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_statuts"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("commande_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("statut")).string().not_null())
                    .col(ColumnDef::new(Alias::new("note")).text().null())
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("commande_statuts"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
