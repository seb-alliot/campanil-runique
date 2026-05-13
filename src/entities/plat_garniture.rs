use runique::prelude::*;

model! {
    PlatGarniture,
    table: "plat_garnitures",
    pk: id => Pk,
    {
        plat_id:      int  [required, fk(plats.id, cascade)],
        garniture_id: int  [required, fk(garnitures.id, cascade)],
        est_defaut:   bool [required, default: false],
    },
    meta: {
        verbose_name: "Garniture de plat",
        verbose_name_plural: "Garnitures de plats",
    }
}
