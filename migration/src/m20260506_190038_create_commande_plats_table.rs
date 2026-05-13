use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared(
            "DO $$ BEGIN CREATE TYPE CuissonViande AS ENUM ('bleu', 'saignant', 'a_point', 'bien_cuit'); EXCEPTION WHEN duplicate_object THEN NULL; END $$"
        ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("commande_plats"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Alias::new("commande_id"))
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alias::new("plat_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("boisson_id")).integer().null())
                    .col(ColumnDef::new(Alias::new("quantite")).integer().not_null())
                    .col(
                        ColumnDef::new(Alias::new("prix_unitaire"))
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type(
                            Alias::new("cuisson"),
                            ColumnType::Enum {
                                name: Alias::new("CuissonViande").into_iden(),
                                variants: vec![
                                    Alias::new("bleu").into_iden(),
                                    Alias::new("saignant").into_iden(),
                                    Alias::new("a_point").into_iden(),
                                    Alias::new("bien_cuit").into_iden(),
                                ],
                            },
                        )
                        .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("commande_plats")).to_owned())
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE IF EXISTS CuissonViande")
            .await?;

        Ok(())
    }
}
