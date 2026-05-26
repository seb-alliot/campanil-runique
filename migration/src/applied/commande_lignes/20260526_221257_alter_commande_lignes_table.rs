use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("avec_legumes"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("menu_enfant_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("formule_jour_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("menu_resto_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("dessert_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("entree_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("menu_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager.get_connection().execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'entree' WHERE type_article = 'menu_resto'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'dessert' WHERE type_article = 'formule_jour'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'menu' WHERE type_article = 'menu_enfant'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'entree'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'dessert'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'menu'"
        ).await?;

        // WARNING: value 'menu_resto' removed from TypeArticle — manual migration required.

        // WARNING: value 'menu_enfant' removed from TypeArticle — manual migration required.

        // WARNING: value 'formule_jour' removed from TypeArticle — manual migration required.
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("dessert_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("entree_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .drop_column(Alias::new("menu_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("avec_legumes")).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("menu_enfant_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("formule_jour_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("commande_lignes"))
                    .add_column(ColumnDef::new(Alias::new("menu_resto_id")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager.get_connection().execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'menu_resto' WHERE type_article = 'entree'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'formule_jour' WHERE type_article = 'dessert'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'menu_enfant' WHERE type_article = 'menu'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'menu_resto'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'menu_enfant'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'formule_jour'"
        ).await?;
        Ok(())
    }
}
