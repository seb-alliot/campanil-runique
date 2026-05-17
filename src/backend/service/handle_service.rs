use crate::backend::service::charger_commandes::{
    charger_commandes_jour, garde_acces, grouper_par_service, grouper_par_statut,
};
use runique::prelude::*;

pub async fn handle_service(request: &mut Request) -> AppResult<Response> {
    if !garde_acces(request) {
        return Ok(Redirect::to("/connexion").into_response());
    }
    let db = request.db();
    let tz_str = &request.engine.config.timezone;
    let commandes = charger_commandes_jour(db, tz_str).await;

    let (midi, soir) = grouper_par_service(commandes);

    let (midi_retraits, midi_livraisons): (Vec<_>, Vec<_>) = midi
        .into_iter()
        .partition(|c| c.type_retrait == "sur_place");
    let (soir_retraits, soir_livraisons): (Vec<_>, Vec<_>) = soir
        .into_iter()
        .partition(|c| c.type_retrait == "sur_place");

    let (mr_at, mr_ac, mr_pr, mr_pt) = grouper_par_statut(midi_retraits);
    let (ml_at, ml_ac, ml_pr, ml_pt) = grouper_par_statut(midi_livraisons);
    let (sr_at, sr_ac, sr_pr, sr_pt) = grouper_par_statut(soir_retraits);
    let (sl_at, sl_ac, sl_pr, sl_pt) = grouper_par_statut(soir_livraisons);

    let nb_midi = mr_at.len()
        + mr_ac.len()
        + mr_pr.len()
        + mr_pt.len()
        + ml_at.len()
        + ml_ac.len()
        + ml_pr.len()
        + ml_pt.len();
    let nb_soir = sr_at.len()
        + sr_ac.len()
        + sr_pr.len()
        + sr_pt.len()
        + sl_at.len()
        + sl_ac.len()
        + sl_pr.len()
        + sl_pt.len();
    let midi_actif = nb_midi > 0;

    let tz: chrono_tz::Tz = tz_str.parse().unwrap_or(chrono_tz::UTC);
    let today = chrono::Utc::now()
        .with_timezone(&tz)
        .format("%A %d %B %Y")
        .to_string();

    context_update!(request => {
        "title"   => "Service du jour",
        "today"   => today,
        "nb_midi"     => nb_midi,
        "nb_soir"     => nb_soir,
        "midi_actif"  => midi_actif,
        "midi_retraits_attente"       => mr_at,
        "midi_retraits_accepte"       => mr_ac,
        "midi_retraits_preparation"   => mr_pr,
        "midi_retraits_pret"          => mr_pt,
        "midi_livraisons_attente"     => ml_at,
        "midi_livraisons_accepte"     => ml_ac,
        "midi_livraisons_preparation" => ml_pr,
        "midi_livraisons_pret"        => ml_pt,
        "soir_retraits_attente"       => sr_at,
        "soir_retraits_accepte"       => sr_ac,
        "soir_retraits_preparation"   => sr_pr,
        "soir_retraits_pret"          => sr_pt,
        "soir_livraisons_attente"     => sl_at,
        "soir_livraisons_accepte"     => sl_ac,
        "soir_livraisons_preparation" => sl_pr,
        "soir_livraisons_pret"        => sl_pt,
    });
    request.render("service.html")
}
