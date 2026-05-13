use crate::backend::panier::{Panier, SESSION_PANIER_KEY};
use runique::prelude::*;

pub async fn panier_save(session: &Session, panier: &Panier) {
    let _ = session.insert(SESSION_PANIER_KEY, panier).await;
}
