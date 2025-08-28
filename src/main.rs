use figment::{
    Figment, Profile,
    providers::{Env, Format, Serialized, Toml},
};
use rocket::fairing::AdHoc;
use rocket_apitoken::ApiToken;

#[macro_use]
extern crate rocket;

mod api;
mod audio;
mod config;
mod error;

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

    rocket::custom(figment)
        .manage(ApiToken::new(auth_tokens, auth_enabled))
        .mount("/api/upload", api::upload_routes())
        .mount("/api/status", api::status_routes())
        .register("/api", api::catchers())
        .attach(AdHoc::config::<config::Config>())
}
