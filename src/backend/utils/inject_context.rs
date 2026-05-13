use crate::backend::panier::panier_get;
use crate::entities::{horaire, info_resto};
use runique::prelude::*;

pub async fn inject_auth(request: &mut Request) {
    let is_admin = is_admin_authenticated(&request.session).await;
    let current_user = request.user.clone();
    let panier_nb = panier_get(&request.session).await.nb_articles();
    let db = request.db();

    let horaires = search!(horaire::Entity => asc Jour,)
        .all(db)
        .await
        .unwrap_or_default();

    let info_resto = search!(info_resto::Entity)
        .first(db)
        .await
        .unwrap_or_default();

    context_update!(request => {
        "user"       => current_user,
        "is_admin"   => is_admin,
        "panier_nb"  => panier_nb,
        "horaires"   => horaires,
        "info_resto" => info_resto,
    });
}
