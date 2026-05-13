use crate::backend::panier::{panier_get, panier_save};
use runique::prelude::*;

pub async fn panier_retirer_boisson(session: &Session, boisson_id: i32) {
    let mut panier = panier_get(session).await;
    panier.lignes.retain(|l| l.boisson_id != Some(boisson_id));
    panier_save(session, &panier).await;
}
