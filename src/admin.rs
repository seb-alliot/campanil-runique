//! L'admin, déclaré avec le builder typé `ModelAdmin`. Les relations many-to-many
//! n'ont pas d'équivalent ici : elles passent par les routes `admin` de `url.rs`.
use crate::entities::{
    allergene, avis, avis_plat, boisson, commande, contact, dessert, devis_traiteur, entree,
    garniture, horaire, info_resto, menu, menu_traiteur, plat, supplement, user_profil,
};
use runique::prelude::*;

pub fn config(admin: AdminStaging) -> AdminStaging {
    admin
        .site_title("Campanile — Administration")
        .sitemap("https://ucampanile.fr/sitemap.xml")
        .path("/admin-campanile/")
        .resource_order([
            "users",
            "plats",
            "menus",
            "commandes",
            "avis",
            "allergenes",
            "horaires",
            "contacts",
            "info_resto",
        ])
        .with_rate_limiter(RateLimiter::new().max_requests(20).retry_after(3600))
        .with_login_guard(LoginGuard::new().max_attempts(20).lockout_secs(300))
        .page_size(10)
        .site(site)
}

fn site(site: AdminSite) -> AdminSite {
    site.accounts_users(comptes())
        .register(allergenes())
        .register(horaires())
        .register(devis())
        .register(contacts())
        .register(garnitures())
        .register(supplements())
        .register(entrees())
        .register(desserts())
        .register(plats())
        .register(menus())
        .register(menus_traiteur())
        .register(boissons())
        .register(commandes())
        .register(avis_clients())
        .register(avis_plats())
        .register(info_restaurant())
}

fn comptes() -> ModelAdmin<user_profil::Entity> {
    use user_profil::Column::*;
    ModelAdmin::new().list(|l| {
        l.display([Username, Email, IsActive, CreatedAt])
            .search([Username, Email])
    })
}

fn allergenes() -> ModelAdmin<allergene::Entity> {
    use allergene::Column::*;
    ModelAdmin::new()
        .title("Allergènes")
        .list(|l| l.display([Libelle]))
}

fn horaires() -> ModelAdmin<horaire::Entity> {
    use horaire::Column::*;
    ModelAdmin::new().title("Horaires").list(|l| {
        l.display([
            Jour,
            OuvertureMidi,
            FermetureMidi,
            OuvertureSoir,
            FermetureSoir,
            Ferme,
        ])
        .filter([Ferme])
        .action(Ferme, true, "Marquer fermé")
    })
}

fn devis() -> ModelAdmin<devis_traiteur::Entity> {
    use devis_traiteur::Column::*;
    ModelAdmin::new().title("Demandes de devis").list(|l| {
        l.display([
            Nom,
            Email,
            DateEvenement,
            NbPersonnes,
            PrixTotal,
            RemiseAppliquee,
            Statut,
            CreatedAt,
        ])
        .filter([Statut])
        .action(Statut, "en_cours", "Marquer en cours")
    })
}

fn contacts() -> ModelAdmin<contact::Entity> {
    use contact::Column::*;
    ModelAdmin::new().title("Messages de contact").list(|l| {
        l.display([Raison, Titre, Email, CreatedAt])
            .filter([Raison])
    })
}

fn garnitures() -> ModelAdmin<garniture::Entity> {
    use garniture::Column::*;
    ModelAdmin::new().title("Garnitures").list(|l| {
        l.display([Libelle, TypeGarniture, Disponible])
            .filter([TypeGarniture, Disponible])
            .action(Disponible, true, "Rendre disponible")
    })
}

fn supplements() -> ModelAdmin<supplement::Entity> {
    use supplement::Column::*;
    ModelAdmin::new().title("Suppléments").list(|l| {
        l.display([Titre, GarnitureId, Prix, Disponible])
            .filter([Disponible])
            .action(Disponible, true, "Rendre disponible")
    })
}

fn entrees() -> ModelAdmin<entree::Entity> {
    use entree::Column::*;
    ModelAdmin::new().title("Entrées").list(|l| {
        l.display([Titre, Usage, Prix, Disponible])
            .filter([Usage, Disponible])
            .action(Disponible, true, "Rendre disponible")
    })
}

fn desserts() -> ModelAdmin<dessert::Entity> {
    use dessert::Column::*;
    ModelAdmin::new().title("Desserts").list(|l| {
        l.display([Titre, Usage, Prix, Disponible])
            .filter([Usage, Disponible])
            .action(Disponible, true, "Rendre disponible")
    })
}

fn plats() -> ModelAdmin<plat::Entity> {
    use plat::Column::*;
    ModelAdmin::new().title("Plats").list(|l| {
        l.display([Titre, TypePlat, Usage, Prix, Disponible, EstViande])
            .filter([TypePlat, Usage, Disponible, EstViande])
            .action(Disponible, true, "Rendre disponible")
    })
}

fn menus() -> ModelAdmin<menu::Entity> {
    use menu::Column::*;
    ModelAdmin::new()
        .title("Menus")
        .list(|l| l.display([Nom, TypeMenu, Prix, Ordre]).filter([TypeMenu]))
}

fn menus_traiteur() -> ModelAdmin<menu_traiteur::Entity> {
    use menu_traiteur::Column::*;
    ModelAdmin::new().title("Menus traiteur").list(|l| {
        l.display([
            Titre,
            Theme,
            Regime,
            PrixParPersonne,
            NbPersonnesMin,
            RemiseGroupe,
            RemiseGroupeMin,
            Stock,
            Actif,
        ])
        .filter([Theme, Regime, Actif])
        .action(Actif, true, "Activer")
    })
}

fn boissons() -> ModelAdmin<boisson::Entity> {
    use boisson::Column::*;
    ModelAdmin::new().title("Boissons").list(|l| {
        l.display([Titre, TypeBoisson, Prix, Disponible])
            .filter([TypeBoisson, Disponible])
            .action(Disponible, true, "Rendre disponible")
    })
}

fn commandes() -> ModelAdmin<commande::Entity> {
    use commande::Column::*;
    ModelAdmin::new()
        .title("Commandes")
        .list(|l| {
            l.display([
                Numero,
                UserId,
                TypeRetrait,
                Statut,
                PretMateriel,
                ModePaiement,
                PrixTotal,
                CreatedAt,
            ])
            .filter([TypeRetrait, Statut, ModePaiement])
        })
        .form(|f| {
            f.fields([
                Numero,
                UserId,
                PrixTotal,
                Statut,
                ModePaiement,
                TypeRetrait,
                HeureRetrait,
                AdresseLivraison,
                VilleLivraison,
                CpLivraison,
                PrixLivraison,
            ])
            .create_only([Numero, UserId, PrixTotal])
        })
}

fn avis_clients() -> ModelAdmin<avis::Entity> {
    use avis::Column::*;
    ModelAdmin::new().title("Avis clients").list(|l| {
        l.display([CommandeId, Note, Statut, CreatedAt])
            .filter([Statut])
            .action(Statut, "valide", "Valider")
            .action(Statut, "refuse", "Refuser")
    })
}

fn avis_plats() -> ModelAdmin<avis_plat::Entity> {
    use avis_plat::Column::*;
    ModelAdmin::new().title("Avis sur les plats").list(|l| {
        l.display([PlatId, Note, Statut, CreatedAt])
            .filter([Statut])
            .action(Statut, "valide", "Valider")
            .action(Statut, "refuse", "Refuser")
    })
}

fn info_restaurant() -> ModelAdmin<info_resto::Entity> {
    use info_resto::Column::*;
    ModelAdmin::new()
        .title("Informations du restaurant")
        .list(|l| {
            l.display([
                Nom,
                Adresse,
                Telephone,
                Email,
                PenaliteMateriel,
                Latitude,
                Longitude,
            ])
        })
}
