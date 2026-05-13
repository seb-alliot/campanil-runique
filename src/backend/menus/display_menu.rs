use crate::backend::{
    menus::{MenuFilters, get_menu_cards, get_regimes_list, get_themes_list},
    utils::inject_auth,
};
use runique::prelude::*;

pub async fn vue_menus(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let filters: MenuFilters = request.query();
    let cards = get_menu_cards(&request.engine.db, &filters).await;
    let themes = get_themes_list(&request.engine.db).await;
    let regimes = get_regimes_list(&request.engine.db).await;
    context_update!(request => {
        "title"   => "Menus traiteur — U Campanile",
        "cards"   => &cards,
        "themes"  => &themes,
        "regimes" => &regimes,
        "filters" => &filters,
    });
    request.render("menus/list.html")
}
