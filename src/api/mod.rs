mod catcher;
mod status;
mod upload;

pub use crate::api::{
    catcher::catchers, status::routes as status_routes, upload::routes as upload_routes,
};
