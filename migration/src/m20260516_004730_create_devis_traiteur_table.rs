use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE StatutDevis AS ENUM ('en_attente', 'en_cours', 'accepte', 'refuse'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("devis_traiteur"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("nom")).string().not_null())
                    .col(ColumnDef::new(Alias::new("email")).string().not_null())
                    .col(ColumnDef::new(Alias::new("telephone")).string().null())
                    .col(ColumnDef::new(Alias::new("date_evenement")).string().not_null())
                    .col(ColumnDef::new(Alias::new("nb_personnes")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("message")).text().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), ColumnType::Enum { name: Alias::new("StatutDevis").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("en_cours").into_iden(), Alias::new("accepte").into_iden(), Alias::new("refuse").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("devis_traiteur"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS StatutDevis"
        ).await?;

        Ok(())
}
}
