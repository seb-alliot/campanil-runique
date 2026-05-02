use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE CuissonViande AS ENUM ('Bleu', 'Saignant', 'APoint', 'BienCuit'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_plats"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("commande_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("quantite")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("prix_unitaire")).decimal().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("cuisson"), ColumnType::Enum { name: Alias::new("CuissonViande").into_iden(), variants: vec![Alias::new("Bleu").into_iden(), Alias::new("Saignant").into_iden(), Alias::new("APoint").into_iden(), Alias::new("BienCuit").into_iden()] }).null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_plats_commande_id_commandes_fkey")
                    .from(Alias::new("commande_plats"), Alias::new("commande_id"))
                    .to(Alias::new("commandes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_plats_plat_id_plats_fkey")
                    .from(Alias::new("commande_plats"), Alias::new("plat_id"))
                    .to(Alias::new("plats"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_plats"))
                    .name("commande_plats_commande_id_commandes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_plats"))
                    .name("commande_plats_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("commande_plats"))
                .to_owned())
            .await?;
        manager.get_connection().execute_unprepared(
            "DROP TYPE IF EXISTS CuissonViande"
        ).await?;

        Ok(())
}
}
