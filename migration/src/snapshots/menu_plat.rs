use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menu_plat"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menu_plat_menu_id_menus_fkey")
                    .from(Alias::new("menu_plat"), Alias::new("menu_id"))
                    .to(Alias::new("menus"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menu_plat_plat_id_plats_fkey")
                    .from(Alias::new("menu_plat"), Alias::new("plat_id"))
                    .to(Alias::new("plats"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("menu_plat_menu_id_plat_id_uniq")
                    .table(Alias::new("menu_plat"))
                    .col(Alias::new("menu_id"))
                    .col(Alias::new("plat_id"))
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menu_plat"))
                    .name("menu_plat_menu_id_menus_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menu_plat"))
                    .name("menu_plat_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("menu_plat_menu_id_plat_id_uniq").table(Alias::new("menu_plat")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("menu_plat"))
                .to_owned())
            .await?;
        Ok(())
}
}
