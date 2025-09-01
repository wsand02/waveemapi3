use rocket::fs::NamedFile;

use rocket::tokio::fs;
use rocket::{State, tokio};

use rocket::{form::Form, fs::TempFile};
use rocket_apitoken::Authorized;

use crate::audio::wav_decode;
use crate::config::Config;
use crate::error::WaveemapiError;
use crate::helpers::wav_path;

pub fn routes() -> Vec<rocket::Route> {
    routes![upload]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routes_len() {
        let r = routes();
        assert!(!r.is_empty());
    }

    #[test]
    fn test_upload_struct_fields() {
        // Just check struct can be constructed (lifetime required, so use None for TempFile)
        fn _dummy<'r>(wav: rocket::fs::TempFile<'r>) -> Upload<'r> {
            Upload { wav }
        }
    }
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
