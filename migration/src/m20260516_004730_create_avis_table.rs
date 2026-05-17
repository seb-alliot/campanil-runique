use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE StatutAvis AS ENUM ('en_attente', 'valide', 'refuse'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("avis"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("commande_id")).integer().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("note")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("commentaire")).text().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), ColumnType::Enum { name: Alias::new("StatutAvis").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("valide").into_iden(), Alias::new("refuse").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("avis"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS StatutAvis"
        ).await?;

        Ok(())
}
}
