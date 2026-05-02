use runique::prelude::*;

model! {
    PlatAllergene,
    table: "plat_allergene",
    pk: id => Pk,
    {
        plat_id:      int [required, fk(plats.id, cascade)],
        allergene_id: int [required, fk(allergenes.id, cascade)],
    },
    relations: {
        belongs_to: Plat via plat_id,
        belongs_to: Allergene via allergene_id,
    },
    meta: {
        unique_together: [(plat_id, allergene_id)],
    }
}
