use crate::backend::menus::{MenuCard, MenuFilters};
use crate::entities::{menu, menu_image, regime, theme};
use runique::prelude::*;
use std::str::FromStr;

pub async fn get_menu_cards(db: &DatabaseConnection, filters: &MenuFilters) -> Vec<MenuCard> {
    let mut query = search!(menu::Entity => Actif eq true, desc Id,);

    if let Some(ref pmin) = filters.prix_min
        && let Ok(d) = Decimal::from_str(pmin)
    {
        query = query.filter(menu::Column::PrixParPersonne.gte(d));
    }
    if let Some(ref pmax) = filters.prix_max
        && let Ok(d) = Decimal::from_str(pmax)
    {
        query = query.filter(menu::Column::PrixParPersonne.lte(d));
    }
    if let Some(tid) = filters.theme_id {
        query = query.filter(menu::Column::ThemeId.eq(tid));
    }
    if let Some(rid) = filters.regime_id {
        query = query.filter(menu::Column::RegimeId.eq(rid));
    }
    if let Some(np) = filters.nb_personnes {
        query = query.filter(menu::Column::NbPersonnesMin.lte(np));
    }

    let menus_themes: Vec<(menu::Model, Option<theme::Model>)> = query
        .also_related(theme::Entity)
        .all(db)
        .await
        .unwrap_or_default();

    if menus_themes.is_empty() {
        return Vec::new();
    }

    let menu_ids: Vec<i32> = menus_themes.iter().map(|(m, _)| m.id).collect();

    let mut first_images: HashMap<i32, String> = HashMap::new();
    for img in search!(menu_image::Entity => MenuId in (menu_ids), asc MenuId, asc Ordre,)
        .all(db)
        .await
        .unwrap_or_default()
    {
        first_images.entry(img.menu_id).or_insert(img.image);
    }

    let regimes: HashMap<i32, String> = search!(regime::Entity)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| (r.id, r.libelle))
        .collect();

    menus_themes
        .into_iter()
        .map(|(m, theme)| MenuCard {
            id: m.id,
            titre: m.titre,
            description: m.description,
            prix_par_personne: format!("{:.2}", m.prix_par_personne),
            nb_personnes_min: m.nb_personnes_min,
            theme_id: m.theme_id,
            theme: theme.map(|t| t.libelle).unwrap_or_default(),
            regime_id: m.regime_id,
            regime: regimes.get(&m.regime_id).cloned().unwrap_or_default(),
            image: first_images.get(&m.id).cloned(),
            stock: m.stock,
        })
        .collect()
}
