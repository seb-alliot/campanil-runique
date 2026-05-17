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
        plat_id:     int      [required, fk(plats.id, cascade)],
        user_id:     int      [fk(eihwaz_users.id, set_null)],
        note:        int      [required, min: 1, max: 5],
        commentaire: textarea [required],
        statut:      choice   [enum(StatutAvisPlat), required, default: "en_attente"],
        created_at:  datetime [auto_now],
    },
    relations: {
        belongs_to: Plat via plat_id,
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Avis plat",
        verbose_name_plural: "Avis plats",
    }
}
