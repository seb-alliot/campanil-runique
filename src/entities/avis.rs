use runique::prelude::*;

model! {
    Avis,
    table: "avis",
    pk: id => Pk,
    enums: {
        StatutAvis: [
            EnAttente = ("en_attente", "En attente"),
            Valide    = ("valide",     "Validé"),
            Refuse    = ("refuse",     "Refusé"),
        ],
    },
    {
        commande_id: int      [required, unique],
        user_id:     int      [required],
        note:        int      [required, min: 1, max: 5],
        commentaire: textarea [required],
        statut:      choice   [enum(StatutAvis), required, default: "en_attente"],
        created_at:  datetime [auto_now],
    },
    relations: {
        belongs_to: Commande via commande_id [cascade],
        belongs_to: eihwaz_users via user_id [restrict],
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Avis",
        verbose_name_plural: "Avis",
    }
}
