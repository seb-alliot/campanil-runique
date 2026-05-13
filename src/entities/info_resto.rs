use runique::prelude::*;

model! {
    InfoResto,
    table: "info_resto",
    pk: id => Pk,
    {
        nom:               text [required, max_length: 150],
        adresse:           text [required, max_length: 200],
        telephone:         text [required, max_length: 20],
        email:             email [max_length: 150],
        periode_ouverture: text [max_length: 100],
        facebook:          url [max_length: 255],
        instagram:         url [max_length: 255],
        tripadvisor:       url [max_length: 255],
        google_maps:       url [max_length: 500],
        description:       text,
        prix_livraison:    decimal [default: 5.00],
    }
}
