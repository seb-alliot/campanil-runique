use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn handle_devis_confirmation(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    context_update!(request => {
        "title" => "Demande envoyée — U Campanile",
    });
    request.render("traiteur/confirmation.html")
}
