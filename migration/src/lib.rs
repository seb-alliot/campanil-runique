use runique::prelude::migrations_table;
use sea_orm_migration::prelude::*;
mod m20260502_063428_extend_eihwaz_users_table;
mod m20260502_063428_create_avis_table;
mod m20260502_063428_create_commande_statuts_table;
mod m20260502_063428_create_commande_plats_table;
mod m20260502_063428_create_commandes_table;
mod m20260502_063428_create_menu_plat_table;
mod m20260502_063428_create_menu_images_table;
mod m20260502_063428_create_plat_allergene_table;
mod m20260502_063428_create_menus_table;
mod m20260502_063428_create_allergenes_table;
mod m20260502_063428_create_plats_table;
mod m20260502_063428_create_themes_table;
mod m20260502_063428_create_regimes_table;
mod m20260502_063428_create_contacts_table;
mod m20260502_063428_create_horaires_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migrations_table::EihwazUsersMigration),
            Box::new(migrations_table::AdminTableMigration),
            Box::new(m20260502_063428_create_horaires_table::Migration),
            Box::new(m20260502_063428_create_contacts_table::Migration),
            Box::new(m20260502_063428_create_regimes_table::Migration),
            Box::new(m20260502_063428_create_themes_table::Migration),
            Box::new(m20260502_063428_create_plats_table::Migration),
            Box::new(m20260502_063428_create_allergenes_table::Migration),
            Box::new(m20260502_063428_create_menus_table::Migration),
            Box::new(m20260502_063428_create_plat_allergene_table::Migration),
            Box::new(m20260502_063428_create_menu_images_table::Migration),
            Box::new(m20260502_063428_create_menu_plat_table::Migration),
            Box::new(m20260502_063428_create_commandes_table::Migration),
            Box::new(m20260502_063428_create_commande_plats_table::Migration),
            Box::new(m20260502_063428_create_commande_statuts_table::Migration),
            Box::new(m20260502_063428_create_avis_table::Migration),
            Box::new(m20260502_063428_extend_eihwaz_users_table::Migration),
        ]
    }
}
