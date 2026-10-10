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
                    .table(Alias::new("menu_desserts"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("dessert_id")).integer().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menu_desserts_menu_id_menus_fkey")
                    .from(Alias::new("menu_desserts"), Alias::new("menu_id"))
                    .to(Alias::new("menus"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menu_desserts_dessert_id_desserts_fkey")
                    .from(Alias::new("menu_desserts"), Alias::new("dessert_id"))
                    .to(Alias::new("desserts"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_menu_desserts_menu_id")
                    .table(Alias::new("menu_desserts"))
                    .col(Alias::new("menu_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_menu_desserts_dessert_id")
                    .table(Alias::new("menu_desserts"))
                    .col(Alias::new("dessert_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menu_desserts"))
                    .name("menu_desserts_menu_id_menus_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menu_desserts"))
                    .name("menu_desserts_dessert_id_desserts_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_menu_desserts_menu_id").table(Alias::new("menu_desserts")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_menu_desserts_dessert_id").table(Alias::new("menu_desserts")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("menu_desserts"))
                .to_owned())
            .await?;
        Ok(())
}
}
