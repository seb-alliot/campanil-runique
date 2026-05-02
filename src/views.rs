use crate::backend;
use crate::backend::utils::*;
use crate::formulaire::{LoginForm, RegisterForm};
use runique::prelude::*;

pub async fn index(mut request: Request) -> AppResult<Response> {
    inject_auth(&mut request).await;
    context_update!(request => {
        "title" => "U Campanile — Restaurant corse à Corte",
    });
    request.render("index.html")
}

pub async fn connexion(mut request: Request) -> AppResult<Response> {
    let mut form: LoginForm = request.form();
    backend::auth::handle_login(&mut request, &mut form).await
}

pub async fn deconnexion(request: Request) -> AppResult<Response> {
    logout(&request.session, None).await.ok();
    Ok(Redirect::to("/").into_response())
}

pub async fn inscription(mut request: Request) -> AppResult<Response> {
    let mut form: RegisterForm = request.form();
    backend::auth::handle_inscription(&mut request, &mut form).await
}

pub async fn activer(mut request: Request) -> AppResult<Response> {
    backend::auth::handle_activate(&mut request).await
}
