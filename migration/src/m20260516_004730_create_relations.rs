use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("supplements_garniture_id_garnitures_fkey")
                    .from(Alias::new("supplements"), Alias::new("garniture_id"))
                    .to(Alias::new("garnitures"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("devis_traiteur_menu_id_menus_fkey")
                    .from(Alias::new("devis_traiteur"), Alias::new("menu_id"))
                    .to(Alias::new("menus"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
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
            .create_foreign_key(
                ForeignKey::create()
                    .name("plat_allergene_plat_id_plats_fkey")
                    .from(Alias::new("plat_allergene"), Alias::new("plat_id"))
                    .to(Alias::new("plats"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("plat_allergene_allergene_id_allergenes_fkey")
                    .from(Alias::new("plat_allergene"), Alias::new("allergene_id"))
                    .to(Alias::new("allergenes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("avis_plats_plat_id_plats_fkey")
                    .from(Alias::new("avis_plats"), Alias::new("plat_id"))
                    .to(Alias::new("plats"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_commande_id_commandes_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("commande_id"))
                    .to(Alias::new("commandes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_plat_id_plats_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("plat_id"))
                    .to(Alias::new("plats"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_boisson_id_boissons_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("boisson_id"))
                    .to(Alias::new("boissons"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("avis_commande_id_commandes_fkey")
                    .from(Alias::new("avis"), Alias::new("commande_id"))
                    .to(Alias::new("commandes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_statuts_commande_id_commandes_fkey")
                    .from(Alias::new("commande_statuts"), Alias::new("commande_id"))
                    .to(Alias::new("commandes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_statuts"))
                    .name("commande_statuts_commande_id_commandes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("avis"))
                    .name("avis_commande_id_commandes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_commande_id_commandes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_boisson_id_boissons_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("avis_plats"))
                    .name("avis_plats_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("plat_allergene"))
                    .name("plat_allergene_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("plat_allergene"))
                    .name("plat_allergene_allergene_id_allergenes_fkey")
                    .to_owned(),
            )
            .await?;

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
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("devis_traiteur"))
                    .name("devis_traiteur_menu_id_menus_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("supplements"))
                    .name("supplements_garniture_id_garnitures_fkey")
                    .to_owned(),
            )
            .await?;

        Ok(())
}
}
