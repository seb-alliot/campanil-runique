use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE Jour AS ENUM ('Lundi', 'Mardi', 'Mercredi', 'Jeudi', 'Vendredi', 'Samedi', 'Dimanche'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("horaires"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new_with_type(Alias::new("jour"), ColumnType::Enum { name: Alias::new("Jour").into_iden(), variants: vec![Alias::new("Lundi").into_iden(), Alias::new("Mardi").into_iden(), Alias::new("Mercredi").into_iden(), Alias::new("Jeudi").into_iden(), Alias::new("Vendredi").into_iden(), Alias::new("Samedi").into_iden(), Alias::new("Dimanche").into_iden()] }).not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("heure_ouverture")).time().null())
                    .col(ColumnDef::new(Alias::new("heure_fermeture")).time().null())
                    .col(ColumnDef::new(Alias::new("ferme")).boolean().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("horaires"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS Jour"
        ).await?;

        Ok(())
}
}
