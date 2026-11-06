//! Les relations many-to-many de l'admin : `ModelAdmin` ne les gère pas, ces pages
//! remplacent donc la page de détail de leur modèle. La route ne vérifie que l'accès
//! à l'admin : la preuve sur le parent et les identifiants passent par la porte.
use crate::entities::{
    allergene, dessert, dessert_allergene, entree, entree_allergene, garniture, menu_traiteur,
    menu_traiteur_plat, plat, plat_allergene, plat_garniture, plat_supplement, supplement,
};
use runique::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set, TransactionTrait,
};
use serde::Serialize;

#[derive(Serialize)]
struct Choix {
    id: i32,
    libelle: String,
    coche: bool,
}

#[derive(Serialize)]
struct Groupe {
    titre: &'static str,
    champ: &'static str,
    choix: Vec<Choix>,
}

fn choix(lignes: impl IntoIterator<Item = (i32, String)>, liens: &[i32]) -> Vec<Choix> {
    lignes
        .into_iter()
        .map(|(id, libelle)| Choix {
            id,
            libelle,
            coche: liens.contains(&id),
        })
        .collect()
}

fn ids_envoyes(data: &StrMap, champ: &str) -> Vec<i32> {
    data.get(champ)
        .map_or("", String::as_str)
        .split(',')
        .filter_map(|id| id.trim().parse().ok())
        .collect()
}

/// Retire les liens décochés et ajoute les nouveaux : une ligne gardée reste
/// intacte, avec ses autres colonnes (`est_defaut` d'une garniture).
macro_rules! synchroniser {
    ($db:expr, $lien:ident, $parent_col:ident, $cible_col:ident, $parent:ident, $cible:ident, $id:expr, $ids:expr) => {{
        $lien::Entity::delete_many()
            .filter($lien::Column::$parent_col.eq($id))
            .filter($lien::Column::$cible_col.is_not_in($ids.iter().copied()))
            .exec($db)
            .await?;
        let deja: Vec<i32> = $lien::Entity::find()
            .filter($lien::Column::$parent_col.eq($id))
            .all($db)
            .await?
            .into_iter()
            .map(|ligne| ligne.$cible)
            .collect();
        for cible in $ids.iter().copied().filter(|cible| !deja.contains(cible)) {
            $lien::ActiveModel {
                $parent: Set($id),
                $cible: Set(cible),
                ..Default::default()
            }
            .insert($db)
            .await?;
        }
    }};
}

/// Page commune : `refuse` renvoie vers la liste du modèle quand la porte dit non.
macro_rules! parent_autorise {
    ($request:expr, $admin:expr, $entite:ty, $cle:literal, $gate:ident) => {{
        let Some(id) = $request.get_path_as::<i32>("id") else {
            return Ok(StatusCode::BAD_REQUEST.into_response());
        };
        if let Err(refus) = authorize_view::<$entite>(&$gate, id) {
            $request.notices.error(refus.message()).await;
            let liste = format!("{}/{}/list", $admin.config.prefix, $cle);
            return Ok(Redirect::to(&liste).into_response());
        }
        id
    }};
}

/// Sur un POST : la preuve de modification du parent, avant toute lecture du corps.
macro_rules! donnees_autorisees {
    ($request:expr, $entite:ty, $gate:ident, $id:expr, $retour:expr) => {{
        if let Err(refus) = authorize_row::<Update, $entite>(&$gate, $id).await {
            $request.notices.error(refus.message()).await;
            return Ok(Redirect::to(&$retour).into_response());
        }
        let Some(data) = $request.prisme.checked_data() else {
            return Ok(StatusCode::FORBIDDEN.into_response());
        };
        data.clone()
    }};
}

/// Des identifiants envoyés par le client : uniquement ce que ce compte peut lire.
macro_rules! ids_lisibles {
    ($request:expr, $entite:ty, $gate:ident, $ids:expr, $retour:expr) => {{
        match readable_ids::<$entite>(&$gate, $ids).await {
            Ok(ids) => ids.ids().to_vec(),
            Err(refus) => {
                $request.notices.error(refus.message()).await;
                return Ok(Redirect::to(&$retour).into_response());
            }
        }
    }};
}

fn afficher(
    request: &mut Request,
    admin: &AdminState,
    site: &AdminSite,
    cle: &str,
    id: i32,
    nom: &str,
    groupes: Vec<Groupe>,
) -> AppResult<Response> {
    admin_page_context(request, admin, site, Page::Detail, cle);
    let liste = format!("{}/{}/list", admin.config.prefix, cle);
    let modifier = format!("{}/{}/{}/edit", admin.config.prefix, cle, id);
    context_update!(request => {
        "title"    => format!("Relations — {nom}"),
        "nom"      => nom,
        "liste"    => liste,
        "modifier" => modifier,
        "groupes"  => groupes,
    });
    request.render("admin/relations.html")
}

async fn allergenes(db: &ADb) -> Vec<(i32, String)> {
    allergene::Entity::find()
        .order_by_asc(allergene::Column::Libelle)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| (a.id, a.libelle))
        .collect()
}

/// Les liens d'un plat, à la charge de l'appelant : la preuve et sa transaction.
pub(crate) async fn ecrire_liens_plat<C: ConnectionTrait>(
    db: &C,
    id: i32,
    allergenes: &[i32],
    garnitures: &[i32],
    supplements: &[i32],
) -> Result<(), DbErr> {
    synchroniser!(
        db,
        plat_allergene,
        PlatId,
        AllergeneId,
        plat_id,
        allergene_id,
        id,
        allergenes
    );
    synchroniser!(
        db,
        plat_garniture,
        PlatId,
        GarnitureId,
        plat_id,
        garniture_id,
        id,
        garnitures
    );
    synchroniser!(
        db,
        plat_supplement,
        PlatId,
        SupplementId,
        plat_id,
        supplement_id,
        id,
        supplements
    );
    Ok(())
}

pub async fn handle_relations_plat(
    request: &mut Request,
    admin: &AdminState,
    site: &AdminSite,
) -> AppResult<Response> {
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to(&format!("{}/plats/list", admin.config.prefix)).into_response());
    };
    let engine = request.engine.clone();
    let db = &engine.db;
    let gate = GateCtx {
        db: &engine.db,
        user: &user,
    };
    let id = parent_autorise!(request, admin, plat::Entity, "plats", gate);
    let retour = format!("/gestion/plats/{id}");
    let Ok(Some(parent)) = plat::Entity::find_by_id(id).one(db).await else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    if request.method == Method::POST {
        let data = donnees_autorisees!(request, plat::Entity, gate, id, retour);
        let a = ids_lisibles!(
            request,
            allergene::Entity,
            gate,
            ids_envoyes(&data, "allergenes"),
            retour
        );
        let g = ids_lisibles!(
            request,
            garniture::Entity,
            gate,
            ids_envoyes(&data, "garnitures"),
            retour
        );
        let s = ids_lisibles!(
            request,
            supplement::Entity,
            gate,
            ids_envoyes(&data, "supplements"),
            retour
        );
        let ecrit: Result<(), DbErr> = async {
            let txn = db.begin().await?;
            ecrire_liens_plat(&txn, id, &a, &g, &s).await?;
            txn.commit().await
        }
        .await;
        notifier(request, ecrit).await;
        return Ok(Redirect::to(&retour).into_response());
    }

    let liens_a: Vec<i32> = search!(plat_allergene::Entity => PlatId eq id,)
        .all(&engine.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.allergene_id)
        .collect();
    let liens_g: Vec<i32> = search!(plat_garniture::Entity => PlatId eq id,)
        .all(&engine.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.garniture_id)
        .collect();
    let liens_s: Vec<i32> = search!(plat_supplement::Entity => PlatId eq id,)
        .all(&engine.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.supplement_id)
        .collect();
    let garnitures = garniture::Entity::find()
        .order_by_asc(garniture::Column::Libelle)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|g| (g.id, g.libelle));
    let supplements = supplement::Entity::find()
        .order_by_asc(supplement::Column::Titre)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|s| (s.id, s.titre.unwrap_or_default()));
    let groupes = vec![
        Groupe {
            titre: "Allergènes",
            champ: "allergenes",
            choix: choix(allergenes(db).await, &liens_a),
        },
        Groupe {
            titre: "Garnitures",
            champ: "garnitures",
            choix: choix(garnitures, &liens_g),
        },
        Groupe {
            titre: "Suppléments",
            champ: "supplements",
            choix: choix(supplements, &liens_s),
        },
    ];
    afficher(request, admin, site, "plats", id, &parent.titre, groupes)
}

pub async fn handle_relations_entree(
    request: &mut Request,
    admin: &AdminState,
    site: &AdminSite,
) -> AppResult<Response> {
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to(&format!("{}/entrees/list", admin.config.prefix)).into_response());
    };
    let engine = request.engine.clone();
    let db = &engine.db;
    let gate = GateCtx {
        db: &engine.db,
        user: &user,
    };
    let id = parent_autorise!(request, admin, entree::Entity, "entrees", gate);
    let retour = format!("/gestion/entrees/{id}");
    let Ok(Some(parent)) = entree::Entity::find_by_id(id).one(db).await else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    if request.method == Method::POST {
        let data = donnees_autorisees!(request, entree::Entity, gate, id, retour);
        let a = ids_lisibles!(
            request,
            allergene::Entity,
            gate,
            ids_envoyes(&data, "allergenes"),
            retour
        );
        let ecrit: Result<(), DbErr> = async {
            let txn = db.begin().await?;
            synchroniser!(
                &txn,
                entree_allergene,
                EntreeId,
                AllergeneId,
                entree_id,
                allergene_id,
                id,
                a
            );
            txn.commit().await
        }
        .await;
        notifier(request, ecrit).await;
        return Ok(Redirect::to(&retour).into_response());
    }

    let liens: Vec<i32> = search!(entree_allergene::Entity => EntreeId eq id,)
        .all(&engine.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.allergene_id)
        .collect();
    let groupes = vec![Groupe {
        titre: "Allergènes",
        champ: "allergenes",
        choix: choix(allergenes(db).await, &liens),
    }];
    afficher(request, admin, site, "entrees", id, &parent.titre, groupes)
}

pub async fn handle_relations_dessert(
    request: &mut Request,
    admin: &AdminState,
    site: &AdminSite,
) -> AppResult<Response> {
    let Some(user) = request.user.clone() else {
        return Ok(Redirect::to(&format!("{}/desserts/list", admin.config.prefix)).into_response());
    };
    let engine = request.engine.clone();
    let db = &engine.db;
    let gate = GateCtx {
        db: &engine.db,
        user: &user,
    };
    let id = parent_autorise!(request, admin, dessert::Entity, "desserts", gate);
    let retour = format!("/gestion/desserts/{id}");
    let Ok(Some(parent)) = dessert::Entity::find_by_id(id).one(db).await else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    if request.method == Method::POST {
        let data = donnees_autorisees!(request, dessert::Entity, gate, id, retour);
        let a = ids_lisibles!(
            request,
            allergene::Entity,
            gate,
            ids_envoyes(&data, "allergenes"),
            retour
        );
        let ecrit: Result<(), DbErr> = async {
            let txn = db.begin().await?;
            synchroniser!(
                &txn,
                dessert_allergene,
                DessertId,
                AllergeneId,
                dessert_id,
                allergene_id,
                id,
                a
            );
            txn.commit().await
        }
        .await;
        notifier(request, ecrit).await;
        return Ok(Redirect::to(&retour).into_response());
    }

    let liens: Vec<i32> = search!(dessert_allergene::Entity => DessertId eq id,)
        .all(&engine.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.allergene_id)
        .collect();
    let groupes = vec![Groupe {
        titre: "Allergènes",
        champ: "allergenes",
        choix: choix(allergenes(db).await, &liens),
    }];
    afficher(request, admin, site, "desserts", id, &parent.titre, groupes)
}

pub async fn handle_relations_menu_traiteur(
    request: &mut Request,
    admin: &AdminState,
    site: &AdminSite,
) -> AppResult<Response> {
    let Some(user) = request.user.clone() else {
        return Ok(
            Redirect::to(&format!("{}/menus_traiteur/list", admin.config.prefix)).into_response(),
        );
    };
    let engine = request.engine.clone();
    let db = &engine.db;
    let gate = GateCtx {
        db: &engine.db,
        user: &user,
    };
    let id = parent_autorise!(
        request,
        admin,
        menu_traiteur::Entity,
        "menus_traiteur",
        gate
    );
    let retour = format!("/gestion/menus-traiteur/{id}");
    let Ok(Some(parent)) = menu_traiteur::Entity::find_by_id(id).one(db).await else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    if request.method == Method::POST {
        let data = donnees_autorisees!(request, menu_traiteur::Entity, gate, id, retour);
        let p = ids_lisibles!(
            request,
            plat::Entity,
            gate,
            ids_envoyes(&data, "plats"),
            retour
        );
        let ecrit: Result<(), DbErr> = async {
            let txn = db.begin().await?;
            synchroniser!(
                &txn,
                menu_traiteur_plat,
                MenuTraiteurId,
                PlatId,
                menu_traiteur_id,
                plat_id,
                id,
                p
            );
            txn.commit().await
        }
        .await;
        notifier(request, ecrit).await;
        return Ok(Redirect::to(&retour).into_response());
    }

    let liens: Vec<i32> = search!(menu_traiteur_plat::Entity => MenuTraiteurId eq id,)
        .all(&engine.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.plat_id)
        .collect();
    let plats = plat::Entity::find()
        .order_by_asc(plat::Column::Titre)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|p| (p.id, p.titre));
    let groupes = vec![Groupe {
        titre: "Plats",
        champ: "plats",
        choix: choix(plats, &liens),
    }];
    afficher(
        request,
        admin,
        site,
        "menus_traiteur",
        id,
        &parent.titre,
        groupes,
    )
}

async fn notifier(request: &Request, ecrit: Result<(), DbErr>) {
    match ecrit {
        Ok(()) => {
            request
                .notices
                .success("Relations enregistrées.".to_string())
                .await
        }
        Err(_) => {
            request
                .notices
                .error("Erreur lors de l'enregistrement.".to_string())
                .await
        }
    };
}
