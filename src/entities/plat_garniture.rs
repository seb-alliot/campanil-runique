use runique::prelude::*;

model! {
    PlatGarniture,
    table: "plat_garnitures",
    pk: id => Pk,
    {
        plat_id:      int  [required],
        garniture_id: int  [required],
        est_defaut:   bool [required, default: false],
    },
    relations: {
        belongs_to: Plat via plat_id [cascade],
        belongs_to: Garniture via garniture_id [cascade],
    },
    meta: {
        verbose_name: "Garniture de plat",
        verbose_name_plural: "Garnitures de plats",
    }
}
