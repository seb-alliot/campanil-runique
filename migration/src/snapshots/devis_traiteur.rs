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
                    .table(Alias::new("devis_traiteur"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("menu_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("user_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("nom")).string_len(150).not_null())
                    .col(ColumnDef::new(Alias::new("email")).string_len(255).not_null())
                    .col(ColumnDef::new(Alias::new("telephone")).string_len(30).null())
                    .col(ColumnDef::new(Alias::new("date_evenement")).date().not_null())
                    .col(ColumnDef::new(Alias::new("nb_personnes")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("message")).text().not_null())
                    .col(ColumnDef::new(Alias::new("prix_total")).decimal().null())
                    .col(ColumnDef::new(Alias::new("remise_appliquee")).decimal().null())
                    .col(ColumnDef::new_with_type(Alias::new("statut"), ColumnType::Enum { name: Alias::new("StatutDevis").into_iden(), variants: vec![Alias::new("en_attente").into_iden(), Alias::new("en_cours").into_iden(), Alias::new("accepte").into_iden(), Alias::new("refuse").into_iden()] }).not_null().default("en_attente"))
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("devis_traiteur_menu_id_menus_traiteur_fkey")
                    .from(Alias::new("devis_traiteur"), Alias::new("menu_id"))
                    .to(Alias::new("menus_traiteur"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("devis_traiteur_user_id_eihwaz_users_fkey")
                    .from(Alias::new("devis_traiteur"), Alias::new("user_id"))
                    .to(Alias::new("eihwaz_users"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devis_traiteur_menu_id")
                    .table(Alias::new("devis_traiteur"))
                    .col(Alias::new("menu_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_devis_traiteur_user_id")
                    .table(Alias::new("devis_traiteur"))
                    .col(Alias::new("user_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("devis_traiteur"))
                    .name("devis_traiteur_menu_id_menus_traiteur_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("devis_traiteur"))
                    .name("devis_traiteur_user_id_eihwaz_users_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_devis_traiteur_menu_id").table(Alias::new("devis_traiteur")).to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_devis_traiteur_user_id").table(Alias::new("devis_traiteur")).to_owned())
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("devis_traiteur"))
                .to_owned())
            .await?;
        Ok(())
}
}
