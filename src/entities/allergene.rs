use runique::prelude::*;

model! {
    Allergene,
    table: "allergenes",
    pk: id => Pk,
    {
        libelle: text [required, unique, max_length: 100],
    }
}
