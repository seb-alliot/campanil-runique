// runique: column lengths recorded
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menu_entrees"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("entree_id")).integer().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menu_entrees_menu_id_menus_fkey")
                    .from(Alias::new("menu_entrees"), Alias::new("menu_id"))
                    .to(Alias::new("menus"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menu_entrees_entree_id_entrees_fkey")
                    .from(Alias::new("menu_entrees"), Alias::new("entree_id"))
                    .to(Alias::new("entrees"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_menu_entrees_menu_id")
                    .table(Alias::new("menu_entrees"))
                    .col(Alias::new("menu_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_menu_entrees_entree_id")
                    .table(Alias::new("menu_entrees"))
                    .col(Alias::new("entree_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menu_entrees"))
                    .name("menu_entrees_menu_id_menus_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menu_entrees"))
                    .name("menu_entrees_entree_id_entrees_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_menu_entrees_menu_id").table(Alias::new("menu_entrees")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_menu_entrees_entree_id").table(Alias::new("menu_entrees")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("menu_entrees"))
                .to_owned())
            .await?;
        Ok(())
}
}
