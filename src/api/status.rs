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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json;
    #[test]
    fn test_status_json_structure() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/status").dispatch();
        let body = response.into_string().unwrap();
        let parsed: json::Value = json::from_str(&body).unwrap();
        assert_eq!(parsed["status"], "Online");
    }

    #[test]
    fn test_status_resp_serialization() {
        let resp = StatusResp {
            status: "Online".to_string(),
        };
        let jsonser = json::to_string(&resp).unwrap();
        assert_eq!(jsonser, r#"{"status":"Online"}"#);
    }
}
