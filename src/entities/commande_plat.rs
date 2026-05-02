use runique::prelude::*;

model! {
    CommandePlat,
    table: "commande_plats",
    pk: id => Pk,
    enums: {
        CuissonViande: [
            Bleu      = ("bleu",      "Bleu"),
            Saignant  = ("saignant",  "Saignant"),
            APoint    = ("a_point",   "À point"),
            BienCuit  = ("bien_cuit", "Bien cuit"),
        ],
    },
    {
        commande_id:   int          [required, fk(commandes.id, cascade)],
        plat_id:       int          [required, fk(plats.id, restrict)],
        quantite:      int          [required, default: 1, min: 1],
        prix_unitaire: decimal [required],
        cuisson:       choice       [enum(CuissonViande)],
    },
    relations: {
        belongs_to: Commande via commande_id,
        belongs_to: Plat via plat_id,
    }
}
