use runique::prelude::*;

model! {
    PlatSupplement,
    table: "plat_supplements",
    pk: id => Pk,
    {
        plat_id:       int [required, fk(plats.id, cascade)],
        supplement_id: int [required, fk(supplements.id, cascade)],
    },
}
