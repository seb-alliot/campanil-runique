use runique::prelude::*;

model! {
    AvisPlat,
    table: "avis_plats",
    pk: id => Pk,
    enums: {
        StatutAvisPlat: [
            EnAttente = ("en_attente", "En attente"),
            Valide    = ("valide",     "Validé"),
            Refuse    = ("refuse",     "Refusé"),
        ],
    },
    {
        plat_id:    int [nullable],
        entree_id:  int [nullable],
        dessert_id: int [nullable],
        user_id:    int [nullable],
        note:        int      [required, min: 1, max: 5],
        commentaire: textarea [required],
        statut:      choice   [enum(StatutAvisPlat), required, default: "en_attente"],
        created_at:  datetime [auto_now],
    },
    relations: {
        belongs_to: Plat via plat_id [cascade],
        belongs_to: Entree via entree_id [cascade],
        belongs_to: Dessert via dessert_id [cascade],
        belongs_to: eihwaz_users via user_id [set_null],
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Avis plat",
        verbose_name_plural: "Avis plats",
    }
}
