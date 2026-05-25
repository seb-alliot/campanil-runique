use crate::entities::{dessert, entree, menu, menu_dessert, menu_entree, menu_plat, plat};
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
    pub entree_libre: String,
    pub plat_libre: String,
    pub dessert_libre: String,
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

    let Some(menu_model) = menu::Entity::find_by_id(id).one(db).await.unwrap_or(None) else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    if request.is_post() {
        let entree_ids = parse_ids(request.prisme.data.get("entree_ids").map(String::as_str));
        let plat_ids = parse_ids(request.prisme.data.get("plat_ids").map(String::as_str));
        let dessert_ids = parse_ids(request.prisme.data.get("dessert_ids").map(String::as_str));
        let entree_libre_txt = request
            .prisme
            .data
            .get("entree_libre")
            .cloned()
            .unwrap_or_default();
        let plat_libre_txt = request
            .prisme
            .data
            .get("plat_libre")
            .cloned()
            .unwrap_or_default();
        let dessert_libre_txt = request
            .prisme
            .data
            .get("dessert_libre")
            .cloned()
            .unwrap_or_default();

        menu_entree::Entity::delete_many()
            .filter(menu_entree::Column::MenuId.eq(id as i32))
            .exec(db)
            .await
            .ok();
        menu_plat::Entity::delete_many()
            .filter(menu_plat::Column::MenuId.eq(id as i32))
            .exec(db)
            .await
            .ok();
        menu_dessert::Entity::delete_many()
            .filter(menu_dessert::Column::MenuId.eq(id as i32))
            .exec(db)
            .await
            .ok();

        for pid in entree_ids {
            menu_entree::ActiveModel {
                menu_id: Set(id as i32),
                entree_id: Set(pid),
                ..Default::default()
            }
            .insert(db)
            .await
            .ok();
        }
        for pid in plat_ids {
            menu_plat::ActiveModel {
                menu_id: Set(id as i32),
                plat_id: Set(pid),
                ..Default::default()
            }
            .insert(db)
            .await
            .ok();
        }
        for pid in dessert_ids {
            menu_dessert::ActiveModel {
                menu_id: Set(id as i32),
                dessert_id: Set(pid),
                ..Default::default()
            }
            .insert(db)
            .await
            .ok();
        }

        let mut active: menu::ActiveModel = menu_model.into();
        active.entree_libre = Set(if entree_libre_txt.is_empty() {
            None
        } else {
            Some(entree_libre_txt)
        });
        active.plat_libre = Set(if plat_libre_txt.is_empty() {
            None
        } else {
            Some(plat_libre_txt)
        });
        active.dessert_libre = Set(if dessert_libre_txt.is_empty() {
            None
        } else {
            Some(dessert_libre_txt)
        });
        active.update(db).await.ok();

        return Ok(
            Redirect::to(&format!("{}/menus/{}/detail", admin.config.prefix, id)).into_response(),
        );
    }

    let entree_ids: Vec<Pk> = search!(menu_entree::Entity => MenuId eq id as i32,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.entree_id as Pk)
        .collect();
    let plat_ids: Vec<Pk> = search!(menu_plat::Entity => MenuId eq id as i32,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.plat_id as Pk)
        .collect();
    let dessert_ids: Vec<Pk> = search!(menu_dessert::Entity => MenuId eq id as i32,)
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|l| l.dessert_id as Pk)
        .collect();

    let entrees_disponibles = search!(entree::Entity => Disponible eq true, Usage ne entree::UsageEntree::Carte, asc Titre,)
        .all(db).await.unwrap_or_default()
        .into_iter()
        .map(|e| PlatDisponible { id: e.id, titre: e.titre })
        .collect();

    let plats_disponibles =
        search!(plat::Entity => Disponible eq true, Usage ne plat::UsagePlat::Carte, asc Titre,)
            .all(db)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|p| PlatDisponible {
                id: p.id,
                titre: p.titre,
            })
            .collect();

    let desserts_disponibles = search!(dessert::Entity => Disponible eq true, Usage ne dessert::UsageDessert::Carte, asc Titre,)
        .all(db).await.unwrap_or_default()
        .into_iter()
        .map(|d| PlatDisponible { id: d.id, titre: d.titre })
        .collect();

    let composition = CompositionMenu {
        entrees_disponibles,
        plats_disponibles,
        desserts_disponibles,
        entree_ids,
        plat_ids,
        dessert_ids,
        entree_libre: menu_model.entree_libre.clone().unwrap_or_default(),
        plat_libre: menu_model.plat_libre.clone().unwrap_or_default(),
        dessert_libre: menu_model.dessert_libre.clone().unwrap_or_default(),
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
        "menu_nom"         => &menu_model.nom,
        "title"            => format!("Composition — {}", menu_model.nom),
        "site_title"       => &admin.config.site_title,
        "current_page"     => "menus",
        "current_resource" => "menus",
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
