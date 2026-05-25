use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // PostgreSQL interdit d'utiliser une nouvelle valeur d'enum dans la même
        // transaction que l'ALTER TYPE → on commit/begin autour.
        db.execute_unprepared("COMMIT").await?;
        db.execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'entree'"
        ).await?;
        db.execute_unprepared(
            "ALTER TYPE TypeArticle ADD VALUE IF NOT EXISTS 'dessert'"
        ).await?;
        db.execute_unprepared("BEGIN").await?;

        // commande_lignes : ajouter entree_id et dessert_id
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             ADD COLUMN IF NOT EXISTS entree_id INTEGER REFERENCES entrees(id) ON DELETE RESTRICT, \
             ADD COLUMN IF NOT EXISTS dessert_id INTEGER REFERENCES desserts(id) ON DELETE RESTRICT"
        ).await?;

        // commande_menu_choix : rendre plat_id nullable, ajouter entree_id et dessert_id
        db.execute_unprepared(
            "ALTER TABLE commande_menu_choix \
             ALTER COLUMN plat_id DROP NOT NULL"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_menu_choix \
             ADD COLUMN IF NOT EXISTS entree_id INTEGER REFERENCES entrees(id) ON DELETE RESTRICT, \
             ADD COLUMN IF NOT EXISTS dessert_id INTEGER REFERENCES desserts(id) ON DELETE RESTRICT"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "ALTER TABLE commande_menu_choix \
             DROP COLUMN IF EXISTS entree_id, \
             DROP COLUMN IF EXISTS dessert_id"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_menu_choix \
             ALTER COLUMN plat_id SET NOT NULL"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes \
             DROP COLUMN IF EXISTS entree_id, \
             DROP COLUMN IF EXISTS dessert_id"
        ).await?;

        Ok(())
    }
}
