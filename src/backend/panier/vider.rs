use crate::backend::panier::{Panier, SESSION_PANIER_KEY};
use runique::prelude::*;

pub async fn panier_vider(session: &Session) {
    let _ = session.remove::<Panier>(SESSION_PANIER_KEY).await;
}
