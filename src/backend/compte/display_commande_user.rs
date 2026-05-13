use crate::backend::compte::{get_commandes_user, load_profil};
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn handle_compte(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to("/connexion").into_response());
    };
    let page: u64 = request
        .get_query("page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(1);
    let db = request.db();
    let ((commandes, page_courante, total_pages), profil) = tokio::join!(
        get_commandes_user(db, user.id, page),
        load_profil(db, user.id),
    );
    let tab = request.get_query("tab").unwrap_or("commandes").to_string();
    let page_precedente = if page_courante > 1 { Some(page_courante - 1) } else { None };
    let page_suivante = if page_courante < total_pages { Some(page_courante + 1) } else { None };
    context_update!(request => {
        "title"           => "Mon compte",
        "commandes"       => &commandes,
        "profil"          => &profil,
        "current_user"    => &user,
        "active_tab"      => &tab,
        "page_courante"   => page_courante,
        "total_pages"     => total_pages,
        "page_precedente" => &page_precedente,
        "page_suivante"   => &page_suivante,
    });
    request.render("compte/index.html")
}
