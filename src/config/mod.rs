use serde::Deserialize;
use std::sync::Arc;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_token_expires_in: String,
    pub jwt_refresh_token_expires_in: String,
}

pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    dotenvy::dotenv().ok();

    let config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config");

    let app_config: AppConfig = config.try_deserialize().expect("Failed to deserialize config");

    Arc::new(app_config)
});