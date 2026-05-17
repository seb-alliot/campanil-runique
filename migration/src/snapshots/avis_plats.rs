use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("avis_plats"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("note")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("commentaire")).text().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), ColumnType::Enum { name: Alias::new("StatutAvisPlat").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("valide").into_iden(), Alias::new("refuse").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("avis_plats_plat_id_plats_fkey")
                    .from(Alias::new("avis_plats"), Alias::new("plat_id"))
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
                    .table(Alias::new("avis_plats"))
                    .name("avis_plats_plat_id_plats_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("avis_plats"))
                .to_owned())
            .await?;
        Ok(())
}
}
