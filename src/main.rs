use runique::prelude::*;
mod admin;
mod backend;
mod entities;
mod formulaire;
mod url;
mod views;
#[cfg(test)]
mod runique_test;
use runique::app::builder::RuniqueAppBuilder as builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    password_init(PasswordConfig::auto_with(Manual::Argon2));
    set_lang(Lang::Fr);

    let config: RuniqueConfig = RuniqueConfig::from_env();

    let db_config = DatabaseConfig::from_env()?.build();
    let db = db_config.connect().await?;

    // mongo client for analytics
    let mongo = mongodb::Client::with_uri_str(
        std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into()),
    )
    .await?;

    let app = builder::new(config)
        .routes(url::routes())
        // Base of the links sent by email; `RUNIQUE_ENV=dev` uses the local server.
        .with_public_url("https://test-itsuki.fr")
        .with_database(db)
        .with_custom_db(mongo)
        .with_mailer_from_env()
        .with_password_reset(|pr| {
            pr.forgot_template("auth/forgot_password.html")
                .reset_template("auth/reset_password.html")
                .email_template("emails/reset_password.html")
                .extra_context(Arc::new(|req: &mut Request| {
                    Box::pin(backend::utils::inject_auth(req))
                }))
        })
        .statics()
        .middleware(|m| {
            m.with_session_memory_limit(5 * 1024 * 1024, 10 * 1024 * 1024)
                .with_session_cleanup_interval(5)
                .with_allowed_hosts(|h| {
                    h.enabled(true)
                        .host("localhost:8000")
                        .host("127.0.0.1:8000")
                        .host("test-itsuki.fr")
                        .host("www.test-itsuki.fr")
                })
                .with_csp(|c| {
                    c.policy(SecurityPolicy::strict())
                        .scripts(vec!["'self'", "'strict-dynamic'"])
                })
                .with_anti_bot()
        })
        .with_admin(admin::config)
        .build()
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

    crate::backend::tache_cron::spawn_penalites(app.engine.clone());

    app.run().await?;

    Ok(())
}
