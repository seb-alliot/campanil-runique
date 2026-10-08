use runique::prelude::*;

model! {
    PlatAllergene,
    table: "plat_allergenes",
    pk: id => Pk,
    {
        plat_id:      int [required],
        allergene_id: int [required],
    },
    relations: {
        belongs_to: Plat via plat_id [cascade],
        belongs_to: Allergene via allergene_id [cascade],
    },
}
