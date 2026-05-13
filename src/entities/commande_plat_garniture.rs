use runique::prelude::*;

model! {
    CommandePlatGarniture,
    table: "commande_plat_garnitures",
    pk: id => Pk,
    {
        commande_plat_id: int [required, fk(commande_plats.id, cascade)],
        garniture_id:     int [required, fk(garnitures.id, restrict)],
    },
    meta: {
        verbose_name: "Garniture de commande",
        verbose_name_plural: "Garnitures de commande",
    }
}
