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
                    .table(Alias::new("supplements"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("garniture_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("titre")).string_len(255).null())
                    .col(ColumnDef::new(Alias::new("libelle")).string_len(500).null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("disponible")).boolean().not_null().default(true))
                    .col(ColumnDef::new(Alias::new("ordre")).integer().not_null().default(0))
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("supplements_garniture_id_garnitures_fkey")
                    .from(Alias::new("supplements"), Alias::new("garniture_id"))
                    .to(Alias::new("garnitures"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_supplements_garniture_id")
                    .table(Alias::new("supplements"))
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
                    .table(Alias::new("supplements"))
                    .name("supplements_garniture_id_garnitures_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_supplements_garniture_id").table(Alias::new("supplements")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("supplements"))
                .to_owned())
            .await?;
        Ok(())
}
}
