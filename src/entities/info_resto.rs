use runique::prelude::*;

model! {
    InfoResto,
    table: "info_resto",
    pk: id => Pk,
    {
        nom:               text [required, max_length: 150],
        adresse:           text [required, max_length: 200],
        telephone:         text [required, max_length: 20],
        email:             email [max_length: 150, nullable],
        periode_ouverture: text [max_length: 100, nullable],
        facebook:          url [max_length: 255, nullable],
        instagram:         url [max_length: 255, nullable],
        tripadvisor:       url [max_length: 255, nullable],
        google_maps:       url [max_length: 500, nullable],
        description:       text [nullable],
        ville:             text [max_length: 100, nullable],
        prix_km_livraison:      decimal [max_digits: 10, decimal_places: 2, default: 0.59, renamed_from: "prix_livraison"],
        prix_livraison_minimal: decimal [max_digits: 10, decimal_places: 2, default: 5.00],
        penalite_materiel:      decimal [max_digits: 10, decimal_places: 2, default: 600.00],
        latitude:          decimal [max_digits: 9, decimal_places: 7 , nullable],
        longitude:         decimal [max_digits: 9, decimal_places: 7, nullable],
    }
}
