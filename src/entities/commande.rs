use runique::prelude::*;

model! {
    Commande,
    table: "commandes",
    pk: id => Pk,
    enums: {
        TypeCommande: [
            Carte    = ("carte",    "Commande carte"),
            Traiteur = ("traiteur", "Commande traiteur"),
        ],
        StatutCommande: [
            EnAttente               = ("en_attente",                "En attente"),
            Accepte                 = ("accepte",                   "Accepté"),
            EnPreparation           = ("en_preparation",            "En préparation"),
            Pret                    = ("pret",                      "Prêt"),
            EnCoursLivraison        = ("en_cours_livraison",        "En cours de livraison"),
            Livre                   = ("livre",                     "Livré"),
            EnAttenteRetourMateriel = ("attente_retour_materiel",   "En attente retour matériel"),
            Termine                 = ("termine",                   "Terminé"),
            Annule                  = ("annule",                    "Annulé"),
        ],
        ModePaiement: [
            Especes       = ("especes",        "Espèces"),
            CarteBancaire = ("carte_bancaire",  "Carte bancaire"),
            EnLigne       = ("en_ligne",        "Paiement en ligne"),
        ],
    },
    {
        numero:          text   [required, unique, max_length: 20],
        user_id:         int    [required, fk(eihwaz_users.id, restrict)],
        type_commande:   choice [enum(TypeCommande), required],
        statut:          choice [enum(StatutCommande), required, default: "en_attente"],
        mode_paiement:   choice [enum(ModePaiement), required],
        prix_total:      decimal [required],

        // Traiteur uniquement (nullable)
        menu_id:         int    [fk(menus.id, restrict)],
        nb_personnes:    int,
        date_prestation: datetime,
        avec_materiel:   bool   [default: false],

        // Retrait / Livraison
        heure_retrait:     datetime,
        avec_livraison:   bool  [required, default: false],
        adresse_livraison: text [max_length: 255],
        ville_livraison:   text [max_length: 100],
        cp_livraison:      text [max_length: 10],
        heure_livraison:   datetime,
        prix_livraison:    decimal [default: 0],

        // Stripe
        stripe_payment_intent_id: text [max_length: 255, skip],

        // Annulation employé
        motif_annulation:        textarea [skip],
        mode_contact_annulation: text     [max_length: 100, skip],

        // Annulation client
        date_annulation: datetime [skip],

        created_at: datetime [auto_now],
        updated_at: datetime [auto_now_update],
    },
    relations: {
        belongs_to: Menu via menu_id,
        has_many: CommandePlat,
        has_many: CommandeStatut,
        has_one: Avis,
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Commande",
        verbose_name_plural: "Commandes",
    }
}
