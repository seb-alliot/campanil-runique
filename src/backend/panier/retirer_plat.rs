use crate::backend::panier::{panier_get, panier_save};
use runique::prelude::*;

pub async fn panier_retirer(session: &Session, plat_id: Pk) {
    let mut panier = panier_get(session).await;
    panier.lignes.retain(|l| l.plat_id != plat_id);
    panier_save(session, &panier).await;
}

pub async fn panier_retirer_menu(session: &Session, menu_id: Pk) {
    let mut panier = panier_get(session).await;
    if let Some(pos) = panier
        .lignes
        .iter()
        .position(|l| l.menu_resto_id == Some(menu_id))
    {
        panier.lignes.remove(pos);
    }
    panier_save(session, &panier).await;
}

pub async fn panier_retirer_supplement(session: &Session, supplement_id: Pk) {
    let mut panier = panier_get(session).await;
    panier
        .lignes
        .retain(|l| l.supplement_id != Some(supplement_id));
    panier_save(session, &panier).await;
}
