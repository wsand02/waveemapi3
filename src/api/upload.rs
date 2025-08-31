use std::fs;
use std::path::Path;

use rocket::fs::NamedFile;

use rocket::tokio;

use rocket::{form::Form, fs::TempFile};
use rocket_apitoken::Authorized;

use crate::audio::wav_decode;
use crate::error::ObaError;
use uuid::Uuid;

pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");

pub fn routes() -> Vec<rocket::Route> {
    routes![upload]
}

#[derive(FromForm)]
struct Upload<'r> {
    wav: TempFile<'r>,
}

#[post("/", data = "<upload>")]
async fn upload(_auth: Authorized, mut upload: Form<Upload<'_>>) -> Result<NamedFile, ObaError> {
    let id = Uuid::new_v4();
    let uploadp = Path::new(ROOT).join(id.to_string());
    upload.wav.persist_to(&uploadp).await?;
    let uploadpc = uploadp.clone();
    let resultp = tokio::task::spawn_blocking(move || {
        let reader = hound::WavReader::open(&uploadp).map_err(|e| ObaError::HoundError(e))?;
        wav_decode(reader)
    })
    .await
    .map_err(|e| ObaError::JoinError(e))?
    .map_err(|e| e);
    fs::remove_file(&uploadpc).unwrap(); // remove wav after mp3 encode
    match resultp {
        Ok(val) => NamedFile::open(&val)
            .await
            .map_err(|e| ObaError::IoError(e)),
        Err(e) => Err(e),
    }
}
