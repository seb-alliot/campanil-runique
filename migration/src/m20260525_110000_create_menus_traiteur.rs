use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "DO $$ BEGIN CREATE TYPE ThemeMenu AS ENUM ('mariage', 'bapteme', 'anniversaire', 'autre'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        db.execute_unprepared(
            "DO $$ BEGIN CREATE TYPE RegimeMenu AS ENUM ('standard', 'vegetarien', 'sans_gluten', 'halal', 'casher'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS menus_traiteur (
                id                SERIAL PRIMARY KEY,
                titre             VARCHAR(255) NOT NULL,
                description       TEXT NOT NULL,
                prix_par_personne DECIMAL(10,2) NOT NULL,
                nb_personnes_min  INTEGER NOT NULL,
                theme             ThemeMenu NOT NULL DEFAULT 'autre',
                regime            RegimeMenu NOT NULL DEFAULT 'standard',
                conditions        TEXT,
                stock             INTEGER NOT NULL DEFAULT 0,
                actif             BOOLEAN NOT NULL DEFAULT TRUE,
                created_at        TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"
        ).await?;

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS menu_traiteur_plats (
                id                SERIAL PRIMARY KEY,
                menu_traiteur_id  INTEGER NOT NULL REFERENCES menus_traiteur(id) ON DELETE CASCADE,
                plat_id           INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
                UNIQUE(menu_traiteur_id, plat_id)
            )"
        ).await?;

        db.execute_unprepared(
            "ALTER TABLE devis_traiteur DROP CONSTRAINT IF EXISTS devis_traiteur_menu_id_menus_fkey"
        ).await?;

        db.execute_unprepared(
            "ALTER TABLE devis_traiteur ADD CONSTRAINT devis_traiteur_menu_id_menus_traiteur_fkey \
             FOREIGN KEY (menu_id) REFERENCES menus_traiteur(id) ON DELETE SET NULL"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "ALTER TABLE devis_traiteur DROP CONSTRAINT IF EXISTS devis_traiteur_menu_id_menus_traiteur_fkey"
        ).await?;

        db.execute_unprepared("DROP TABLE IF EXISTS menu_traiteur_plats").await?;
        db.execute_unprepared("DROP TABLE IF EXISTS menus_traiteur").await?;

        db.execute_unprepared("DROP TYPE IF EXISTS ThemeMenu").await?;
        db.execute_unprepared("DROP TYPE IF EXISTS RegimeMenu").await?;

        Ok(())
    }
}
