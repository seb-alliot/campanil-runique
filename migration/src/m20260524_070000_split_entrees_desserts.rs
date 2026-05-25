use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1. Créer les enums usage
        db.execute_unprepared(
            "DO $$ BEGIN CREATE TYPE UsageEntree AS ENUM ('carte', 'menu', 'les_deux'); \
             EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;
        db.execute_unprepared(
            "DO $$ BEGIN CREATE TYPE UsageDessert AS ENUM ('carte', 'menu', 'les_deux'); \
             EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;
        db.execute_unprepared(
            "DO $$ BEGIN CREATE TYPE UsagePlat AS ENUM ('carte', 'menu', 'les_deux'); \
             EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        // 2. Créer la table entrees
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS entrees ( \
               id          SERIAL PRIMARY KEY, \
               titre       VARCHAR(255) NOT NULL, \
               label       VARCHAR(80), \
               description TEXT, \
               image       VARCHAR(500), \
               prix        NUMERIC(10,2) NOT NULL, \
               disponible  BOOLEAN NOT NULL DEFAULT TRUE, \
               usage       UsageEntree NOT NULL DEFAULT 'les_deux', \
               ordre       INTEGER NOT NULL DEFAULT 0 \
             )"
        ).await?;

        // 3. Migrer les données depuis plats WHERE type_plat = 'entree'
        db.execute_unprepared(
            "INSERT INTO entrees (id, titre, label, description, image, prix, disponible, usage, ordre) \
             SELECT id, titre, label, description, image, prix, disponible, 'les_deux'::UsageEntree, \
                    COALESCE(ordre, 0) \
             FROM plats WHERE type_plat = 'entree' \
             ON CONFLICT (id) DO NOTHING"
        ).await?;

        // 4. Créer la table desserts
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS desserts ( \
               id          SERIAL PRIMARY KEY, \
               titre       VARCHAR(255) NOT NULL, \
               label       VARCHAR(80), \
               description TEXT, \
               image       VARCHAR(500), \
               prix        NUMERIC(10,2) NOT NULL, \
               disponible  BOOLEAN NOT NULL DEFAULT TRUE, \
               usage       UsageDessert NOT NULL DEFAULT 'les_deux', \
               ordre       INTEGER NOT NULL DEFAULT 0 \
             )"
        ).await?;

        // 5. Migrer les données depuis plats WHERE type_plat = 'dessert'
        db.execute_unprepared(
            "INSERT INTO desserts (id, titre, label, description, image, prix, disponible, usage, ordre) \
             SELECT id, titre, label, description, image, prix, disponible, 'les_deux'::UsageDessert, \
                    COALESCE(ordre, 0) \
             FROM plats WHERE type_plat = 'dessert' \
             ON CONFLICT (id) DO NOTHING"
        ).await?;

        // 6. Créer entree_allergene
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS entree_allergene ( \
               id           SERIAL PRIMARY KEY, \
               entree_id    INTEGER NOT NULL REFERENCES entrees(id) ON DELETE CASCADE, \
               allergene_id INTEGER NOT NULL REFERENCES allergenes(id) ON DELETE CASCADE, \
               UNIQUE (entree_id, allergene_id) \
             )"
        ).await?;

        // 7. Migrer allergenes des entrées
        db.execute_unprepared(
            "INSERT INTO entree_allergene (entree_id, allergene_id) \
             SELECT pa.plat_id, pa.allergene_id FROM plat_allergene pa \
             INNER JOIN plats p ON p.id = pa.plat_id AND p.type_plat = 'entree' \
             ON CONFLICT DO NOTHING"
        ).await?;

        // 8. Créer dessert_allergene
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS dessert_allergene ( \
               id           SERIAL PRIMARY KEY, \
               dessert_id   INTEGER NOT NULL REFERENCES desserts(id) ON DELETE CASCADE, \
               allergene_id INTEGER NOT NULL REFERENCES allergenes(id) ON DELETE CASCADE, \
               UNIQUE (dessert_id, allergene_id) \
             )"
        ).await?;

        // 9. Migrer allergenes des desserts
        db.execute_unprepared(
            "INSERT INTO dessert_allergene (dessert_id, allergene_id) \
             SELECT pa.plat_id, pa.allergene_id FROM plat_allergene pa \
             INNER JOIN plats p ON p.id = pa.plat_id AND p.type_plat = 'dessert' \
             ON CONFLICT DO NOTHING"
        ).await?;

        // 10. Ajouter colonne usage sur plats
        db.execute_unprepared(
            "ALTER TABLE plats ADD COLUMN IF NOT EXISTS usage UsagePlat NOT NULL DEFAULT 'les_deux'"
        ).await?;

        // 11. Supprimer allergenes des entrées/desserts dans plat_allergene
        db.execute_unprepared(
            "DELETE FROM plat_allergene \
             WHERE plat_id IN (SELECT id FROM plats WHERE type_plat IN ('entree', 'dessert'))"
        ).await?;

        // 12. Supprimer les entrées et desserts de la table plats
        db.execute_unprepared(
            "DELETE FROM plats WHERE type_plat IN ('entree', 'dessert')"
        ).await?;

        // 13. Mettre à jour menu_entrees : renommer plat_id -> entree_id, changer FK
        db.execute_unprepared(
            "ALTER TABLE menu_entrees DROP CONSTRAINT IF EXISTS menu_entrees_plat_id_plats_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees DROP CONSTRAINT IF EXISTS menu_entrees_plat_id_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees DROP CONSTRAINT IF EXISTS menu_entrees_menu_id_plat_id_key"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees RENAME COLUMN plat_id TO entree_id"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees \
             ADD CONSTRAINT menu_entrees_entree_id_entrees_fkey \
             FOREIGN KEY (entree_id) REFERENCES entrees(id) ON DELETE CASCADE"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees \
             ADD CONSTRAINT menu_entrees_menu_id_entree_id_key UNIQUE (menu_id, entree_id)"
        ).await?;

        // 14. Mettre à jour menu_desserts : renommer plat_id -> dessert_id, changer FK
        db.execute_unprepared(
            "ALTER TABLE menu_desserts DROP CONSTRAINT IF EXISTS menu_desserts_plat_id_plats_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts DROP CONSTRAINT IF EXISTS menu_desserts_plat_id_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts DROP CONSTRAINT IF EXISTS menu_desserts_menu_id_plat_id_key"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts RENAME COLUMN plat_id TO dessert_id"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts \
             ADD CONSTRAINT menu_desserts_dessert_id_desserts_fkey \
             FOREIGN KEY (dessert_id) REFERENCES desserts(id) ON DELETE CASCADE"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts \
             ADD CONSTRAINT menu_desserts_menu_id_dessert_id_key UNIQUE (menu_id, dessert_id)"
        ).await?;

        // 15. Resynchroniser les séquences
        db.execute_unprepared(
            "SELECT setval(pg_get_serial_sequence('entrees', 'id'), COALESCE(MAX(id), 1)) FROM entrees"
        ).await?;
        db.execute_unprepared(
            "SELECT setval(pg_get_serial_sequence('desserts', 'id'), COALESCE(MAX(id), 1)) FROM desserts"
        ).await?;
        db.execute_unprepared(
            "SELECT setval(pg_get_serial_sequence('entree_allergene', 'id'), COALESCE(MAX(id), 1)) FROM entree_allergene"
        ).await?;
        db.execute_unprepared(
            "SELECT setval(pg_get_serial_sequence('dessert_allergene', 'id'), COALESCE(MAX(id), 1)) FROM dessert_allergene"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Restaurer menu_desserts
        db.execute_unprepared(
            "ALTER TABLE menu_desserts DROP CONSTRAINT IF EXISTS menu_desserts_menu_id_dessert_id_key"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts DROP CONSTRAINT IF EXISTS menu_desserts_dessert_id_desserts_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts RENAME COLUMN dessert_id TO plat_id"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_desserts \
             ADD CONSTRAINT menu_desserts_plat_id_plats_fkey \
             FOREIGN KEY (plat_id) REFERENCES plats(id) ON DELETE CASCADE"
        ).await?;

        // Restaurer menu_entrees
        db.execute_unprepared(
            "ALTER TABLE menu_entrees DROP CONSTRAINT IF EXISTS menu_entrees_menu_id_entree_id_key"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees DROP CONSTRAINT IF EXISTS menu_entrees_entree_id_entrees_fkey"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees RENAME COLUMN entree_id TO plat_id"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE menu_entrees \
             ADD CONSTRAINT menu_entrees_plat_id_plats_fkey \
             FOREIGN KEY (plat_id) REFERENCES plats(id) ON DELETE CASCADE"
        ).await?;

        // Réinsérer dans plats
        db.execute_unprepared(
            "INSERT INTO plats (id, titre, label, description, image, prix, disponible, ordre, \
               est_viande, avec_legumes, type_plat) \
             SELECT id, titre, label, description, image, prix, disponible, ordre, \
               false, false, 'entree'::TypePlat \
             FROM entrees ON CONFLICT DO NOTHING"
        ).await?;
        db.execute_unprepared(
            "INSERT INTO plats (id, titre, label, description, image, prix, disponible, ordre, \
               est_viande, avec_legumes, type_plat) \
             SELECT id, titre, label, description, image, prix, disponible, ordre, \
               false, false, 'dessert'::TypePlat \
             FROM desserts ON CONFLICT DO NOTHING"
        ).await?;

        // Réinsérer allergenes
        db.execute_unprepared(
            "INSERT INTO plat_allergene (plat_id, allergene_id) \
             SELECT entree_id, allergene_id FROM entree_allergene ON CONFLICT DO NOTHING"
        ).await?;
        db.execute_unprepared(
            "INSERT INTO plat_allergene (plat_id, allergene_id) \
             SELECT dessert_id, allergene_id FROM dessert_allergene ON CONFLICT DO NOTHING"
        ).await?;

        db.execute_unprepared("ALTER TABLE plats DROP COLUMN IF EXISTS usage").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS dessert_allergene").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS entree_allergene").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS desserts").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS entrees").await?;
        db.execute_unprepared("DROP TYPE IF EXISTS UsagePlat").await?;
        db.execute_unprepared("DROP TYPE IF EXISTS UsageDessert").await?;
        db.execute_unprepared("DROP TYPE IF EXISTS UsageEntree").await?;

        Ok(())
    }
}
