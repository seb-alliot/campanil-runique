use crate::backend::menus::get_menu_detail;
use crate::backend::utils::inject_auth;
use runique::prelude::*;

pub async fn vue_menu_details(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let Some(id) = request.get_path_as::<i32>("id") else {
        return Ok((StatusCode::NOT_FOUND, "Menu introuvable").into_response());
    };
    let Some(menu) = get_menu_detail(&request.engine.db, id).await else {
        return Ok((StatusCode::NOT_FOUND, "Menu introuvable").into_response());
    };
    context_update!(request => {
        "title" => format!("{} — U Campanile", menu.titre),
        "menu"  => &menu,
    });
    request.render("menus/detail.html")
}
