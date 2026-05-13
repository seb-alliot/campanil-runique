use crate::backend::panier::{Panier, SESSION_PANIER_KEY};
use runique::prelude::*;

pub async fn panier_get(session: &Session) -> Panier {
    let r: Result<Option<Panier>, _> = session.get(SESSION_PANIER_KEY).await;
    r.ok().flatten().unwrap_or_default()
}
