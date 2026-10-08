use runique::prelude::*;

extend! {
    table: "eihwaz_users",
    fields: {
        telephone:   phone [max_length: 20, nullable],
        adresse:     text [max_length: 255, nullable],
        ville:       text [max_length: 100, nullable],
        code_postal: text [max_length: 10, nullable],
        pays:        text [max_length: 100, default: "France"],
    }
}
