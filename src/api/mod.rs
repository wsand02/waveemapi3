mod catcher;
mod status;
mod upload;

pub use crate::api::{
    catcher::DefaultErrorResp, catcher::catchers, status::routes as status_routes,
    upload::routes as upload_routes,
};

#[cfg(test)]
mod tests {
    use crate::api::status::StatusResp;
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::serde::json;

    #[test]
    fn test_upload_auth_no_head() {
        use rocket::local::blocking::Client;

        // Construct a client to use for dispatching requests.
        let client = Client::tracked(rocket()).expect("valid `Rocket`");

        // Dispatch a request to 'GET /' and validate the response.
        let response = client.post("/api/upload").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    fn test_upload_auth_invalid_token() {
        use rocket::local::blocking::Client;

        // Construct a client to use for dispatching requests.
        let client = Client::tracked(rocket()).expect("valid `Rocket`");
        // Dispatch a request to 'GET /' and validate the response.
        let response = client
            .post("/api/upload")
            .header(rocket::http::Header::new("Authorization", "Bearer uwu"))
            .dispatch();
        assert_eq!(response.status(), rocket::http::Status::Unauthorized);
    }

    #[test]
    fn test_status_json_structure() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/status").dispatch();
        let body = response.into_string().unwrap();
        let parsed: json::Value = json::from_str(&body).unwrap();
        assert_eq!(parsed["status"], "Online");
    }

    #[test]
    fn test_status_endpoint() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/status").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
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
