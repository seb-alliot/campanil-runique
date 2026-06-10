use runique::prelude::*;

model! {
    Commande,
    table: "commandes",
    pk: id => Pk,
    enums: {
        TypeRetrait: [
            SurPlace  = ("sur_place",  "Sur place"),
            Livraison = ("livraison",  "Livraison"),
        ],
        StatutCommande: [
            EnAttente        = ("en_attente",        "En attente"),
            Accepte          = ("accepte",            "Accepté"),
            EnPreparation    = ("en_preparation",     "En préparation"),
            Pret             = ("pret",               "Prêt"),
            EnCoursLivraison = ("en_cours_livraison", "En cours de livraison"),
            Livre            = ("livre",              "Livré"),
            Termine          = ("termine",            "Terminé"),
            Annule           = ("annule",             "Annulé"),
        ],
        ModePaiement: [
            Especes       = ("especes",       "Espèces"),
            CarteBancaire = ("carte_bancaire", "Carte bancaire"),
            EnLigne       = ("en_ligne",       "Paiement en ligne"),
        ],
    },
    {
        numero:        text   [required, unique, max_length: 20],
        user_id:       int    [required, fk(eihwaz_users.id, restrict)],
        statut:        choice [enum(StatutCommande), required, default: "en_attente"],
        mode_paiement: choice [enum(ModePaiement), required],
        prix_total:    decimal [required],

        type_retrait:      choice [enum(TypeRetrait), required],
        heure_retrait:     datetime,
        adresse_livraison: text   [max_length: 255],
        ville_livraison:   text   [max_length: 100],
        cp_livraison:      text   [max_length: 10],
        prix_livraison:    decimal [default: 0],

        modifiable:               bool     [required, default: true, skip],
        pret_materiel:            bool     [required, default: false, skip],
        penalite_envoyee:         bool     [required, default: false, skip],
        stripe_payment_intent_id: text     [max_length: 255, skip],
        motif_annulation:         textarea [skip],
        mode_contact_annulation:  text     [max_length: 100, skip],
        date_annulation:          datetime [skip],

        created_at: datetime [auto_now],
        updated_at: datetime [auto_now_update],
    },
    relations: {
        has_many: CommandeLigne,
        has_many: CommandeStatut,
        has_one: Avis,
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Commande",
        verbose_name_plural: "Commandes",
    }
}
