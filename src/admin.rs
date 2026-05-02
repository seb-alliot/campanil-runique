admin! {
    allergenes: entities::allergene::Model => entities::allergene::AdminForm {
        title: "Allergènes",
        list_display: [["libelle", "Libellé"]],
    }
    themes: entities::theme::Model => entities::theme::AdminForm {
        title: "Thèmes",
        list_display: [["libelle", "Libellé"]],
    }
    regimes: entities::regime::Model => entities::regime::AdminForm {
        title: "Régimes alimentaires",
        list_display: [["libelle", "Libellé"]],
    }
    horaires: entities::horaire::Model => entities::horaire::AdminForm {
        title: "Horaires",
        list_display: [
            ["jour", "Jour"],
            ["heure_ouverture", "Ouverture"],
            ["heure_fermeture", "Fermeture"],
            ["ferme", "Fermé"],
        ],
        list_filter: [["ferme", "Fermé"]],
    }
    contacts: entities::contact::Model => entities::contact::AdminForm {
        title: "Messages de contact",
        list_display: [
            ["titre", "Sujet"],
            ["email", "Email"],
            ["created_at", "Reçu le"],
        ],
    }
    plats: entities::plat::Model => entities::plat::AdminForm {
        title: "Plats",
        list_display: [
            ["titre", "Titre"],
            ["type_plat", "Type"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
            ["est_viande", "Viande"],
        ],
        list_filter: [
            ["type_plat", "Type de plat"],
            ["disponible", "Disponibilité"],
            ["est_viande", "Viande"],
        ],
        group_action: [["disponible", "Rendre disponible"]],
    }
    menus: entities::menu::Model => entities::menu::AdminForm {
        title: "Menus traiteur",
        list_display: [
            ["titre", "Titre"],
            ["prix_par_personne", "Prix/pers."],
            ["nb_personnes_min", "Min. pers."],
            ["actif", "Actif"],
        ],
        list_filter: [
            ["actif", "Actif"],
            ["theme_id", "Thème"],
            ["regime_id", "Régime"],
        ],
        group_action: [["actif", "Activer"]],
    }
    commandes: entities::commande::Model => entities::commande::AdminForm {
        title: "Commandes",
        list_display: [
            ["numero", "N°"],
            ["type_commande", "Type"],
            ["statut", "Statut"],
            ["mode_paiement", "Paiement"],
            ["prix_total", "Total"],
            ["created_at", "Date"],
        ],
        list_filter: [
            ["type_commande", "Type"],
            ["statut", "Statut"],
            ["mode_paiement", "Paiement"],
            ["avec_livraison", "Livraison"],
        ],
    }
    avis: entities::avis::Model => entities::avis::AdminForm {
        title: "Avis clients",
        list_display: [
            ["commande_id", "Commande"],
            ["note", "Note"],
            ["statut", "Statut"],
            ["created_at", "Date"],
        ],
        list_filter: [["statut", "Statut"]],
        group_action: [["statut", "Valider"]],
    }

    configure {
        users: {
            list_display: [
                ["username", "Nom d'utilisateur"],
                ["email", "Email"],
                ["is_active", "Actif"],
                ["is_staff", "Staff"],
                ["created_at", "Inscrit le"],
            ],
            list_filter: [["is_active", "Actif"], ["is_staff", "Staff"]],
            group_action: [["is_active", "Activer"]],
        }
    }
}
