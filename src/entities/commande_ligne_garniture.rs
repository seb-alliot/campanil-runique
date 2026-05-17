use runique::prelude::*;

model! {
    CommandeLigneGarniture,
    table: "commande_ligne_garnitures",
    pk: id => Pk,
    {
        commande_ligne_id: int [required, fk(commande_lignes.id, cascade)],
        garniture_id:      int [required, fk(garnitures.id, restrict)],
    },
    meta: {
        verbose_name: "Garniture de ligne",
        verbose_name_plural: "Garnitures de ligne",
    }
}
