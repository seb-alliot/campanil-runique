use runique::prelude::*;

model! {
    DevisTraiteur,
    table: "devis_traiteur",
    pk: id => Pk,
    enums: {
        StatutDevis: [
            EnAttente = ("en_attente", "En attente"),
            EnCours   = ("en_cours",   "En cours"),
            Accepte   = ("accepte",    "Accepté"),
            Refuse    = ("refuse",     "Refusé"),
        ],
    },
    {
        menu_id:        int      [nullable],
        user_id:        int      [required],
        nom:            text     [required, max_length: 150],
        email:          text     [required, max_length: 255],
        telephone:      text     [max_length: 30, nullable],
        date_evenement: date     [required],
        nb_personnes:   int      [required],
        message:        textarea [required],
        prix_total:       decimal [nullable],
        remise_appliquee: decimal [nullable],
        statut:         choice   [enum(StatutDevis), required, default: "en_attente"],
        created_at:     datetime [auto_now],
    },
    relations: {
        belongs_to: MenuTraiteur via menu_id [set_null],
        belongs_to: eihwaz_users via user_id [restrict],
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Demande de devis",
        verbose_name_plural: "Demandes de devis",
    }
}
