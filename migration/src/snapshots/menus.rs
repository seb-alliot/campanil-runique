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
                    .col(ColumnDef::new_with_type(Alias::new("type_menu"), ColumnType::Enum { name: Alias::new("TypeMenu").into_iden(), variants: vec![Alias::new("menu_resto").into_iden(), Alias::new("menu_enfant").into_iden(), Alias::new("formule_jour").into_iden()] }).not_null())
                    .col(ColumnDef::new(Alias::new("nom")).string().not_null())
                    .col(ColumnDef::new(Alias::new("description")).text().null())
                    .col(ColumnDef::new(Alias::new("image")).string().null())
                    .col(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .col(ColumnDef::new(Alias::new("ordre")).integer().null())
                    .col(ColumnDef::new(Alias::new("entree_libre")).string().null())
                    .col(ColumnDef::new(Alias::new("plat_libre")).string().null())
                    .col(ColumnDef::new(Alias::new("dessert_libre")).string().null())
                    .to_owned()
            )
            .await?;

        Ok(())
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop()
                .table(Alias::new("menus"))
                .to_owned())
            .await?;
        Ok(())
}
}
