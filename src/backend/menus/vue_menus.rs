use crate::backend::{
    menus::{MenuFilters, get_menu_cards, get_regimes_static, get_themes_static},
    stats::get_menu_filters,
    utils::inject_auth,
};
use runique::prelude::*;

pub async fn vue_menus(request: &mut Request) -> AppResult<Response> {
    inject_auth(request).await;
    let filters: MenuFilters = request.query();

    let _ = get_menu_filters(request, &filters).await;

    let cards = get_menu_cards(&request.engine.db, &filters).await;
    let themes = get_themes_static();
    let regimes = get_regimes_static();
    context_update!(request => {
        "title"   => "Menus traiteur — U Campanile",
        "cards"   => &cards,
        "themes"  => &themes,
        "regimes" => &regimes,
        "filters" => &filters,
    });
    request.render("menus/list.html")
}
