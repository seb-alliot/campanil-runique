use runique::prelude::*;

model! {
    Theme,
    table: "themes",
    pk: id => Pk,
    {
        libelle: text [required, unique, max_length: 100],
    }
}
