use runique::prelude::*;

extend! {
    table: "eihwaz_users",
    fields: {
        telephone:   phone [max_length: 20],
        adresse:     text [max_length: 255],
        ville:       text [max_length: 100],
        code_postal: text [max_length: 10],
        pays:        text [max_length: 100, default: "France"],
    }
}
