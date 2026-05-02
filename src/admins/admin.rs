// AUTO-admin — DO NOT EDIT MANUALLY
// admin by `runique start` from src/admin.rs

use runique::prelude::*;

use crate::entities::allergene;
use crate::entities::avis;
use crate::entities::commande;
use crate::entities::contact;
use crate::entities::horaire;
use crate::entities::menu;
use crate::entities::plat;
use crate::entities::regime;
use crate::entities::theme;

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

// ── DynForm wrapper for theme::AdminForm ──
struct ThemeAdminFormDynWrapper(pub theme::AdminForm);

#[async_trait]
impl DynForm for ThemeAdminFormDynWrapper {
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

// ── DynForm wrapper for regime::AdminForm ──
struct RegimeAdminFormDynWrapper(pub regime::AdminForm);

#[async_trait]
impl DynForm for RegimeAdminFormDynWrapper {
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
struct HoraireAdminFormDynWrapper(pub horaire::AdminForm);

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
        "crate::entities::entities::allergene::Model",
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
            allergene::admin_from_form(&data, Some(id.into()))
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
            allergene::admin_partial_update(&data, id.into())
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

    // ── Resource: themes ──
    let meta = AdminResource::new(
        "themes",
        "crate::entities::entities::theme::Model",
        "AdminForm",
        "Thèmes",
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
                let form = theme::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(ThemeAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = theme::Entity::find();
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = theme::Entity::find();
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
            let row = theme::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            theme::Entity::delete_by_id(id).exec(&*db).await.map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            theme::admin_from_form(&data, None)
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
            theme::admin_from_form(&data, Some(id.into()))
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
            theme::admin_partial_update(&data, id.into())
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

    // ── Resource: regimes ──
    let meta = AdminResource::new(
        "regimes",
        "crate::entities::entities::regime::Model",
        "AdminForm",
        "Régimes alimentaires",
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
                let form = regime::AdminForm::build_with_data(&data, tera, &csrf, method).await;
                Box::new(RegimeAdminFormDynWrapper(form)) as Box<dyn DynForm>
            })
        },
    );

    let list_fn: ListFn = Arc::new(|db: ADb, params: ListParams| {
        Box::pin(async move {
            use sea_orm::{
                QueryFilter,
                sea_query::{Alias, Expr, Order},
            };
            let mut query = regime::Entity::find();
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
        })
    });

    let count_fn: CountFn = Arc::new(|db: ADb, _search: Option<String>| {
        Box::pin(async move {
            use sea_orm::{QueryFilter, sea_query::Expr};
            let mut query = regime::Entity::find();
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
            let row = regime::Entity::find_by_id(id).one(&*db).await?;
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
        })
    });

    let delete_fn: DeleteFn = Arc::new(|db: ADb, id: String| {
        Box::pin(async move {
            let id = id
                .parse::<i32>()
                .map_err(|_| DbErr::Custom("invalid id".to_string().to_string()))?;
            regime::Entity::delete_by_id(id)
                .exec(&*db)
                .await
                .map(|_| ())
        })
    });

    let create_fn: CreateFn = Arc::new(|db: ADb, data: StrMap| {
        Box::pin(async move {
            regime::admin_from_form(&data, None)
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
            regime::admin_from_form(&data, Some(id.into()))
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
            regime::admin_partial_update(&data, id.into())
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
        "crate::entities::entities::horaire::Model",
        "AdminForm",
        "Horaires",
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
                let form = horaire::AdminForm::build_with_data(&data, tera, &csrf, method).await;
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
                    "heure_ouverture", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "heure_fermeture", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "ferme", escaped
                )));
                query = query.filter(search_cond);
            }
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
                    "heure_ouverture", escaped
                )));
                search_cond = search_cond.add(Expr::cust(format!(
                    "LOWER(CAST({} AS TEXT)) LIKE LOWER('%%{}%%')",
                    "heure_fermeture", escaped
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
            horaire::admin_from_form(&data, None)
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
            horaire::admin_from_form(&data, Some(id.into()))
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
            horaire::admin_partial_update(&data, id.into())
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("jour", "Jour"),
                ("heure_ouverture", "Ouverture"),
                ("heure_fermeture", "Fermeture"),
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
            .with_filter_fn(filter_fn),
    );

    // ── Resource: contacts ──
    let meta = AdminResource::new(
        "contacts",
        "crate::entities::entities::contact::Model",
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
            contact::admin_from_form(&data, Some(id.into()))
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
            contact::admin_partial_update(&data, id.into())
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

    // ── Resource: plats ──
    let meta = AdminResource::new(
        "plats",
        "crate::entities::entities::plat::Model",
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
                query = query.filter(search_cond);
            }
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
            plat::admin_from_form(&data, None)
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
            plat::admin_from_form(&data, Some(id.into()))
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
            plat::admin_partial_update(&data, id.into())
                .update(&*db)
                .await
                .map(|_| ())
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
            ])
            .list_filter(vec![
                ("type_plat", "Type de plat", 10u64),
                ("disponible", "Disponibilité", 10u64),
                ("est_viande", "Viande", 10u64),
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

    // ── Resource: menus ──
    let meta = AdminResource::new(
        "menus",
        "crate::entities::entities::menu::Model",
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
            menu::admin_from_form(&data, None)
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
            menu::admin_from_form(&data, Some(id.into()))
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
            menu::admin_partial_update(&data, id.into())
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("titre", "Titre"),
                ("prix_par_personne", "Prix/pers."),
                ("nb_personnes_min", "Min. pers."),
                ("actif", "Actif"),
            ])
            .list_filter(vec![
                ("actif", "Actif", 10u64),
                ("theme_id", "Thème", 10u64),
                ("regime_id", "Régime", 10u64),
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
            let page_size_theme_id = 10u64;
            let cur_page_theme_id = pages.get("theme_id").copied().unwrap_or(0);
            let count_stmt_theme_id = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT theme_id)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("theme_id")).is_not_null())
                .to_owned();
            let count_row_theme_id = match db.query_one(&count_stmt_theme_id).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.theme_id`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_theme_id = count_row_theme_id
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_theme_id = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(theme_id AS TEXT)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("theme_id")).is_not_null())
                .limit(page_size_theme_id)
                .offset(cur_page_theme_id * page_size_theme_id)
                .to_owned();
            let rows_theme_id = match db.query_all(&stmt_theme_id).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.theme_id`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_theme_id: Vec<String> = rows_theme_id
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_theme_id.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("theme_id".to_string(), (vals_theme_id, total_theme_id));
            let page_size_regime_id = 10u64;
            let cur_page_regime_id = pages.get("regime_id").copied().unwrap_or(0);
            let count_stmt_regime_id = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT regime_id)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("regime_id")).is_not_null())
                .to_owned();
            let count_row_regime_id = match db.query_one(&count_stmt_regime_id).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.regime_id`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_regime_id = count_row_regime_id
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_regime_id = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(regime_id AS TEXT)"))
                .from(Alias::new(menu::Entity.table_name()))
                .and_where(Expr::col(Alias::new("regime_id")).is_not_null())
                .limit(page_size_regime_id)
                .offset(cur_page_regime_id * page_size_regime_id)
                .to_owned();
            let rows_regime_id = match db.query_all(&stmt_regime_id).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `menus.regime_id`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_regime_id: Vec<String> = rows_regime_id
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_regime_id.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert("regime_id".to_string(), (vals_regime_id, total_regime_id));
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
            .with_group_actions(vec![GroupAction::bool("actif", "Activer")]),
    );

    // ── Resource: commandes ──
    let meta = AdminResource::new(
        "commandes",
        "crate::entities::entities::commande::Model",
        "AdminForm",
        "Commandes",
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
                let form = commande::AdminForm::build_with_data(&data, tera, &csrf, method).await;
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
                    "type_commande", escaped
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
                    "type_commande", escaped
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
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
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
            commande::admin_from_form(&data, Some(id.into()))
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
            commande::admin_partial_update(&data, id.into())
                .update(&*db)
                .await
                .map(|_| ())
        })
    });

    let meta = meta.display(
        DisplayConfig::new()
            .columns_include(vec![
                ("numero", "N°"),
                ("type_commande", "Type"),
                ("statut", "Statut"),
                ("mode_paiement", "Paiement"),
                ("prix_total", "Total"),
                ("created_at", "Date"),
            ])
            .list_filter(vec![
                ("type_commande", "Type", 10u64),
                ("statut", "Statut", 10u64),
                ("mode_paiement", "Paiement", 10u64),
                ("avec_livraison", "Livraison", 10u64),
            ]),
    );
    let filter_fn: FilterFn = Arc::new(|db: ADb, pages: std::collections::HashMap<String, u64>| {
        Box::pin(async move {
            use sea_orm::sea_query::{Alias, Expr, Query};
            use sea_orm::{ConnectionTrait, ExprTrait};
            let mut result: std::collections::HashMap<String, (Vec<String>, u64)> =
                std::collections::HashMap::new();
            let page_size_type_commande = 10u64;
            let cur_page_type_commande = pages.get("type_commande").copied().unwrap_or(0);
            let count_stmt_type_commande = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT type_commande)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_commande")).is_not_null())
                .to_owned();
            let count_row_type_commande = match db.query_one(&count_stmt_type_commande).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.type_commande`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_type_commande = count_row_type_commande
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_type_commande = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(type_commande AS TEXT)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("type_commande")).is_not_null())
                .limit(page_size_type_commande)
                .offset(cur_page_type_commande * page_size_type_commande)
                .to_owned();
            let rows_type_commande = match db.query_all(&stmt_type_commande).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.type_commande`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_type_commande: Vec<String> = rows_type_commande
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_type_commande.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "type_commande".to_string(),
                (vals_type_commande, total_type_commande),
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
            let page_size_avec_livraison = 10u64;
            let cur_page_avec_livraison = pages.get("avec_livraison").copied().unwrap_or(0);
            let count_stmt_avec_livraison = Query::select()
                .expr(Expr::cust("COUNT(DISTINCT avec_livraison)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("avec_livraison")).is_not_null())
                .to_owned();
            let count_row_avec_livraison = match db.query_one(&count_stmt_avec_livraison).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.avec_livraison`: column not found in DB — {}",
                        e
                    );
                    None
                }
            };
            let total_avec_livraison = count_row_avec_livraison
                .and_then(|r| r.try_get_by_index::<i64>(0).ok())
                .unwrap_or(0) as u64;
            let stmt_avec_livraison = Query::select()
                .distinct()
                .expr(Expr::cust("CAST(avec_livraison AS TEXT)"))
                .from(Alias::new(commande::Entity.table_name()))
                .and_where(Expr::col(Alias::new("avec_livraison")).is_not_null())
                .limit(page_size_avec_livraison)
                .offset(cur_page_avec_livraison * page_size_avec_livraison)
                .to_owned();
            let rows_avec_livraison = match db.query_all(&stmt_avec_livraison).await {
                Ok(r) => r,
                Err(e) => {
                    tracing::error!(
                        "[runique admin] list_filter `commandes.avec_livraison`: column not found in DB — {}",
                        e
                    );
                    vec![]
                }
            };
            let mut vals_avec_livraison: Vec<String> = rows_avec_livraison
                .iter()
                .filter_map(|r| r.try_get_by_index::<String>(0).ok())
                .collect();
            vals_avec_livraison.sort_by(|a, b| match (a.parse::<i64>(), b.parse::<i64>()) {
                (Ok(x), Ok(y)) => x.cmp(&y),
                _ => a.cmp(b),
            });
            result.insert(
                "avec_livraison".to_string(),
                (vals_avec_livraison, total_avec_livraison),
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
        "crate::entities::entities::avis::Model",
        "AdminForm",
        "Avis clients",
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
                let form = avis::AdminForm::build_with_data(&data, tera, &csrf, method).await;
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
                    "commande_id", escaped
                )));
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
            let rows = query
                .offset(params.offset)
                .limit(params.limit)
                .all(&*db)
                .await?;
            Ok(rows
                .into_iter()
                .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
                .collect())
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
                    "commande_id", escaped
                )));
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
            Ok(row.map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null)))
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
            avis::admin_from_form(&data, Some(id.into()))
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
            avis::admin_partial_update(&data, id.into())
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
            .with_group_actions(vec![GroupAction::bool("statut", "Valider")]),
    );

    registry.configure(
        "users",
        DisplayConfig::new()
            .columns_include(vec![
                ("username", "Nom d'utilisateur"),
                ("email", "Email"),
                ("is_active", "Actif"),
                ("is_staff", "Staff"),
                ("created_at", "Inscrit le"),
            ])
            .list_filter(vec![
                ("is_active", "Actif", 10u64),
                ("is_staff", "Staff", 10u64),
            ]),
    );
    registry.configure_group_actions("users", vec![GroupAction::bool("is_active", "Activer")]);
    registry
}

/// Builds the Axum Router of the admin prototype for the given prefix.
/// To be passed to `.with_admin(|a| a.routes(admins::routes("/admin")))` in main.rs.
pub fn routes(prefix: &str) -> runique::axum::Router {
    let p = prefix.trim_end_matches('/');
    runique::axum::Router::new()
        .route(
            &format!("{}/{{resource}}/{{action}}", p),
            get(admin_get).post(admin_post),
        )
        .route(
            &format!("{}/{{resource}}/{{id}}/{{action}}", p),
            get(admin_get_id).post(admin_post_id),
        )
}

/// Returns the shared state of the admin prototype (for the dashboard).
pub fn admin_state() -> std::sync::Arc<PrototypeAdminState> {
    let config = Arc::new(AdminConfig::new());
    std::sync::Arc::new(PrototypeAdminState {
        registry: Arc::new(admin_register()),
        config,
    })
}
