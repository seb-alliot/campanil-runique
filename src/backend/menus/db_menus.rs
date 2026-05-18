use crate::backend::menus::{MenuCard, MenuFilters};
use crate::entities::menu;
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
    if let Some(ref theme) = filters.theme {
        query = query.filter(menu::Column::Theme.eq(theme.as_str()));
    }
    if let Some(ref regime) = filters.regime {
        query = query.filter(menu::Column::Regime.eq(regime.as_str()));
    }
    if let Some(np) = filters.nb_personnes {
        query = query.filter(menu::Column::NbPersonnesMin.lte(np));
    }

    query
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|m| MenuCard {
            id: m.id,
            titre: m.titre,
            description: m.description,
            prix_par_personne: format!("{:.2}", m.prix_par_personne),
            nb_personnes_min: m.nb_personnes_min,
            theme: m.theme.to_value(),
            regime: m.regime.to_value(),
            image: None,
            stock: m.stock,
        })
        .collect()
}
