use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "CREATE TYPE raisoncontact AS ENUM ('reservation', 'traiteur', 'commande', 'autre')"
        ).await?;
        manager.get_connection().execute_unprepared(
            "ALTER TABLE contacts ADD COLUMN raison raisoncontact NOT NULL DEFAULT 'autre'"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("contacts"))
                    .drop_column(Alias::new("raison"))
                    .to_owned(),
            )
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS raisoncontact"
        ).await?;
        Ok(())
    }
}
