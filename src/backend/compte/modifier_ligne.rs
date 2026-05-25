use crate::backend::utils::inject_auth;
use crate::entities::{commande, commande_ligne};
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, Set};

pub async fn handle_modifier_ligne(mut request: Request) -> AppResult<Response> {
    inject_auth(&mut request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };
    if !request.prisme.csrf_valid {
        return Ok(Redirect::to("/compte").into_response());
    }

    let numero = request.get_path("numero").unwrap_or("").to_string();
    let ligne_id: Pk = request
        .get_path("ligne_id")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let db = request.db();

    let Some(cmd) = search!(commande::Entity => Numero eq &numero, UserId eq user.id,)
        .first(db)
        .await
        .unwrap_or(None)
    else {
        return Ok(Redirect::to("/compte").into_response());
    };

    if !cmd.modifiable {
        return Ok(Redirect::to("/compte").into_response());
    }

    let Some(ligne) = search!(commande_ligne::Entity => Id eq ligne_id, CommandeId eq cmd.id,)
        .first(db)
        .await
        .unwrap_or(None)
    else {
        return Ok(Redirect::to("/compte").into_response());
    };

    let note = request
        .prisme
        .data
        .get("note")
        .cloned()
        .filter(|s| !s.is_empty());
    let sans_sel = request
        .prisme
        .data
        .get("sans_sel")
        .map(|v| v == "on" || v == "1")
        .unwrap_or(false);
    let cuisson = request
        .prisme
        .data
        .get("cuisson")
        .cloned()
        .filter(|s| !s.is_empty())
        .and_then(|s| match s.as_str() {
            "bleu" => Some(commande_ligne::CuissonViande::Bleu),
            "saignant" => Some(commande_ligne::CuissonViande::Saignant),
            "a_point" => Some(commande_ligne::CuissonViande::APoint),
            "bien_cuit" => Some(commande_ligne::CuissonViande::BienCuit),
            _ => ligne.cuisson.clone(),
        });

    commande_ligne::ActiveModel {
        id: Set(ligne.id),
        note: Set(note),
        sans_sel: Set(sans_sel),
        cuisson: Set(cuisson),
        ..Default::default()
    }
    .update(db)
    .await
    .ok();

    Ok(Redirect::to("/compte").into_response())
}
