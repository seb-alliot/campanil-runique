use runique::prelude::*;

model! {
    CommandeLigneGarniture,
    table: "commande_ligne_garnitures",
    pk: id => Pk,
    {
        ligne_id:          int [required, renamed_from: "commande_ligne_id"],
        garniture_id:      int [required],
    },
    relations: {
        belongs_to: CommandeLigne via ligne_id [cascade],
        belongs_to: Garniture via garniture_id [restrict],
    },
    meta: {
        verbose_name: "Garniture de ligne",
        verbose_name_plural: "Garnitures de ligne",
    }
}
