use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE devis_traiteur ALTER COLUMN date_evenement TYPE DATE USING date_evenement::DATE",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE devis_traiteur ALTER COLUMN date_evenement TYPE VARCHAR(10) USING date_evenement::TEXT",
            )
            .await?;
        Ok(())
    }
}
