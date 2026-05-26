use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("contacts"))
                    .add_column(ColumnDef::new_with_type(Alias::new("raison"), ColumnType::Enum { name: Alias::new("RaisonContact").into_iden(), variants: vec![Alias::new("reservation").into_iden(), Alias::new("traiteur").into_iden(), Alias::new("commande").into_iden(), Alias::new("autre").into_iden()] }).not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("contacts"))
                    .drop_column(Alias::new("raison"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
