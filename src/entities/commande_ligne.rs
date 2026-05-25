use runique::prelude::*;

model! {
    CommandeLigne,
    table: "commande_lignes",
    pk: id => Pk,
    enums: {
        TypeArticle: [
            Plat       = ("plat",        "Plat"),
            Entree     = ("entree",      "Entrée"),
            Dessert    = ("dessert",     "Dessert"),
            Menu       = ("menu",        "Menu"),
            Boisson    = ("boisson",     "Boisson"),
            Supplement = ("supplement",  "Supplément"),
        ],
        CuissonViande: [
            Bleu     = ("bleu",      "Bleu"),
            Saignant = ("saignant",  "Saignant"),
            APoint   = ("a_point",   "À point"),
            BienCuit = ("bien_cuit", "Bien cuit"),
        ],
    },
    {
        commande_id:   int     [required, fk(commandes.id, cascade)],
        type_article:  choice  [enum(TypeArticle), required],
        plat_id:       int     [fk(plats.id, restrict)],
        entree_id:     int     [fk(entrees.id, restrict)],
        dessert_id:    int     [fk(desserts.id, restrict)],
        menu_id:       int     [fk(menus.id, restrict)],
        boisson_id:    int     [fk(boissons.id, restrict)],
        supplement_id: int     [fk(supplements.id, restrict)],
        cuisson:       choice  [enum(CuissonViande)],
        sans_sel:      bool    [required, default: false],
        note:          text    [max_length: 500],
        quantite:      int     [required, default: 1, min: 1],
        prix_unitaire: decimal [required],
    },
    relations: {
        belongs_to: Commande via commande_id,
        belongs_to: Plat via plat_id,
        belongs_to: Boisson via boisson_id,
        many_to_many: Garniture through CommandeLigneGarniture via commande_ligne_id,
    },
    meta: {
        ordering: [id],
        verbose_name: "Ligne de commande",
        verbose_name_plural: "Lignes de commande",
    }
}
