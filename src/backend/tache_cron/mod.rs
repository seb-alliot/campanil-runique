use runique::prelude::*;

pub fn spawn_penalites(engine: Arc<RuniqueEngine>) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
            crate::backend::commande::process_penalites(&engine.db, &engine.tera).await;
        }
    });
}
