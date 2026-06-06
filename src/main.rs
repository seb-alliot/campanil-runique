use runique::prelude::*;
mod admin;
mod admins;
mod backend;
mod entities;
mod formulaire;
mod url;
mod views;
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

    builder::new(config)
        .routes(url::routes())
        .with_database(db)
        .with_custom_db(mongo)
        .with_mailer_from_env()
        .with_password_reset::<BuiltinUserEntity>(|pr| {
            pr.forgot_template("auth/forgot_password.html")
                .reset_template("auth/reset_password.html")
                .email_template("emails/reset_password.html")
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
                        .with_header_security(true)
                        .with_upgrade_insecure(!is_debug())
                        .scripts(vec!["'self'", "'strict-dynamic'"])
                })
                .with_anti_bot()
        })
        .with_admin(|a| {
            a.sitemap("https://ucampanile.fr/sitemap.xml")
                .routes(admins::routes("/admin-campanile"))
                .extra_routes(url::admin_extra_routes())
                .with_state(admins::admin_state())
                .auth(RuniqueAdminAuth::new())
                .site_title("Campanile — Administration")
                .hot_reload(is_debug())
                .resource_order([
                    "users",
                    "plats",
                    "menus",
                    "commandes",
                    "avis",
                    "themes",
                    "regimes",
                    "allergenes",
                    "horaires",
                    "contacts",
                    "info resto",
                ])
                .with_rate_limiter(RateLimiter::new().max_requests(20).retry_after(3600))
                .with_login_guard(LoginGuard::new().max_attempts(20).lockout_secs(300))
                .page_size(10)
        })
        .build()
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?
        .run()
        .await?;

    Ok(())
}
