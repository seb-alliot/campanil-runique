use runique::prelude::*;

model! {
    PlatSupplement,
    table: "plat_supplements",
    pk: id => Pk,
    {
        plat_id:       int [required],
        supplement_id: int [required],
    },
    relations: {
        belongs_to: Plat via plat_id [cascade],
        belongs_to: Supplement via supplement_id [cascade],
    },
}
