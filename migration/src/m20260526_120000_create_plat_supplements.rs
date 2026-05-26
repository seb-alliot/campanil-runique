use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS plat_supplements (
                id            SERIAL PRIMARY KEY,
                plat_id       INTEGER NOT NULL REFERENCES plats(id) ON DELETE CASCADE,
                supplement_id INTEGER NOT NULL REFERENCES supplements(id) ON DELETE CASCADE,
                UNIQUE(plat_id, supplement_id)
            )",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE IF EXISTS plat_supplements").await?;
        Ok(())
    }
}
