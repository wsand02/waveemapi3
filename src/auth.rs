use crate::error::AuthError;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use std::env;

pub struct ApiToken;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match req.headers().get_one("x-api-key") {
            Some(tt) => tt,
            None => return Outcome::Error((Status::BadRequest, AuthError::Missing)),
        };

        let env_token = match env::var("WAVEEMAPI_TOKEN") {
            Ok(val) => val,
            Err(_) => {
                return Outcome::Error((
                    Status::InternalServerError,
                    AuthError::InvalidServerSetup,
                ));
            }
        };

        if token == env_token {
            Outcome::Success(ApiToken)
        } else {
            Outcome::Error((Status::Unauthorized, AuthError::Invalid))
        }
    }
}
