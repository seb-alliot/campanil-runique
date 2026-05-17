use crate::backend::{menus::MenuFilters, stats::get_menu_filters};
use runique::prelude::*;

pub async fn vue_track_menu_filters(request: &mut Request) -> AppResult<Response> {
    let filters: MenuFilters = request.query();
    get_menu_filters(request, &filters).await.ok();
    Ok(StatusCode::NO_CONTENT.into_response())
}
