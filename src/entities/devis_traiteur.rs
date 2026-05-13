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
        menu_id:        int      [fk(menus.id, set_null)],
        nom:            text     [required, max_length: 150],
        email:          text     [required, max_length: 255],
        telephone:      text     [max_length: 30],
        date_evenement: text     [required, max_length: 10],
        nb_personnes:   int      [required],
        message:        textarea [required],
        statut:         choice   [enum(StatutDevis), required, default: "en_attente"],
        created_at:     datetime [auto_now],
    },
    relations: {
        belongs_to: Menu via menu_id,
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Demande de devis",
        verbose_name_plural: "Demandes de devis",
    }
}
