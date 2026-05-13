use crate::backend::carte::{CarteBoisson, CarteBoissonGroupe};
use crate::entities::boisson::{self, TypeBoisson};
use runique::prelude::*;

pub fn slug_from_type(t: &TypeBoisson) -> &'static str {
    match t {
        TypeBoisson::VinRouge  => "vin-rouge",
        TypeBoisson::VinRose   => "vin-rose",
        TypeBoisson::VinBlanc  => "vin-blanc",
        TypeBoisson::Champagne => "champagne",
        TypeBoisson::Biere     => "biere",
        TypeBoisson::Aperitif  => "aperitif",
        TypeBoisson::Soft      => "soft",
        TypeBoisson::Eau       => "eau-bouteille",
        TypeBoisson::Digestif  => "digestif",
    }
}

pub fn slug_to_type(slug: &str) -> Option<TypeBoisson> {
    match slug {
        "vin-rouge"     => Some(TypeBoisson::VinRouge),
        "vin-rose"      => Some(TypeBoisson::VinRose),
        "vin-blanc"     => Some(TypeBoisson::VinBlanc),
        "champagne"     => Some(TypeBoisson::Champagne),
        "biere"         => Some(TypeBoisson::Biere),
        "aperitif"      => Some(TypeBoisson::Aperitif),
        "soft"          => Some(TypeBoisson::Soft),
        "eau-bouteille" => Some(TypeBoisson::Eau),
        "digestif"      => Some(TypeBoisson::Digestif),
        _               => None,
    }
}

pub async fn build_boissons(db: &sea_orm::DatabaseConnection) -> Vec<CarteBoissonGroupe> {
    let items = search!(boisson::Entity => asc TypeBoisson, asc Titre,)
        .all(db)
        .await
        .unwrap_or_default();

    let mut groupes: Vec<CarteBoissonGroupe> = Vec::new();
    for b in items {
        let slug = slug_from_type(&b.type_boisson).to_string();
        let type_label = b.type_boisson.to_string();
        let carte_boisson = CarteBoisson {
            id: b.id,
            titre: b.titre,
            description: b.description,
            prix: format!("{:.2}", b.prix),
            image: b.image,
            disponible: b.disponible,
        };
        if groupes.last().map(|g: &CarteBoissonGroupe| g.type_slug.as_str()) == Some(slug.as_str()) {
            groupes.last_mut().unwrap().items.push(carte_boisson);
        } else {
            groupes.push(CarteBoissonGroupe { type_label, type_slug: slug, items: vec![carte_boisson] });
        }
    }
    groupes
}

pub async fn get_boissons_par_type(
    db: &sea_orm::DatabaseConnection,
    type_val: TypeBoisson,
) -> Vec<CarteBoisson> {
    use sea_orm::QueryFilter;
    boisson::Entity::find()
        .filter(boisson::Column::TypeBoisson.eq(type_val))
        .filter(boisson::Column::Disponible.eq(true))
        .order_by_asc(boisson::Column::Titre)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|b| CarteBoisson {
            id: b.id,
            titre: b.titre,
            description: b.description,
            prix: format!("{:.2}", b.prix),
            image: b.image,
            disponible: b.disponible,
        })
        .collect()
}
