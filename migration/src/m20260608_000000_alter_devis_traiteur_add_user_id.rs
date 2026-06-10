use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("devis_traiteur"))
                    .add_column(
                        ColumnDef::new(Alias::new("user_id"))
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_devis_traiteur_user_id")
                    .from(Alias::new("devis_traiteur"), Alias::new("user_id"))
                    .to(Alias::new("eihwaz_users"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_devis_traiteur_user_id")
                    .table(Alias::new("devis_traiteur"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("devis_traiteur"))
                    .drop_column(Alias::new("user_id"))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
