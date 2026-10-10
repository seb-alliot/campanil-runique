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
                    .table(Alias::new("entree_allergenes"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("entree_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("allergene_id")).integer().not_null())
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("entree_allergenes_entree_id_entrees_fkey")
                    .from(Alias::new("entree_allergenes"), Alias::new("entree_id"))
                    .to(Alias::new("entrees"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("entree_allergenes_allergene_id_allergenes_fkey")
                    .from(Alias::new("entree_allergenes"), Alias::new("allergene_id"))
                    .to(Alias::new("allergenes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_entree_allergenes_entree_id")
                    .table(Alias::new("entree_allergenes"))
                    .col(Alias::new("entree_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_entree_allergenes_allergene_id")
                    .table(Alias::new("entree_allergenes"))
                    .col(Alias::new("allergene_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("entree_allergenes"))
                    .name("entree_allergenes_entree_id_entrees_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("entree_allergenes"))
                    .name("entree_allergenes_allergene_id_allergenes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_entree_allergenes_entree_id").table(Alias::new("entree_allergenes")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_entree_allergenes_allergene_id").table(Alias::new("entree_allergenes")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("entree_allergenes"))
                .to_owned())
            .await?;
        Ok(())
}
}
