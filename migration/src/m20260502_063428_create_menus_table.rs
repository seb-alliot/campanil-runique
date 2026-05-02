use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menus"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().not_null())
                    .col(ColumnDef::new(Alias::new("prix_par_personne")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("nb_personnes_min")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("theme_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("regime_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("conditions")).text().null())
                    .col(ColumnDef::new(Alias::new("stock")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("actif")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menus_theme_id_themes_fkey")
                    .from(Alias::new("menus"), Alias::new("theme_id"))
                    .to(Alias::new("themes"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("menus_regime_id_regimes_fkey")
                    .from(Alias::new("menus"), Alias::new("regime_id"))
                    .to(Alias::new("regimes"), Alias::new("id"))
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
                    .table(Alias::new("menus"))
                    .name("menus_theme_id_themes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Alias::new("menus"))
                    .name("menus_regime_id_regimes_fkey")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop()
                .table(Alias::new("menus"))
                .to_owned())
            .await?;
        Ok(())
}
}
