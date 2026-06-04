use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "ALTER TABLE avis_plats ALTER COLUMN plat_id DROP NOT NULL;
             ALTER TABLE avis_plats ADD COLUMN IF NOT EXISTS entree_id INTEGER NULL REFERENCES entrees(id) ON DELETE CASCADE;
             ALTER TABLE avis_plats ADD COLUMN IF NOT EXISTS dessert_id INTEGER NULL REFERENCES desserts(id) ON DELETE CASCADE;"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "ALTER TABLE avis_plats DROP COLUMN IF EXISTS entree_id;
             ALTER TABLE avis_plats DROP COLUMN IF EXISTS dessert_id;
             ALTER TABLE avis_plats ALTER COLUMN plat_id SET NOT NULL;"
        ).await?;
        Ok(())
    }
}
