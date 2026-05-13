use crate::backend::{carte::get_carte, utils::inject_auth};
use runique::prelude::*;

pub async fn vue_carte(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let carte = get_carte(&request.engine.db).await;
    context_update!(request => {
        "title" => "La Carte — U Campanile",
        "carte" => &carte,
    });
    request.render("carte.html")
}
