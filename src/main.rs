#[macro_use]
extern crate rocket;

mod api;
mod audio;
mod auth;
mod error;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/upload", api::upload_routes())
        .mount("/api/status", api::status_routes())
        .register("/api", api::catchers())
}
