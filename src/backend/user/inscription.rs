use crate::formulaire::RegisterForm;

pub async fn register_user(
    form: &RegisterForm,
    db: &sea_orm::DatabaseConnection,
) -> Result<runique::prelude::runique_users::Model, sea_orm::DbErr> {
    form.save(db).await
}
