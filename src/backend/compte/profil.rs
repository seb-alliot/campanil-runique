use crate::backend::utils::inject_auth;
use crate::entities::user_profil;
use crate::formulaire::ProfilForm;
use runique::prelude::*;
use sea_orm::{ConnectionTrait, DbBackend, Statement};

pub async fn load_profil(db: &DatabaseConnection, user_id: Pk) -> Option<user_profil::Model> {
    search!(user_profil::Entity => Id eq user_id,)
        .first(db)
        .await
        .unwrap_or(None)
}

pub async fn handle_profil_post(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };
    if !request.prisme.csrf_valid {
        return Ok(Redirect::to("/compte").into_response());
    }
    let mut form: ProfilForm = request.form();
    if !form.is_valid().await {
        return Ok(Redirect::to("/compte?tab=profil").into_response());
    }
    let telephone = form.cleaned_string("telephone").unwrap_or_default();
    let adresse = form.cleaned_string("adresse").unwrap_or_default();
    let ville = form.cleaned_string("ville").unwrap_or_default();
    let code_postal = form.cleaned_string("code_postal").unwrap_or_default();

    request
        .db()
        .execute_raw(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"UPDATE eihwaz_users
               SET telephone=$1, adresse=$2, ville=$3, code_postal=$4
               WHERE id=$5"#,
            [
                telephone.into(),
                adresse.into(),
                ville.into(),
                code_postal.into(),
                user.id.into(),
            ],
        ))
        .await
        .ok();

    request.notices.success("Informations mises à jour.").await;
    Ok(Redirect::to("/compte?tab=profil").into_response())
}
