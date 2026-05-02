use runique::prelude::*;

model! {
    Regime,
    table: "regimes",
    pk: id => Pk,
    {
        libelle: text [required, unique, max_length: 100],
    }
}
