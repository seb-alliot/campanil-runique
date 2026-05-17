use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE TypeArticle AS ENUM ('plat', 'menu_resto', 'formule_jour', 'menu_enfant', 'boisson', 'supplement'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE CuissonViande AS ENUM ('bleu', 'saignant', 'a_point', 'bien_cuit'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_lignes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("commande_id")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("type_article"), ColumnType::Enum { name: Alias::new("TypeArticle").into_iden(), variants: vec![Alias::new("plat").into_iden(), Alias::new("menu_resto").into_iden(), Alias::new("formule_jour").into_iden(), Alias::new("menu_enfant").into_iden(), Alias::new("boisson").into_iden(), Alias::new("supplement").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("menu_resto_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("formule_jour_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("menu_enfant_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("boisson_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("supplement_id")).integer().null())
                    .col(ColumnDef::new_with_type(Alias::new("cuisson"), ColumnType::Enum { name: Alias::new("CuissonViande").into_iden(), variants: vec![Alias::new("bleu").into_iden(), Alias::new("saignant").into_iden(), Alias::new("a_point").into_iden(), Alias::new("bien_cuit").into_iden()] }).null())
                    .col(ColumnDef::new(Alias::new("avec_legumes")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("sans_sel")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("note")).string().null())
                    .col(ColumnDef::new(Alias::new("quantite")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("prix_unitaire")).decimal().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("commande_lignes"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS TypeArticle"
        ).await?;

        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS CuissonViande"
        ).await?;

        Ok(())
}
}
