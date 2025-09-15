use figment::{
    Figment, Profile,
    providers::{Env, Format, Serialized, Toml},
};
use rocket::{fairing::AdHoc, tokio};
use rocket_apitoken::ApiToken;

use crate::helpers::{check_data_path, clear_data_path};
use std::time::Duration;

#[macro_use]
extern crate rocket;

mod api;
mod audio;
mod config;
mod error;
mod helpers;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[launch]
fn rocket() -> _ {
    println!("----------------------------------");
    println!("WAVEEMAPI3 API v{}", VERSION);
    println!("----------------------------------");

    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(config::Config::default()))
        .merge(Toml::file("waveemapi.toml").nested())
        .merge(Env::prefixed("WAVEEMAPI_").global())
        .select(Profile::from_env_or("WAVEEMAPI_PROFILE", "default"));

    let auth_enabled: bool = figment.extract_inner("auth_enabled").expect("auth_enabled");
    println!("Auth bypass: {}", !auth_enabled);
    let auth_tokens: Vec<String> = figment.extract_inner("auth_tokens").expect("auth_tokens");
    println!("Loaded {} auth_tokens", auth_tokens.len());

    let data_path: String = figment.extract_inner("data_path").expect("data_path");
    check_data_path(&data_path).expect("data_path");

    rocket::custom(figment)
        .manage(ApiToken::new(auth_tokens, auth_enabled))
        .mount("/api/upload", api::upload_routes())
        .mount("/api/status", api::status_routes())
        .register("/api", api::catchers())
        .attach(AdHoc::config::<config::Config>())
        .attach(AdHoc::on_liftoff("cleanup-scheduler", |rocket| {
            Box::pin(async move {
                setup_cleanup_scheduler(rocket.figment());
            })
        }))
}

fn setup_cleanup_scheduler(figment: &Figment) {
    let data_path: String = figment.extract_inner("data_path").expect("data_path");

    let cleanup_interval_minutes: u64 = figment
        .extract_inner("cleanup_interval_minutes")
        .expect("cleanup_interval_minutes");
    let cleanup_interval_seconds = cleanup_interval_minutes * 60;

    let file_expiry_minutes: u64 = figment
        .extract_inner("file_expiry_minutes")
        .expect("file_expiry_minutes");
    let file_expiry_seconds = file_expiry_minutes * 60;

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(cleanup_interval_seconds));
        loop {
            interval.tick().await;
            println!("Running scheduled cleanup of data path: {}", data_path);
            if let Err(e) = clear_data_path(&data_path, Duration::from_secs(file_expiry_seconds)) {
                eprintln!("Error during scheduled cleanup: {}", e);
            } else {
                println!("Scheduled cleanup completed successfully.");
            }
        }
    });
}
