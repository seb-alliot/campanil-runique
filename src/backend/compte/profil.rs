use crate::backend::utils::inject_auth;
use crate::entities::user_profil;
use crate::formulaire::ProfilForm;
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

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

    user_profil::ActiveModel {
        id: Set(user.id),
        telephone: Set(Some(telephone)),
        adresse: Set(Some(adresse)),
        ville: Set(Some(ville)),
        code_postal: Set(Some(code_postal)),
        ..Default::default()
    }
    .update(request.db())
    .await
    .ok();

    request.notices.success("Informations mises à jour.").await;
    Ok(Redirect::to("/compte?tab=profil").into_response())
}
