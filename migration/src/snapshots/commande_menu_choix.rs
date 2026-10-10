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
                    .table(Alias::new("commande_menu_choix"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("commande_ligne_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("cours")).string_len(20).not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("entree_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("dessert_id")).integer().null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_menu_choix_commande_ligne_id_commande_lignes_fkey")
                    .from(Alias::new("commande_menu_choix"), Alias::new("commande_ligne_id"))
                    .to(Alias::new("commande_lignes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_menu_choix_plat_id_plats_fkey")
                    .from(Alias::new("commande_menu_choix"), Alias::new("plat_id"))
                    .to(Alias::new("plats"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_menu_choix_entree_id_entrees_fkey")
                    .from(Alias::new("commande_menu_choix"), Alias::new("entree_id"))
                    .to(Alias::new("entrees"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_menu_choix_dessert_id_desserts_fkey")
                    .from(Alias::new("commande_menu_choix"), Alias::new("dessert_id"))
                    .to(Alias::new("desserts"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_menu_choix_commande_ligne_id")
                    .table(Alias::new("commande_menu_choix"))
                    .col(Alias::new("commande_ligne_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_menu_choix_plat_id")
                    .table(Alias::new("commande_menu_choix"))
                    .col(Alias::new("plat_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_menu_choix_entree_id")
                    .table(Alias::new("commande_menu_choix"))
                    .col(Alias::new("entree_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_menu_choix_dessert_id")
                    .table(Alias::new("commande_menu_choix"))
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
                    .table(Alias::new("commande_menu_choix"))
                    .name("commande_menu_choix_commande_ligne_id_commande_lignes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_menu_choix"))
                    .name("commande_menu_choix_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_menu_choix"))
                    .name("commande_menu_choix_entree_id_entrees_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_menu_choix"))
                    .name("commande_menu_choix_dessert_id_desserts_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_menu_choix_commande_ligne_id").table(Alias::new("commande_menu_choix")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_menu_choix_plat_id").table(Alias::new("commande_menu_choix")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_menu_choix_entree_id").table(Alias::new("commande_menu_choix")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_menu_choix_dessert_id").table(Alias::new("commande_menu_choix")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("commande_menu_choix"))
                .to_owned())
            .await?;
        Ok(())
}
}
