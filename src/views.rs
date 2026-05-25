use crate::backend::*;
use crate::formulaire::{ContactForm, DevisTraiteurForm, LoginForm, RegisterForm};
use runique::prelude::*;

pub async fn index(mut request: Request) -> AppResult<Response> {
    inject_auth(&mut request).await;
    let avis = get_avis_valides(request.db()).await;
    context_update!(request => {
        "title" => "U Campanile — Restaurant corse à Corte",
        "avis_list" => &avis,
    });
    request.render("index.html")
}

pub async fn connexion(mut request: Request) -> AppResult<Response> {
    let mut form: LoginForm = request.form();
    handle_login(&mut request, &mut form).await
}

pub async fn deconnexion(request: Request) -> AppResult<Response> {
    logout(&request.session, None).await.ok();
    Ok(Redirect::to("/").into_response())
}

pub async fn inscription(headers: HeaderMap, mut request: Request) -> AppResult<Response> {
    let mut form: RegisterForm = request.form();
    handle_inscription(&mut request, &mut form, &headers).await
}

pub async fn activer(
    Path((token, encrypted_email)): Path<(String, String)>,
    mut request: Request,
) -> AppResult<Response> {
    handle_activate(&mut request, token, encrypted_email).await
}

pub async fn contact(mut request: Request) -> AppResult<Response> {
    let mut form: ContactForm = request.form();
    handle_contact(&mut request, &mut form).await
}

pub async fn menus(mut request: Request) -> AppResult<Response> {
    vue_menus(&mut request).await
}

pub async fn menu_detail(mut request: Request) -> AppResult<Response> {
    vue_menu_details(&mut request).await
}

pub async fn carte(mut request: Request) -> AppResult<Response> {
    vue_carte(&mut request).await
}

pub async fn boissons_type(mut request: Request) -> AppResult<Response> {
    vue_boissons_type(&mut request).await
}

pub async fn plat_avis_json(request: Request) -> AppResult<Response> {
    ajax_avis_plat(request).await
}

pub async fn panier_page(mut request: Request) -> AppResult<Response> {
    vue_panier(&mut request).await
}

pub async fn panier_ajouter_view(request: Request) -> AppResult<Response> {
    vue_ajouter_panier(request).await
}

pub async fn panier_retirer_view(request: Request) -> AppResult<Response> {
    vue_retirer_panier(request).await
}

pub async fn panier_commander_view(mut request: Request) -> AppResult<Response> {
    vue_panier_commander(&mut request).await
}

pub async fn commande_confirmation(mut request: Request) -> AppResult<Response> {
    vue_commande_confirmation(&mut request).await
}

pub async fn commande_annuler(mut request: Request) -> AppResult<Response> {
    handle_commande_annuler(&mut request).await
}

pub async fn commande_modifier_ligne(request: Request) -> AppResult<Response> {
    handle_modifier_ligne(request).await
}

pub async fn commande_supprimer_ligne(request: Request) -> AppResult<Response> {
    handle_supprimer_ligne(request).await
}

pub async fn compte(mut request: Request) -> AppResult<Response> {
    handle_compte(&mut request).await
}

pub async fn compte_commandes_json(mut request: Request) -> AppResult<Response> {
    compte_commandes_ajax(&mut request).await
}

pub async fn avis_poster(request: Request) -> AppResult<Response> {
    handle_avis(request).await
}
pub async fn avis_supprimer(request: Request) -> AppResult<Response> {
    handle_avis_supprimer(request).await
}
pub async fn avis_plat_poster(request: Request) -> AppResult<Response> {
    handle_avis_plat(request).await
}
pub async fn avis_plat_supprimer(request: Request) -> AppResult<Response> {
    handle_avis_plat_supprimer(request).await
}

pub async fn profil_post(mut request: Request) -> AppResult<Response> {
    handle_profil_post(&mut request).await
}

pub async fn supprimer_compte(mut request: Request) -> AppResult<Response> {
    handle_supprimer_compte(&mut request).await
}

pub async fn panier_livraison_prix(request: Request) -> AppResult<Response> {
    vue_livraison_prix(request).await
}

pub async fn admin_commande_detail(
    Extension(admin): Extension<Arc<runique::admin::AdminState>>,
    Extension(proto): Extension<Arc<runique::admin::PrototypeAdminState>>,
    mut request: Request,
) -> AppResult<Response> {
    handle_admin_commande_detail(&mut request, &admin, Some(proto)).await
}

pub async fn admin_menu_resto_composition(
    Extension(admin): Extension<Arc<runique::admin::AdminState>>,
    Extension(proto): Extension<Arc<runique::admin::PrototypeAdminState>>,
    mut request: Request,
) -> AppResult<Response> {
    handle_menu_resto_composition(&mut request, &admin, Some(proto)).await
}

pub async fn track_menu_filters(mut request: Request) -> AppResult<Response> {
    vue_track_menu_filters(&mut request).await
}

pub async fn service(mut request: Request) -> AppResult<Response> {
    handle_service(&mut request).await
}

pub async fn service_commandes_json(request: Request) -> AppResult<Response> {
    ajax_commandes(&request).await
}

pub async fn service_stats_json(request: Request) -> AppResult<Response> {
    ajax_stats(&request).await
}

pub async fn service_statut(mut request: Request) -> AppResult<Response> {
    handle_service_statut(&mut request).await
}

pub async fn service_stock_json(request: Request) -> AppResult<Response> {
    ajax_stock_get(&request).await
}

pub async fn service_stock_update(mut request: Request) -> AppResult<Response> {
    ajax_stock_update(&mut request).await
}

pub async fn devis_traiteur(mut request: Request) -> AppResult<Response> {
    let mut form: DevisTraiteurForm = request.form();
    handle_devis_traiteur(&mut request, &mut form).await
}

pub async fn devis_confirmation(mut request: Request) -> AppResult<Response> {
    handle_devis_confirmation(&mut request).await
}

pub async fn mentions_legales(mut request: Request) -> AppResult<Response> {
    inject_auth(&mut request).await;
    context_update!(request => { "title" => "Mentions légales — U Campanile" });
    request.render("mentions-legales.html")
}

const DOMAIN: &str = "https://u-campanile.fr";

pub async fn sitemap_xml() -> impl IntoResponse {
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>{d}/</loc><changefreq>weekly</changefreq><priority>1.0</priority></url>
  <url><loc>{d}/carte</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>
  <url><loc>{d}/menus</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>
  <url><loc>{d}/traiteur/devis</loc><changefreq>monthly</changefreq><priority>0.7</priority></url>
  <url><loc>{d}/contact</loc><changefreq>monthly</changefreq><priority>0.6</priority></url>
  <url><loc>{d}/mentions-legales</loc><changefreq>never</changefreq><priority>0.3</priority></url>
  <url><loc>{d}/connexion</loc><changefreq>never</changefreq><priority>0.2</priority></url>
  <url><loc>{d}/inscription</loc><changefreq>never</changefreq><priority>0.2</priority></url>
</urlset>"#,
        d = DOMAIN
    );
    ([("content-type", "application/xml; charset=utf-8")], body)
}

pub async fn llms_txt() -> impl IntoResponse {
    let body = format!(
        "# U Campanile\n\
         \n\
         > Restaurant de cuisine corse traditionnelle à Corte, Haute-Corse, France\n\
         \n\
         U Campanile est un restaurant familial proposant une cuisine corse de saison, \
         préparée chaque jour avec des produits locaux du terroir corse. \
         Situé à Corte, au cœur de la Haute-Corse.\n\
         \n\
         ## Pages\n\
         \n\
         - [Accueil]({d}/) — présentation du restaurant\n\
         - [La Carte]({d}/carte) — entrées, plats et desserts du moment\n\
         - [Menus traiteur]({d}/menus) — menus pour événements et réceptions\n\
         - [Devis traiteur]({d}/traiteur/devis) — demande de devis en ligne\n\
         - [Contact]({d}/contact) — formulaire de contact\n\
         - [Mentions légales]({d}/mentions-legales) — informations légales\n\
         \n\
         ## Informations\n\
         \n\
         Cuisine : corse, méditerranéenne, produits locaux\n\
         Localisation : Corte, 20250, Haute-Corse, France\n",
        d = DOMAIN
    );
    ([("content-type", "text/plain; charset=utf-8")], body)
}
