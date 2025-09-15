use rocket::fs::NamedFile;

use rocket::tokio::fs;
use rocket::{State, tokio};

use rocket::{form::Form, fs::TempFile};
use rocket_apitoken::Authorized;

use crate::audio::wav_decode;
use crate::config::Config;
use crate::error::WaveemapiError;
use crate::helpers::{check_data_path, wav_path};

pub fn routes() -> Vec<rocket::Route> {
    routes![upload]
}

#[derive(FromForm)]
struct Upload<'r> {
    wav: TempFile<'r>,
}

#[post("/", data = "<upload>")]
async fn upload(
    _auth: Authorized,
    mut upload: Form<Upload<'_>>,
    config: &State<Config>,
) -> Result<NamedFile, WaveemapiError> {
    let data_path = config.data_path.clone();
    check_data_path(&data_path)?;
    let uploadp = wav_path(&data_path);
    upload.wav.persist_to(&uploadp).await?;
    let uploadpc = uploadp.clone();
    let resultp = tokio::task::spawn_blocking(move || {
        let reader = hound::WavReader::open(&uploadp).map_err(WaveemapiError::Hound)?;
        wav_decode(reader, &data_path)
    })
    .await?;
    fs::remove_file(&uploadpc).await?; // remove wav after mp3 encode
    let val = resultp?;
    NamedFile::open(&val).await.map_err(WaveemapiError::Io)
}

#[cfg(test)]
mod tests {

    use crate::rocket;

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
}
