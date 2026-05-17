use crate::entities::plat::TypePlat;
use crate::entities::{menu_resto, menu_resto_plat, plat};
use runique::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, Set};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct PlatDisponible {
    pub id: Pk,
    pub titre: String,
}

#[derive(Serialize)]
pub struct CompositionMenu {
    pub entrees_disponibles: Vec<PlatDisponible>,
    pub plats_disponibles: Vec<PlatDisponible>,
    pub desserts_disponibles: Vec<PlatDisponible>,
    pub entree_ids: Vec<Pk>,
    pub plat_ids: Vec<Pk>,
    pub dessert_ids: Vec<Pk>,
    pub dessert: String,
}

pub async fn handle_menu_resto_composition(
    request: &mut Request,
    admin: &AdminState,
    proto: Option<Arc<PrototypeAdminState>>,
) -> AppResult<Response> {
    let id: Pk = match request.get_path_as::<Pk>("id") {
        Some(v) => v,
        None => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    let db = request.db();

    let Some(menu) = menu_resto::Entity::find_by_id(id)
        .one(db)
        .await
        .unwrap_or(None)
    else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    if request.is_post() {
        let entree_ids = parse_ids(request.prisme.data.get("entree_ids").map(String::as_str));
        let plat_ids = parse_ids(request.prisme.data.get("plat_ids").map(String::as_str));
        let dessert_ids = parse_ids(request.prisme.data.get("dessert_ids").map(String::as_str));
        let dessert_txt = request
            .prisme
            .data
            .get("dessert")
            .cloned()
            .unwrap_or_default();

        menu_resto_plat::Entity::delete_many()
            .filter(menu_resto_plat::Column::MenuId.eq(id as i32))
            .exec(db)
            .await
            .ok();

        for pid in entree_ids {
            menu_resto_plat::ActiveModel {
                menu_id: Set(id as i32),
                plat_id: Set(pid),
                cours: Set("entree".into()),
                ..Default::default()
            }
            .insert(db)
            .await
            .ok();
        }
        for pid in plat_ids {
            menu_resto_plat::ActiveModel {
                menu_id: Set(id as i32),
                plat_id: Set(pid),
                cours: Set("plat".into()),
                ..Default::default()
            }
            .insert(db)
            .await
            .ok();
        }
        for pid in dessert_ids {
            menu_resto_plat::ActiveModel {
                menu_id: Set(id as i32),
                plat_id: Set(pid),
                cours: Set("dessert".into()),
                ..Default::default()
            }
            .insert(db)
            .await
            .ok();
        }

        let mut active: menu_resto::ActiveModel = menu.into();
        active.dessert = Set(if dessert_txt.is_empty() {
            None
        } else {
            Some(dessert_txt)
        });
        active.update(db).await.ok();

        return Ok(Redirect::to(&format!(
            "{}/menus_resto/{}/detail",
            admin.config.prefix, id
        ))
        .into_response());
    }

    let liens = search!(menu_resto_plat::Entity => MenuId eq id as i32,)
        .all(db)
        .await
        .unwrap_or_default();

    let mut entree_ids: Vec<Pk> = Vec::new();
    let mut plat_ids: Vec<Pk> = Vec::new();
    let mut dessert_ids: Vec<Pk> = Vec::new();
    for l in &liens {
        match l.cours.as_str() {
            "entree" => entree_ids.push(l.plat_id as Pk),
            "plat" => plat_ids.push(l.plat_id as Pk),
            "dessert" => dessert_ids.push(l.plat_id as Pk),
            _ => {}
        }
    }

    let tous_plats: Vec<plat::Model> = search!(plat::Entity => asc Titre,)
        .filter(plat::Column::Disponible.eq(true))
        .all(db)
        .await
        .unwrap_or_default();

    let entrees_disponibles: Vec<PlatDisponible> = tous_plats
        .iter()
        .filter(|p| p.type_plat == TypePlat::Entree)
        .map(|p| PlatDisponible {
            id: p.id,
            titre: p.titre.clone(),
        })
        .collect();

    let plats_disponibles: Vec<PlatDisponible> = tous_plats
        .iter()
        .filter(|p| {
            matches!(
                p.type_plat,
                TypePlat::Specialite | TypePlat::Viande | TypePlat::Poisson | TypePlat::Plat
            )
        })
        .map(|p| PlatDisponible {
            id: p.id,
            titre: p.titre.clone(),
        })
        .collect();

    let desserts_disponibles: Vec<PlatDisponible> = tous_plats
        .iter()
        .filter(|p| p.type_plat == TypePlat::Dessert)
        .map(|p| PlatDisponible {
            id: p.id,
            titre: p.titre.clone(),
        })
        .collect();

    let dessert = menu.dessert.clone().unwrap_or_default();

    let composition = CompositionMenu {
        entrees_disponibles,
        plats_disponibles,
        desserts_disponibles,
        entree_ids,
        plat_ids,
        dessert_ids,
        dessert,
    };

    inject_admin_prefix(&mut request.context, &admin.config.prefix);
    insert_admin_messages(&mut request.context, "base");

    let resources: Vec<&AdminResource> = proto
        .as_ref()
        .map(|s| s.registry.all().map(|e| &e.meta).collect())
        .unwrap_or_default();

    context_update!(request => {
        "composition"      => composition,
        "menu_id"          => id,
        "menu_nom"         => &menu.nom,
        "title"            => format!("Composition — {}", menu.nom),
        "site_title"       => &admin.config.site_title,
        "current_page"     => "menus_resto",
        "current_resource" => "menus_resto",
        "resources"        => resources,
    });

    request.render("admin/menu_resto_composition.html")
}

fn parse_ids(val: Option<&str>) -> Vec<i32> {
    val.unwrap_or("")
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect()
}
