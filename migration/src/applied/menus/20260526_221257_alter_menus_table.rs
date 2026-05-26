use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("created_at"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("actif"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("stock"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("conditions"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("regime"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("theme"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("nb_personnes_min"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("prix_par_personne"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("titre"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .modify_column(ColumnDef::new(Alias::new("description")).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("prix")).decimal().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new_with_type(Alias::new("type_menu"), ColumnType::Enum { name: Alias::new("TypeMenu").into_iden(), variants: vec![Alias::new("menu_resto").into_iden(), Alias::new("menu_enfant").into_iden(), Alias::new("formule_jour").into_iden()] }).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("dessert_libre")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("image")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("entree_libre")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("ordre")).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("plat_libre")).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("nom")).string().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("prix"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("type_menu"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("dessert_libre"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("image"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("entree_libre"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("ordre"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("plat_libre"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .drop_column(Alias::new("nom"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .modify_column(ColumnDef::new(Alias::new("description")).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("created_at")).date_time().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("actif")).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("stock")).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("conditions")).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new_with_type(Alias::new("regime"), ColumnType::Enum { name: Alias::new("RegimeMenu").into_iden(), variants: vec![Alias::new("standard").into_iden(), Alias::new("vegetarien").into_iden(), Alias::new("sans_gluten").into_iden(), Alias::new("halal").into_iden(), Alias::new("casher").into_iden()] }).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new_with_type(Alias::new("theme"), ColumnType::Enum { name: Alias::new("ThemeMenu").into_iden(), variants: vec![Alias::new("mariage").into_iden(), Alias::new("bapteme").into_iden(), Alias::new("anniversaire").into_iden(), Alias::new("autre").into_iden()] }).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("nb_personnes_min")).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("prix_par_personne")).decimal().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("menus"))
                    .add_column(ColumnDef::new(Alias::new("titre")).string().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
