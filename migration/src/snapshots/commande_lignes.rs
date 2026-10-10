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
                    .table(Alias::new("commande_lignes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("commande_id")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_article"), ColumnType::Enum { name: Alias::new("TypeArticle").into_iden(), variants: vec![Alias::new("plat").into_iden(), Alias::new("entree").into_iden(), Alias::new("dessert").into_iden(), Alias::new("menu").into_iden(), Alias::new("boisson").into_iden(), Alias::new("supplement").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("entree_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("dessert_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("boisson_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("supplement_id")).integer().null())
                    .col(ColumnDef::new_with_type(Alias::new("cuisson"), ColumnType::Enum { name: Alias::new("CuissonViande").into_iden(), variants: vec![Alias::new("bleu").into_iden(), Alias::new("saignant").into_iden(), Alias::new("a_point").into_iden(), Alias::new("bien_cuit").into_iden()] }).null())
                    .col(ColumnDef::new(Alias::new("sans_sel")).boolean().not_null().default(false))
                    .col(ColumnDef::new(Alias::new("note")).string_len(500).null())
                    .col(ColumnDef::new(Alias::new("quantite")).integer().not_null().default(1))
                    .col(ColumnDef::new(Alias::new("prix_unitaire")).decimal().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_commande_id_commandes_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("commande_id"))
                    .to(Alias::new("commandes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
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
                    .on_delete(ForeignKeyAction::Restrict)
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
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_entree_id_entrees_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("entree_id"))
                    .to(Alias::new("entrees"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_dessert_id_desserts_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("dessert_id"))
                    .to(Alias::new("desserts"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_menu_id_menus_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("menu_id"))
                    .to(Alias::new("menus"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_lignes_supplement_id_supplements_fkey")
                    .from(Alias::new("commande_lignes"), Alias::new("supplement_id"))
                    .to(Alias::new("supplements"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_commande_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("commande_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_plat_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("plat_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_boisson_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("boisson_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_entree_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("entree_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_dessert_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("dessert_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_menu_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("menu_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_lignes_supplement_id")
                    .table(Alias::new("commande_lignes"))
                    .col(Alias::new("supplement_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_entree_id_entrees_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_dessert_id_desserts_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_menu_id_menus_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_lignes"))
                    .name("commande_lignes_supplement_id_supplements_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_commande_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_plat_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_boisson_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_entree_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_dessert_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_menu_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_lignes_supplement_id").table(Alias::new("commande_lignes")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("commande_lignes"))
                .to_owned())
            .await?;
        Ok(())
}
}
