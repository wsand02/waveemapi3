use std::path::Path;

use uuid::Uuid;

const MP3_EXT: &str = ".mp3";
const WAV_EXT: &str = ".wav";

pub fn wav_path(data_path: &String) -> String {
    let id = Uuid::new_v4();
    let filename = format!("{}{}", id, WAV_EXT);
    Path::new(&data_path)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

pub fn mp3_path(data_path: &String) -> String {
    let id = Uuid::new_v4();
    let filename = format!("{}{}", id, MP3_EXT);
    Path::new(&data_path)
        .join(filename)
        .to_string_lossy()
        .to_string()
}
