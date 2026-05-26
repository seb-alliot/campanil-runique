use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "UPDATE supplements SET garniture_id = NULL \
             WHERE garniture_id IN (SELECT id FROM garnitures WHERE type_garniture = 'accompagnement')",
        )
        .await?;
        db.execute_unprepared(
            "DELETE FROM garnitures WHERE type_garniture = 'accompagnement'",
        )
        .await?;
        db.execute_unprepared(
            "ALTER TABLE garnitures DROP COLUMN IF EXISTS prix",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE garnitures ADD COLUMN IF NOT EXISTS prix DECIMAL(10,2)",
        )
        .await?;
        Ok(())
    }
}
