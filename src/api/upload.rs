use std::path::Path;

use rocket::fs::NamedFile;
use rocket::serde::json::to_string;
use rocket::tokio;
use rocket::tokio::fs::File;
use rocket::{form::Form, fs::TempFile, http::Status, serde::json::Json, tokio::io::AsyncReadExt};
use rocket_apitoken::Authorized;

use crate::api::catcher::DefaultErrorResp;
use crate::audio::wav_decode;
use uuid::Uuid;

pub const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");

pub fn routes() -> Vec<rocket::Route> {
    routes![upload]
}

#[derive(Responder)]
#[response(status = 200, content_type = "audio/mpeg")]
struct ConvertedMp3(Vec<u8>);

#[derive(FromForm)]
struct Upload<'r> {
    wav: TempFile<'r>,
}

#[post("/", data = "<upload>")]
async fn upload(_auth: Authorized, mut upload: Form<Upload<'_>>) -> Option<NamedFile> {
    let id = Uuid::new_v4();
    let uploadp = Path::new(ROOT).join(id.to_string());
    upload.wav.persist_to(&uploadp).await.unwrap();

    let resultp = tokio::task::spawn_blocking(move || {
        let reader = hound::WavReader::open(&uploadp).unwrap();
        wav_decode(reader)
    })
    .await
    .unwrap()
    .ok()?;

    NamedFile::open(resultp).await.ok()
}
