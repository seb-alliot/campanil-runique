use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE TypeCommande AS ENUM ('Carte', 'Traiteur'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE StatutCommande AS ENUM ('EnAttente', 'Accepte', 'EnPreparation', 'Pret', 'EnCoursLivraison', 'Livre', 'EnAttenteRetourMateriel', 'Termine', 'Annule'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE ModePaiement AS ENUM ('Especes', 'CarteBancaire', 'EnLigne'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commandes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("numero")).string().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_commande"), ColumnType::Enum { name: Alias::new("TypeCommande").into_iden(), variants: vec![Alias::new("Carte").into_iden(), Alias::new("Traiteur").into_iden()] }).not_null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), ColumnType::Enum { name: Alias::new("StatutCommande").into_iden(), variants: vec![Alias::new("EnAttente").into_iden(), Alias::new("Accepte").into_iden(), Alias::new("EnPreparation").into_iden(), Alias::new("Pret").into_iden(), Alias::new("EnCoursLivraison").into_iden(), Alias::new("Livre").into_iden(), Alias::new("EnAttenteRetourMateriel").into_iden(), Alias::new("Termine").into_iden(), Alias::new("Annule").into_iden()] }).not_null())
                    .col(ColumnDef::new_with_type(Alias::new("mode_paiement"), ColumnType::Enum { name: Alias::new("ModePaiement").into_iden(), variants: vec![Alias::new("Especes").into_iden(), Alias::new("CarteBancaire").into_iden(), Alias::new("EnLigne").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("prix_total")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("nb_personnes")).integer().null())
                    .col(ColumnDef::new(Alias::new("date_prestation")).date_time().null())
                    .col(ColumnDef::new(Alias::new("avec_materiel")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("avec_livraison")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("adresse_livraison")).string().null())
                    .col(ColumnDef::new(Alias::new("ville_livraison")).string().null())
                    .col(ColumnDef::new(Alias::new("cp_livraison")).string().null())
                    .col(ColumnDef::new(Alias::new("heure_livraison")).time().null())
                    .col(ColumnDef::new(Alias::new("prix_livraison")).decimal().null())
                    .col(ColumnDef::new(Alias::new("stripe_payment_intent_id")).string().null())
                    .col(ColumnDef::new(Alias::new("motif_annulation")).text().null())
                    .col(ColumnDef::new(Alias::new("mode_contact_annulation")).string().null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updated_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commandes_menu_id_menus_fkey")
                    .from(Alias::new("commandes"), Alias::new("menu_id"))
                    .to(Alias::new("menus"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager.get_connection().execute_unprepared(
            "CREATE OR REPLACE FUNCTION set_updated_at_commandes() RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW(); RETURN NEW; END; $$ LANGUAGE plpgsql;"
        ).await?;
        manager.get_connection().execute_unprepared(
            "CREATE TRIGGER trg_commandes_updated_at BEFORE UPDATE ON commandes FOR EACH ROW EXECUTE PROCEDURE set_updated_at_commandes();"
        ).await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DROP TRIGGER IF EXISTS trg_commandes_updated_at ON commandes;"
        ).await?;
        manager.get_connection().execute_unprepared(
            "DROP FUNCTION IF EXISTS set_updated_at_commandes();"
        ).await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commandes"))
                    .name("commandes_menu_id_menus_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("commandes"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS TypeCommande"
        ).await?;

        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS StatutCommande"
        ).await?;

        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS ModePaiement"
        ).await?;

        Ok(())
}
}
