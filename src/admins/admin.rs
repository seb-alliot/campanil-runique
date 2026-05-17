// AUTO-admin — DO NOT EDIT MANUALLY
// admin by `runique start` from src/admin.rs

use runique::prelude::*;

use crate::entities::allergene;
use crate::entities::avis;
use crate::entities::avis_plat;
use crate::entities::boisson;
use crate::entities::commande;
use crate::entities::contact;
use crate::entities::devis_traiteur;
use crate::entities::garniture;
use crate::entities::horaire;
use crate::entities::info_resto;
use crate::entities::menu;
use crate::entities::menu_enfant;
use crate::entities::menu_resto;
use crate::entities::menu_resto_plat;
use crate::entities::plat;
use crate::entities::supplement;

// ── DynForm wrapper for allergene::AdminForm ──
struct AllergeneAdminFormDynWrapper(pub allergene::AdminForm);

#[async_trait]
impl DynForm for AllergeneAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for horaire::AdminForm ──
struct HoraireAdminFormDynWrapper(pub crate::formulaire::HorairesGroupeForm);

#[async_trait]
impl DynForm for HoraireAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for devis_traiteur::AdminForm ──
struct DevisTraiteurAdminFormDynWrapper(pub devis_traiteur::AdminForm);

#[async_trait]
impl DynForm for DevisTraiteurAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for contact::AdminForm ──
struct ContactAdminFormDynWrapper(pub contact::AdminForm);

#[async_trait]
impl DynForm for ContactAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for garniture::AdminForm ──
struct GarnitureAdminFormDynWrapper(pub garniture::AdminForm);

#[async_trait]
impl DynForm for GarnitureAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for supplement::AdminForm ──
struct SupplementAdminFormDynWrapper(pub supplement::AdminForm);

#[async_trait]
impl DynForm for SupplementAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for plat::AdminForm ──
struct PlatAdminFormDynWrapper(pub plat::AdminForm);

#[async_trait]
impl DynForm for PlatAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for menu::AdminForm ──
struct MenuAdminFormDynWrapper(pub menu::AdminForm);

#[async_trait]
impl DynForm for MenuAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for menu_resto::AdminForm ──
struct MenuRestoAdminFormDynWrapper(pub menu_resto::AdminForm);

#[async_trait]
impl DynForm for MenuRestoAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for menu_resto_plat::AdminForm ──
struct MenuRestoPlatAdminFormDynWrapper(pub menu_resto_plat::AdminForm);

#[async_trait]
impl DynForm for MenuRestoPlatAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for boisson::AdminForm ──
struct BoissonAdminFormDynWrapper(pub boisson::AdminForm);

#[async_trait]
impl DynForm for BoissonAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for menu_enfant::AdminForm ──
struct MenuEnfantAdminFormDynWrapper(pub menu_enfant::AdminForm);

#[async_trait]
impl DynForm for MenuEnfantAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for commande::AdminForm ──
struct CommandeAdminFormDynWrapper(pub commande::AdminForm);

#[async_trait]
impl DynForm for CommandeAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for avis::AdminForm ──
struct AvisAdminFormDynWrapper(pub avis::AdminForm);

#[async_trait]
impl DynForm for AvisAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for avis_plat::AdminForm ──
struct AvisPlatAdminFormDynWrapper(pub avis_plat::AdminForm);

#[async_trait]
impl DynForm for AvisPlatAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

// ── DynForm wrapper for info_resto::AdminForm ──
struct InfoRestoAdminFormDynWrapper(pub info_resto::AdminForm);

#[async_trait]
impl DynForm for InfoRestoAdminFormDynWrapper {
    async fn is_valid(&mut self) -> bool {
        self.0.is_valid().await
    }

    async fn save(&mut self, db: &DatabaseConnection) -> Result<(), DbErr> {
        self.0.save(db).await
    }

    fn get_form(&self) -> &Forms {
        self.0.get_form()
    }

    fn get_form_mut(&mut self) -> &mut Forms {
        self.0.get_form_mut()
    }
}

/// Builds the admin registry at boot.
/// Called by the application builder.
pub fn admin_register() -> AdminRegistry {
    let mut registry = AdminRegistry::new();
    for entry in runique::admin::builtin_resources() {
        registry.register(entry);
    }

    // ── Resource: allergenes ──
    let meta = AdminResource::new(
        "allergenes",
        "crate::entities::allergene::Model",
        "AdminForm",
        "Allergènes",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = allergene::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(AllergeneAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = allergene::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "libelle", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = allergene::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "libelle", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = allergene::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            allergene::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            allergene::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            allergene::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            allergene::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(DisplayConfig::new().columns_include(vec![("libelle", "Libellé")]));
    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn),
    );

    // ── Resource: horaires ──
    let meta = AdminResource::new(
        "horaires",
        "crate::entities::horaire::Model",
        "AdminForm",
        "Horaires",
        vec![],
    );
    let meta = meta.inject_password(true);
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = crate::formulaire::HorairesGroupeForm::build_with_data(
                    &data, tera, &csrf, method,
                )
                .await;
                Box::new(HoraireAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = horaire::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "jour", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ouverture_midi", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "fermeture_midi", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ouverture_soir", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "fermeture_soir", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ferme", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = horaire::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "jour", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ouverture_midi", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "fermeture_midi", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ouverture_soir", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "fermeture_soir", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ferme", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = horaire::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            horaire::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            let raw = data.get("jour").cloned().unwrap_or_default();
            let values: Vec<&str> = raw
                .split(',')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .collect();
            for val in values {
                let mut row = data.clone();
                row.insert("jour".to_string(), val.to_string());
                horaire::admin_from_form(&row, None).insert(&*db).await?;
            }
            Ok(())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            horaire::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            horaire::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("jour", "Jour"),
                ("ouverture_midi", "Ouverture midi"),
                ("fermeture_midi", "Fermeture midi"),
                ("ouverture_soir", "Ouverture soir"),
                ("fermeture_soir", "Fermeture soir"),
                ("ferme", "Fermé"),
            ])
            .list_filter(vec![("ferme", "Fermé", 10u64)]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_ferme = 10u64;
            let cur_page_ferme = pages.get("ferme").copied().unwrap_or(0);
            let count_stmt_ferme = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT ferme)"))
                .from(Alias::new(horaire::Entity.table_name()))
                .and_where(Expr::col(Alias::new("ferme")).is_not_null())
                .to_owned();
            let count_row_ferme = match db.query_one(&count_stmt_ferme).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `horaires.ferme`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_ferme = count_row_ferme
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_ferme = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(ferme AS TEXT)"))
                .from(Alias::new(horaire::Entity.table_name()))
                .and_where(Expr::col(Alias::new("ferme")).is_not_null())
                .limit(page_size_ferme)
                .offset(cur_page_ferme * page_size_ferme)
                .to_owned();
            let rows_ferme = match db.query_all(&stmt_ferme).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `horaires.ferme`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_ferme: Vec<String> = rows_ferme
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_ferme.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("ferme".to_string(), (vals_ferme, total_ferme));
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("ferme", "Marquer fermé")]),
    );

    // ── Resource: devis_traiteur ──
    let meta = AdminResource::new(
        "devis_traiteur",
        "crate::entities::devis_traiteur::Model",
        "AdminForm",
        "Demandes de devis",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form =
                    devis_traiteur::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(DevisTraiteurAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = devis_traiteur::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nom", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "email", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "date_evenement", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nb_personnes", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = devis_traiteur::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nom", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "email", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "date_evenement", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nb_personnes", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = devis_traiteur::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            devis_traiteur::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            devis_traiteur::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            devis_traiteur::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            devis_traiteur::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("nom", "Nom"),
                ("email", "Email"),
                ("date_evenement", "Date événement"),
                ("nb_personnes", "Personnes"),
                ("statut", "Statut"),
                ("created_at", "Reçu le"),
            ])
            .list_filter(vec![("statut", "Statut", 10u64)]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_statut = 10u64;
            let cur_page_statut = pages.get("statut").copied().unwrap_or(0);
            let count_stmt_statut = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT statut)"))
                .from(Alias::new(devis_traiteur::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .to_owned();
            let count_row_statut = match db.query_one(&count_stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `devis_traiteur.statut`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_statut = count_row_statut
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_statut = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(statut AS TEXT)"))
                .from(Alias::new(devis_traiteur::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .limit(page_size_statut)
                .offset(cur_page_statut * page_size_statut)
                .to_owned();
            let rows_statut = match db.query_all(&stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `devis_traiteur.statut`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_statut: Vec<String> = rows_statut
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_statut.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("statut".to_string(), (vals_statut, total_statut));
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("statut", "Marquer en cours")]),
    );

    // ── Resource: contacts ──
    let meta = AdminResource::new(
        "contacts",
        "crate::entities::contact::Model",
        "AdminForm",
        "Messages de contact",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = contact::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(ContactAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = contact::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "email", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = contact::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "email", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = contact::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            contact::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            contact::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            contact::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            contact::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(DisplayConfig::new().columns_include(vec![
        ("titre", "Sujet"),
        ("email", "Email"),
        ("created_at", "Reçu le"),
    ]));
    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn),
    );

    // ── Resource: garnitures ──
    let meta = AdminResource::new(
        "garnitures",
        "crate::entities::garniture::Model",
        "AdminForm",
        "Garnitures",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = garniture::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(GarnitureAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = garniture::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "libelle", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_garniture", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = garniture::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "libelle", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_garniture", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = garniture::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            garniture::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            garniture::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            garniture::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            garniture::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("libelle", "Libellé"),
                ("type_garniture", "Type"),
                ("disponible", "Disponible"),
            ])
            .list_filter(vec![
                ("type_garniture", "Type", 10u64),
                ("disponible", "Disponibilité", 10u64),
            ]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_type_garniture = 10u64;
            let cur_page_type_garniture = pages.get("type_garniture").copied().unwrap_or(0);
            let count_stmt_type_garniture = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT type_garniture)"))
                .from(Alias::new(garniture::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_garniture")).is_not_null())
                .to_owned();
            let count_row_type_garniture = match db.query_one(&count_stmt_type_garniture).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `garnitures.type_garniture`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_type_garniture = count_row_type_garniture
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_type_garniture = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(type_garniture AS TEXT)"))
                .from(Alias::new(garniture::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_garniture")).is_not_null())
                .limit(page_size_type_garniture)
                .offset(cur_page_type_garniture * page_size_type_garniture)
                .to_owned();
            let rows_type_garniture = match db.query_all(&stmt_type_garniture).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `garnitures.type_garniture`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_type_garniture: Vec<String> = rows_type_garniture
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_type_garniture.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "type_garniture".to_string(),
                (vals_type_garniture, total_type_garniture),
            );
            let page_size_disponible = 10u64;
            let cur_page_disponible = pages.get("disponible").copied().unwrap_or(0);
            let count_stmt_disponible = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT disponible)"))
                .from(Alias::new(garniture::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .to_owned();
            let count_row_disponible = match db.query_one(&count_stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `garnitures.disponible`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_disponible = count_row_disponible
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_disponible = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(disponible AS TEXT)"))
                .from(Alias::new(garniture::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .limit(page_size_disponible)
                .offset(cur_page_disponible * page_size_disponible)
                .to_owned();
            let rows_disponible = match db.query_all(&stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `garnitures.disponible`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_disponible: Vec<String> = rows_disponible
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_disponible.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "disponible".to_string(),
                (vals_disponible, total_disponible),
            );
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("disponible", "Rendre disponible")]),
    );

    // ── Resource: supplements ──
    let meta = AdminResource::new(
        "supplements",
        "crate::entities::supplement::Model",
        "AdminForm",
        "Suppléments",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let mut form =
                    supplement::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                {
                    use sea_orm::ConnectionTrait;
                    let _fk_opt_stmt_garniture_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("libelle"))
                        .from(sea_orm::sea_query::Alias::new("garnitures"))
                        .to_owned();
                    let _fk_opt_choices_garniture_id: Vec<(String, String)> = db
                        .query_all(&_fk_opt_stmt_garniture_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let lbl = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, lbl))
                        })
                        .collect();
                    form.get_form_mut().field_choices(
                        "garniture_id",
                        "Garniture",
                        _fk_opt_choices_garniture_id,
                    );
                }
                Box::new(SupplementAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = supplement::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let mut rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            {
                use sea_orm::ConnectionTrait;
                let fk_ids: Vec<String> = rows
                    .iter()
                    .filter_map(|r| {
                        r.get("garniture_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        })
                    })
                    .collect::<std::collections::HashSet<String>>()
                    .into_iter()
                    .collect();
                if !fk_ids.is_empty() {
                    let ids_csv = fk_ids
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect::<Vec<_>>()
                        .join(",");
                    let _fk_stmt_garniture_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("libelle"))
                        .from(sea_orm::sea_query::Alias::new("garnitures"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    let label_map_garniture_id: std::collections::HashMap<String, String> = db
                        .query_all(&_fk_stmt_garniture_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let label = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, label))
                        })
                        .collect();
                    for row in &mut rows {
                        if let Some(key) = row.get("garniture_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        }) && let Some(label) = label_map_garniture_id.get(&key)
                        {
                            row["garniture_id"] = serde_json::Value::String(label.clone());
                        }
                    }
                }
            }
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = supplement::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = supplement::Entity::find_by_id(id).one(&*db).await?;
            let Some(model) = row else {
                return Ok(None);
            };
            let mut row = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
            {
                use sea_orm::ConnectionTrait;
                if let Some(fk_key) = row.get("garniture_id").and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                }) {
                    let ids_csv = format!("'{}'", fk_key.replace('\'', "''"));
                    let _fk_stmt_garniture_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("libelle"))
                        .from(sea_orm::sea_query::Alias::new("garnitures"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    if let Some(fk_row) = db.query_one(&_fk_stmt_garniture_id).await.ok().flatten()
                        && let Ok(label) = fk_row.try_get_by_index::<String>(1)
                    {
                        row["garniture_id"] = serde_json::Value::String(label);
                    }
                }
            }
            Ok(Some(row))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            supplement::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            supplement::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            supplement::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            supplement::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("titre", "Nom"),
                ("garniture_id", "Garniture"),
                ("prix", "Prix"),
                ("disponible", "Disponible"),
            ])
            .list_filter(vec![("disponible", "Disponible", 10u64)]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_disponible = 10u64;
            let cur_page_disponible = pages.get("disponible").copied().unwrap_or(0);
            let count_stmt_disponible = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT disponible)"))
                .from(Alias::new(supplement::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .to_owned();
            let count_row_disponible = match db.query_one(&count_stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `supplements.disponible`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_disponible = count_row_disponible
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_disponible = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(disponible AS TEXT)"))
                .from(Alias::new(supplement::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .limit(page_size_disponible)
                .offset(cur_page_disponible * page_size_disponible)
                .to_owned();
            let rows_disponible = match db.query_all(&stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `supplements.disponible`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_disponible: Vec<String> = rows_disponible
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_disponible.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "disponible".to_string(),
                (vals_disponible, total_disponible),
            );
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("disponible", "Rendre disponible")]),
    );

    // ── Resource: plats ──
    let meta = AdminResource::new(
        "plats",
        "crate::entities::plat::Model",
        "AdminForm",
        "Plats",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = plat::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(PlatAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = plat::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_plat", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "est_viande", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "avec_legumes", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = plat::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_plat", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "est_viande", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "avec_legumes", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = plat::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            plat::Entity::delete_by_id(id).exec(&*db).await.map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            use sea_orm::ConnectionTrait;
            let result = plat::admin_from_form(&data, None).insert(&*db).await?;
            let inserted_id = result.id.to_string();
            for key in data.keys() {
                if let Some(target_id) = key.strip_prefix("m2m_allergenes__")
                    && !target_id.is_empty()
                    && target_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
                {
                    let sql = format!(
                        "INSERT INTO plat_allergene (plat_id, allergene_id) VALUES ({}, {}) ON CONFLICT DO NOTHING",
                        inserted_id, target_id
                    );
                    let _ = db.execute_unprepared(&sql).await;
                }
            }
            for key in data.keys() {
                if let Some(target_id) = key.strip_prefix("m2m_garnitures__")
                    && !target_id.is_empty()
                    && target_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
                {
                    let sql = format!(
                        "INSERT INTO plat_garnitures (plat_id, garniture_id) VALUES ({}, {}) ON CONFLICT DO NOTHING",
                        inserted_id, target_id
                    );
                    let _ = db.execute_unprepared(&sql).await;
                }
            }

            Ok(())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            use sea_orm::ConnectionTrait;
            let id_str = id.to_string();
            plat::admin_from_form(&data, Some(id)).update(&*db).await?;
            let _ = db
                .execute_unprepared(&format!(
                    "DELETE FROM plat_allergene WHERE plat_id = {}",
                    id_str
                ))
                .await;
            for key in data.keys() {
                if let Some(target_id) = key.strip_prefix("m2m_allergenes__")
                    && !target_id.is_empty()
                    && target_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
                {
                    let sql = format!(
                        "INSERT INTO plat_allergene (plat_id, allergene_id) VALUES ({}, {})",
                        id_str, target_id
                    );
                    let _ = db.execute_unprepared(&sql).await;
                }
            }
            let _ = db
                .execute_unprepared(&format!(
                    "DELETE FROM plat_garnitures WHERE plat_id = {}",
                    id_str
                ))
                .await;
            for key in data.keys() {
                if let Some(target_id) = key.strip_prefix("m2m_garnitures__")
                    && !target_id.is_empty()
                    && target_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
                {
                    let sql = format!(
                        "INSERT INTO plat_garnitures (plat_id, garniture_id) VALUES ({}, {})",
                        id_str, target_id
                    );
                    let _ = db.execute_unprepared(&sql).await;
                }
            }
            Ok(())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            plat::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let m2m_loader: M2mLoaderFn = Arc::new(|db: ADb, object_id: Option<String>| {
        Box::pin(async move {
            use sea_orm::{ConnectionTrait, EntityTrait};
            let mut fields: Vec<M2mFieldOptions> = Vec::new();
            {
                let rows = crate::entities::allergene::Entity::find()
                    .all(&*db)
                    .await
                    .unwrap_or_default();
                let choices: Vec<(String, String)> = rows
                    .iter()
                    .map(|r| {
                        let v = serde_json::to_value(r).unwrap_or_default();
                        let id = v
                            .get("id")
                            .map(|i| i.to_string().trim_matches('"').to_string())
                            .unwrap_or_default();
                        let label = v
                            .get("libelle")
                            .and_then(|n| n.as_str())
                            .unwrap_or("")
                            .to_string();
                        (id, label)
                    })
                    .collect();
                let selected = if let Some(ref oid) = object_id {
                    use sea_orm::sea_query::{Alias, Expr, Query};
                    let stmt = Query::select()
                        .expr(Expr::cust("CAST(allergene_id AS TEXT)"))
                        .from(Alias::new("plat_allergene"))
                        .and_where(Expr::cust(format!("CAST(plat_id AS TEXT) = '{}'", oid)))
                        .to_owned();
                    db.query_all(&stmt)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                        .collect()
                } else {
                    vec![]
                };
                fields.push(M2mFieldOptions {
                    field_name: "allergenes".to_string(),
                    label: "Allergènes".to_string(),
                    choices,
                    selected,
                });
            }
            {
                let rows = crate::entities::garniture::Entity::find()
                    .all(&*db)
                    .await
                    .unwrap_or_default();
                let choices: Vec<(String, String)> = rows
                    .iter()
                    .map(|r| {
                        let v = serde_json::to_value(r).unwrap_or_default();
                        let id = v
                            .get("id")
                            .map(|i| i.to_string().trim_matches('"').to_string())
                            .unwrap_or_default();
                        let label = v
                            .get("libelle")
                            .and_then(|n| n.as_str())
                            .unwrap_or("")
                            .to_string();
                        (id, label)
                    })
                    .collect();
                let selected = if let Some(ref oid) = object_id {
                    use sea_orm::sea_query::{Alias, Expr, Query};
                    let stmt = Query::select()
                        .expr(Expr::cust("CAST(garniture_id AS TEXT)"))
                        .from(Alias::new("plat_garnitures"))
                        .and_where(Expr::cust(format!("CAST(plat_id AS TEXT) = '{}'", oid)))
                        .to_owned();
                    db.query_all(&stmt)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                        .collect()
                } else {
                    vec![]
                };
                fields.push(M2mFieldOptions {
                    field_name: "garnitures".to_string(),
                    label: "Garnitures".to_string(),
                    choices,
                    selected,
                });
            }
            fields
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("titre", "Titre"),
                ("type_plat", "Type"),
                ("prix", "Prix"),
                ("disponible", "Disponible"),
                ("est_viande", "Viande"),
                ("avec_legumes", "Légumes inclus"),
            ])
            .list_filter(vec![
                ("type_plat", "Type de plat", 10u64),
                ("disponible", "Disponibilité", 10u64),
                ("est_viande", "Viande", 10u64),
                ("avec_legumes", "Légumes", 10u64),
            ]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_type_plat = 10u64;
            let cur_page_type_plat = pages.get("type_plat").copied().unwrap_or(0);
            let count_stmt_type_plat = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT type_plat)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_plat")).is_not_null())
                .to_owned();
            let count_row_type_plat = match db.query_one(&count_stmt_type_plat).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.type_plat`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_type_plat = count_row_type_plat
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_type_plat = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(type_plat AS TEXT)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_plat")).is_not_null())
                .limit(page_size_type_plat)
                .offset(cur_page_type_plat * page_size_type_plat)
                .to_owned();
            let rows_type_plat = match db.query_all(&stmt_type_plat).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.type_plat`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_type_plat: Vec<String> = rows_type_plat
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_type_plat.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("type_plat".to_string(), (vals_type_plat, total_type_plat));
            let page_size_disponible = 10u64;
            let cur_page_disponible = pages.get("disponible").copied().unwrap_or(0);
            let count_stmt_disponible = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT disponible)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .to_owned();
            let count_row_disponible = match db.query_one(&count_stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.disponible`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_disponible = count_row_disponible
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_disponible = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(disponible AS TEXT)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .limit(page_size_disponible)
                .offset(cur_page_disponible * page_size_disponible)
                .to_owned();
            let rows_disponible = match db.query_all(&stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.disponible`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_disponible: Vec<String> = rows_disponible
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_disponible.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "disponible".to_string(),
                (vals_disponible, total_disponible),
            );
            let page_size_est_viande = 10u64;
            let cur_page_est_viande = pages.get("est_viande").copied().unwrap_or(0);
            let count_stmt_est_viande = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT est_viande)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("est_viande")).is_not_null())
                .to_owned();
            let count_row_est_viande = match db.query_one(&count_stmt_est_viande).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.est_viande`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_est_viande = count_row_est_viande
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_est_viande = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(est_viande AS TEXT)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("est_viande")).is_not_null())
                .limit(page_size_est_viande)
                .offset(cur_page_est_viande * page_size_est_viande)
                .to_owned();
            let rows_est_viande = match db.query_all(&stmt_est_viande).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.est_viande`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_est_viande: Vec<String> = rows_est_viande
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_est_viande.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "est_viande".to_string(),
                (vals_est_viande, total_est_viande),
            );
            let page_size_avec_legumes = 10u64;
            let cur_page_avec_legumes = pages.get("avec_legumes").copied().unwrap_or(0);
            let count_stmt_avec_legumes = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT avec_legumes)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("avec_legumes")).is_not_null())
                .to_owned();
            let count_row_avec_legumes = match db.query_one(&count_stmt_avec_legumes).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.avec_legumes`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_avec_legumes = count_row_avec_legumes
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_avec_legumes = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(avec_legumes AS TEXT)"))
                .from(Alias::new(plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("avec_legumes")).is_not_null())
                .limit(page_size_avec_legumes)
                .offset(cur_page_avec_legumes * page_size_avec_legumes)
                .to_owned();
            let rows_avec_legumes = match db.query_all(&stmt_avec_legumes).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `plats.avec_legumes`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_avec_legumes: Vec<String> = rows_avec_legumes
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_avec_legumes.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "avec_legumes".to_string(),
                (vals_avec_legumes, total_avec_legumes),
            );
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("disponible", "Rendre disponible")])
            .with_m2m_loader(m2m_loader),
    );

    // ── Resource: menus ──
    let meta = AdminResource::new(
        "menus",
        "crate::entities::menu::Model",
        "AdminForm",
        "Menus traiteur",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = menu::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(MenuAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = menu::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "theme", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "regime", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix_par_personne", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nb_personnes_min", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "actif", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = menu::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "theme", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "regime", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix_par_personne", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nb_personnes_min", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "actif", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = menu::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu::Entity::delete_by_id(id).exec(&*db).await.map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            use sea_orm::ConnectionTrait;
            let result = menu::admin_from_form(&data, None).insert(&*db).await?;
            let inserted_id = result.id.to_string();
            for key in data.keys() {
                if let Some(target_id) = key.strip_prefix("m2m_plats__")
                    && !target_id.is_empty()
                    && target_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
                {
                    let sql = format!(
                        "INSERT INTO menu_plat (menu_id, plat_id) VALUES ({}, {}) ON CONFLICT DO NOTHING",
                        inserted_id, target_id
                    );
                    let _ = db.execute_unprepared(&sql).await;
                }
            }
            Ok(())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            use sea_orm::ConnectionTrait;
            let id_str = id.to_string();
            menu::admin_from_form(&data, Some(id)).update(&*db).await?;
            let _ = db
                .execute_unprepared(&format!("DELETE FROM menu_plat WHERE menu_id = {}", id_str))
                .await;
            for key in data.keys() {
                if let Some(target_id) = key.strip_prefix("m2m_plats__")
                    && !target_id.is_empty()
                    && target_id
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-')
                {
                    let sql = format!(
                        "INSERT INTO menu_plat (menu_id, plat_id) VALUES ({}, {})",
                        id_str, target_id
                    );
                    let _ = db.execute_unprepared(&sql).await;
                }
            }
            Ok(())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let m2m_loader: M2mLoaderFn = Arc::new(|db: ADb, object_id: Option<String>| {
        Box::pin(async move {
            use sea_orm::{ConnectionTrait, EntityTrait};
            let mut fields: Vec<M2mFieldOptions> = Vec::new();
            {
                let rows = crate::entities::plat::Entity::find()
                    .all(&*db)
                    .await
                    .unwrap_or_default();
                let choices: Vec<(String, String)> = rows
                    .iter()
                    .map(|r| {
                        let v = serde_json::to_value(r).unwrap_or_default();
                        let id = v
                            .get("id")
                            .map(|i| i.to_string().trim_matches('"').to_string())
                            .unwrap_or_default();
                        let label = v
                            .get("titre")
                            .and_then(|n| n.as_str())
                            .unwrap_or("")
                            .to_string();
                        (id, label)
                    })
                    .collect();
                let selected = if let Some(ref oid) = object_id {
                    use sea_orm::sea_query::{Alias, Expr, Query};
                    let stmt = Query::select()
                        .expr(Expr::cust("CAST(plat_id AS TEXT)"))
                        .from(Alias::new("menu_plat"))
                        .and_where(Expr::cust(format!("CAST(menu_id AS TEXT) = '{}'", oid)))
                        .to_owned();
                    db.query_all(&stmt)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                        .collect()
                } else {
                    vec![]
                };
                fields.push(M2mFieldOptions {
                    field_name: "plats".to_string(),
                    label: "Plats".to_string(),
                    choices,
                    selected,
                });
            }
            fields
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("titre", "Titre"),
                ("theme", "Thème"),
                ("regime", "Régime"),
                ("prix_par_personne", "Prix/pers."),
                ("nb_personnes_min", "Min. pers."),
                ("actif", "Actif"),
            ])
            .list_filter(vec![
                ("actif", "Actif", 10u64),
                ("theme", "Thème", 10u64),
                ("regime", "Régime", 10u64),
            ]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_actif = 10u64;
            let cur_page_actif = pages.get("actif").copied().unwrap_or(0);
            let count_stmt_actif = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT actif)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("actif")).is_not_null())
                .to_owned();
            let count_row_actif = match db.query_one(&count_stmt_actif).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.actif`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_actif = count_row_actif
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_actif = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(actif AS TEXT)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("actif")).is_not_null())
                .limit(page_size_actif)
                .offset(cur_page_actif * page_size_actif)
                .to_owned();
            let rows_actif = match db.query_all(&stmt_actif).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.actif`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_actif: Vec<String> = rows_actif
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_actif.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("actif".to_string(), (vals_actif, total_actif));
            let page_size_theme = 10u64;
            let cur_page_theme = pages.get("theme").copied().unwrap_or(0);
            let count_stmt_theme = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT theme)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("theme")).is_not_null())
                .to_owned();
            let count_row_theme = match db.query_one(&count_stmt_theme).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.theme`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_theme = count_row_theme
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_theme = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(theme AS TEXT)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("theme")).is_not_null())
                .limit(page_size_theme)
                .offset(cur_page_theme * page_size_theme)
                .to_owned();
            let rows_theme = match db.query_all(&stmt_theme).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.theme`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_theme: Vec<String> = rows_theme
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_theme.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("theme".to_string(), (vals_theme, total_theme));
            let page_size_regime = 10u64;
            let cur_page_regime = pages.get("regime").copied().unwrap_or(0);
            let count_stmt_regime = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT regime)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("regime")).is_not_null())
                .to_owned();
            let count_row_regime = match db.query_one(&count_stmt_regime).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.regime`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_regime = count_row_regime
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_regime = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(regime AS TEXT)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("regime")).is_not_null())
                .limit(page_size_regime)
                .offset(cur_page_regime * page_size_regime)
                .to_owned();
            let rows_regime = match db.query_all(&stmt_regime).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.regime`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_regime: Vec<String> = rows_regime
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_regime.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("regime".to_string(), (vals_regime, total_regime));
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("actif", "Activer")])
            .with_m2m_loader(m2m_loader),
    );

    // ── Resource: menus_resto ──
    let meta = AdminResource::new(
        "menus_resto",
        "crate::entities::menu_resto::Model",
        "AdminForm",
        "Menus restaurant",
        vec![],
    );
    let meta = meta.template_detail("admin/menu_resto_detail.html");
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = menu_resto::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(MenuRestoAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = menu_resto::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nom", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = menu_resto::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nom", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = menu_resto::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_resto::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            menu_resto::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_resto::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_resto::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(DisplayConfig::new().columns_include(vec![
        ("nom", "Nom"),
        ("prix", "Prix"),
        ("disponible", "Disponible"),
    ]));
    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_group_actions(vec![GroupAction::bool("disponible", "Rendre disponible")]),
    );

    // ── Resource: menus_resto_composition ──
    let meta = AdminResource::new(
        "menus_resto_composition",
        "crate::entities::menu_resto_plat::Model",
        "AdminForm",
        "Composition des menus restaurant",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let mut form =
                    menu_resto_plat::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                {
                    use sea_orm::ConnectionTrait;
                    let _fk_opt_stmt_menu_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("nom"))
                        .from(sea_orm::sea_query::Alias::new("menus_resto"))
                        .to_owned();
                    let _fk_opt_choices_menu_id: Vec<(String, String)> = db
                        .query_all(&_fk_opt_stmt_menu_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let lbl = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, lbl))
                        })
                        .collect();
                    form.get_form_mut()
                        .field_choices("menu_id", "Menu", _fk_opt_choices_menu_id);
                }
                {
                    use sea_orm::ConnectionTrait;
                    let _fk_opt_stmt_plat_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("titre"))
                        .from(sea_orm::sea_query::Alias::new("plats"))
                        .to_owned();
                    let _fk_opt_choices_plat_id: Vec<(String, String)> = db
                        .query_all(&_fk_opt_stmt_plat_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let lbl = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, lbl))
                        })
                        .collect();
                    form.get_form_mut()
                        .field_choices("plat_id", "Plat", _fk_opt_choices_plat_id);
                }
                Box::new(MenuRestoPlatAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = menu_resto_plat::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "cours", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let mut rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            {
                use sea_orm::ConnectionTrait;
                let fk_ids: Vec<String> = rows
                    .iter()
                    .filter_map(|r| {
                        r.get("menu_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        })
                    })
                    .collect::<std::collections::HashSet<String>>()
                    .into_iter()
                    .collect();
                if !fk_ids.is_empty() {
                    let ids_csv = fk_ids
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect::<Vec<_>>()
                        .join(",");
                    let _fk_stmt_menu_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("nom"))
                        .from(sea_orm::sea_query::Alias::new("menus_resto"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    let label_map_menu_id: std::collections::HashMap<String, String> = db
                        .query_all(&_fk_stmt_menu_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let label = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, label))
                        })
                        .collect();
                    for row in &mut rows {
                        if let Some(key) = row.get("menu_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        }) && let Some(label) = label_map_menu_id.get(&key)
                        {
                            row["menu_id"] = serde_json::Value::String(label.clone());
                        }
                    }
                }
            }
            {
                use sea_orm::ConnectionTrait;
                let fk_ids: Vec<String> = rows
                    .iter()
                    .filter_map(|r| {
                        r.get("plat_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        })
                    })
                    .collect::<std::collections::HashSet<String>>()
                    .into_iter()
                    .collect();
                if !fk_ids.is_empty() {
                    let ids_csv = fk_ids
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect::<Vec<_>>()
                        .join(",");
                    let _fk_stmt_plat_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("titre"))
                        .from(sea_orm::sea_query::Alias::new("plats"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    let label_map_plat_id: std::collections::HashMap<String, String> = db
                        .query_all(&_fk_stmt_plat_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let label = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, label))
                        })
                        .collect();
                    for row in &mut rows {
                        if let Some(key) = row.get("plat_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        }) && let Some(label) = label_map_plat_id.get(&key)
                        {
                            row["plat_id"] = serde_json::Value::String(label.clone());
                        }
                    }
                }
            }
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = menu_resto_plat::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "cours", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = menu_resto_plat::Entity::find_by_id(id).one(&*db).await?;
            let Some(model) = row else {
                return Ok(None);
            };
            let mut row = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
            {
                use sea_orm::ConnectionTrait;
                if let Some(fk_key) = row.get("menu_id").and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                }) {
                    let ids_csv = format!("'{}'", fk_key.replace('\'', "''"));
                    let _fk_stmt_menu_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("nom"))
                        .from(sea_orm::sea_query::Alias::new("menus_resto"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    if let Some(fk_row) = db.query_one(&_fk_stmt_menu_id).await.ok().flatten()
                        && let Ok(label) = fk_row.try_get_by_index::<String>(1)
                    {
                        row["menu_id"] = serde_json::Value::String(label);
                    }
                }
            }
            {
                use sea_orm::ConnectionTrait;
                if let Some(fk_key) = row.get("plat_id").and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                }) {
                    let ids_csv = format!("'{}'", fk_key.replace('\'', "''"));
                    let _fk_stmt_plat_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("titre"))
                        .from(sea_orm::sea_query::Alias::new("plats"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    if let Some(fk_row) = db.query_one(&_fk_stmt_plat_id).await.ok().flatten()
                        && let Ok(label) = fk_row.try_get_by_index::<String>(1)
                    {
                        row["plat_id"] = serde_json::Value::String(label);
                    }
                }
            }
            Ok(Some(row))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_resto_plat::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            menu_resto_plat::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_resto_plat::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_resto_plat::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("menu_id", "Menu"),
                ("plat_id", "Plat"),
                ("cours", "Cours"),
            ])
            .list_filter(vec![("cours", "Cours", 10u64)]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_cours = 10u64;
            let cur_page_cours = pages.get("cours").copied().unwrap_or(0);
            let count_stmt_cours = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT cours)"))
                .from(Alias::new(menu_resto_plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("cours")).is_not_null())
                .to_owned();
            let count_row_cours = match db.query_one(&count_stmt_cours).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus_resto_composition.cours`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_cours = count_row_cours
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_cours = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(cours AS TEXT)"))
                .from(Alias::new(menu_resto_plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("cours")).is_not_null())
                .limit(page_size_cours)
                .offset(cur_page_cours * page_size_cours)
                .to_owned();
            let rows_cours = match db.query_all(&stmt_cours).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus_resto_composition.cours`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_cours: Vec<String> = rows_cours
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_cours.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("cours".to_string(), (vals_cours, total_cours));
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn),
    );

    // ── Resource: boissons ──
    let meta = AdminResource::new(
        "boissons",
        "crate::entities::boisson::Model",
        "AdminForm",
        "Boissons",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = boisson::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(BoissonAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = boisson::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_boisson", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = boisson::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_boisson", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "disponible", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = boisson::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            boisson::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            boisson::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            boisson::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            boisson::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("titre", "Titre"),
                ("type_boisson", "Type"),
                ("prix", "Prix"),
                ("disponible", "Disponible"),
            ])
            .list_filter(vec![
                ("type_boisson", "Type", 10u64),
                ("disponible", "Disponibilité", 10u64),
            ]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_type_boisson = 10u64;
            let cur_page_type_boisson = pages.get("type_boisson").copied().unwrap_or(0);
            let count_stmt_type_boisson = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT type_boisson)"))
                .from(Alias::new(boisson::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_boisson")).is_not_null())
                .to_owned();
            let count_row_type_boisson = match db.query_one(&count_stmt_type_boisson).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `boissons.type_boisson`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_type_boisson = count_row_type_boisson
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_type_boisson = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(type_boisson AS TEXT)"))
                .from(Alias::new(boisson::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_boisson")).is_not_null())
                .limit(page_size_type_boisson)
                .offset(cur_page_type_boisson * page_size_type_boisson)
                .to_owned();
            let rows_type_boisson = match db.query_all(&stmt_type_boisson).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `boissons.type_boisson`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_type_boisson: Vec<String> = rows_type_boisson
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_type_boisson.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "type_boisson".to_string(),
                (vals_type_boisson, total_type_boisson),
            );
            let page_size_disponible = 10u64;
            let cur_page_disponible = pages.get("disponible").copied().unwrap_or(0);
            let count_stmt_disponible = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT disponible)"))
                .from(Alias::new(boisson::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .to_owned();
            let count_row_disponible = match db.query_one(&count_stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `boissons.disponible`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_disponible = count_row_disponible
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_disponible = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(disponible AS TEXT)"))
                .from(Alias::new(boisson::Entity.table_name()))
                .and_where(Expr::col(Alias::new("disponible")).is_not_null())
                .limit(page_size_disponible)
                .offset(cur_page_disponible * page_size_disponible)
                .to_owned();
            let rows_disponible = match db.query_all(&stmt_disponible).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `boissons.disponible`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_disponible: Vec<String> = rows_disponible
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_disponible.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "disponible".to_string(),
                (vals_disponible, total_disponible),
            );
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![GroupAction::bool("disponible", "Rendre disponible")]),
    );

    // ── Resource: menu_enfants ──
    let meta = AdminResource::new(
        "menu_enfants",
        "crate::entities::menu_enfant::Model",
        "AdminForm",
        "Menus enfant",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form =
                    menu_enfant::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(MenuEnfantAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = menu_enfant::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "actif", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = menu_enfant::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "titre", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "actif", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = menu_enfant::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_enfant::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            menu_enfant::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_enfant::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            menu_enfant::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(DisplayConfig::new().columns_include(vec![
        ("titre", "Titre"),
        ("prix", "Prix"),
        ("actif", "Actif"),
    ]));
    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_group_actions(vec![GroupAction::bool("actif", "Activer")]),
    );

    // ── Resource: commandes ──
    let meta = AdminResource::new(
        "commandes",
        "crate::entities::commande::Model",
        "AdminForm",
        "Commandes",
        vec![],
    );
    let meta = meta.template_detail("admin/commande_detail.html");
    let form_builder: FormBuilder = Arc::new(
        |db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let mut form =
                    commande::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                {
                    use sea_orm::ConnectionTrait;
                    let _fk_opt_stmt_user_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("username"))
                        .from(sea_orm::sea_query::Alias::new("eihwaz_users"))
                        .to_owned();
                    let _fk_opt_choices_user_id: Vec<(String, String)> = db
                        .query_all(&_fk_opt_stmt_user_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let lbl = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, lbl))
                        })
                        .collect();
                    form.get_form_mut()
                        .field_choices("user_id", "Client", _fk_opt_choices_user_id);
                }
                Box::new(CommandeAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = commande::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "numero", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_retrait", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "mode_paiement", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix_total", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let mut rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            {
                use sea_orm::ConnectionTrait;
                let fk_ids: Vec<String> = rows
                    .iter()
                    .filter_map(|r| {
                        r.get("user_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        })
                    })
                    .collect::<std::collections::HashSet<String>>()
                    .into_iter()
                    .collect();
                if !fk_ids.is_empty() {
                    let ids_csv = fk_ids
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect::<Vec<_>>()
                        .join(",");
                    let _fk_stmt_user_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("username"))
                        .from(sea_orm::sea_query::Alias::new("eihwaz_users"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    let label_map_user_id: std::collections::HashMap<String, String> = db
                        .query_all(&_fk_stmt_user_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let label = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, label))
                        })
                        .collect();
                    for row in &mut rows {
                        if let Some(key) = row.get("user_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        }) && let Some(label) = label_map_user_id.get(&key)
                        {
                            row["user_id"] = serde_json::Value::String(label.clone());
                        }
                    }
                }
            }
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = commande::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "numero", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "type_retrait", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "mode_paiement", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "prix_total", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = commande::Entity::find_by_id(id).one(&*db).await?;
            let Some(model) = row else {
                return Ok(None);
            };
            let mut row = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
            {
                use sea_orm::ConnectionTrait;
                if let Some(fk_key) = row.get("user_id").and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                }) {
                    let ids_csv = format!("'{}'", fk_key.replace('\'', "''"));
                    let _fk_stmt_user_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("username"))
                        .from(sea_orm::sea_query::Alias::new("eihwaz_users"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    if let Some(fk_row) = db.query_one(&_fk_stmt_user_id).await.ok().flatten()
                        && let Ok(label) = fk_row.try_get_by_index::<String>(1)
                    {
                        row["user_id"] = serde_json::Value::String(label);
                    }
                }
            }
            Ok(Some(row))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            commande::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            commande::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            commande::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            commande::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("numero", "N°"),
                ("user_id", "Client"),
                ("type_retrait", "Type"),
                ("statut", "Statut"),
                ("mode_paiement", "Paiement"),
                ("prix_total", "Total"),
                ("created_at", "Date"),
            ])
            .list_filter(vec![
                ("type_retrait", "Type", 10u64),
                ("statut", "Statut", 10u64),
                ("mode_paiement", "Paiement", 10u64),
            ]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_type_retrait = 10u64;
            let cur_page_type_retrait = pages.get("type_retrait").copied().unwrap_or(0);
            let count_stmt_type_retrait = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT type_retrait)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_retrait")).is_not_null())
                .to_owned();
            let count_row_type_retrait = match db.query_one(&count_stmt_type_retrait).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.type_retrait`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_type_retrait = count_row_type_retrait
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_type_retrait = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(type_retrait AS TEXT)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_retrait")).is_not_null())
                .limit(page_size_type_retrait)
                .offset(cur_page_type_retrait * page_size_type_retrait)
                .to_owned();
            let rows_type_retrait = match db.query_all(&stmt_type_retrait).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.type_retrait`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_type_retrait: Vec<String> = rows_type_retrait
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_type_retrait.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "type_retrait".to_string(),
                (vals_type_retrait, total_type_retrait),
            );
            let page_size_statut = 10u64;
            let cur_page_statut = pages.get("statut").copied().unwrap_or(0);
            let count_stmt_statut = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT statut)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .to_owned();
            let count_row_statut = match db.query_one(&count_stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.statut`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_statut = count_row_statut
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_statut = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(statut AS TEXT)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .limit(page_size_statut)
                .offset(cur_page_statut * page_size_statut)
                .to_owned();
            let rows_statut = match db.query_all(&stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.statut`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_statut: Vec<String> = rows_statut
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_statut.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("statut".to_string(), (vals_statut, total_statut));
            let page_size_mode_paiement = 10u64;
            let cur_page_mode_paiement = pages.get("mode_paiement").copied().unwrap_or(0);
            let count_stmt_mode_paiement = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT mode_paiement)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("mode_paiement")).is_not_null())
                .to_owned();
            let count_row_mode_paiement = match db.query_one(&count_stmt_mode_paiement).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.mode_paiement`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_mode_paiement = count_row_mode_paiement
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_mode_paiement = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(mode_paiement AS TEXT)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("mode_paiement")).is_not_null())
                .limit(page_size_mode_paiement)
                .offset(cur_page_mode_paiement * page_size_mode_paiement)
                .to_owned();
            let rows_mode_paiement = match db.query_all(&stmt_mode_paiement).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.mode_paiement`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_mode_paiement: Vec<String> = rows_mode_paiement
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_mode_paiement.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "mode_paiement".to_string(),
                (vals_mode_paiement, total_mode_paiement),
            );
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn),
    );

    // ── Resource: avis ──
    let meta = AdminResource::new(
        "avis",
        "crate::entities::avis::Model",
        "AdminForm",
        "Avis clients",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let mut form = avis::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                {
                    use sea_orm::ConnectionTrait;
                    let _fk_opt_stmt_commande_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("numero"))
                        .from(sea_orm::sea_query::Alias::new("commandes"))
                        .to_owned();
                    let _fk_opt_choices_commande_id: Vec<(String, String)> = db
                        .query_all(&_fk_opt_stmt_commande_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let lbl = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, lbl))
                        })
                        .collect();
                    form.get_form_mut().field_choices(
                        "commande_id",
                        "Commande",
                        _fk_opt_choices_commande_id,
                    );
                }
                Box::new(AvisAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = avis::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "note", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let mut rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            {
                use sea_orm::ConnectionTrait;
                let fk_ids: Vec<String> = rows
                    .iter()
                    .filter_map(|r| {
                        r.get("commande_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        })
                    })
                    .collect::<std::collections::HashSet<String>>()
                    .into_iter()
                    .collect();
                if !fk_ids.is_empty() {
                    let ids_csv = fk_ids
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect::<Vec<_>>()
                        .join(",");
                    let _fk_stmt_commande_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("numero"))
                        .from(sea_orm::sea_query::Alias::new("commandes"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    let label_map_commande_id: std::collections::HashMap<String, String> = db
                        .query_all(&_fk_stmt_commande_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let label = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, label))
                        })
                        .collect();
                    for row in &mut rows {
                        if let Some(key) = row.get("commande_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        }) && let Some(label) = label_map_commande_id.get(&key)
                        {
                            row["commande_id"] = serde_json::Value::String(label.clone());
                        }
                    }
                }
            }
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = avis::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "note", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = avis::Entity::find_by_id(id).one(&*db).await?;
            let Some(model) = row else {
                return Ok(None);
            };
            let mut row = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
            {
                use sea_orm::ConnectionTrait;
                if let Some(fk_key) = row.get("commande_id").and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                }) {
                    let ids_csv = format!("'{}'", fk_key.replace('\'', "''"));
                    let _fk_stmt_commande_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("numero"))
                        .from(sea_orm::sea_query::Alias::new("commandes"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    if let Some(fk_row) = db.query_one(&_fk_stmt_commande_id).await.ok().flatten()
                        && let Ok(label) = fk_row.try_get_by_index::<String>(1)
                    {
                        row["commande_id"] = serde_json::Value::String(label);
                    }
                }
            }
            Ok(Some(row))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            avis::Entity::delete_by_id(id).exec(&*db).await.map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            avis::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            avis::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            avis::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("commande_id", "Commande"),
                ("note", "Note"),
                ("statut", "Statut"),
                ("created_at", "Date"),
            ])
            .list_filter(vec![("statut", "Statut", 10u64)]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_statut = 10u64;
            let cur_page_statut = pages.get("statut").copied().unwrap_or(0);
            let count_stmt_statut = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT statut)"))
                .from(Alias::new(avis::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .to_owned();
            let count_row_statut = match db.query_one(&count_stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `avis.statut`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_statut = count_row_statut
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_statut = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(statut AS TEXT)"))
                .from(Alias::new(avis::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .limit(page_size_statut)
                .offset(cur_page_statut * page_size_statut)
                .to_owned();
            let rows_statut = match db.query_all(&stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `avis.statut`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_statut: Vec<String> = rows_statut
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_statut.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("statut".to_string(), (vals_statut, total_statut));
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![
                GroupAction::val("statut", "Valider", "valide"),
                GroupAction::val("statut", "Refuser", "refuse"),
            ]),
    );

    // ── Resource: avis_plats ──
    let meta = AdminResource::new(
        "avis_plats",
        "crate::entities::avis_plat::Model",
        "AdminForm",
        "Avis sur les plats",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let mut form =
                    avis_plat::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                {
                    use sea_orm::ConnectionTrait;
                    let _fk_opt_stmt_plat_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("titre"))
                        .from(sea_orm::sea_query::Alias::new("plats"))
                        .to_owned();
                    let _fk_opt_choices_plat_id: Vec<(String, String)> = db
                        .query_all(&_fk_opt_stmt_plat_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let lbl = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, lbl))
                        })
                        .collect();
                    form.get_form_mut()
                        .field_choices("plat_id", "Plat", _fk_opt_choices_plat_id);
                }
                Box::new(AvisPlatAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = avis_plat::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "note", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let mut rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            {
                use sea_orm::ConnectionTrait;
                let fk_ids: Vec<String> = rows
                    .iter()
                    .filter_map(|r| {
                        r.get("plat_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        })
                    })
                    .collect::<std::collections::HashSet<String>>()
                    .into_iter()
                    .collect();
                if !fk_ids.is_empty() {
                    let ids_csv = fk_ids
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "''")))
                        .collect::<Vec<_>>()
                        .join(",");
                    let _fk_stmt_plat_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("titre"))
                        .from(sea_orm::sea_query::Alias::new("plats"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    let label_map_plat_id: std::collections::HashMap<String, String> = db
                        .query_all(&_fk_stmt_plat_id)
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter_map(|row| {
                            let id = row.try_get_by_index::<String>(0).ok()?;
                            let label = row.try_get_by_index::<String>(1).ok()?;
                            Some((id, label))
                        })
                        .collect();
                    for row in &mut rows {
                        if let Some(key) = row.get("plat_id").and_then(|v| {
                            v.as_i64()
                                .map(|n| n.to_string())
                                .or_else(|| v.as_str().map(str::to_string))
                        }) && let Some(label) = label_map_plat_id.get(&key)
                        {
                            row["plat_id"] = serde_json::Value::String(label.clone());
                        }
                    }
                }
            }
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = avis_plat::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "note", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "statut", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "created_at", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = avis_plat::Entity::find_by_id(id).one(&*db).await?;
            let Some(model) = row else {
                return Ok(None);
            };
            let mut row = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
            {
                use sea_orm::ConnectionTrait;
                if let Some(fk_key) = row.get("plat_id").and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                }) {
                    let ids_csv = format!("'{}'", fk_key.replace('\'', "''"));
                    let _fk_stmt_plat_id = sea_orm::sea_query::Query::select()
                        .expr(sea_orm::sea_query::Expr::cust("CAST(id AS TEXT)"))
                        .expr(sea_orm::sea_query::Expr::cust("titre"))
                        .from(sea_orm::sea_query::Alias::new("plats"))
                        .and_where(sea_orm::sea_query::Expr::cust(format!(
                            "CAST(id AS TEXT) IN ({})",
                            ids_csv
                        )))
                        .to_owned();
                    if let Some(fk_row) = db.query_one(&_fk_stmt_plat_id).await.ok().flatten()
                        && let Ok(label) = fk_row.try_get_by_index::<String>(1)
                    {
                        row["plat_id"] = serde_json::Value::String(label);
                    }
                }
            }
            Ok(Some(row))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            avis_plat::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            avis_plat::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            avis_plat::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            avis_plat::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("plat_id", "Plat"),
                ("note", "Note"),
                ("statut", "Statut"),
                ("created_at", "Date"),
            ])
            .list_filter(vec![("statut", "Statut", 10u64)]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_statut = 10u64;
            let cur_page_statut = pages.get("statut").copied().unwrap_or(0);
            let count_stmt_statut = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT statut)"))
                .from(Alias::new(avis_plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .to_owned();
            let count_row_statut = match db.query_one(&count_stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `avis_plats.statut`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_statut = count_row_statut
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_statut = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(statut AS TEXT)"))
                .from(Alias::new(avis_plat::Entity.table_name()))
                .and_where(Expr::col(Alias::new("statut")).is_not_null())
                .limit(page_size_statut)
                .offset(cur_page_statut * page_size_statut)
                .to_owned();
            let rows_statut = match db.query_all(&stmt_statut).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `avis_plats.statut`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_statut: Vec<String> = rows_statut
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_statut.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("statut".to_string(), (vals_statut, total_statut));
            Ok(result)
        })
    });

    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn)
            .with_filter_fn(filter_fn)
            .with_group_actions(vec![
                GroupAction::val("statut", "Valider", "valide"),
                GroupAction::val("statut", "Refuser", "refuse"),
            ]),
    );

    // ── Resource: info_resto ──
    let meta = AdminResource::new(
        "info_resto",
        "crate::entities::info_resto::Model",
        "AdminForm",
        "Informations du restaurant",
        vec![],
    );
    let form_builder: FormBuilder = Arc::new(
        |_db: ADb,
         _vec: Vec<std::string::String>,
         data: StrMap,
         tera: ATera,
         csrf: String,
         method: Method| {
            Box::pin(async move {
                let form = info_resto::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(InfoRestoAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = info_resto::Entity::find();
            if let Some(ref col) = params.sort_by {
                let order = if params.sort_dir == SortDir::Desc {
                    Order::Desc
                } else {
                    Order::Asc
                };
                query = query.order_by(Expr::col(Alias::new(col.as_str())), order);
            }
            for (col, val) in &params.column_filters {
                let escaped = val.replace('\'', "''");
                query = query.filter(Expr::cust(format!("CAST({} AS TEXT) = '{}'", col, escaped)));
            }
            if let Some(ref search_str) = params.search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nom", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "adresse", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "telephone", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "email", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "latitude", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "longitude", escaped
                )));
                query = query.filter(search_cond);
            }
            let db_rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            let rows: Vec<serde_json::Value> = db_rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect();
            Ok(rows)
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = info_resto::Entity::find();
            if let Some(ref search_str) = _search {
                let escaped = search_str.replace('\'', "''");
                let mut search_cond = sea_orm::Condition::any();
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "nom", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "adresse", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "telephone", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "email", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "latitude", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "longitude", escaped
                )));
                query = query.filter(search_cond);
            }
            query.count(&*db).await
        })
    });

    let get_fn: GetFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            let row = info_resto::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            info_resto::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            info_resto::admin_from_form(&data, None)
                .insert(&*db)
                .await
                .map(|_| ())
        })
    });

    let update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            info_resto::admin_from_form(&data, Some(id))
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let partial_update_fn: UpdateFn = Arc::new(|db: ADb, id: String, data: StrMap| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            info_resto::admin_partial_update(&data, id)
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(DisplayConfig::new().columns_include(vec![
        ("nom", "Nom"),
        ("adresse", "Adresse"),
        ("telephone", "Téléphone"),
        ("email", "Email"),
        ("latitude", "Latitude"),
        ("longitude", "Longitude"),
    ]));
    registry.register(
        ResourceEntry::new(meta, form_builder)
            .with_list_fn(list_fn)
            .with_get_fn(get_fn)
            .with_delete_fn(delete_fn)
            .with_create_fn(create_fn)
            .with_update_fn(update_fn)
            .with_partial_update_fn(partial_update_fn)
            .with_count_fn(count_fn),
    );

    registry.configure(
        "users",
        DisplayConfig::new().columns_include(vec![
            ("username", "Nom d'utilisateur"),
            ("email", "Email"),
            ("is_active", "Actif"),
            ("is_staff", "Staff"),
            ("created_at", "Inscrit le"),
        ]),
    );
    registry.configure_group_actions("users", vec![GroupAction::bool("is_active", "Activer")]);
    registry
}

/// Builds the admin CRUD routes for the given prefix.
/// To be passed to `.with_admin(|a| a.routes(admins::routes("/admin")))` in main.rs.
/// The prefix is automatically propagated to `AdminConfig` — no need to call `.prefix()` separately.
pub fn routes(prefix: &str) -> runique::admin::AdminRoutes {
    let p = prefix.trim_end_matches('/');
    let router = runique::axum::Router::new()
        .route(
            &format!("{}/{{resource}}/{{action}}", p),
            get(admin_get).post(admin_post),
        )
        .route(
            &format!("{}/{{resource}}/{{id}}/{{action}}", p),
            get(admin_get_id).post(admin_post_id),
        );
    runique::admin::AdminRoutes::new(p, router)
}

/// Returns the shared state of the admin prototype (for the dashboard).
pub fn admin_state() -> std::sync::Arc<PrototypeAdminState> {
    let config = Arc::new(AdminConfig::new());
    std::sync::Arc::new(PrototypeAdminState {
        registry: Arc::new(admin_register()),
        config,
    })
}
