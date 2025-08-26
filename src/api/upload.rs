use rocket::{form::Form, fs::TempFile, http::Status, serde::json::Json, tokio::io::AsyncReadExt};

use crate::audio::wav_decode;
use crate::auth::ApiToken;
use crate::{api::catcher::DefaultErrorResp, audio::mp3_encode};

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
async fn upload(
    _token: ApiToken,
    upload: Form<Upload<'_>>,
) -> Result<ConvertedMp3, (Status, Json<DefaultErrorResp>)> {
    let mut wav_buf = Vec::new();
    let mut wav_file = upload.wav.open().await.map_err(|_| {
        (
            Status::InternalServerError,
            Json(DefaultErrorResp {
                error: "Failed to open file".to_string(),
            }),
        )
    })?;

    wav_file.read_to_end(&mut wav_buf).await.unwrap();
    let cursor = std::io::Cursor::new(wav_buf);
    let reader = hound::WavReader::new(cursor).map_err(|e| {
        (
            Status::InternalServerError,
            Json(DefaultErrorResp {
                error: format!("Invalid WAV file: {}", e),
            }),
        )
    })?;
    let sample_rate = reader.spec().sample_rate as u32;
    let channels = reader.spec().channels as u8;

    let (left, right) = match wav_decode(reader) {
        Ok(data) => data,
        Err(e) => {
            return Err((
                Status::BadRequest,
                if cfg!(debug_assertions) {
                    Json(DefaultErrorResp {
                        error: format!("Failed to encode MP3: {:?}", e),
                    })
                } else {
                    Json(DefaultErrorResp {
                        error: "Failed to encode MP3".to_string(),
                    })
                },
            ));
        }
    };

    let encoder_result = mp3_encode(&left, &right, channels, sample_rate);
    let mp3_data = match encoder_result {
        Ok(data) => data,
        Err(e) => {
            return Err((
                Status::BadRequest,
                if cfg!(debug_assertions) {
                    Json(DefaultErrorResp {
                        error: format!("Failed to encode MP3: {:?}", e),
                    })
                } else {
                    Json(DefaultErrorResp {
                        error: "Failed to encode MP3".to_string(),
                    })
                },
            ));
        }
    };
    Ok(ConvertedMp3(mp3_data))
}
