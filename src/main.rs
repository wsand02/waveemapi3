use figment::{
    Figment, Profile,
    providers::{Env, Format, Serialized, Toml},
};
use rocket::{data, fairing::AdHoc, tokio};
use rocket_apitoken::ApiToken;

use crate::helpers::clear_data_path;
use job_scheduler_ng::{Job, JobScheduler, Schedule};
use std::sync::{Arc, Mutex};
use std::thread;
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

    // Get data_path from config

    rocket::custom(figment)
        .manage(ApiToken::new(auth_tokens, auth_enabled))
        .mount("/api/upload", api::upload_routes())
        .mount("/api/status", api::status_routes())
        .register("/api", api::catchers())
        .attach(AdHoc::config::<config::Config>())
        .attach(AdHoc::on_liftoff("Cleanup Scheduler", |rocket| {
            let figment = rocket.figment().clone();
            Box::pin(async move {
                setup_cleanup_scheduler(&figment).await;
            })
        }))
}

async fn setup_cleanup_scheduler(figment: &Figment) {
    let data_path: String = figment.extract_inner("data_path").expect("data_path");
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // 24 hours
    loop {
        interval.tick().await;
        println!("Running scheduled cleanup of data path: {}", data_path);
        if let Err(e) = clear_data_path(&data_path) {
            eprintln!("Error during scheduled cleanup: {}", e);
        } else {
            println!("Scheduled cleanup completed successfully.");
        }
    }
}
