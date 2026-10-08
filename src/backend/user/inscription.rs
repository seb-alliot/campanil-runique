use crate::formulaire::RegisterForm;

pub async fn register_user(
    form: &RegisterForm,
    db: &runique::prelude::ADb,
) -> Result<runique::prelude::runique_users::Model, sea_orm::DbErr> {
    form.save(db).await
}
