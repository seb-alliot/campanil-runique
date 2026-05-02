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
        commande_id: int      [required, fk(commandes.id, cascade), unique],
        note:        int      [required, min: 1, max: 5],
        commentaire: textarea [required],
        statut:      choice   [enum(StatutAvis), required, default: "en_attente"],
        created_at:  datetime [auto_now],
    },
    relations: {
        belongs_to: Commande via commande_id,
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Avis",
        verbose_name_plural: "Avis",
    }
}
