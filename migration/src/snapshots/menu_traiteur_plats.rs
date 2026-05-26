use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menu_traiteur_plats"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_traiteur_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("menu_traiteur_plats_menu_traiteur_id_plat_id_uniq")
                    .table(Alias::new("menu_traiteur_plats"))
                    .col(Alias::new("menu_traiteur_id"))
                    .col(Alias::new("plat_id"))
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("menu_traiteur_plats_menu_traiteur_id_plat_id_uniq").table(Alias::new("menu_traiteur_plats")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("menu_traiteur_plats"))
                .to_owned())
            .await?;
        Ok(())
}
}
