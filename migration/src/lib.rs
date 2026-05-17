use runique::prelude::migrations_table;
use sea_orm_migration::prelude::*;
mod m20260516_010432_alter_plat_allergene_table;
mod m20260516_020000_alter_info_resto_add_coords;
mod m20260516_040000_alter_menu_resto_add_dessert;
mod m20260516_010432_alter_menu_plat_table;
mod m20260516_010422_alter_menu_resto_table;
mod m20260516_010422_alter_menu_plat_table;
mod m20260516_010422_alter_boissons_table;
mod m20260516_010422_alter_plat_allergene_table;
mod m20260516_010422_alter_plats_table;
mod m20260516_004730_extend_eihwaz_users_table;
mod m20260516_004730_create_relations;
mod m20260516_004730_create_commande_statuts_table;
mod m20260516_004730_create_avis_table;
mod m20260516_004730_create_commande_lignes_table;
mod m20260516_004730_create_avis_plats_table;
mod m20260516_004730_create_plat_allergene_table;
mod m20260516_004730_create_menu_plat_table;
mod m20260516_004730_create_devis_traiteur_table;
mod m20260516_004730_create_supplements_table;
mod m20260516_004730_create_commandes_table;
mod m20260516_004730_create_menu_enfant_allergenes_table;
mod m20260516_004730_create_info_resto_table;
mod m20260516_004730_create_menu_resto_table;
mod m20260516_004730_create_horaires_table;
mod m20260516_004730_create_plats_table;
mod m20260516_004730_create_commande_menu_choix_table;
mod m20260516_004730_create_menus_table;
mod m20260516_004730_create_contacts_table;
mod m20260516_004730_create_allergenes_table;
mod m20260516_004730_create_commande_ligne_garnitures_table;
mod m20260516_004730_create_boissons_table;
mod m20260516_004730_create_plat_garnitures_table;
mod m20260516_004730_create_menu_resto_plat_table;
mod m20260516_004730_create_menu_enfants_table;
mod m20260516_004730_create_garnitures_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations_table::EihwazUsersMigration),
            Box::new(migrations_table::EihwazSessionsMigration),
            Box::new(migrations_table::AdminTableMigration),
            Box::new(m20260516_004730_create_garnitures_table::Migration),
            Box::new(m20260516_004730_create_menu_enfants_table::Migration),
            Box::new(m20260516_004730_create_menu_resto_plat_table::Migration),
            Box::new(m20260516_004730_create_plat_garnitures_table::Migration),
            Box::new(m20260516_004730_create_boissons_table::Migration),
            Box::new(m20260516_004730_create_commande_ligne_garnitures_table::Migration),
            Box::new(m20260516_004730_create_allergenes_table::Migration),
            Box::new(m20260516_004730_create_contacts_table::Migration),
            Box::new(m20260516_004730_create_menus_table::Migration),
            Box::new(m20260516_004730_create_commande_menu_choix_table::Migration),
            Box::new(m20260516_004730_create_plats_table::Migration),
            Box::new(m20260516_004730_create_horaires_table::Migration),
            Box::new(m20260516_004730_create_menu_resto_table::Migration),
            Box::new(m20260516_004730_create_info_resto_table::Migration),
            Box::new(m20260516_004730_create_menu_enfant_allergenes_table::Migration),
            Box::new(m20260516_004730_create_commandes_table::Migration),
            Box::new(m20260516_004730_create_supplements_table::Migration),
            Box::new(m20260516_004730_create_devis_traiteur_table::Migration),
            Box::new(m20260516_004730_create_menu_plat_table::Migration),
            Box::new(m20260516_004730_create_plat_allergene_table::Migration),
            Box::new(m20260516_004730_create_avis_plats_table::Migration),
            Box::new(m20260516_004730_create_commande_lignes_table::Migration),
            Box::new(m20260516_004730_create_avis_table::Migration),
            Box::new(m20260516_004730_create_commande_statuts_table::Migration),
            Box::new(m20260516_004730_create_relations::Migration),
            Box::new(m20260516_004730_extend_eihwaz_users_table::Migration),
            Box::new(m20260516_010422_alter_plats_table::Migration),
            Box::new(m20260516_010422_alter_plat_allergene_table::Migration),
            Box::new(m20260516_010422_alter_boissons_table::Migration),
            Box::new(m20260516_010422_alter_menu_plat_table::Migration),
            Box::new(m20260516_010422_alter_menu_resto_table::Migration),
            Box::new(m20260516_010432_alter_menu_plat_table::Migration),
            Box::new(m20260516_010432_alter_plat_allergene_table::Migration),
            Box::new(m20260516_020000_alter_info_resto_add_coords::Migration),
            Box::new(m20260516_040000_alter_menu_resto_add_dessert::Migration),
        ]
    }
}
