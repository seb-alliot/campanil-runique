use runique::prelude::migrations_table;
use sea_orm_migration::prelude::*;
mod m20261106_152121_create_allergenes_table;
mod m20261106_152121_create_boissons_table;
mod m20261106_152121_create_commandes_table;
mod m20261106_152121_create_avis_table;
mod m20261106_152121_create_commande_statuts_table;
mod m20261106_152121_create_contacts_table;
mod m20261106_152121_create_desserts_table;
mod m20261106_152121_create_dessert_allergenes_table;
mod m20261106_152121_create_entrees_table;
mod m20261106_152121_create_entree_allergenes_table;
mod m20261106_152121_create_garnitures_table;
mod m20261106_152121_create_horaires_table;
mod m20261106_152121_create_info_resto_table;
mod m20261106_152121_create_menus_table;
mod m20261106_152121_create_menu_desserts_table;
mod m20261106_152121_create_menu_entrees_table;
mod m20261106_152121_create_menus_traiteur_table;
mod m20261106_152121_create_devis_traiteur_table;
mod m20261106_152121_create_plats_table;
mod m20261106_152121_create_avis_plats_table;
mod m20261106_152121_create_menu_plats_table;
mod m20261106_152121_create_menu_traiteur_plats_table;
mod m20261106_152121_create_plat_allergenes_table;
mod m20261106_152121_create_plat_garnitures_table;
mod m20261106_152121_create_supplements_table;
mod m20261106_152121_create_commande_lignes_table;
mod m20261106_152121_create_commande_ligne_garnitures_table;
mod m20261106_152121_create_commande_menu_choix_table;
mod m20261106_152121_create_plat_supplements_table;
mod m20261106_152121_extend_eihwaz_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations_table::EihwazUsersMigration),
            Box::new(migrations_table::EihwazSessionsMigration),
            Box::new(migrations_table::AdminTableMigration),
            Box::new(migrations_table::EihwazHistoryMigration),
            Box::new(migrations_table::EihwazResetTokensMigration),
            Box::new(m20261106_152121_create_allergenes_table::Migration),
            Box::new(m20261106_152121_create_boissons_table::Migration),
            Box::new(m20261106_152121_create_commandes_table::Migration),
            Box::new(m20261106_152121_create_avis_table::Migration),
            Box::new(m20261106_152121_create_commande_statuts_table::Migration),
            Box::new(m20261106_152121_create_contacts_table::Migration),
            Box::new(m20261106_152121_create_desserts_table::Migration),
            Box::new(m20261106_152121_create_dessert_allergenes_table::Migration),
            Box::new(m20261106_152121_create_entrees_table::Migration),
            Box::new(m20261106_152121_create_entree_allergenes_table::Migration),
            Box::new(m20261106_152121_create_garnitures_table::Migration),
            Box::new(m20261106_152121_create_horaires_table::Migration),
            Box::new(m20261106_152121_create_info_resto_table::Migration),
            Box::new(m20261106_152121_create_menus_table::Migration),
            Box::new(m20261106_152121_create_menu_desserts_table::Migration),
            Box::new(m20261106_152121_create_menu_entrees_table::Migration),
            Box::new(m20261106_152121_create_menus_traiteur_table::Migration),
            Box::new(m20261106_152121_create_devis_traiteur_table::Migration),
            Box::new(m20261106_152121_create_plats_table::Migration),
            Box::new(m20261106_152121_create_avis_plats_table::Migration),
            Box::new(m20261106_152121_create_menu_plats_table::Migration),
            Box::new(m20261106_152121_create_menu_traiteur_plats_table::Migration),
            Box::new(m20261106_152121_create_plat_allergenes_table::Migration),
            Box::new(m20261106_152121_create_plat_garnitures_table::Migration),
            Box::new(m20261106_152121_create_supplements_table::Migration),
            Box::new(m20261106_152121_create_commande_lignes_table::Migration),
            Box::new(m20261106_152121_create_commande_ligne_garnitures_table::Migration),
            Box::new(m20261106_152121_create_commande_menu_choix_table::Migration),
            Box::new(m20261106_152121_create_plat_supplements_table::Migration),
            Box::new(m20261106_152121_extend_eihwaz_users_table::Migration),
        ]
    }
}
