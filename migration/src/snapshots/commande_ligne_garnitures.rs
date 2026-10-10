// runique: column lengths recorded
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_ligne_garnitures"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("ligne_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("garniture_id")).integer().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_ligne_garnitures_ligne_id_commande_lignes_fkey")
                    .from(Alias::new("commande_ligne_garnitures"), Alias::new("ligne_id"))
                    .to(Alias::new("commande_lignes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("commande_ligne_garnitures_garniture_id_garnitures_fkey")
                    .from(Alias::new("commande_ligne_garnitures"), Alias::new("garniture_id"))
                    .to(Alias::new("garnitures"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_ligne_garnitures_ligne_id")
                    .table(Alias::new("commande_ligne_garnitures"))
                    .col(Alias::new("ligne_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_commande_ligne_garnitures_garniture_id")
                    .table(Alias::new("commande_ligne_garnitures"))
                    .col(Alias::new("garniture_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_ligne_garnitures"))
                    .name("commande_ligne_garnitures_ligne_id_commande_lignes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("commande_ligne_garnitures"))
                    .name("commande_ligne_garnitures_garniture_id_garnitures_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_ligne_garnitures_ligne_id").table(Alias::new("commande_ligne_garnitures")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_commande_ligne_garnitures_garniture_id").table(Alias::new("commande_ligne_garnitures")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("commande_ligne_garnitures"))
                .to_owned())
            .await?;
        Ok(())
}
}
