#[macro_use]
extern crate rocket;

mod api;
mod audio;
mod auth;
mod error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[launch]
fn rocket() -> _ {
    println!("WAVEEMAPI3 API v{}", VERSION);
    rocket::build()
        .mount("/api/upload", api::upload_routes())
        .mount("/api/status", api::status_routes())
        .register("/api", api::catchers())
}
