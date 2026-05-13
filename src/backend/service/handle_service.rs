use crate::backend::service::charger_commandes::{charger_commandes_jour, garde_acces, grouper_par_statut};
use runique::prelude::*;

pub async fn handle_service(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok(Redirect::to("/connexion").into_response());
    }
    let db = request.db();
    let tz_str = &request.engine.config.timezone;
    let commandes = charger_commandes_jour(db, tz_str).await;

    let (retraits, livraisons): (Vec<_>, Vec<_>) =
        commandes.into_iter().partition(|c| !c.avec_livraison);

    let (retraits_attente, retraits_accepte, retraits_preparation, retraits_pret) =
        grouper_par_statut(retraits);
    let (livraisons_attente, livraisons_accepte, livraisons_preparation, livraisons_pret) =
        grouper_par_statut(livraisons);

    let nb_retraits = retraits_attente.len()
        + retraits_accepte.len()
        + retraits_preparation.len()
        + retraits_pret.len();
    let nb_livraisons = livraisons_attente.len()
        + livraisons_accepte.len()
        + livraisons_preparation.len()
        + livraisons_pret.len();

    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let today = chrono::Utc::now().with_timezone(&tz).format("%A %d %B %Y").to_string();

    context_update!(request => {
        "title"                  => "Service du jour",
        "retraits_attente"       => retraits_attente,
        "retraits_accepte"       => retraits_accepte,
        "retraits_preparation"   => retraits_preparation,
        "retraits_pret"          => retraits_pret,
        "livraisons_attente"     => livraisons_attente,
        "livraisons_accepte"     => livraisons_accepte,
        "livraisons_preparation" => livraisons_preparation,
        "livraisons_pret"        => livraisons_pret,
        "nb_retraits"            => nb_retraits,
        "nb_livraisons"          => nb_livraisons,
        "today"                  => today,
    });
    request.render("service.html")
}
