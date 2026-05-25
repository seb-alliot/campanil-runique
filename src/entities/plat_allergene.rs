use runique::prelude::*;

model! {
    PlatAllergene,
    table: "plat_allergenes",
    pk: id => Pk,
    {
        plat_id:      int [required, fk(plats.id, cascade)],
        allergene_id: int [required, fk(allergenes.id, cascade)],
    }
}
