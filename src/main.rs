use figment::{
    Figment, Profile,
    providers::{Env, Format, Serialized, Toml},
};
use rocket::fairing::AdHoc;
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
    let data_path: String = figment.extract_inner("data_path").expect("data_path");

    // Schedule clear_data_path job (every day at midnight)
    let scheduler = Arc::new(Mutex::new(JobScheduler::new()));
    let dp = data_path.clone();
    {
        let mut sched = scheduler.lock().unwrap();
        sched.add(Job::new(
            "0 0 * * * *".parse::<Schedule>().unwrap(),
            move || {
                let _ = clear_data_path(&dp);
            },
        ));
    }
    let scheduler_clone = Arc::clone(&scheduler);
    thread::spawn(move || {
        loop {
            scheduler_clone.lock().unwrap().tick();
            thread::sleep(Duration::from_secs(60));
        }
    });

    rocket::custom(figment)
        .manage(ApiToken::new(auth_tokens, auth_enabled))
        .mount("/api/upload", api::upload_routes())
        .mount("/api/status", api::status_routes())
        .register("/api", api::catchers())
        .attach(AdHoc::config::<config::Config>())
}
