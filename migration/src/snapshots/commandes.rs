use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commandes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("numero")).string().not_null().unique_key())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), ColumnType::Enum { name: Alias::new("StatutCommande").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("accepte").into_iden(), Alias::new("en_preparation").into_iden(), Alias::new("pret").into_iden(), Alias::new("en_cours_livraison").into_iden(), Alias::new("livre").into_iden(), Alias::new("termine").into_iden(), Alias::new("annule").into_iden()] }).not_null())
                    .col(ColumnDef::new_with_type(Alias::new("mode_paiement"), ColumnType::Enum { name: Alias::new("ModePaiement").into_iden(), variants: vec![Alias::new("especes").into_iden(), Alias::new("carte_bancaire").into_iden(), Alias::new("en_ligne").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("prix_total")).decimal().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_retrait"), ColumnType::Enum { name: Alias::new("TypeRetrait").into_iden(), variants: vec![Alias::new("sur_place").into_iden(), Alias::new("livraison").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("heure_retrait")).date_time().null())
                    .col(ColumnDef::new(Alias::new("adresse_livraison")).string().null())
                    .col(ColumnDef::new(Alias::new("ville_livraison")).string().null())
                    .col(ColumnDef::new(Alias::new("cp_livraison")).string().null())
                    .col(ColumnDef::new(Alias::new("prix_livraison")).decimal().null())
                    .col(ColumnDef::new(Alias::new("stripe_payment_intent_id")).string().null())
                    .col(ColumnDef::new(Alias::new("motif_annulation")).text().null())
                    .col(ColumnDef::new(Alias::new("mode_contact_annulation")).string().null())
                    .col(ColumnDef::new(Alias::new("date_annulation")).date_time().null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Alias::new("updated_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("commandes"))
                .to_owned())
            .await?;
        Ok(())
}
}
