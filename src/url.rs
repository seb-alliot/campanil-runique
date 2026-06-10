use crate::views::*;
use runique::axum::routing::{get as get_route, post as post_route};
use runique::prelude::*;

pub fn routes() -> Router {
    urlpatterns! {
        "/"                                     => view!{ index },                  name = "index",
        "/deconnexion"                          => view!{ deconnexion },            name = "deconnexion",
        "/activer/{token}/{encrypted_email}"    => view!{ activer },                name = "activer",
        "/carte"                                => view!{ carte },                  name = "carte",
        "/boissons/{type}"                      => view!{ boissons_type },          name = "boissons_type",
        "/plat/{id}/avis.json"                  => view!{ plat_avis_json },         name = "plat_avis_json",
        "/menus"                                => view!{ menus },                  name = "menus",
        "/menus/{id}"                           => view!{ menu_detail },            name = "menu_detail",
        "/menus/track"                          => view!{ track_menu_filters },     name = "track_menu_filters",
        "/compte"                               => view!{ compte },                 name = "compte",
        "/compte/commandes.json"                => view!{ compte_commandes_json },  name = "compte_commandes_json",
        "/compte/supprimer"                     => view!{ supprimer_compte },       name = "supprimer_compte",
        "/panier"                               => view!{ panier_page },            name = "panier",
        "/panier/ajouter"                       => post_route(panier_ajouter_view),  name = "panier_ajouter",
        "/panier/retirer"                       => post_route(panier_retirer_view), name = "panier_retirer",
        "/panier/livraison-prix"                => view!{ panier_livraison_prix },  name = "panier_livraison_prix",
        "/commande/{numero}/confirmation"       => view!{ commande_confirmation },  name = "commande_confirmation",
        "/service"                              => view!{ service },                 name = "service",
        "/service/commandes.json"               => view!{ service_commandes_json },   name = "service_commandes_json",
        "/service/stats.json"                   => view!{ service_stats_json },      name = "service_stats_json",
        "/service/commandes/{numero}/statut"    => view!{ service_statut },          name = "service_statut",
        "/service/stock.json"                   => view!{ service_stock_json },      name = "service_stock_json",
        "/service/menus/{id}/stock"             => view!{ service_stock_update },    name = "service_stock_update",
        "/commande/{numero}/annuler"                      => view!{ commande_annuler },        name = "commande_annuler",
        "/commande/{numero}/ligne/{ligne_id}/modifier"    => view!{ commande_modifier_ligne },  name = "commande_modifier_ligne",
        "/commande/{numero}/ligne/{ligne_id}/supprimer"   => view!{ commande_supprimer_ligne }, name = "commande_supprimer_ligne",
        "/compte/avis/{commande_id}"            => view!{ avis_poster },            name = "avis_poster",
        "/compte/avis/{commande_id}/supprimer"  => view!{ avis_supprimer },         name = "avis_supprimer",
        "/compte/avis-plat/{plat_id}"           => view!{ avis_plat_poster },            name = "avis_plat_poster",
        "/compte/avis-plat/{plat_id}/supprimer" => view!{ avis_plat_supprimer },         name = "avis_plat_supprimer",
        "/compte/avis-article/{type_article}/{id}"           => view!{ avis_article_poster },    name = "avis_article_poster",
        "/compte/avis-article/{type_article}/{id}/supprimer" => view!{ avis_article_supprimer }, name = "avis_article_supprimer",
        "/compte/profil"                        => view!{ profil_post },            name = "profil_post",
        "/traiteur/devis/confirmation"          => view!{ devis_confirmation },     name = "devis_confirmation",
        "/mentions-legales"                     => view!{ mentions_legales },       name = "mentions_legales",
        "/sitemap.xml"                          => view!{ sitemap_xml },            name = "sitemap_xml",
        "/llms.txt"                             => view!{ llms_txt },               name = "llms_txt",
        "/api/admin/materiel-count"             => get_route(ajax_materiel_count),  name = "admin_materiel_count",
        "/api/admin/commandes/{id}/marquer-rendu"    => post_route(ajax_marquer_rendu),    name = "admin_marquer_rendu",
        "/api/admin/commandes/{id}/appliquer-penalite" => post_route(ajax_appliquer_penalite), name = "admin_appliquer_penalite",
    }
    // 10 req / 5 min — brute force login (toutes méthodes)
    .rate_limit_many(10, 300, vec![], vec![
        ("/connexion".into(),  "connexion".into(),  view!{ connexion }, ),
        ("/inscription".into(), "inscription".into(), view!{ inscription }),
    ])
    // 10 req / 10 min — anti-spam email - commande etc (POST uniquement)
    .rate_limit_many(10, 600, vec![Method::POST], vec![
        ("/contact".into(),        "contact".into(),        view!{ contact }),
        ("/traiteur/devis".into(), "devis_traiteur".into(), view!{ devis_traiteur }),
    ])
    // 3 req / 5 min — anti-spam commandes (POST uniquement)
    .rate_limit("/panier/commander", "panier_commander", view!{ panier_commander_view }, 3, 300, vec![Method::POST])
}

pub fn admin_extra_routes() -> Vec<(&'static str, runique::axum::routing::MethodRouter)> {
    vec![
        (
            "/commandes/{numero}/detail",
            view! { admin_commande_detail },
        ),
        (
            "/menus/{id}/composition",
            view! { admin_menu_resto_composition },
        ),
    ]
}
