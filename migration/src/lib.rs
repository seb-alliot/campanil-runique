use runique::prelude::migrations_table;
use sea_orm_migration::prelude::*;
mod m20260506_190038_create_allergenes_table;
mod m20260506_190038_create_avis_table;
mod m20260506_190038_create_boissons_table;
mod m20260506_190038_create_commande_plats_table;
mod m20260506_190038_create_commande_statuts_table;
mod m20260506_190038_create_commandes_table;
mod m20260506_190038_create_contacts_table;
mod m20260506_190038_create_devis_traiteur_table;
mod m20260506_190038_create_horaires_table;
mod m20260506_190038_create_info_resto_table;
mod m20260506_190038_create_menu_images_table;
mod m20260506_190038_create_menu_plat_table;
mod m20260506_190038_create_menus_table;
mod m20260506_190038_create_plat_allergene_table;
mod m20260506_190038_create_plats_table;
mod m20260506_190038_create_regimes_table;
mod m20260506_190038_create_relations;
mod m20260506_190038_create_themes_table;
mod m20260506_190038_extend_eihwaz_users_table;
mod m20260507_000000_alter_commandes_add_heure_retrait;
mod m20260512_000000_create_supplements_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations_table::EihwazUsersMigration),
            Box::new(migrations_table::EihwazSessionsMigration),
            Box::new(migrations_table::AdminTableMigration),
            Box::new(m20260506_190038_create_contacts_table::Migration),
            Box::new(m20260506_190038_create_horaires_table::Migration),
            Box::new(m20260506_190038_create_plats_table::Migration),
            Box::new(m20260506_190038_create_boissons_table::Migration),
            Box::new(m20260506_190038_create_regimes_table::Migration),
            Box::new(m20260506_190038_create_allergenes_table::Migration),
            Box::new(m20260506_190038_create_info_resto_table::Migration),
            Box::new(m20260506_190038_create_themes_table::Migration),
            Box::new(m20260506_190038_create_plat_allergene_table::Migration),
            Box::new(m20260506_190038_create_menus_table::Migration),
            Box::new(m20260506_190038_create_menu_images_table::Migration),
            Box::new(m20260506_190038_create_commandes_table::Migration),
            Box::new(m20260506_190038_create_menu_plat_table::Migration),
            Box::new(m20260506_190038_create_devis_traiteur_table::Migration),
            Box::new(m20260506_190038_create_avis_table::Migration),
            Box::new(m20260506_190038_create_commande_plats_table::Migration),
            Box::new(m20260506_190038_create_commande_statuts_table::Migration),
            Box::new(m20260506_190038_create_relations::Migration),
            Box::new(m20260506_190038_extend_eihwaz_users_table::Migration),
            Box::new(m20260512_000000_create_supplements_table::Migration),
        ]
    }
}
