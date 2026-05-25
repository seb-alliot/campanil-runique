use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE plats DROP COLUMN IF EXISTS avec_legumes"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes DROP COLUMN IF EXISTS avec_legumes"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE plats ADD COLUMN IF NOT EXISTS avec_legumes BOOLEAN NOT NULL DEFAULT TRUE"
        ).await?;
        db.execute_unprepared(
            "ALTER TABLE commande_lignes ADD COLUMN IF NOT EXISTS avec_legumes BOOLEAN NOT NULL DEFAULT FALSE"
        ).await?;
        Ok(())
    }
}
