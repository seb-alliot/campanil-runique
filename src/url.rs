use crate::views::*;
use runique::prelude::*;

pub fn routes() -> Router {
    urlpatterns! {
        "/"                                     => view!{ index },                  name = "index",
        "/deconnexion"                          => view!{ deconnexion },            name = "deconnexion",
        "/activer/{token}/{encrypted_email}"    => view!{ activer },                name = "activer",
        "/carte"                                => view!{ carte },                  name = "carte",
        "/boissons/{type}"                      => view!{ boissons_type },          name = "boissons_type",
        "/menus"                                => view!{ menus },                  name = "menus",
        "/menus/{id}"                           => view!{ menu_detail },            name = "menu_detail",
        "/compte"                               => view!{ compte },                 name = "compte",
        "/compte/supprimer"                     => view!{ supprimer_compte },       name = "supprimer_compte",
        "/panier"                               => view!{ panier_page },            name = "panier",
        "/panier/ajouter"                       => view!{ panier_ajouter_view },    name = "panier_ajouter",
        "/panier/retirer"                       => view!{ panier_retirer_view },    name = "panier_retirer",
        "/panier/commander"                     => view!{ panier_commander_view },  name = "panier_commander",
        "/commande/{numero}/confirmation"       => view!{ commande_confirmation },  name = "commande_confirmation",
        "/service"                              => view!{ service },                 name = "service",
        "/service/commandes.json"               => view!{ service_json },            name = "service_json",
        "/service/commandes/{numero}/statut"    => view!{ service_statut },          name = "service_statut",
        "/commande/{numero}/annuler"            => view!{ commande_annuler },       name = "commande_annuler",
        "/compte/avis/{commande_id}"            => view!{ avis_poster },            name = "avis_poster",
        "/compte/avis/{commande_id}/supprimer"  => view!{ avis_supprimer },         name = "avis_supprimer",
        "/compte/profil"                        => view!{ profil_post },            name = "profil_post",
        "/traiteur/devis/confirmation"          => view!{ devis_confirmation },     name = "devis_confirmation",
        "/mentions-legales"                     => view!{ mentions_legales },       name = "mentions_legales",
        "/sitemap.xml"                          => view!{ sitemap_xml },            name = "sitemap_xml",
        "/llms.txt"                             => view!{ llms_txt },               name = "llms_txt",
    }
    // 10 req / 5 min — brute force login
    .rate_limit_many(10, 300, vec![
        ("/connexion".into(),  "connexion".into(),  view!{ connexion }, ),
        ("/inscription".into(), "inscription".into(), view!{ inscription }),
    ])
    // 5 req / 10 min — anti-spam email
    .rate_limit_many(5, 600, vec![
        ("/contact".into(),        "contact".into(),        view!{ contact }),
        ("/traiteur/devis".into(), "devis_traiteur".into(), view!{ devis_traiteur }),
    ])
}

pub fn admin_extra_routes() -> Vec<(&'static str, runique::axum::routing::MethodRouter)> {
    vec![(
        "/commandes/{numero}/detail",
        view! { admin_commande_detail },
    )]
}
