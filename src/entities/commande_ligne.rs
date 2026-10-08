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
        commande_id:   int     [required],
        type_article:  choice  [enum(TypeArticle), required],
        plat_id:       int     [nullable],
        entree_id:     int     [nullable],
        dessert_id:    int     [nullable],
        menu_id:       int     [nullable],
        boisson_id:    int     [nullable],
        supplement_id: int     [nullable],
        cuisson:       choice  [enum(CuissonViande), nullable],
        sans_sel:      bool    [required, default: false],
        note:          text    [max_length: 500, nullable],
        quantite:      int     [required, default: 1, min: 1],
        prix_unitaire: decimal [required],
    },
    relations: {
        belongs_to: Commande via commande_id [cascade],
        belongs_to: Plat via plat_id [restrict],
        belongs_to: Boisson via boisson_id [restrict],
        many_to_many: Garniture through CommandeLigneGarniture via commande_ligne_id,
        belongs_to: Entree via entree_id [restrict],
        belongs_to: Dessert via dessert_id [restrict],
        belongs_to: Menu via menu_id [restrict],
        belongs_to: Supplement via supplement_id [restrict],
    },
    meta: {
        ordering: [id],
        verbose_name: "Ligne de commande",
        verbose_name_plural: "Lignes de commande",
    }
}
