use rocket::serde::{Serialize, json::Json};

pub fn routes() -> Vec<rocket::Route> {
    routes![status]
}

#[get("/")]
fn status() -> Json<StatusResp> {
    Json(StatusResp {
        status: "Online".to_string(),
    })
}

#[derive(Serialize)]
struct StatusResp {
    status: String,
}
