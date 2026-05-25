use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Drop FK: devis_traiteur.menu_id -> old traiteur menus
        db.execute_unprepared(
            "ALTER TABLE devis_traiteur DROP CONSTRAINT IF EXISTS devis_traiteur_menu_id_menus_fkey"
        ).await?;

        // Drop FK: traiteur junction menu_plat -> old menus
        db.execute_unprepared(
            "ALTER TABLE menu_plat DROP CONSTRAINT IF EXISTS menu_plat_menu_id_menus_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_plat DROP CONSTRAINT IF EXISTS menu_plat_plat_id_plats_fkey"
        ).await?;

        // Drop traiteur-related tables (children before parents)
        db.execute_unprepared("DROP TABLE IF EXISTS menu_plat").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS menu_enfant_allergenes").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS menu_enfants").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS formule_jour_plat").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS formule_jours").await?;
        // Drop old traiteur menus table (was the menus table for traiteur)
        db.execute_unprepared("DROP TABLE IF EXISTS menus").await?;

        // Create TypeMenu enum
        db.execute_unprepared(
            "DO $$ BEGIN \
               CREATE TYPE TypeMenu AS ENUM ('menu_resto', 'menu_enfant', 'formule_jour'); \
             EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        // Rename menu_resto -> menus (becomes the unified menus table)
        db.execute_unprepared("ALTER TABLE menu_resto RENAME TO menus").await?;

        // Drop disponible (not in new model)
        db.execute_unprepared(
            "ALTER TABLE menus DROP COLUMN IF EXISTS disponible"
        ).await?;

        // Rename dessert -> dessert_libre; add entree_libre, plat_libre, image, type_menu
        db.execute_unprepared(
            "ALTER TABLE menus \
             RENAME COLUMN dessert TO dessert_libre"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menus \
             ADD COLUMN IF NOT EXISTS type_menu TypeMenu NOT NULL DEFAULT 'menu_resto', \
             ADD COLUMN IF NOT EXISTS entree_libre VARCHAR(500), \
             ADD COLUMN IF NOT EXISTS plat_libre VARCHAR(500), \
             ADD COLUMN IF NOT EXISTS image VARCHAR(500)"
        ).await?;

        // Create new junction tables
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS menu_entrees ( \
               id SERIAL PRIMARY KEY, \
               menu_id INTEGER NOT NULL REFERENCES menus(id) ON DELETE CASCADE, \
               plat_id INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE, \
               UNIQUE (menu_id, plat_id) \
             )"
        ).await?;

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS menu_plats ( \
               id SERIAL PRIMARY KEY, \
               menu_id INTEGER NOT NULL REFERENCES menus(id) ON DELETE CASCADE, \
               plat_id INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE, \
               UNIQUE (menu_id, plat_id) \
             )"
        ).await?;

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS menu_desserts ( \
               id SERIAL PRIMARY KEY, \
               menu_id INTEGER NOT NULL REFERENCES menus(id) ON DELETE CASCADE, \
               plat_id INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE, \
               UNIQUE (menu_id, plat_id) \
             )"
        ).await?;

        // Migrate menu_resto_plat data into the new junction tables
        db.execute_unprepared(
            "INSERT INTO menu_entrees (menu_id, plat_id) \
             SELECT menu_id, plat_id FROM menu_resto_plat WHERE cours = 'entree' \
             ON CONFLICT DO NOTHING"
        ).await?;
        db.execute_unprepared(
            "INSERT INTO menu_plats (menu_id, plat_id) \
             SELECT menu_id, plat_id FROM menu_resto_plat WHERE cours = 'plat' \
             ON CONFLICT DO NOTHING"
        ).await?;
        db.execute_unprepared(
            "INSERT INTO menu_desserts (menu_id, plat_id) \
             SELECT menu_id, plat_id FROM menu_resto_plat WHERE cours = 'dessert' \
             ON CONFLICT DO NOTHING"
        ).await?;

        // Drop old junction table
        db.execute_unprepared("DROP TABLE IF EXISTS menu_resto_plat").await?;

        // Add 'menu' value to TypeArticle enum
        // COMMIT requis : PostgreSQL interdit d'utiliser une nouvelle valeur d'enum
        // dans la même transaction que l'ALTER TYPE qui l'a créée.
        db.execute_unprepared("COMMIT").await?;
        db.execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'menu'"
        ).await?;
        db.execute_unprepared("BEGIN").await?;

        // Migrate commande_lignes
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             ADD COLUMN IF NOT EXISTS menu_id INTEGER"
        ).await?;
        db.execute_unprepared(
            "UPDATE commande_lignes \
             SET menu_id = menu_resto_id, type_article = 'menu' \
             WHERE menu_resto_id IS NOT NULL"
        ).await?;
        db.execute_unprepared(
            "UPDATE commande_lignes \
             SET menu_id = menu_enfant_id, type_article = 'menu' \
             WHERE menu_enfant_id IS NOT NULL"
        ).await?;
        db.execute_unprepared(
            "UPDATE commande_lignes \
             SET type_article = 'menu' \
             WHERE type_article IN ('formule_jour')"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             DROP COLUMN IF EXISTS menu_resto_id, \
             DROP COLUMN IF EXISTS formule_jour_id, \
             DROP COLUMN IF EXISTS menu_enfant_id"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             ADD CONSTRAINT commande_lignes_menu_id_menus_fkey \
             FOREIGN KEY (menu_id) REFERENCES menus(id) ON DELETE RESTRICT"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Restore commande_lignes
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             DROP CONSTRAINT IF EXISTS commande_lignes_menu_id_menus_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             ADD COLUMN IF NOT EXISTS menu_resto_id INTEGER, \
             ADD COLUMN IF NOT EXISTS formule_jour_id INTEGER, \
             ADD COLUMN IF NOT EXISTS menu_enfant_id INTEGER"
        ).await?;
        db.execute_unprepared(
            "UPDATE commande_lignes SET menu_resto_id = menu_id WHERE menu_id IS NOT NULL"
        ).await?;
        db.execute_unprepared(
            "UPDATE commande_lignes SET type_article = 'menu_resto' WHERE type_article = 'menu'"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes DROP COLUMN IF EXISTS menu_id"
        ).await?;

        // Drop junction tables
        db.execute_unprepared("DROP TABLE IF EXISTS menu_desserts").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS menu_plats").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS menu_entrees").await?;

        // Restore menu_resto
        db.execute_unprepared(
            "ALTER TABLE menus \
             DROP COLUMN IF EXISTS type_menu, \
             DROP COLUMN IF EXISTS entree_libre, \
             DROP COLUMN IF EXISTS plat_libre, \
             DROP COLUMN IF EXISTS image"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menus RENAME COLUMN dessert_libre TO dessert"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menus ADD COLUMN IF NOT EXISTS disponible BOOLEAN NOT NULL DEFAULT true"
        ).await?;
        db.execute_unprepared("ALTER TABLE menus RENAME TO menu_resto").await?;
        db.execute_unprepared("DROP TYPE IF EXISTS TypeMenu").await?;

        Ok(())
    }
}
