use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_menu_choix"))
                    .modify_column(ColumnDef::new(Alias::new("plat_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_menu_choix"))
                    .add_column(ColumnDef::new(Alias::new("entree_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_menu_choix"))
                    .add_column(ColumnDef::new(Alias::new("dessert_id")).integer().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_menu_choix"))
                    .drop_column(Alias::new("entree_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_menu_choix"))
                    .drop_column(Alias::new("dessert_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_menu_choix"))
                    .modify_column(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
