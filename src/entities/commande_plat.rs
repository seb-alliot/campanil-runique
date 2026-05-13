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
        commande_id:   int     [required, fk(commandes.id, cascade)],
        plat_id:       int     [fk(plats.id, restrict)],
        boisson_id:    int     [fk(boissons.id, restrict)],
        menu_id:       int     [fk(menu_resto.id, restrict)],
        supplement_id: int     [fk(supplements.id, restrict)],
        quantite:      int     [required, default: 1, min: 1],
        prix_unitaire: decimal [required],
        cuisson:       choice  [enum(CuissonViande)],
        avec_legumes:  bool    [default: false],
        sans_sel:      bool    [default: false],
        note:          text    [max_length: 500],
    },
    relations: {
        belongs_to: Commande via commande_id,
        belongs_to: Plat via plat_id,
        belongs_to: Boisson via boisson_id,
    }
}
