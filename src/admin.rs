use crate::entities::*;

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
            ["raison", "Raison"],
            ["titre", "Sujet"],
            ["email", "Email"],
            ["created_at", "Reçu le"],
        ],
        list_filter: [["raison", "Raison", 10]],
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
    supplements: supplement::Model => supplement::AdminForm {
        title: "Suppléments",
        list_display: [
            ["titre", "Nom"],
            ["garniture_id", "Garniture", "garnitures.libelle"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
        ],
        list_filter: [
            ["disponible", "Disponible", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
    }
    entrees: entree::Model => entree::AdminForm {
        title: "Entrées",
        list_display: [
            ["titre", "Titre"],
            ["usage", "Usage"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
        ],
        list_filter: [
            ["usage", "Usage", 10],
            ["disponible", "Disponibilité", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
        m2m: [
            ["allergenes", "Allergènes", "entree_allergene", "entree_id", "allergene_id", "crate::entities::allergene", "libelle"],
        ],
    }
    desserts: dessert::Model => dessert::AdminForm {
        title: "Desserts",
        list_display: [
            ["titre", "Titre"],
            ["usage", "Usage"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
        ],
        list_filter: [
            ["usage", "Usage", 10],
            ["disponible", "Disponibilité", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
        m2m: [
            ["allergenes", "Allergènes", "dessert_allergene", "dessert_id", "allergene_id", "crate::entities::allergene", "libelle"],
        ],
    }
    plats: plat::Model => plat::AdminForm {
        title: "Plats",
        list_display: [
            ["titre", "Titre"],
            ["type_plat", "Type"],
            ["usage", "Usage"],
            ["prix", "Prix"],
            ["disponible", "Disponible"],
            ["est_viande", "Viande"],
        ],
        list_filter: [
            ["type_plat", "Type de plat", 10],
            ["usage", "Usage", 10],
            ["disponible", "Disponibilité", 10],
            ["est_viande", "Viande", 10],
        ],
        group_action: [["disponible", "Rendre disponible"]],
        m2m: [
            ["allergenes",   "Allergènes",   "plat_allergene",   "plat_id", "allergene_id",   "crate::entities::allergene",   "libelle"],
            ["garnitures",   "Garnitures",   "plat_garnitures",  "plat_id", "garniture_id",   "crate::entities::garniture",   "libelle"],
            ["supplements",  "Suppléments",  "plat_supplements", "plat_id", "supplement_id",  "crate::entities::supplement",  "titre"],
        ],
    }
    menus: menu::Model => menu::AdminForm {
        title: "Menus",
        list_display: [
            ["nom", "Nom"],
            ["type_menu", "Type"],
            ["prix", "Prix"],
            ["ordre", "Ordre"],
        ],
        list_filter: [
            ["type_menu", "Type", 10],
        ],
        template_detail: "admin/menu_resto_detail.html",
        m2m: [
            ["entrees",  "Entrées",  "menu_entrees",  "menu_id", "entree_id", "crate::entities::entree", "titre"],
            ["plats",    "Plats",    "menu_plats",    "menu_id", "plat_id",   "crate::entities::plat",   "titre"],
            ["desserts", "Desserts", "menu_desserts", "menu_id", "dessert_id","crate::entities::dessert", "titre"],
        ],
    }
    menus_traiteur: menu_traiteur::Model => menu_traiteur::AdminForm {
        title: "Menus traiteur",
        list_display: [
            ["titre", "Titre"],
            ["theme", "Thème"],
            ["regime", "Régime"],
            ["prix_par_personne", "Prix / pers."],
            ["nb_personnes_min", "Personnes min"],
            ["stock", "Stock"],
            ["actif", "Actif"],
        ],
        list_filter: [
            ["theme", "Thème", 10],
            ["regime", "Régime", 10],
            ["actif", "Actif", 10],
        ],
        group_action: [["actif", "Activer"]],
        m2m: [
            ["plats", "Plats", "menu_traiteur_plats", "menu_traiteur_id", "plat_id", "crate::entities::plat", "titre"],
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
            ["type_retrait", "Type"],
            ["statut", "Statut"],
            ["mode_paiement", "Paiement"],
            ["prix_total", "Total"],
            ["created_at", "Date"],
        ],
        list_filter: [
            ["type_retrait", "Type", 10],
            ["statut", "Statut", 10],
            ["mode_paiement", "Paiement", 10],
        ],
    }
    avis: avis::Model => avis::AdminForm {
        title: "Avis clients",
        list_display: [
            ["commande_id", "Commande", "commandes.numero"],
            ["note", "Note"],
            ["statut", "Statut"],
            ["created_at", "Date"],
        ],
        list_filter: [["statut", "Statut", 10]],
        group_action: [
            ["statut", "Valider", "valide"],
            ["statut", "Refuser", "refuse"],
        ],
    }
    avis_plats: avis_plat::Model => avis_plat::AdminForm {
        title: "Avis sur les plats",
        list_display: [
            ["plat_id", "Plat", "plats.titre"],
            ["note", "Note"],
            ["statut", "Statut"],
            ["created_at", "Date"],
        ],
        list_filter: [["statut", "Statut", 10]],
        group_action: [
            ["statut", "Valider", "valide"],
            ["statut", "Refuser", "refuse"],
        ],
    }
    info_resto: info_resto::Model => info_resto::AdminForm {
        title: "Informations du restaurant",
        list_display: [
            ["nom", "Nom"],
            ["adresse", "Adresse"],
            ["telephone", "Téléphone"],
            ["email", "Email"],
            ["latitude", "Latitude"],
            ["longitude", "Longitude"],
        ],
    }
}
