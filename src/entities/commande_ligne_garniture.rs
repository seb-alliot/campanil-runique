use runique::prelude::*;

model! {
    CommandeLigneGarniture,
    table: "commande_ligne_garnitures",
    pk: id => Pk,
    {
        commande_ligne_id: int [required],
        garniture_id:      int [required],
    },
    relations: {
        belongs_to: CommandeLigne via commande_ligne_id [cascade],
        belongs_to: Garniture via garniture_id [restrict],
    },
    meta: {
        verbose_name: "Garniture de ligne",
        verbose_name_plural: "Garnitures de ligne",
    }
}
