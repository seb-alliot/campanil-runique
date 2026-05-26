use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("plats"))
                    .drop_column(Alias::new("avec_legumes"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("plats"))
                    .add_column(ColumnDef::new_with_type(Alias::new("usage"), ColumnType::Enum { name: Alias::new("UsagePlat").into_iden(), variants: vec![Alias::new("carte").into_iden(), Alias::new("menu").into_iden(), Alias::new("les_deux").into_iden()] }).not_null())
                    .to_owned(),
            )
            .await?;

        manager.get_connection().execute_unprepared(
            "UPDATE plats SET type_plat = 'specialite' WHERE type_plat = 'entree'"
        ).await?;

        // WARNING: value 'entree' removed from TypePlat — manual migration required.

        // WARNING: value 'dessert' removed from TypePlat — manual migration required.
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("plats"))
                    .drop_column(Alias::new("usage"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("plats"))
                    .add_column(ColumnDef::new(Alias::new("avec_legumes")).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager.get_connection().execute_unprepared(
            "UPDATE plats SET type_plat = 'entree' WHERE type_plat = 'specialite'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypePlat ADD VALUE IF NOT EXISTS 'entree'"
        ).await?;

        manager.get_connection().execute_unprepared(
            "ALTER TYPE TypePlat ADD VALUE IF NOT EXISTS 'dessert'"
        ).await?;
        Ok(())
    }
}
