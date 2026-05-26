use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("menus_traiteur"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("titre")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().not_null())
                    .col(ColumnDef::new(Alias::new("prix_par_personne")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("nb_personnes_min")).integer().not_null())
                    .col(ColumnDef::new_with_type(Alias::new("theme"), ColumnType::Enum { name: Alias::new("ThemeMenu").into_iden(), variants: vec![Alias::new("mariage").into_iden(), Alias::new("bapteme").into_iden(), Alias::new("anniversaire").into_iden(), Alias::new("autre").into_iden()] }).not_null())
                    .col(ColumnDef::new_with_type(Alias::new("regime"), ColumnType::Enum { name: Alias::new("RegimeMenu").into_iden(), variants: vec![Alias::new("standard").into_iden(), Alias::new("vegetarien").into_iden(), Alias::new("sans_gluten").into_iden(), Alias::new("halal").into_iden(), Alias::new("casher").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("conditions")).text().null())
                    .col(ColumnDef::new(Alias::new("stock")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("actif")).boolean().not_null())
                    .col(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("menus_traiteur"))
                .to_owned())
            .await?;
        Ok(())
}
}
