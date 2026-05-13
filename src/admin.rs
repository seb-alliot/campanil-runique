use crate::entities::*;
use crate::entities::menu_resto_plat;
use runique::admin;

admin! {
    configure {
        users: {
            list_display: [
                ["username", "Nom d'utilisateur"],
                ["email", "Email"],
                ["is_active", "Actif"],
                ["is_staff", "Staff"],
                ["created_at", "Inscrit le"],
            ],
            group_action: [["is_active", "Activer"]],
        }
    }

    allergenes: allergene::Model => allergene::AdminForm {
        title: "Allergènes",
        list_display: [["libelle", "Libellé"]],
    }
    themes: theme::Model => theme::AdminForm {
        title: "Thèmes",
        list_display: [["libelle", "Libellé"]],
    }
    regimes: regime::Model => regime::AdminForm {
        title: "Régimes alimentaires",
        list_display: [["libelle", "Libellé"]],
    }
    horaires: horaire::Model => horaire::AdminForm {
        title: "Horaires",
        create_form: crate::formulaire::HorairesGroupeForm,
        bulk_create: jour,
        list_display: [
            ["jour", "Jour"],
            ["ouverture_midi", "Ouverture midi"],
            ["fermeture_midi", "Fermeture midi"],
            ["ouverture_soir", "Ouverture soir"],
            ["fermeture_soir", "Fermeture soir"],
            ["ferme", "Fermé"],
        ],
        list_filter: [["ferme", "Fermé", 10]],
        group_action: [
            ["ferme", "Marquer fermé"],
        ],
    }
    devis_traiteur: devis_traiteur::Model => devis_traiteur::AdminForm {
        title: "Demandes de devis",
        list_display: [
            ["nom", "Nom"],
            ["email", "Email"],
            ["menu_id", "Menu", "menus.titre"],
            ["date_evenement", "Date événement"],
            ["nb_personnes", "Personnes"],
            ["statut", "Statut"],
            ["created_at", "Reçu le"],
        ],
        list_filter: [["statut", "Statut", 10]],
        group_action: [["statut", "Marquer en cours"]],
    }
    contacts: contact::Model => contact::AdminForm {
        title: "Messages de contact",
        list_display: [
            ["titre", "Sujet"],
            ["email", "Email"],
            ["created_at", "Reçu le"],
        ],
    }
    garnitures: garniture::Model => garniture::AdminForm {
        title: "Garnitures",
        list_display: [
            ["libelle", "Libellé"],
            ["type_garniture", "Type"],
            ["disponible", "Disponible"],
        ],
        list_filter: [
            ["type_garniture", "Type", 10],
            ["disponible", "Disponibilité", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
    }
    plats: plat::Model => plat::AdminForm {
        title: "Plats",
        list_display: [
            ["titre", "Titre"],
            ["type_plat", "Type"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
            ["est_viande", "Viande"],
            ["avec_legumes", "Légumes inclus"],
        ],
        list_filter: [
            ["type_plat", "Type de plat", 10],
            ["disponible", "Disponibilité", 10],
            ["est_viande", "Viande", 10],
            ["avec_legumes", "Légumes", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
        m2m: [
            ["allergenes", "Allergènes", "plat_allergene", "plat_id", "allergene_id", "crate::entities::allergene", "libelle"],
            ["garnitures", "Garnitures", "plat_garnitures", "plat_id", "garniture_id", "crate::entities::garniture", "libelle"],
        ],
    }
    menus: menu::Model => menu::AdminForm {
        title: "Menus traiteur",
        list_display: [
            ["titre", "Titre"],
            ["theme_id", "Thème", "themes.libelle"],
            ["regime_id", "Régime", "regimes.libelle"],
            ["prix_par_personne", "Prix/pers."],
            ["nb_personnes_min", "Min. pers."],
            ["actif", "Actif"],
        ],
        list_filter: [
            ["actif", "Actif", 10],
            ["theme_id", "Thème", 10],
            ["regime_id", "Régime", 10],
        ],
        group_action: [["actif", "Activer"]],
        m2m: [
            ["plats", "Plats", "menu_plat", "menu_id", "plat_id", "crate::entities::plat", "titre"]
        ],
    }
    menus_resto: menu_resto::Model => menu_resto::AdminForm {
        title: "Menus restaurant",
        list_display: [
            ["titre", "Titre"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
        ],
        group_action: [["disponible", "Rendre disponible"]],
    }
    menus_resto_composition: menu_resto_plat::Model => menu_resto_plat::AdminForm {
        title: "Composition des menus restaurant",
        list_display: [
            ["menu_id", "Menu", "menus_resto.titre"],
            ["plat_id", "Plat", "plats.titre"],
            ["cours", "Cours"],
        ],
        list_filter: [
            ["cours", "Cours", 10],
        ],
    }
    boissons: boisson::Model => boisson::AdminForm {
        title: "Boissons",
        list_display: [
            ["titre", "Titre"],
            ["type_boisson", "Type"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
        ],
        list_filter: [
            ["type_boisson", "Type", 10],
            ["disponible", "Disponibilité", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
    }
    commandes: commande::Model => commande::AdminForm {
        title: "Commandes",
        template_detail: "admin/commande_detail.html",
        list_display: [
            ["numero", "N°"],
            ["user_id", "Client", "eihwaz_users.username"],
            ["type_commande", "Type"],
            ["statut", "Statut"],
            ["mode_paiement", "Paiement"],
            ["prix_total", "Total"],
            ["created_at", "Date"],
        ],
        list_filter: [
            ["type_commande", "Type", 10],
            ["statut", "Statut", 10],
            ["mode_paiement", "Paiement", 10],
            ["avec_livraison", "Livraison", 10],
        ],
    }
    avis: avis::Model => avis::AdminForm {
        title: "Avis clients",
        list_display: [
            ["commande_id", "Commande"],
            ["note", "Note"],
            ["statut", "Statut"],
            ["created_at", "Date"],
        ],
        list_filter: [["statut", "Statut", 10]],
        group_action: [["statut", "Valider"]],
    }
    info_resto: info_resto::Model => info_resto::AdminForm {
        title: "Informations du restaurant",
        list_display: [
            ["nom", "Nom"],
            ["adresse", "Adresse"],
            ["telephone", "Téléphone"],
            ["email", "Email"],
        ],
    }
}
