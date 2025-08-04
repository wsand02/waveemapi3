use rocket::{
    Request,
    http::Status,
    serde::{Serialize, json::Json},
};

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![default_catcher]
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> Json<DefaultErrorResp> {
    Json(DefaultErrorResp {
        error: status.reason_lossy().to_string(),
    })
}

#[derive(Serialize)]
pub struct DefaultErrorResp {
    pub error: String,
}
