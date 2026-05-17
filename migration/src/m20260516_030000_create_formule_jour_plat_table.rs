use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("formule_jour_plat"))
                    .if_not_exists()
                    .col(ColumnDef::new(Alias::new("id")).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Alias::new("formule_jour_id")).integer().not_null())
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().not_null())
                    .foreign_key(ForeignKey::create().from(Alias::new("formule_jour_plat"), Alias::new("formule_jour_id")).to(Alias::new("formule_jours"), Alias::new("id")).on_delete(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().from(Alias::new("formule_jour_plat"), Alias::new("plat_id")).to(Alias::new("plats"), Alias::new("id")).on_delete(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("formule_jour_plat")).to_owned())
            .await
    }
}
